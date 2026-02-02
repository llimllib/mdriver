use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::LazyLock;

use htmlentity::entity::{decode as decode_html_entity_bytes, ICodedDataTrait};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use two_face::theme::{EmbeddedLazyThemeSet, EmbeddedThemeName};
use unicode_width::UnicodeWidthStr;

// Static theme set using two-face's extended themes
static THEME_SET: LazyLock<EmbeddedLazyThemeSet> = LazyLock::new(two_face::theme::extra);

/// Column alignment in tables
#[derive(Debug, Clone, Copy, PartialEq)]
enum Alignment {
    Left,
    Center,
    Right,
}

/// List item type
#[derive(Debug, Clone, Copy, PartialEq)]
enum ListItemType {
    Unordered,
    Ordered(usize), // Stores the original number from the markdown
}

impl ListItemType {
    /// Check if two list item types are the same kind (both ordered or both unordered),
    /// ignoring the specific number for ordered lists.
    fn same_kind(&self, other: &ListItemType) -> bool {
        matches!(
            (self, other),
            (ListItemType::Unordered, ListItemType::Unordered)
                | (ListItemType::Ordered(_), ListItemType::Ordered(_))
        )
    }
}

/// Callout type for GitHub-style alerts
#[derive(Debug, Clone, Copy, PartialEq)]
enum CalloutType {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

/// Image protocol for rendering images
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageProtocol {
    None,
    Kitty,
}

/// Image data parsed from markdown
#[derive(Debug)]
struct ImageData {
    alt: String,
    src: String,
    end_pos: usize,
}

/// Streaming markdown parser that emits formatted blocks incrementally
pub struct StreamingParser {
    buffer: String,
    state: ParserState,
    current_block: BlockBuilder,
    syntax_set: SyntaxSet,
    theme_set: &'static EmbeddedLazyThemeSet,
    theme_name: String,
    image_protocol: ImageProtocol,
    width: usize,
    /// Cache for prefetched image data (URL -> image bytes)
    image_cache: HashMap<String, Vec<u8>>,
    /// Link reference definitions: normalized_label -> (url, optional_title)
    link_definitions: HashMap<String, (String, Option<String>)>,
    /// Pending citations for bibliography: (citation_number, label, display_text)
    pending_citations: RefCell<Vec<(usize, String, String)>>,
    /// Next citation number to assign
    next_citation_number: RefCell<usize>,
}

/// Calculate the default output width: min(terminal_width, 80)
fn default_width() -> usize {
    term_size::dimensions()
        .map(|(w, _)| w.min(80))
        .unwrap_or(80)
}

#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    Ready,
    InParagraph,
    InCodeBlock {
        info: String,
        fence: String,
        indent_offset: usize,
    },
    InList,
    InListAfterBlank, // In a list but just saw a blank line
    InTable,
    InBlockquote {
        nesting_level: usize,
    },
    InIndentedCodeBlock,
}

#[derive(Debug, Clone)]
enum BlockBuilder {
    None,
    Paragraph {
        lines: Vec<String>,
    },
    CodeBlock {
        lines: Vec<String>,
        #[allow(dead_code)]
        info: String, // Language info for future syntax highlighting
    },
    List {
        items: Vec<(usize, ListItemType, String)>, // (indentation_level, type, content)
    },
    Table {
        header: Vec<String>,
        alignments: Vec<Alignment>,
        rows: Vec<Vec<String>>,
    },
    Blockquote {
        lines: Vec<(usize, String)>,
        current_nesting: usize,
    },
    IndentedCodeBlock {
        lines: Vec<String>,
    },
}

struct LinkData {
    text: String,
    url: String,
    end_pos: usize,
}

/// Result from parsing a reference-style link
struct ReferenceLinkData {
    /// The link text (what to display)
    text: String,
    /// The reference label (for lookup, not necessarily same as text)
    label: String,
    /// Position after the link syntax
    end_pos: usize,
}

/// Result from parsing an HTML tag
struct HtmlTagResult {
    formatted: String,
    end_pos: usize,
}

impl StreamingParser {
    pub fn new() -> Self {
        Self::with_theme("base16-ocean.dark", ImageProtocol::None)
    }

    /// Create a new parser with a specific syntax highlighting theme
    pub fn with_theme(theme_name: &str, image_protocol: ImageProtocol) -> Self {
        Self {
            buffer: String::new(),
            state: ParserState::Ready,
            current_block: BlockBuilder::None,
            syntax_set: two_face::syntax::extra_newlines(),
            theme_set: &THEME_SET,
            theme_name: theme_name.to_string(),
            image_protocol,
            width: default_width(),
            image_cache: HashMap::new(),
            link_definitions: HashMap::new(),
            pending_citations: RefCell::new(Vec::new()),
            next_citation_number: RefCell::new(1),
        }
    }

    /// Create a new parser with a specific width for line wrapping
    pub fn with_width(theme_name: &str, image_protocol: ImageProtocol, width: usize) -> Self {
        Self {
            buffer: String::new(),
            state: ParserState::Ready,
            current_block: BlockBuilder::None,
            syntax_set: two_face::syntax::extra_newlines(),
            theme_set: &THEME_SET,
            theme_name: theme_name.to_string(),
            image_protocol,
            width,
            image_cache: HashMap::new(),
            link_definitions: HashMap::new(),
            pending_citations: RefCell::new(Vec::new()),
            next_citation_number: RefCell::new(1),
        }
    }

    /// List available syntax highlighting themes
    pub fn list_themes() -> Vec<String> {
        // Get all theme names from two-face's embedded themes
        let mut themes: Vec<String> = EmbeddedLazyThemeSet::theme_names()
            .iter()
            .map(|name| name.as_name().to_string())
            .collect();
        themes.sort();
        themes
    }

    /// Extract all image URLs from text content.
    /// Finds both markdown images ![alt](src) and HTML <img src="..."> tags.
    fn extract_image_urls(&self, text: &str) -> Vec<String> {
        let mut urls = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check for ![alt](src) markdown images
            if chars[i] == '!' && i + 1 < chars.len() && chars[i + 1] == '[' {
                if let Some(img) = self.parse_image(&chars, i) {
                    urls.push(img.src.clone());
                    i = img.end_pos;
                    continue;
                }
            }

            // Check for <img src="..."> HTML tags
            if chars[i] == '<' {
                let remaining: String = chars[i..].iter().collect();
                let lower = remaining.to_lowercase();
                if lower.starts_with("<img ") || lower.starts_with("<img/") {
                    // Find the closing >
                    if let Some(end_offset) = remaining.find('>') {
                        let tag_content = &remaining[1..end_offset];
                        if let Some(src) = self.extract_attr(tag_content, "src") {
                            urls.push(src);
                        }
                        i += end_offset + 1;
                        continue;
                    }
                }
            }

            // Check for [text](url) links that might contain images
            if chars[i] == '[' {
                if let Some(link) = self.parse_link(&chars, i) {
                    // Recursively extract image URLs from link text
                    urls.extend(self.extract_image_urls(&link.text));
                    i = link.end_pos;
                    continue;
                }
            }

            i += 1;
        }

        urls
    }

    /// Prefetch images in parallel, storing results in the cache.
    /// Only fetches URLs that aren't already cached.
    fn prefetch_images(&mut self, urls: &[String]) {
        use std::sync::mpsc;
        use std::thread;

        // Filter to URLs we haven't cached yet
        let urls_to_fetch: Vec<String> = urls
            .iter()
            .filter(|url| !self.image_cache.contains_key(*url))
            .cloned()
            .collect();

        if urls_to_fetch.is_empty() {
            return;
        }

        // Spawn threads to download in parallel
        let (tx, rx) = mpsc::channel();

        for url in urls_to_fetch.iter().cloned() {
            let tx = tx.clone();
            thread::spawn(move || {
                let result = Self::fetch_image_static(&url);
                let _ = tx.send((url, result));
            });
        }

        // Drop the original sender so rx.iter() terminates
        drop(tx);

        // Collect results
        for (url, result) in rx {
            if let Ok(data) = result {
                self.image_cache.insert(url, data);
            }
        }
    }

    /// Static method to fetch image data (can be called from threads)
    fn fetch_image_static(src: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        if src.starts_with("http://") || src.starts_with("https://") {
            let response = ureq::get(src).call()?;
            let mut bytes = Vec::new();
            std::io::Read::read_to_end(&mut response.into_reader(), &mut bytes)?;
            Ok(bytes)
        } else {
            std::fs::read(src).map_err(|e| e.into())
        }
    }

    /// Feed a chunk of markdown to the parser
    /// Returns any completed blocks as formatted terminal output (with ANSI codes)
    pub fn feed(&mut self, chunk: &str) -> String {
        self.buffer.push_str(chunk);

        let mut output = String::new();

        // Process complete lines
        while let Some(newline_pos) = self.buffer.find('\n') {
            let line = self.buffer[..=newline_pos].to_string();
            self.buffer.drain(..=newline_pos);

            if let Some(emission) = self.process_line(&line) {
                output.push_str(&emission);
            }
        }

        output
    }

    /// Flush any remaining buffered content
    pub fn flush(&mut self) -> String {
        let mut output = String::new();

        // Process any remaining partial line
        if !self.buffer.is_empty() {
            let remaining = self.buffer.clone();
            self.buffer.clear();
            if let Some(emission) = self.process_line(&remaining) {
                output.push_str(&emission);
            }
        }

        // Emit any incomplete block
        if let Some(emission) = self.emit_current_block() {
            output.push_str(&emission);
        }

        // Emit bibliography if there are pending citations
        if let Some(bibliography) = self.format_bibliography() {
            output.push_str(&bibliography);
        }

        output
    }

    /// Format the bibliography section with pending citations
    fn format_bibliography(&self) -> Option<String> {
        let citations = self.pending_citations.borrow();
        if citations.is_empty() {
            return None;
        }

        let mut output = String::new();

        // Header with a horizontal rule and title
        output.push_str("\n\u{001b}[1;34m─── References ───\u{001b}[0m\n\n");

        for (num, label, _text) in citations.iter() {
            let normalized_label = self.normalize_link_label(label);

            // Check if we now have a definition
            if let Some((url, title)) = self.link_definitions.get(&normalized_label) {
                // Render as OSC8 hyperlink
                output.push_str(&format!(
                    "[{}] {}: \u{001b}]8;;{}\u{001b}\\\u{001b}[34;4m{}\u{001b}[0m\u{001b}]8;;\u{001b}\\",
                    num, label, url, url
                ));

                if let Some(t) = title {
                    output.push_str(&format!(" \"{}\"", t));
                }
            } else {
                // No definition found - mark as unresolved
                output.push_str(&format!(
                    "[{}] {}: \u{001b}[31m(unresolved)\u{001b}[0m",
                    num, label
                ));
            }
            output.push('\n');
        }

        output.push('\n');
        Some(output)
    }

    fn process_line(&mut self, line: &str) -> Option<String> {
        match &self.state {
            ParserState::Ready => self.handle_ready_state(line),
            ParserState::InParagraph => self.handle_in_paragraph(line),
            ParserState::InCodeBlock { .. } => self.handle_in_code_block(line),
            ParserState::InList => self.handle_in_list(line),
            ParserState::InListAfterBlank => self.handle_in_list_after_blank(line),
            ParserState::InTable => self.handle_in_table(line),
            ParserState::InBlockquote { .. } => self.handle_in_blockquote(line),
            ParserState::InIndentedCodeBlock => self.handle_in_indented_code_block(line),
        }
    }

    fn handle_ready_state(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Check for blank line
        if trimmed.is_empty() {
            return None;
        }

        // Check for HTML comment line (<!-- ... -->)
        // These should be silently skipped
        if self.is_html_comment_line(trimmed) {
            return None;
        }

        // Check for ATX heading (# )
        if let Some(level) = self.parse_atx_heading(trimmed) {
            let text = trimmed[level..].trim_start().to_string();
            // Headings complete on the same line - emit immediately
            return Some(self.format_heading(level, &text));
        }

        // Check for code fence (```)
        if let Some((info, fence, indent_offset)) = self.parse_code_fence(trimmed) {
            self.state = ParserState::InCodeBlock {
                info: info.clone(),
                fence: fence.clone(),
                indent_offset,
            };
            self.current_block = BlockBuilder::CodeBlock {
                lines: Vec::new(),
                info,
            };
            return None;
        }

        // Check for blockquote
        if let Some(nesting_level) = self.parse_blockquote_marker(trimmed) {
            let content = self.strip_blockquote_markers(trimmed, nesting_level);
            self.state = ParserState::InBlockquote { nesting_level };
            self.current_block = BlockBuilder::Blockquote {
                lines: vec![(nesting_level, content)],
                current_nesting: nesting_level,
            };
            return None;
        }

        // Check for horizontal rule (thematic break)
        // Must be checked before list items per GFM spec
        if self.is_horizontal_rule(trimmed) {
            return Some(self.format_horizontal_rule());
        }

        // Check for list item (- or digit.)
        if let Some((indent, item_type)) = self.parse_list_item(trimmed) {
            self.state = ParserState::InList;
            self.current_block = BlockBuilder::List {
                items: vec![(indent, item_type, trimmed.to_string())],
            };
            return None;
        }

        // Check for link reference definition [label]: url "title"
        // These are stored but never emit content
        if let Some((label, url, title)) = self.parse_link_definition(trimmed) {
            let normalized_label = self.normalize_link_label(&label);
            // First definition wins (don't overwrite)
            self.link_definitions
                .entry(normalized_label)
                .or_insert((url, title));
            return None;
        }

        // Check for indented code block (4+ spaces or tab)
        // Must come after list check (lists take precedence)
        if let Some(content) = self.is_indented_code_line(trimmed) {
            self.state = ParserState::InIndentedCodeBlock;
            self.current_block = BlockBuilder::IndentedCodeBlock {
                lines: vec![content.to_string()],
            };
            return None;
        }

        // Otherwise, start a paragraph
        self.state = ParserState::InParagraph;
        self.current_block = BlockBuilder::Paragraph {
            lines: vec![trimmed.to_string()],
        };
        None
    }

    fn handle_in_paragraph(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Blank line completes paragraph
        if trimmed.is_empty() {
            return self.emit_current_block();
        }

        // Check if this is a setext heading underline
        if let Some(level) = self.parse_setext_underline(trimmed) {
            if let BlockBuilder::Paragraph { lines } = &self.current_block {
                // Join all lines to form the heading text
                let text = lines.join(" ");
                self.state = ParserState::Ready;
                self.current_block = BlockBuilder::None;
                return Some(self.format_heading(level, &text));
            }
        }

        // Check if this might be a table delimiter row
        if let BlockBuilder::Paragraph { lines } = &self.current_block {
            if lines.len() == 1 && self.is_table_delimiter_row(trimmed) {
                // Extract header cells from first line
                let header = self.parse_table_row(&lines[0]);
                let alignments = self.parse_alignments(trimmed);

                // Promote to table
                self.current_block = BlockBuilder::Table {
                    header,
                    alignments,
                    rows: Vec::new(),
                };
                self.state = ParserState::InTable;
                return None; // No emission yet
            }
        }

        // Add line to paragraph
        if let BlockBuilder::Paragraph { lines } = &mut self.current_block {
            lines.push(trimmed.to_string());
        }
        None
    }

    fn handle_in_code_block(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Check if this is the closing fence
        if let ParserState::InCodeBlock {
            fence,
            indent_offset,
            ..
        } = &self.state
        {
            // Closing fence can have 0-3 spaces of indentation (normal case)
            // or 4+ spaces (when inside a list item)
            let line_trimmed = trimmed.trim_start();

            // Check if this line is just the fence (possibly with trailing spaces)
            if line_trimmed.starts_with(fence) && line_trimmed.trim() == fence.trim() {
                // Closing fence - emit the block
                return self.emit_current_block();
            }

            // Add line to code block, stripping the indent offset
            if let BlockBuilder::CodeBlock { lines, .. } = &mut self.current_block {
                // Strip indent_offset spaces from the beginning if present
                let line_to_add = if *indent_offset > 0 && trimmed.len() >= *indent_offset {
                    &trimmed[*indent_offset..]
                } else {
                    trimmed
                };
                lines.push(line_to_add.to_string());
            }
        }

        None
    }

    fn handle_in_list(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Blank lines can appear within multi-paragraph list items
        // Transition to InListAfterBlank to check if list continues
        if trimmed.is_empty() {
            self.state = ParserState::InListAfterBlank;
            return None;
        }

        // Check for horizontal rule (takes precedence over list items per GFM spec)
        if self.is_horizontal_rule(trimmed) {
            let emission = self.emit_current_block();
            let hr = self.format_horizontal_rule();
            return match emission {
                Some(e) => Some(format!("{}{}", e, hr)),
                None => Some(hr),
            };
        }

        // Check if it's another list item
        if let Some((indent, item_type)) = self.parse_list_item(trimmed) {
            if let BlockBuilder::List { items } = &mut self.current_block {
                items.push((indent, item_type, trimmed.to_string()));
            }
            return None;
        }

        // Check if this is indented content (4+ spaces) - could be list continuation or code fence
        let leading_spaces = trimmed.len() - trimmed.trim_start().len();
        if leading_spaces >= 4 {
            let after_indent = &trimmed[4..];
            if let Some((info, fence, fence_indent)) = self.parse_code_fence(after_indent) {
                // This is a code fence inside the list - emit the list first
                let emission = self.emit_current_block();
                // Then transition to code block state with 4-space + fence indent offset
                self.state = ParserState::InCodeBlock {
                    info: info.clone(),
                    fence: fence.clone(),
                    indent_offset: 4 + fence_indent,
                };
                self.current_block = BlockBuilder::CodeBlock {
                    lines: Vec::new(),
                    info,
                };
                return emission;
            } else {
                // Indented content that's not a code fence - it's list continuation
                // Append to the current list item (preserving a space for line joining)
                if let BlockBuilder::List { items } = &mut self.current_block {
                    if let Some((_, _, content)) = items.last_mut() {
                        // Append this line's content, trimming excess leading whitespace
                        // Use a single space to join, which allows multi-line markdown
                        // constructs like links to be parsed correctly
                        let continuation = after_indent.trim_start();
                        if !continuation.is_empty() {
                            content.push(' ');
                            content.push_str(continuation);
                        }
                    }
                }
                return None;
            }
        }

        // Not a list item and not blank - list ends, but we need to process this line
        // For now, emit the list and start over
        let emission = self.emit_current_block();
        // Process the new line as if we're in Ready state
        let new_emission = self.handle_ready_state(line);

        match (emission, new_emission) {
            (Some(e1), Some(e2)) => Some(format!("{}{}", e1, e2)),
            (Some(e), None) | (None, Some(e)) => Some(e),
            (None, None) => None,
        }
    }

    fn handle_in_list_after_blank(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Check if it's another list item
        if let Some((_, new_type)) = self.parse_list_item(trimmed) {
            // Check if it's the same type as the current list
            if let BlockBuilder::List { items } = &self.current_block {
                if !items.is_empty() {
                    let (_, current_type, _) = &items[0];
                    // If same kind (both ordered or both unordered), continue the list
                    if current_type.same_kind(&new_type) {
                        self.state = ParserState::InList;
                        return self.handle_in_list(line);
                    }
                }
            }
            // Different type - emit current list and start new one
            let emission = self.emit_current_block();
            let new_emission = self.handle_ready_state(line);
            return match (emission, new_emission) {
                (Some(e1), Some(e2)) => Some(format!("{}{}", e1, e2)),
                (Some(e), None) | (None, Some(e)) => Some(e),
                (None, None) => None,
            };
        }

        // Check if it's indented content (4+ spaces) after a blank line
        // This could be a fenced code block or an indented code block
        let leading_spaces = trimmed.len() - trimmed.trim_start().len();
        if leading_spaces >= 4 && !trimmed.is_empty() {
            let after_indent = &trimmed[4..];

            // Check for fenced code block first
            if let Some((info, fence, fence_indent)) = self.parse_code_fence(after_indent) {
                // Emit the list, then start code block
                let emission = self.emit_current_block();
                self.state = ParserState::InCodeBlock {
                    info: info.clone(),
                    fence: fence.clone(),
                    indent_offset: 4 + fence_indent,
                };
                self.current_block = BlockBuilder::CodeBlock {
                    lines: Vec::new(),
                    info,
                };
                return emission;
            }

            // After a blank line, indented content is an indented code block
            // Emit the list first, then start the indented code block
            let emission = self.emit_current_block();
            self.state = ParserState::InIndentedCodeBlock;
            self.current_block = BlockBuilder::IndentedCodeBlock {
                lines: vec![after_indent.to_string()],
            };
            return emission;
        }

        // Otherwise (blank line, non-indented content, or anything else), emit the list
        let emission = self.emit_current_block();

        // If this line is not blank, process it in Ready state
        if !trimmed.is_empty() {
            let new_emission = self.handle_ready_state(line);
            match (emission, new_emission) {
                (Some(e1), Some(e2)) => Some(format!("{}{}", e1, e2)),
                (Some(e), None) | (None, Some(e)) => Some(e),
                (None, None) => None,
            }
        } else {
            emission
        }
    }

    fn handle_in_table(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Blank line ends table
        if trimmed.is_empty() {
            return self.emit_current_block();
        }

        // Check if line looks like a table row (contains |)
        if !trimmed.contains('|') {
            // Not a table row - emit table and start new block
            let emission = self.emit_current_block();
            let new_emission = self.handle_ready_state(line);

            match (emission, new_emission) {
                (Some(e1), Some(e2)) => Some(format!("{}{}", e1, e2)),
                (Some(e), None) | (None, Some(e)) => Some(e),
                (None, None) => None,
            }
        } else {
            // Parse and accumulate data row
            let cells = self.parse_table_row(trimmed);
            if let BlockBuilder::Table { rows, .. } = &mut self.current_block {
                rows.push(cells);
            }
            None
        }
    }

    fn handle_in_blockquote(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Blank line terminates
        if trimmed.is_empty() {
            return self.emit_current_block();
        }

        // Check if line has blockquote marker
        if let Some(nesting_level) = self.parse_blockquote_marker(trimmed) {
            let content = self.strip_blockquote_markers(trimmed, nesting_level);

            if let BlockBuilder::Blockquote {
                lines,
                current_nesting,
            } = &mut self.current_block
            {
                // Update state nesting
                if let ParserState::InBlockquote {
                    nesting_level: ref mut state_nesting,
                } = &mut self.state
                {
                    *state_nesting = nesting_level;
                }

                lines.push((nesting_level, content));
                *current_nesting = nesting_level;
            }
            return None;
        }

        // Lazy continuation: line without '>' continues at current nesting
        if let BlockBuilder::Blockquote {
            lines,
            current_nesting,
        } = &mut self.current_block
        {
            lines.push((*current_nesting, trimmed.to_string()));
            return None;
        }

        None
    }

    fn handle_in_indented_code_block(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Blank line - preserve in code block
        if trimmed.is_empty() {
            if let BlockBuilder::IndentedCodeBlock { lines } = &mut self.current_block {
                lines.push(String::new());
            }
            return None;
        }

        // Still indented? Continue accumulating
        if let Some(content) = self.is_indented_code_line(trimmed) {
            if let BlockBuilder::IndentedCodeBlock { lines } = &mut self.current_block {
                lines.push(content.to_string());
            }
            return None;
        }

        // Not indented - emit block and process line in ready state
        let output = self.emit_current_block();
        let next_output = self.handle_ready_state(line);

        match (output, next_output) {
            (Some(o1), Some(o2)) => Some(format!("{}{}", o1, o2)),
            (Some(o), None) | (None, Some(o)) => Some(o),
            (None, None) => None,
        }
    }

    fn parse_atx_heading(&self, line: &str) -> Option<usize> {
        let mut level = 0;
        for ch in line.chars() {
            if ch == '#' {
                level += 1;
                if level > 6 {
                    return None;
                }
            } else if ch == ' ' && level > 0 {
                return Some(level);
            } else {
                return None;
            }
        }
        None
    }

    fn parse_setext_underline(&self, line: &str) -> Option<usize> {
        // Setext underline: 0-3 spaces, then sequence of = or -, with trailing spaces allowed
        let leading_spaces = line.len() - line.trim_start().len();

        if leading_spaces > 3 {
            return None;
        }

        let trimmed = line.trim();

        // Check for all = (level 1)
        if !trimmed.is_empty() && trimmed.chars().all(|c| c == '=') {
            return Some(1);
        }

        // Check for all - (level 2)
        if !trimmed.is_empty() && trimmed.chars().all(|c| c == '-') {
            return Some(2);
        }

        None
    }

    /// Check if a line is entirely an HTML comment (<!-- ... -->)
    fn is_html_comment_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        if !trimmed.starts_with("<!--") {
            return false;
        }
        if !trimmed.ends_with("-->") {
            return false;
        }
        // Ensure the comment is properly formed (has content or is empty)
        // and doesn't have an early --> before the final one
        let inner = &trimmed[4..trimmed.len() - 3];
        // Make sure there's no --> in the middle (which would mean malformed)
        !inner.contains("-->")
    }

    /// Check if a line is indented code (4+ spaces or tab), return content with indentation stripped
    fn is_indented_code_line<'a>(&self, line: &'a str) -> Option<&'a str> {
        line.strip_prefix("    ")
            .or_else(|| line.strip_prefix('\t'))
    }

    fn is_horizontal_rule(&self, line: &str) -> bool {
        // Horizontal rule: 0-3 spaces, then 3+ matching -, _, or * chars
        // with optional spaces/tabs between them
        let leading_spaces = line.len() - line.trim_start().len();

        if leading_spaces > 3 {
            return false;
        }

        let trimmed = line.trim();

        // Count matching characters
        let mut rule_char: Option<char> = None;
        let mut count = 0;

        for ch in trimmed.chars() {
            match ch {
                '-' | '_' | '*' => {
                    if let Some(rc) = rule_char {
                        if rc != ch {
                            return false; // Mixed characters
                        }
                    } else {
                        rule_char = Some(ch);
                    }
                    count += 1;
                }
                ' ' | '\t' => {
                    // Spaces/tabs allowed between rule characters
                    continue;
                }
                _ => {
                    // Any other character invalidates the rule
                    return false;
                }
            }
        }

        // Must have at least 3 matching characters
        count >= 3 && rule_char.is_some()
    }

    fn parse_code_fence(&self, line: &str) -> Option<(String, String, usize)> {
        // Code fences can have 0-3 spaces of indentation
        let leading_spaces = line.len() - line.trim_start().len();

        if leading_spaces > 3 {
            return None;
        }

        let trimmed = line.trim_start();

        if let Some(rest) = trimmed.strip_prefix("```") {
            let fence = "```".to_string();
            let info = rest.trim().to_string();
            Some((info, fence, leading_spaces))
        } else if let Some(rest) = trimmed.strip_prefix("~~~") {
            let fence = "~~~".to_string();
            let info = rest.trim().to_string();
            Some((info, fence, leading_spaces))
        } else {
            None
        }
    }

    /// Normalize a link label for matching per GFM spec:
    /// - Strip leading/trailing whitespace
    /// - Collapse internal whitespace to single space
    /// - Unicode case fold (lowercase for ASCII)
    fn normalize_link_label(&self, label: &str) -> String {
        label
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase()
    }

    /// Try to parse a link reference definition from a line.
    /// Returns Some((label, url, optional_title)) if successful.
    /// Link definition format: [label]: url "optional title"
    fn parse_link_definition(&self, line: &str) -> Option<(String, String, Option<String>)> {
        let trimmed = line.trim_end_matches('\n');

        // Check indentation (0-3 spaces allowed)
        let leading_spaces = trimmed.len() - trimmed.trim_start().len();
        if leading_spaces > 3 {
            return None;
        }

        let trimmed = trimmed.trim_start();

        // Must start with [
        if !trimmed.starts_with('[') {
            return None;
        }

        // Find the closing ] for the label - handle escaped brackets
        let mut label_end = None;
        let mut in_escape = false;
        for (i, ch) in trimmed[1..].char_indices() {
            if in_escape {
                in_escape = false;
                continue;
            }
            if ch == '\\' {
                in_escape = true;
                continue;
            }
            if ch == ']' {
                label_end = Some(i);
                break;
            }
        }

        let label_end = label_end?;
        let label = &trimmed[1..label_end + 1];

        // Label must have at least one non-whitespace character
        if label.trim().is_empty() {
            return None;
        }

        // Must be followed by :
        let after_label = &trimmed[label_end + 2..];
        if !after_label.starts_with(':') {
            return None;
        }

        // Parse URL (after : and optional whitespace)
        let url_part = after_label[1..].trim_start();

        // Empty URL part means this isn't a valid definition
        if url_part.is_empty() {
            return None;
        }

        // URL can be:
        // 1. Angle-bracketed URL: <url>
        // 2. Bare URL (no spaces, ends at whitespace or end of line)
        let (url, remaining) = if let Some(stripped) = url_part.strip_prefix('<') {
            // Angle-bracketed URL
            if let Some(end) = stripped.find('>') {
                (&stripped[..end], &stripped[end + 1..])
            } else {
                return None; // Unclosed angle bracket
            }
        } else {
            // Bare URL - take until whitespace
            let end = url_part.find(char::is_whitespace).unwrap_or(url_part.len());
            (&url_part[..end], &url_part[end..])
        };

        // Parse optional title
        let remaining = remaining.trim_start();
        let title = if remaining.is_empty() {
            None
        } else if let Some(stripped) = remaining.strip_prefix('"') {
            stripped.find('"').map(|end| stripped[..end].to_string())
        } else if let Some(stripped) = remaining.strip_prefix('\'') {
            stripped.find('\'').map(|end| stripped[..end].to_string())
        } else if let Some(stripped) = remaining.strip_prefix('(') {
            stripped.find(')').map(|end| stripped[..end].to_string())
        } else {
            None
        };

        Some((label.to_string(), url.to_string(), title))
    }

    fn parse_list_item(&self, line: &str) -> Option<(usize, ListItemType)> {
        // GFM: 0-3 spaces for top-level, but nested lists can have more indentation
        // For simplicity, we allow up to 12 spaces (3 levels of nesting at 4 spaces each)
        let leading_spaces = line.len() - line.trim_start().len();

        if leading_spaces > 12 {
            return None;
        }

        let trimmed = line.trim_start();

        // Check for bullet list markers: -, +, *
        for marker in ['-', '+', '*'] {
            if trimmed.starts_with(marker) {
                let rest = &trimmed[1..];
                // Must be followed by 1-4 spaces
                let spaces = rest.len() - rest.trim_start().len();
                if (1..=4).contains(&spaces) && !rest.trim_start().is_empty() {
                    return Some((leading_spaces, ListItemType::Unordered));
                }
            }
        }

        // Check for ordered list: digit(s) followed by "." and 1-4 spaces
        if let Some(dot_pos) = trimmed.find('.') {
            if dot_pos > 0 && dot_pos < trimmed.len() - 1 {
                let before_dot = &trimmed[..dot_pos];
                let after_dot = &trimmed[dot_pos + 1..];
                let spaces = after_dot.len() - after_dot.trim_start().len();
                if before_dot.chars().all(|c| c.is_ascii_digit())
                    && (1..=4).contains(&spaces)
                    && !after_dot.trim_start().is_empty()
                {
                    // Parse the original number from the markdown
                    let num = before_dot.parse::<usize>().unwrap_or(1);
                    return Some((leading_spaces, ListItemType::Ordered(num)));
                }
            }
        }

        None
    }

    fn parse_blockquote_marker(&self, line: &str) -> Option<usize> {
        // GFM: 0-3 spaces, then one or more '>', each optionally followed by space
        let trimmed = line.trim_start();
        let leading_spaces = line.len() - trimmed.len();

        if leading_spaces > 3 {
            return None;
        }

        let mut nesting = 0;
        let mut chars = trimmed.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if ch == '>' {
                nesting += 1;
                chars.next();
                if chars.peek() == Some(&' ') {
                    chars.next();
                }
            } else {
                break;
            }
        }

        if nesting > 0 {
            Some(nesting)
        } else {
            None
        }
    }

    fn strip_blockquote_markers(&self, line: &str, expected_nesting: usize) -> String {
        let mut remaining = line.trim_start();
        let mut removed = 0;

        while removed < expected_nesting {
            if let Some(rest) = remaining.strip_prefix('>') {
                remaining = rest;
                removed += 1;
                if let Some(rest) = remaining.strip_prefix(' ') {
                    remaining = rest;
                }
            } else {
                break;
            }
        }

        remaining.to_string()
    }

    fn emit_current_block(&mut self) -> Option<String> {
        let block = std::mem::replace(&mut self.current_block, BlockBuilder::None);
        self.state = ParserState::Ready;

        // If images are enabled, prefetch all images in the block in parallel
        if self.image_protocol != ImageProtocol::None {
            let block_text = self.extract_block_text(&block);
            let urls = self.extract_image_urls(&block_text);
            if !urls.is_empty() {
                self.prefetch_images(&urls);
            }
        }

        match block {
            BlockBuilder::None => None,
            BlockBuilder::Paragraph { lines } => Some(self.format_paragraph(&lines)),
            BlockBuilder::CodeBlock { lines, info } => Some(self.format_code_block(&lines, &info)),
            BlockBuilder::List { items } => Some(self.format_list(&items)),
            BlockBuilder::Table {
                header,
                alignments,
                rows,
            } => Some(self.format_table(&header, &alignments, &rows)),
            BlockBuilder::Blockquote { lines, .. } => Some(self.format_blockquote(&lines)),
            BlockBuilder::IndentedCodeBlock { lines } => {
                let mut lines = lines.clone();
                // Strip trailing blank lines
                while lines.last().is_some_and(|l| l.is_empty()) {
                    lines.pop();
                }
                Some(self.format_code_block(&lines, ""))
            }
        }
    }

    /// Extract all text content from a block for image URL scanning
    fn extract_block_text(&self, block: &BlockBuilder) -> String {
        match block {
            BlockBuilder::None => String::new(),
            BlockBuilder::Paragraph { lines } => lines.join("\n"),
            BlockBuilder::CodeBlock { .. } => String::new(), // Code blocks don't have images
            BlockBuilder::List { items } => items
                .iter()
                .map(|(_, _, s)| s.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
            BlockBuilder::Table { header, rows, .. } => {
                let mut text = header.join("\n");
                for row in rows {
                    text.push('\n');
                    text.push_str(&row.join("\n"));
                }
                text
            }
            BlockBuilder::Blockquote { lines, .. } => lines
                .iter()
                .map(|(_, s)| s.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
            BlockBuilder::IndentedCodeBlock { .. } => String::new(), // Code blocks don't have images
        }
    }

    fn format_heading(&self, level: usize, text: &str) -> String {
        let formatted_text = self.format_inline(text);
        // Heading: blue and bold, with line break after for spacing
        // Replace any ANSI reset codes within the formatted text to restore heading style
        // This prevents inline formatting (like _italic_) from breaking the heading color
        let heading_style = "\u{001b}[1;34m";
        let formatted_text =
            formatted_text.replace("\u{001b}[0m", &format!("\u{001b}[0m{}", heading_style));
        format!(
            "{}{} {}\u{001b}[0m\n\n",
            heading_style,
            "#".repeat(level),
            formatted_text
        )
    }

    fn format_horizontal_rule(&self) -> String {
        // Use a line of dashes with dim/gray color
        let rule = "─".repeat(self.width);
        format!("\u{001b}[2m{}\u{001b}[0m\n\n", rule)
    }

    fn format_paragraph(&self, lines: &[String]) -> String {
        let mut result = String::new();

        for (i, line) in lines.iter().enumerate() {
            // Check for hard line break: 2+ trailing spaces or trailing backslash
            let has_hard_break = line.ends_with("  ")
                || line.ends_with("   ")
                || line.ends_with("    ")
                || line.ends_with('\\');

            // Remove trailing spaces/backslash for formatting
            let trimmed = if line.ends_with('\\') {
                &line[..line.len() - 1]
            } else {
                line.trim_end()
            };

            result.push_str(trimmed);

            // Add line break or space depending on hard break
            if has_hard_break && i < lines.len() - 1 {
                result.push('\n');
            } else if i < lines.len() - 1 {
                result.push(' ');
            }
        }

        // Apply inline formatting first, then wrap
        // Handle hard line breaks by wrapping each segment independently
        let formatted_text = self.format_inline(&result);
        let mut wrapped_segments: Vec<String> = Vec::new();

        for segment in formatted_text.split('\n') {
            wrapped_segments.push(self.wrap_text(segment, "", ""));
        }

        format!("{}\n\n", wrapped_segments.join("\n"))
    }

    /// Convert a theme name string to an EmbeddedThemeName enum variant
    fn theme_name_to_enum(name: &str) -> Option<EmbeddedThemeName> {
        // Find matching theme by comparing string names
        EmbeddedLazyThemeSet::theme_names()
            .iter()
            .find(|theme| theme.as_name().eq_ignore_ascii_case(name))
            .copied()
    }

    fn format_code_block(&self, lines: &[String], info: &str) -> String {
        let mut output = String::new();

        // Map common aliases to their syntect language names
        let language = match info.to_lowercase().as_str() {
            "jsx" => "javascript",
            "tsx" => "typescript",
            _ => info,
        };

        // Try to find syntax definition for the language
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        // Get theme from two-face's embedded themes, with fallback
        let theme = Self::theme_name_to_enum(&self.theme_name)
            .map(|name| self.theme_set.get(name))
            .unwrap_or_else(|| self.theme_set.get(EmbeddedThemeName::Base16OceanDark));

        let mut highlighter = HighlightLines::new(syntax, theme);

        // Process lines and collect highlighted output
        let mut highlighted_lines = Vec::new();
        for line in lines {
            // Add newline for proper syntax highlighting state management
            let line_with_newline = format!("{}\n", line);
            let ranges = highlighter
                .highlight_line(&line_with_newline, &self.syntax_set)
                .unwrap_or_default();
            let highlighted = as_24_bit_terminal_escaped(&ranges[..], false);
            // Remove the trailing newline from highlighted output
            let highlighted = highlighted.trim_end_matches('\n').to_string();
            highlighted_lines.push(highlighted);
        }

        // Each line: 4 space indent + highlighted content (no background)
        for highlighted in highlighted_lines.iter() {
            output.push_str("    ");
            output.push_str(highlighted);
            output.push('\n');
        }

        // Reset ANSI codes to prevent color bleeding
        output.push_str("\u{001b}[0m");

        // Add blank line after code block for spacing
        output.push('\n');
        output
    }

    /// Parse a task list item marker at the start of content.
    /// Returns Some((is_checked, remaining_content)) if a task list marker is found.
    /// Task list markers are: [ ] (unchecked) or [x]/[X] (checked), followed by whitespace.
    fn parse_task_list_marker<'a>(&self, content: &'a str) -> Option<(bool, &'a str)> {
        let trimmed = content.trim_start();

        // Check for [ ] (unchecked)
        if let Some(rest) = trimmed.strip_prefix("[ ]") {
            // Must be followed by at least one whitespace
            if rest.starts_with(' ') || rest.starts_with('\t') {
                return Some((false, rest.trim_start()));
            }
        }

        // Check for [x] or [X] (checked)
        if let Some(rest) = trimmed
            .strip_prefix("[x]")
            .or_else(|| trimmed.strip_prefix("[X]"))
        {
            // Must be followed by at least one whitespace
            if rest.starts_with(' ') || rest.starts_with('\t') {
                return Some((true, rest.trim_start()));
            }
        }

        None
    }

    fn format_list(&self, items: &[(usize, ListItemType, String)]) -> String {
        let mut output = String::new();

        for (indent_level, item_type, item) in items {
            let trimmed = item.trim_start();

            // Extract the content after the marker
            let content = if let Some(rest) = trimmed.strip_prefix("- ") {
                rest
            } else if let Some(rest) = trimmed.strip_prefix("+ ") {
                rest
            } else if let Some(rest) = trimmed.strip_prefix("* ") {
                rest
            } else if let Some(dot_pos) = trimmed.find(". ") {
                &trimmed[dot_pos + 2..]
            } else {
                trimmed
            };

            // Use indentation level to determine nesting (each 4 spaces = 1 level)
            let nesting_level = indent_level / 4;
            let indent = "  ".repeat(nesting_level);

            // Check for task list item marker and handle specially
            if let Some((is_checked, remaining)) = self.parse_task_list_marker(content) {
                // Task list item: use checkbox as the marker (no bullet)
                let checkbox = if is_checked { "☑" } else { "☐" };
                let formatted_content = self.format_inline(remaining);
                let first_indent = format!("{}  {} ", indent, checkbox);
                let cont_indent = format!("{}    ", indent); // align with content after checkbox
                let wrapped = self.wrap_text(&formatted_content, &first_indent, &cont_indent);
                output.push_str(&wrapped);
                output.push('\n');
                continue;
            }

            let formatted_content = self.format_inline(content);

            // Format based on item type, with wrapping
            match item_type {
                ListItemType::Unordered => {
                    let first_indent = format!("{}  • ", indent);
                    let cont_indent = format!("{}    ", indent); // align with content after bullet
                    let wrapped = self.wrap_text(&formatted_content, &first_indent, &cont_indent);
                    output.push_str(&wrapped);
                    output.push('\n');
                }
                ListItemType::Ordered(num) => {
                    // Use the original number from the markdown
                    let first_indent = format!("{}  {}. ", indent, num);
                    // Continuation indent aligns with content (after "N. ")
                    let cont_indent =
                        format!("{}  {}  ", indent, " ".repeat(num.to_string().len()));
                    let wrapped = self.wrap_text(&formatted_content, &first_indent, &cont_indent);
                    output.push_str(&wrapped);
                    output.push('\n');
                }
            }
        }
        // Add blank line after list for spacing
        output.push('\n');
        output
    }

    // Table parsing and formatting functions

    fn is_table_delimiter_row(&self, line: &str) -> bool {
        // Must contain pipes
        if !line.contains('|') {
            return false;
        }

        // Split by pipes, check each cell
        let cells: Vec<&str> = line
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if cells.is_empty() {
            return false;
        }

        // Each cell must match pattern: optional :, at least 3 dashes, optional :
        for cell in cells {
            let chars: Vec<char> = cell.chars().collect();
            if chars.is_empty() {
                return false;
            }

            let starts_colon = chars[0] == ':';
            let ends_colon = chars[chars.len() - 1] == ':';

            let dash_section = if starts_colon && ends_colon {
                &chars[1..chars.len() - 1]
            } else if starts_colon {
                &chars[1..]
            } else if ends_colon {
                &chars[..chars.len() - 1]
            } else {
                &chars[..]
            };

            // Must have at least 3 dashes
            if dash_section.len() < 3 || !dash_section.iter().all(|&c| c == '-') {
                return false;
            }
        }

        true
    }

    fn parse_table_row(&self, line: &str) -> Vec<String> {
        let mut cells = Vec::new();
        let mut current_cell = String::new();
        let mut escaped = false;

        for ch in line.chars() {
            if escaped {
                current_cell.push(ch);
                escaped = false;
                continue;
            }

            if ch == '\\' {
                escaped = true;
                continue;
            }

            if ch == '|' {
                cells.push(current_cell.trim().to_string());
                current_cell.clear();
            } else {
                current_cell.push(ch);
            }
        }

        // Add last cell if not empty
        if !current_cell.trim().is_empty() {
            cells.push(current_cell.trim().to_string());
        }

        // Filter out empty leading/trailing cells (from leading/trailing pipes)
        if !cells.is_empty() && cells[0].is_empty() {
            cells.remove(0);
        }
        if !cells.is_empty() && cells[cells.len() - 1].is_empty() {
            cells.pop();
        }

        cells
    }

    fn parse_alignments(&self, delimiter_row: &str) -> Vec<Alignment> {
        let cells = self.parse_table_row(delimiter_row);

        cells
            .iter()
            .map(|cell| {
                let trimmed = cell.trim();
                let starts_colon = trimmed.starts_with(':');
                let ends_colon = trimmed.ends_with(':');

                match (starts_colon, ends_colon) {
                    (true, true) => Alignment::Center,
                    (false, true) => Alignment::Right,
                    _ => Alignment::Left,
                }
            })
            .collect()
    }

    /// Strip ANSI escape sequences from text for width calculation.
    /// Handles both SGR sequences (\x1b[...m) and OSC8 hyperlinks (\x1b]8;;...\x1b\\).
    pub fn strip_ansi(&self, text: &str) -> String {
        // Strip ANSI escape sequences for width calculation
        // Handles both SGR sequences (\x1b[...m) and OSC8 hyperlinks (\x1b]8;;...\x1b\\)
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '\x1b' {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '[' => {
                            // SGR sequence: \x1b[...m - skip until 'm'
                            i += 2;
                            while i < chars.len() && chars[i] != 'm' {
                                i += 1;
                            }
                            if i < chars.len() {
                                i += 1; // skip the 'm'
                            }
                        }
                        ']' => {
                            // OSC sequence: \x1b]...ST where ST is \x1b\\ or BEL
                            // Used for OSC8 hyperlinks: \x1b]8;;URL\x1b\\
                            i += 2;
                            while i < chars.len() {
                                if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '\\'
                                {
                                    i += 2; // skip \x1b\\
                                    break;
                                } else if chars[i] == '\x07' {
                                    // BEL is also a valid string terminator
                                    i += 1;
                                    break;
                                }
                                i += 1;
                            }
                        }
                        '_' => {
                            // APC sequence: \x1b_...ST where ST is \x1b\\
                            // Used for Kitty graphics protocol: \x1b_Gf=100,...;base64data\x1b\\
                            i += 2;
                            while i < chars.len() {
                                if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '\\'
                                {
                                    i += 2; // skip \x1b\\
                                    break;
                                }
                                i += 1;
                            }
                        }
                        _ => {
                            // Unknown escape sequence, skip the ESC and next char
                            i += 2;
                        }
                    }
                } else {
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }

        result
    }

    /// Calculate the display width of text after stripping ANSI codes.
    /// Uses Unicode width to properly handle wide characters (like emoji and CJK).
    pub fn display_width(&self, text: &str) -> usize {
        self.strip_ansi(text).width()
    }

    /// Wrap text to self.width, preserving ANSI codes and not breaking words.
    /// `first_indent` is prepended to the first line, `cont_indent` to continuation lines.
    /// Long words that exceed width are kept whole on their own line.
    pub fn wrap_text(&self, text: &str, first_indent: &str, cont_indent: &str) -> String {
        let first_indent_width = first_indent.width();
        let cont_indent_width = cont_indent.width();

        // Split text into "tokens" preserving ANSI codes with adjacent words
        // We need to split on whitespace while preserving the ANSI codes
        // Handles both SGR sequences (\x1b[...m) and OSC8 hyperlinks (\x1b]8;;...\x1b\\)
        let mut tokens: Vec<String> = Vec::new();
        let mut current_token = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '\x1b' {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '[' => {
                            // SGR sequence: \x1b[...m - keep until 'm'
                            current_token.push(chars[i]);
                            current_token.push(chars[i + 1]);
                            i += 2;
                            while i < chars.len() && chars[i] != 'm' {
                                current_token.push(chars[i]);
                                i += 1;
                            }
                            if i < chars.len() {
                                current_token.push(chars[i]); // the 'm'
                                i += 1;
                            }
                        }
                        ']' => {
                            // OSC sequence: \x1b]...ST where ST is \x1b\\ or BEL
                            // Used for OSC8 hyperlinks: \x1b]8;;URL\x1b\\
                            current_token.push(chars[i]);
                            current_token.push(chars[i + 1]);
                            i += 2;
                            while i < chars.len() {
                                if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '\\'
                                {
                                    current_token.push(chars[i]);
                                    current_token.push(chars[i + 1]);
                                    i += 2;
                                    break;
                                } else if chars[i] == '\x07' {
                                    // BEL is also a valid string terminator
                                    current_token.push(chars[i]);
                                    i += 1;
                                    break;
                                }
                                current_token.push(chars[i]);
                                i += 1;
                            }
                        }
                        '_' => {
                            // APC sequence: \x1b_...ST where ST is \x1b\\
                            // Used for Kitty graphics protocol: \x1b_Gf=100,...;base64data\x1b\\
                            current_token.push(chars[i]);
                            current_token.push(chars[i + 1]);
                            i += 2;
                            while i < chars.len() {
                                if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '\\'
                                {
                                    current_token.push(chars[i]);
                                    current_token.push(chars[i + 1]);
                                    i += 2;
                                    break;
                                }
                                current_token.push(chars[i]);
                                i += 1;
                            }
                        }
                        _ => {
                            // Unknown escape sequence, include ESC and next char
                            current_token.push(chars[i]);
                            current_token.push(chars[i + 1]);
                            i += 2;
                        }
                    }
                } else {
                    current_token.push(chars[i]);
                    i += 1;
                }
            } else if chars[i] == '!' && i + 1 < chars.len() && chars[i + 1] == '[' {
                // Image markdown: ![alt](url) - keep as single token
                // First, flush any existing token
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                // Start building the image token
                current_token.push(chars[i]); // '!'
                current_token.push(chars[i + 1]); // '['
                i += 2;
                // Find closing ]
                let mut bracket_depth = 1;
                while i < chars.len() && bracket_depth > 0 {
                    if chars[i] == '[' {
                        bracket_depth += 1;
                    } else if chars[i] == ']' {
                        bracket_depth -= 1;
                    }
                    current_token.push(chars[i]);
                    i += 1;
                }
                // Check for ( following ]
                if i < chars.len() && chars[i] == '(' {
                    current_token.push(chars[i]);
                    i += 1;
                    // Find closing )
                    let mut paren_depth = 1;
                    while i < chars.len() && paren_depth > 0 {
                        if chars[i] == '(' {
                            paren_depth += 1;
                        } else if chars[i] == ')' {
                            paren_depth -= 1;
                        }
                        current_token.push(chars[i]);
                        i += 1;
                    }
                }
                // The image token is complete, push it
                tokens.push(current_token);
                current_token = String::new();
            } else if chars[i].is_whitespace() {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                i += 1;
            } else {
                current_token.push(chars[i]);
                i += 1;
            }
        }
        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        if tokens.is_empty() {
            return format!("{}\n", first_indent);
        }

        let mut lines: Vec<String> = Vec::new();
        let mut current_line = first_indent.to_string();
        let mut current_width = first_indent_width;
        let mut is_first_line = true;

        for token in tokens {
            let token_width = self.display_width(&token);

            // Check if we need to wrap
            if current_width
                > (if is_first_line {
                    first_indent_width
                } else {
                    cont_indent_width
                })
            {
                // Not at start of line, check if token fits
                if current_width + 1 + token_width > self.width {
                    // Token doesn't fit, start new line
                    lines.push(current_line);
                    current_line = format!("{}{}", cont_indent, token);
                    current_width = cont_indent_width + token_width;
                    is_first_line = false;
                } else {
                    // Token fits, add space and token
                    current_line.push(' ');
                    current_line.push_str(&token);
                    current_width += 1 + token_width;
                }
            } else {
                // At start of line, add token (even if it exceeds width)
                current_line.push_str(&token);
                current_width += token_width;
            }
        }

        // Don't forget the last line
        if !current_line.is_empty() && current_line != first_indent && current_line != cont_indent {
            lines.push(current_line);
        } else if lines.is_empty() {
            // Edge case: only whitespace after indent
            lines.push(first_indent.to_string());
        }

        lines.join("\n")
    }

    fn align_cell(&self, content: &str, width: usize, alignment: Alignment) -> String {
        let visible_len = self.display_width(content);

        if visible_len >= width {
            return content.to_string();
        }

        let padding = width - visible_len;

        match alignment {
            Alignment::Left => {
                format!("{}{}", content, " ".repeat(padding))
            }
            Alignment::Right => {
                format!("{}{}", " ".repeat(padding), content)
            }
            Alignment::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!(
                    "{}{}{}",
                    " ".repeat(left_pad),
                    content,
                    " ".repeat(right_pad)
                )
            }
        }
    }

    fn format_table(
        &self,
        header: &[String],
        alignments: &[Alignment],
        rows: &[Vec<String>],
    ) -> String {
        let mut output = String::new();

        // Calculate column widths
        let num_cols = header
            .len()
            .max(rows.iter().map(|r| r.len()).max().unwrap_or(0));

        let mut col_widths = vec![0; num_cols];

        // Measure header (with inline formatting stripped, using Unicode width)
        for (i, cell) in header.iter().enumerate() {
            col_widths[i] = self.strip_ansi(cell).width();
        }

        // Measure all data rows
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < num_cols {
                    let width = self.strip_ansi(cell).width();
                    col_widths[i] = col_widths[i].max(width);
                }
            }
        }

        // Ensure minimum column width
        for width in &mut col_widths {
            *width = (*width).max(3);
        }

        // Render top border: ┌───┬───┐
        output.push('┌');
        for (i, &width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                output.push('┬');
            }
        }
        output.push_str("┐\n");

        // Render header row: │ Header │ Header │
        output.push('│');
        for (i, cell) in header.iter().enumerate() {
            let formatted = self.format_inline(cell);
            let aligned = self.align_cell(
                &formatted,
                col_widths[i],
                alignments.get(i).copied().unwrap_or(Alignment::Left),
            );
            output.push_str(&format!(" {} │", aligned));
        }
        output.push('\n');

        // Render separator: ├───┼───┤
        output.push('├');
        for (i, &width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                output.push('┼');
            }
        }
        output.push_str("┤\n");

        // Render data rows
        for row in rows {
            output.push('│');
            for (i, &width) in col_widths.iter().enumerate().take(num_cols) {
                let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                let formatted = self.format_inline(cell);
                let aligned = self.align_cell(
                    &formatted,
                    width,
                    alignments.get(i).copied().unwrap_or(Alignment::Left),
                );
                output.push_str(&format!(" {} │", aligned));
            }
            output.push('\n');
        }

        // Render bottom border: └───┴───┘
        output.push('└');
        for (i, &width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                output.push('┴');
            }
        }
        output.push_str("┘\n\n");

        output
    }

    /// Try to parse a callout type from the first line of a blockquote.
    /// Callouts start with `[!TYPE]` where TYPE is NOTE, TIP, IMPORTANT, WARNING, or CAUTION.
    fn parse_callout_type(&self, first_line: &str) -> Option<CalloutType> {
        let trimmed = first_line.trim();

        // Check for [!TYPE] pattern at the start
        if !trimmed.starts_with("[!") {
            return None;
        }

        // Find the closing bracket
        let end = trimmed.find(']')?;
        let type_str = &trimmed[2..end];

        match type_str.to_uppercase().as_str() {
            "NOTE" => Some(CalloutType::Note),
            "TIP" => Some(CalloutType::Tip),
            "IMPORTANT" => Some(CalloutType::Important),
            "WARNING" => Some(CalloutType::Warning),
            "CAUTION" => Some(CalloutType::Caution),
            _ => None,
        }
    }

    /// Get the display properties for a callout type: (icon, name, ANSI color code)
    fn callout_style(
        &self,
        callout_type: CalloutType,
    ) -> (&'static str, &'static str, &'static str) {
        match callout_type {
            // Blue for informational notes
            CalloutType::Note => ("ℹ", "Note", "\u{001b}[34m"),
            // Green for helpful tips
            CalloutType::Tip => ("💡", "Tip", "\u{001b}[32m"),
            // Purple/magenta for important information
            CalloutType::Important => ("❗", "Important", "\u{001b}[35m"),
            // Yellow for warnings
            CalloutType::Warning => ("⚠", "Warning", "\u{001b}[33m"),
            // Red for critical cautions
            CalloutType::Caution => ("🛑", "Caution", "\u{001b}[31m"),
        }
    }

    fn format_blockquote(&self, lines: &[(usize, String)]) -> String {
        // Check if this is a callout (first line starts with [!TYPE])
        if let Some((_, first_content)) = lines.first() {
            if let Some(callout_type) = self.parse_callout_type(first_content) {
                return self.format_callout(callout_type, lines);
            }
        }

        // Regular blockquote formatting
        let mut output = String::new();

        for (nesting_level, content) in lines {
            // Generate prefix: " │ " for each nesting level (U+2502 box drawing character)
            let prefix = " │ ".repeat(*nesting_level);

            // Apply inline formatting to content
            let formatted_content = self.format_inline(content);

            // Wrap content with prefix on each line
            let wrapped = self.wrap_text(&formatted_content, &prefix, &prefix);
            output.push_str(&wrapped);
            output.push('\n');
        }

        // Add blank line after blockquote
        output.push('\n');

        output
    }

    /// Format a callout (GitHub-style alert) with colored styling
    fn format_callout(&self, callout_type: CalloutType, lines: &[(usize, String)]) -> String {
        let mut output = String::new();
        let (icon, name, color) = self.callout_style(callout_type);
        let reset = "\u{001b}[0m";

        // Build the colored bar prefix
        let bar = format!("{} │ {}", color, reset);

        // Output header line with icon and type name (bold + colored)
        output.push_str(&format!(
            "{} │ {}\u{001b}[1m{} {}{}\n",
            color, reset, color, icon, reset
        ));
        output.push_str(&format!(
            "{} │ {}\u{001b}[1m{}{}{}\n",
            color, reset, color, name, reset
        ));

        // Process content lines (skip the [!TYPE] marker from first line)
        let mut content_lines: Vec<String> = Vec::new();
        for (i, (_, content)) in lines.iter().enumerate() {
            if i == 0 {
                // First line: strip the [!TYPE] marker
                let trimmed = content.trim();
                if let Some(end_bracket) = trimmed.find(']') {
                    let after_marker = trimmed[end_bracket + 1..].trim();
                    if !after_marker.is_empty() {
                        content_lines.push(after_marker.to_string());
                    }
                }
            } else {
                content_lines.push(content.clone());
            }
        }

        // Join content and format
        if !content_lines.is_empty() {
            let combined = content_lines.join(" ");
            let formatted_content = self.format_inline(&combined);

            // Wrap with the colored bar as prefix
            let wrapped = self.wrap_text(&formatted_content, &bar, &bar);
            output.push_str(&wrapped);
            output.push('\n');
        }

        // Add blank line after callout
        output.push('\n');

        output
    }

    /// Format inline markdown elements (bold, italic, code, links, etc.) to ANSI codes.
    pub fn format_inline(&self, text: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check for backslash escapes FIRST (before any other markdown processing)
            // Per GFM spec: any ASCII punctuation can be backslash-escaped
            if chars[i] == '\\' && i + 1 < chars.len() {
                let next = chars[i + 1];
                // ASCII punctuation characters that can be escaped
                if is_ascii_punctuation(next) {
                    result.push(next);
                    i += 2;
                    continue;
                }
                // Backslash before non-punctuation is literal (kept as backslash)
                // But we still advance past it so the next char gets normal processing
                result.push('\\');
                i += 1;
                continue;
            }

            // Check for ![alt](src) images
            if chars[i] == '!' {
                if let Some(img) = self.parse_image(&chars, i) {
                    result.push_str(&self.render_image(&img.alt, &img.src));
                    i = img.end_pos;
                    continue;
                }
            }

            // Check for [text](url) hyperlinks or [text][ref]/[ref][]/[ref] reference links
            if chars[i] == '[' {
                // First try inline link [text](url)
                if let Some(link) = self.parse_link(&chars, i) {
                    // Process link text through format_inline to handle images, formatting, etc.
                    let formatted_text = self.format_inline(&link.text);
                    // OSC8 format with blue and underline styling
                    result.push_str("\u{001b}]8;;");
                    result.push_str(&link.url);
                    result.push_str("\u{001b}\\");
                    // Blue and underlined
                    result.push_str("\u{001b}[34;4m");
                    result.push_str(&formatted_text);
                    result.push_str("\u{001b}[0m");
                    result.push_str("\u{001b}]8;;\u{001b}\\");
                    i = link.end_pos;
                    continue;
                }

                // Then try reference link [text][label], [label][], or [label]
                if let Some(ref_link) = self.parse_reference_link(&chars, i) {
                    result.push_str(&self.render_reference_link(&ref_link));
                    i = ref_link.end_pos;
                    continue;
                }
            }

            // Check for ~~strikethrough~~
            if i + 1 < chars.len() && chars[i] == '~' && chars[i + 1] == '~' {
                if let Some(end) = self.find_closing("~~", &chars, i + 2) {
                    let inner: String = chars[i + 2..end].iter().collect();
                    let formatted_inner = self.format_inline(&inner);
                    result.push_str("\u{001b}[9m");
                    result.push_str(&formatted_inner);
                    result.push_str("\u{001b}[0m");
                    i = end + 2;
                    continue;
                }
            }

            // Check for **bold**
            if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
                if let Some(end) = self.find_closing("**", &chars, i + 2) {
                    let inner: String = chars[i + 2..end].iter().collect();
                    let formatted_inner = self.format_inline(&inner);
                    result.push_str("\u{001b}[1m");
                    result.push_str(&formatted_inner);
                    result.push_str("\u{001b}[0m");
                    i = end + 2;
                    continue;
                }
            }

            // Check for *italic*
            if chars[i] == '*' {
                if let Some(end) = self.find_closing("*", &chars, i + 1) {
                    let inner: String = chars[i + 1..end].iter().collect();
                    let formatted_inner = self.format_inline(&inner);
                    result.push_str("\u{001b}[3m");
                    result.push_str(&formatted_inner);
                    result.push_str("\u{001b}[0m");
                    i = end + 1;
                    continue;
                }
            }

            // Check for `code`
            if chars[i] == '`' {
                if let Some(end) = self.find_closing("`", &chars, i + 1) {
                    result.push_str("\u{001b}[38;5;167;48;5;235m ");
                    result.extend(&chars[i + 1..end]);
                    result.push_str(" \u{001b}[0m");
                    i = end + 1;
                    continue;
                }
            }

            // Check for __bold__ (underscore variant)
            if i + 1 < chars.len() && chars[i] == '_' && chars[i + 1] == '_' {
                if let Some(end) = self.find_closing("__", &chars, i + 2) {
                    let inner: String = chars[i + 2..end].iter().collect();
                    let formatted_inner = self.format_inline(&inner);
                    result.push_str("\u{001b}[1m");
                    result.push_str(&formatted_inner);
                    result.push_str("\u{001b}[0m");
                    i = end + 2;
                    continue;
                }
            }

            // Check for _italic_ (underscore variant)
            if chars[i] == '_' {
                if let Some(end) = self.find_closing("_", &chars, i + 1) {
                    let inner: String = chars[i + 1..end].iter().collect();
                    let formatted_inner = self.format_inline(&inner);
                    result.push_str("\u{001b}[3m");
                    result.push_str(&formatted_inner);
                    result.push_str("\u{001b}[0m");
                    i = end + 1;
                    continue;
                }
            }

            // Check for <html> tags
            if chars[i] == '<' {
                if let Some(html) = self.parse_html_tag(&chars, i) {
                    result.push_str(&html.formatted);
                    i = html.end_pos;
                    continue;
                }
            }

            // Check for HTML entities (&amp;, &#123;, &#x7B;)
            if chars[i] == '&' {
                if let Some((decoded, consumed)) = decode_html_entity(&chars, i) {
                    result.push_str(&decoded);
                    i += consumed;
                    continue;
                }
            }

            result.push(chars[i]);
            i += 1;
        }

        result
    }

    fn render_image(&self, alt: &str, src: &str) -> String {
        match self.image_protocol {
            ImageProtocol::None => format!("![{}]({})", alt, src),
            ImageProtocol::Kitty => {
                // Load image data (local file or HTTP)
                match self.load_image_data(src) {
                    Ok(data) => {
                        // Process and render
                        match self.process_image(&data) {
                            Ok(kitty_output) => kitty_output,
                            Err(_) => alt.to_string(), // Fallback to alt text
                        }
                    }
                    Err(_) => alt.to_string(), // Fallback to alt text
                }
            }
        }
    }

    fn load_image_data(&self, src: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Check cache first (populated by prefetch_images)
        if let Some(data) = self.image_cache.get(src) {
            return Ok(data.clone());
        }

        // Not in cache, fetch directly (fallback for non-prefetched images)
        if src.starts_with("http://") || src.starts_with("https://") {
            // Fetch remote image
            let response = ureq::get(src).call()?;
            let mut bytes = Vec::new();
            std::io::Read::read_to_end(&mut response.into_reader(), &mut bytes)?;
            Ok(bytes)
        } else {
            // Load local file
            std::fs::read(src).map_err(|e| e.into())
        }
    }

    fn process_image(&self, data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        use image::ImageReader;
        use std::io::Cursor;

        // Check if data is SVG (XML-based vector graphics)
        let is_svg = Self::is_svg(data);

        let img = if is_svg {
            self.render_svg(data)?
        } else {
            // Decode raster image
            ImageReader::new(Cursor::new(data))
                .with_guessed_format()?
                .decode()?
        };

        // Convert image pixel width to terminal columns
        // Assume ~9 pixels per terminal column (typical for monospace fonts)
        const PIXELS_PER_COLUMN: f64 = 9.0;
        let natural_cols = (img.width() as f64 / PIXELS_PER_COLUMN).ceil() as usize;

        // Use the smaller of natural size or configured max width
        let display_cols = natural_cols.min(self.width);

        // Resize large images to reduce transfer size (cap at 2000px width)
        // but let kitty handle the display scaling
        let max_transfer_width = 2000u32;
        let resized = if img.width() > max_transfer_width {
            img.resize(
                max_transfer_width,
                u32::MAX,
                image::imageops::FilterType::Lanczos3,
            )
        } else {
            img
        };

        // Re-encode as PNG
        let mut png_data = Vec::new();
        resized.write_to(&mut Cursor::new(&mut png_data), image::ImageFormat::Png)?;

        // Render using kitty protocol with display width in columns
        // Kitty will automatically calculate the row count to maintain aspect ratio
        Ok(self.render_kitty_image(&png_data, display_cols))
    }

    /// Check if data appears to be SVG content
    fn is_svg(data: &[u8]) -> bool {
        // Skip leading whitespace/BOM
        let trimmed = match std::str::from_utf8(data) {
            Ok(s) => s.trim_start(),
            Err(_) => return false, // SVG must be valid UTF-8
        };

        // Check for common SVG signatures
        trimmed.starts_with("<?xml")
            || trimmed.starts_with("<svg")
            || trimmed.starts_with("<!DOCTYPE svg")
    }

    /// Render SVG to a raster image using resvg
    fn render_svg(&self, data: &[u8]) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        use resvg::tiny_skia::Pixmap;
        use resvg::usvg::{fontdb, Options, Tree};

        // Load system fonts for text rendering
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();

        let opts = Options {
            fontdb: std::sync::Arc::new(fontdb),
            ..Options::default()
        };

        let svg_str = std::str::from_utf8(data)?;
        let tree = Tree::from_str(svg_str, &opts)?;

        let size = tree.size();
        let width = size.width() as u32;
        let height = size.height() as u32;

        // Create pixmap for rendering
        let mut pixmap =
            Pixmap::new(width, height).ok_or("Failed to create pixmap for SVG rendering")?;

        // Render SVG to pixmap
        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );

        // Convert to image::DynamicImage
        let img = image::RgbaImage::from_raw(width, height, pixmap.take())
            .ok_or("Failed to create image from SVG pixels")?;

        Ok(image::DynamicImage::ImageRgba8(img))
    }

    fn render_kitty_image(&self, png_data: &[u8], columns: usize) -> String {
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let encoded = STANDARD.encode(png_data);
        let chunk_size = 4096;
        let mut output = String::new();

        let chunks: Vec<&str> = encoded
            .as_bytes()
            .chunks(chunk_size)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();

        for (i, chunk) in chunks.iter().enumerate() {
            let is_last = i == chunks.len() - 1;
            let m = if is_last { 0 } else { 1 };

            if i == 0 {
                // First chunk: include format, transmission parameters, and display width
                // c=columns tells kitty the width; it calculates rows to maintain aspect ratio
                output.push_str(&format!(
                    "\x1b_Gf=100,a=T,c={},m={};{}\x1b\\",
                    columns, m, chunk
                ));
            } else {
                // Continuation chunks
                output.push_str(&format!("\x1b_Gm={};{}\x1b\\", m, chunk));
            }
        }

        output
    }

    fn parse_link(&self, chars: &[char], start: usize) -> Option<LinkData> {
        // Looking for [text](url) or [text](url "title")
        // start points to '['

        // Find closing ]
        let text_end = self.find_closing("]", chars, start + 1)?;

        // Check if followed by (
        if text_end + 1 >= chars.len() || chars[text_end + 1] != '(' {
            return None;
        }

        // Find closing )
        let url_end = self.find_closing(")", chars, text_end + 2)?;

        // Extract text
        let text: String = chars[start + 1..text_end].iter().collect();

        // Parse the content between ( and )
        let link_content: String = chars[text_end + 2..url_end].iter().collect();
        let link_content = link_content.trim();

        // Split URL and optional title
        // Title is separated by whitespace and enclosed in quotes or parentheses
        let url = if let Some(space_pos) = link_content.find(|c: char| c.is_whitespace()) {
            // There's whitespace, so there might be a title
            let url_part = link_content[..space_pos].trim();
            let after_url = link_content[space_pos..].trim();

            // Check if there's a title (starts with ", ', or ()
            if after_url.is_empty() {
                url_part.to_string()
            } else {
                // Title exists, just use the URL part
                url_part.to_string()
            }
        } else {
            // No whitespace, entire content is the URL
            link_content.to_string()
        };

        Some(LinkData {
            text,
            url,
            end_pos: url_end + 1,
        })
    }

    /// Parse a reference-style link: [text][label], [label][], or [label]
    fn parse_reference_link(&self, chars: &[char], start: usize) -> Option<ReferenceLinkData> {
        // Looking for [text][label], [label][], or [label]
        // start points to '['

        // Find the first closing ]
        let text_end = self.find_closing("]", chars, start + 1)?;
        let text: String = chars[start + 1..text_end].iter().collect();

        // Empty text is not a valid reference link
        if text.trim().is_empty() {
            return None;
        }

        // Check what follows the first ]
        let after_bracket = text_end + 1;

        if after_bracket < chars.len() && chars[after_bracket] == '[' {
            // Could be [text][label] or [label][]
            let label_end = self.find_closing("]", chars, after_bracket + 1)?;
            let label: String = chars[after_bracket + 1..label_end].iter().collect();

            if label.is_empty() {
                // Collapsed reference: [label][]
                return Some(ReferenceLinkData {
                    text: text.clone(),
                    label: text,
                    end_pos: label_end + 1,
                });
            } else {
                // Full reference: [text][label]
                return Some(ReferenceLinkData {
                    text,
                    label,
                    end_pos: label_end + 1,
                });
            }
        }

        // Check if this is a shortcut reference [label]
        // Must not be followed by [ or ( immediately
        let is_shortcut = after_bracket >= chars.len()
            || (chars[after_bracket] != '[' && chars[after_bracket] != '(');

        if is_shortcut {
            return Some(ReferenceLinkData {
                text: text.clone(),
                label: text,
                end_pos: text_end + 1,
            });
        }

        None
    }

    /// Render a reference link, either as a resolved hyperlink or as a citation
    fn render_reference_link(&self, ref_link: &ReferenceLinkData) -> String {
        let normalized_label = self.normalize_link_label(&ref_link.label);

        // Check if we have a definition for this label
        if let Some((url, _title)) = self.link_definitions.get(&normalized_label) {
            // Definition found - render as normal OSC8 hyperlink
            let formatted_text = self.format_inline(&ref_link.text);
            format!(
                "\u{001b}]8;;{}\u{001b}\\\u{001b}[34;4m{}\u{001b}[0m\u{001b}]8;;\u{001b}\\",
                url, formatted_text
            )
        } else {
            // No definition (yet) - use citation style
            let citation_num = {
                let mut num = self.next_citation_number.borrow_mut();
                let current = *num;
                *num += 1;
                current
            };

            // Store for bibliography
            self.pending_citations.borrow_mut().push((
                citation_num,
                ref_link.label.clone(),
                ref_link.text.clone(),
            ));

            // Render as text[n]
            let formatted_text = self.format_inline(&ref_link.text);
            format!("{}[{}]", formatted_text, citation_num)
        }
    }

    /// Parse an HTML tag and return formatted output
    /// Handles: em, i, strong, b, u, s, strike, del, code, a, pre
    /// HTML comments (<!-- ... -->) are stripped entirely
    /// Unknown tags are stripped but inner content is preserved
    fn parse_html_tag(&self, chars: &[char], start: usize) -> Option<HtmlTagResult> {
        if chars[start] != '<' {
            return None;
        }

        // Check for HTML comments: <!-- ... -->
        if start + 3 < chars.len()
            && chars[start + 1] == '!'
            && chars[start + 2] == '-'
            && chars[start + 3] == '-'
        {
            // Find the closing -->
            let mut i = start + 4;
            while i + 2 < chars.len() {
                if chars[i] == '-' && chars[i + 1] == '-' && chars[i + 2] == '>' {
                    return Some(HtmlTagResult {
                        formatted: String::new(),
                        end_pos: i + 3,
                    });
                }
                i += 1;
            }
            // No closing --> found, don't consume anything
            return None;
        }

        // Find the closing '>' of the opening tag
        let mut tag_end = start + 1;
        while tag_end < chars.len() && chars[tag_end] != '>' {
            tag_end += 1;
        }
        if tag_end >= chars.len() {
            return None;
        }

        // Extract the tag content (between < and >)
        let tag_content: String = chars[start + 1..tag_end].iter().collect();
        let tag_content = tag_content.trim();

        // Check for self-closing tags like <br/> or <hr/> or <img src="..."/>
        if tag_content.ends_with('/') {
            // Extract tag name (first word only)
            let tag_trimmed = tag_content.trim_end_matches('/').trim();
            let tag_name: String = tag_trimmed
                .chars()
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();

            if tag_name == "br" {
                return Some(HtmlTagResult {
                    formatted: "\n".to_string(),
                    end_pos: tag_end + 1,
                });
            }
            if tag_name == "img" {
                // Extract src attribute and render image
                if let Some(src) = self.extract_attr(tag_trimmed, "src") {
                    let alt = self.extract_attr(tag_trimmed, "alt").unwrap_or_default();
                    return Some(HtmlTagResult {
                        formatted: self.render_image(&alt, &src),
                        end_pos: tag_end + 1,
                    });
                }
            }
            // Skip other self-closing tags
            return Some(HtmlTagResult {
                formatted: String::new(),
                end_pos: tag_end + 1,
            });
        }

        // Check for void elements (like <img>, <br>, <hr> without trailing /)
        // Extract tag name first to check
        let tag_name_check: String = tag_content
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        // Handle void elements that don't need closing tags
        if matches!(
            tag_name_check.as_str(),
            "img" | "br" | "hr" | "meta" | "link" | "input"
        ) {
            if tag_name_check == "br" {
                return Some(HtmlTagResult {
                    formatted: "\n".to_string(),
                    end_pos: tag_end + 1,
                });
            }
            if tag_name_check == "img" {
                // Extract src attribute and render image
                if let Some(src) = self.extract_attr(tag_content, "src") {
                    let alt = self.extract_attr(tag_content, "alt").unwrap_or_default();
                    return Some(HtmlTagResult {
                        formatted: self.render_image(&alt, &src),
                        end_pos: tag_end + 1,
                    });
                }
            }
            // Skip other void elements
            return Some(HtmlTagResult {
                formatted: String::new(),
                end_pos: tag_end + 1,
            });
        }

        // Extract tag name (first word, lowercased)
        let tag_name: String = tag_content
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        if tag_name.is_empty() {
            return None;
        }

        // Find the closing tag </tagname>
        let closing_tag = format!("</{}", tag_name);
        let mut depth = 1;
        let mut search_pos = tag_end + 1;
        let mut content_end = None;

        while search_pos < chars.len() {
            if chars[search_pos] == '<' {
                // Check for closing tag
                let remaining: String = chars[search_pos..].iter().collect();
                let remaining_lower = remaining.to_lowercase();
                if remaining_lower.starts_with(&closing_tag) {
                    // Find the > of the closing tag
                    let mut close_end = search_pos;
                    while close_end < chars.len() && chars[close_end] != '>' {
                        close_end += 1;
                    }
                    depth -= 1;
                    if depth == 0 {
                        content_end = Some((search_pos, close_end + 1));
                        break;
                    }
                    search_pos = close_end + 1;
                    continue;
                }
                // Check for nested opening tag of same type
                let open_tag = format!("<{}", tag_name);
                if remaining_lower.starts_with(&open_tag) {
                    let next_char_pos = search_pos + open_tag.len();
                    if next_char_pos < chars.len() {
                        let next_char = chars[next_char_pos];
                        if next_char == '>' || next_char == ' ' || next_char == '/' {
                            depth += 1;
                        }
                    }
                }
            }
            search_pos += 1;
        }

        let (inner_end, end_pos) = content_end?;

        // Extract inner content
        let inner: String = chars[tag_end + 1..inner_end].iter().collect();

        // Format based on tag type
        let formatted = match tag_name.as_str() {
            "em" | "i" => {
                let formatted_inner = self.format_inline(&inner);
                format!("\u{001b}[3m{}\u{001b}[0m", formatted_inner)
            }
            "strong" | "b" => {
                let formatted_inner = self.format_inline(&inner);
                format!("\u{001b}[1m{}\u{001b}[0m", formatted_inner)
            }
            "u" => {
                let formatted_inner = self.format_inline(&inner);
                format!("\u{001b}[4m{}\u{001b}[0m", formatted_inner)
            }
            "s" | "strike" | "del" => {
                let formatted_inner = self.format_inline(&inner);
                format!("\u{001b}[9m{}\u{001b}[0m", formatted_inner)
            }
            "code" => {
                // Inline code - don't recursively format
                format!("\u{001b}[38;5;167;48;5;235m {} \u{001b}[0m", inner)
            }
            "pre" => {
                // Code block style - dark background, no recursive formatting
                let lines: Vec<&str> = inner.lines().collect();
                let mut result = String::new();
                for line in lines {
                    result.push_str("\u{001b}[38;5;167;48;5;235m ");
                    result.push_str(line);
                    result.push_str(" \u{001b}[0m\n");
                }
                result
            }
            "a" => {
                // Extract href attribute
                let href = self.extract_href(tag_content);
                let formatted_inner = self.format_inline(&inner);
                if let Some(url) = href {
                    // OSC8 hyperlink format
                    format!(
                        "\u{001b}]8;;{}\u{001b}\\\u{001b}[34;4m{}\u{001b}[0m\u{001b}]8;;\u{001b}\\",
                        url, formatted_inner
                    )
                } else {
                    // No href, just format the inner content
                    formatted_inner
                }
            }
            _ => {
                // Unknown tag - strip it but keep inner content
                self.format_inline(&inner)
            }
        };

        Some(HtmlTagResult { formatted, end_pos })
    }

    /// Extract an attribute value from tag content like 'img src="url"'
    fn extract_attr(&self, tag_content: &str, attr_name: &str) -> Option<String> {
        let lower = tag_content.to_lowercase();
        let attr_pos = lower.find(&attr_name.to_lowercase())?;
        let after_attr = &tag_content[attr_pos + attr_name.len()..];
        let trimmed = after_attr.trim_start();

        // Expect '='
        if !trimmed.starts_with('=') {
            return None;
        }
        let after_eq = trimmed[1..].trim_start();

        // Extract quoted value
        if let Some(rest) = after_eq.strip_prefix('"') {
            // Double-quoted value
            let end = rest.find('"')?;
            Some(rest[..end].to_string())
        } else if let Some(rest) = after_eq.strip_prefix('\'') {
            // Single-quoted value
            let end = rest.find('\'')?;
            Some(rest[..end].to_string())
        } else {
            // Unquoted - take until whitespace or >
            let end = after_eq
                .find(|c: char| c.is_whitespace() || c == '>')
                .unwrap_or(after_eq.len());
            Some(after_eq[..end].to_string())
        }
    }

    /// Extract href attribute value from tag content like 'a href="url"'
    pub fn extract_href(&self, tag_content: &str) -> Option<String> {
        self.extract_attr(tag_content, "href")
    }

    fn parse_image(&self, chars: &[char], start: usize) -> Option<ImageData> {
        // Looking for ![alt](src) or ![alt](src "title")
        // start points to '!'

        // Must start with ![
        if start + 1 >= chars.len() || chars[start] != '!' || chars[start + 1] != '[' {
            return None;
        }

        // Find closing ]
        let text_end = self.find_closing("]", chars, start + 2)?;

        // Check if followed by (
        if text_end + 1 >= chars.len() || chars[text_end + 1] != '(' {
            return None;
        }

        // Find closing )
        let url_end = self.find_closing(")", chars, text_end + 2)?;

        // Extract alt text
        let alt: String = chars[start + 2..text_end].iter().collect();

        // Parse the content between ( and )
        let src_content: String = chars[text_end + 2..url_end].iter().collect();
        let src_content = src_content.trim();

        // Strip optional title from src (e.g., "url "title"")
        let src = if let Some(space_pos) = src_content.find(|c: char| c.is_whitespace()) {
            // There's whitespace, so there might be a title
            let src_part = src_content[..space_pos].trim();
            src_part.to_string()
        } else {
            // No whitespace, entire content is the src
            src_content.to_string()
        };

        Some(ImageData {
            alt,
            src,
            end_pos: url_end + 1,
        })
    }

    fn find_closing(&self, marker: &str, chars: &[char], start: usize) -> Option<usize> {
        let marker_chars: Vec<char> = marker.chars().collect();
        let marker_len = marker_chars.len();

        // Track bracket nesting for balanced matching
        let mut bracket_depth = 0; // for [ ]
        let mut paren_depth = 0; // for ( )

        let mut i = start;
        while i + marker_len <= chars.len() {
            let c = chars[i];

            // Check for match BEFORE updating depth
            // This ensures we match the closing bracket at depth 0
            let at_balanced_depth = match marker {
                "]" => bracket_depth == 0 && c == ']',
                ")" => paren_depth == 0 && c == ')',
                _ => true, // For other markers, don't require balancing
            };

            if at_balanced_depth {
                let mut matches = true;
                for (j, &mc) in marker_chars.iter().enumerate() {
                    if chars[i + j] != mc {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    return Some(i);
                }
            }

            // Update nesting depth AFTER checking for match
            match c {
                '[' => bracket_depth += 1,
                ']' if bracket_depth > 0 => bracket_depth -= 1,
                '(' => paren_depth += 1,
                ')' if paren_depth > 0 => paren_depth -= 1,
                _ => {}
            }

            i += 1;
        }
        None
    }
}

impl Default for StreamingParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a character is ASCII punctuation (for backslash escape handling).
/// Per GFM spec, these are the characters that can be backslash-escaped:
/// !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~
fn is_ascii_punctuation(c: char) -> bool {
    matches!(
        c,
        '!' | '"'
            | '#'
            | '$'
            | '%'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | '/'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '_'
            | '`'
            | '{'
            | '|'
            | '}'
            | '~'
    )
}

/// Try to decode an HTML entity starting at the given position.
/// Returns Some((decoded_string, chars_consumed)) if an entity is found, None otherwise.
/// Supports all HTML5 named entities (&amp;, &alpha;, etc.), decimal numeric (&#123;),
/// and hex numeric (&#x7B;).
fn decode_html_entity(chars: &[char], start: usize) -> Option<(String, usize)> {
    // Must start with '&'
    if start >= chars.len() || chars[start] != '&' {
        return None;
    }

    // Find the semicolon (entity terminator) or end of entity name
    // HTML5 entities can be longer (e.g., &CounterClockwiseContourIntegral; is 31 chars)
    let mut end = start + 1;
    while end < chars.len() && end - start < 40 {
        if chars[end] == ';' {
            end += 1; // Include the semicolon
            break;
        }
        // Stop if we hit a character that can't be part of an entity name
        if chars[end].is_whitespace() || chars[end] == '&' || chars[end] == '<' {
            break;
        }
        // Entity names are alphanumeric (and # for numeric entities)
        if !chars[end].is_ascii_alphanumeric() && chars[end] != '#' {
            break;
        }
        end += 1;
    }

    // Need at least "&X" to be a potential entity
    if end <= start + 1 {
        return None;
    }

    // Extract the potential entity (including & and optional ;)
    let entity_str: String = chars[start..end].iter().collect();

    // Use htmlentity crate to decode
    let decoded = decode_html_entity_bytes(entity_str.as_bytes());
    if let Ok(decoded_str) = decoded.to_string() {
        // Check if decoding actually changed anything
        if decoded_str != entity_str {
            return Some((decoded_str, end - start));
        }
    }

    None
}
