use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

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
    Ordered,
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
    theme_set: ThemeSet,
    theme_name: String,
    image_protocol: ImageProtocol,
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
    InBlockquote { nesting_level: usize },
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
}

struct LinkData {
    text: String,
    url: String,
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
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            theme_name: theme_name.to_string(),
            image_protocol,
        }
    }

    /// List available syntax highlighting themes
    pub fn list_themes() -> Vec<String> {
        let theme_set = ThemeSet::load_defaults();
        let mut themes: Vec<String> = theme_set.themes.keys().cloned().collect();
        themes.sort();
        themes
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

        output
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
        }
    }

    fn handle_ready_state(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Check for blank line
        if trimmed.is_empty() {
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
                // Indented content that's not a code fence - it's list continuation (e.g., multi-paragraph item)
                // Just ignore it and stay in the list
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
                    // If same type, continue the list
                    if *current_type == new_type {
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

        // Check if it's indented content (4+ spaces) - list continuation
        let leading_spaces = trimmed.len() - trimmed.trim_start().len();
        if leading_spaces >= 4 && !trimmed.is_empty() {
            // This is list continuation - go back to InList and process it
            self.state = ParserState::InList;
            return self.handle_in_list(line);
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
                    return Some((leading_spaces, ListItemType::Ordered));
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
        }
    }

    fn format_heading(&self, level: usize, text: &str) -> String {
        let formatted_text = self.format_inline(text);
        // Heading: blue and bold, with line break after for spacing
        format!(
            "\u{001b}[1;34m{} {}\u{001b}[0m\n\n",
            "#".repeat(level),
            formatted_text
        )
    }

    fn format_horizontal_rule(&self) -> String {
        // Use a line of dashes with dim/gray color
        // Terminal width aware, but default to 80 if unavailable
        let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
        let rule = "─".repeat(width);
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

        let formatted_text = self.format_inline(&result);
        format!("{}\n\n", formatted_text)
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

        let theme = self
            .theme_set
            .themes
            .get(&self.theme_name)
            .or_else(|| self.theme_set.themes.get("base16-ocean.dark"))
            .expect("Failed to load syntax highlighting theme");
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

    fn format_list(&self, items: &[(usize, ListItemType, String)]) -> String {
        let mut output = String::new();
        // Track numbering for each nesting level
        let mut counters: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

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

            let formatted_content = self.format_inline(content);

            // Use indentation level to determine nesting (each 4 spaces = 1 level)
            let nesting_level = indent_level / 4;
            let indent = "  ".repeat(nesting_level);

            // Format based on item type
            match item_type {
                ListItemType::Unordered => {
                    output.push_str(&format!("{}  • {}\n", indent, formatted_content));
                }
                ListItemType::Ordered => {
                    // Increment counter for this nesting level
                    let counter = counters.entry(nesting_level).or_insert(0);
                    *counter += 1;
                    output.push_str(&format!("{}  {}. {}\n", indent, counter, formatted_content));
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

    fn strip_ansi(&self, text: &str) -> String {
        // Simple ANSI stripper for width calculation
        let mut result = String::new();
        let mut in_escape = false;

        for ch in text.chars() {
            if ch == '\x1b' {
                in_escape = true;
            } else if in_escape {
                if ch == 'm' {
                    in_escape = false;
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn align_cell(&self, content: &str, width: usize, alignment: Alignment) -> String {
        let visible_len = self.strip_ansi(content).chars().count();

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

        // Measure header (with inline formatting stripped)
        for (i, cell) in header.iter().enumerate() {
            col_widths[i] = self.strip_ansi(cell).chars().count();
        }

        // Measure all data rows
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < num_cols {
                    let width = self.strip_ansi(cell).chars().count();
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

    fn format_blockquote(&self, lines: &[(usize, String)]) -> String {
        let mut output = String::new();

        for (nesting_level, content) in lines {
            // Generate prefix: " │ " for each nesting level (U+2502 box drawing character)
            let prefix = " │ ".repeat(*nesting_level);

            // Apply inline formatting to content
            let formatted_content = self.format_inline(content);

            output.push_str(&prefix);
            output.push_str(&formatted_content);
            output.push('\n');
        }

        // Add blank line after blockquote
        output.push('\n');

        output
    }

    fn format_inline(&self, text: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check for ![alt](src) images
            if chars[i] == '!' {
                if let Some(img) = self.parse_image(&chars, i) {
                    result.push_str(&self.render_image(&img.alt, &img.src));
                    i = img.end_pos;
                    continue;
                }
            }

            // Check for [text](url) hyperlinks
            if chars[i] == '[' {
                if let Some(link) = self.parse_link(&chars, i) {
                    // OSC8 format with blue and underline styling
                    result.push_str("\u{001b}]8;;");
                    result.push_str(&link.url);
                    result.push_str("\u{001b}\\");
                    // Blue and underlined
                    result.push_str("\u{001b}[34;4m");
                    result.push_str(&link.text);
                    result.push_str("\u{001b}[0m");
                    result.push_str("\u{001b}]8;;\u{001b}\\");
                    i = link.end_pos;
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
                    result.push_str("\u{001b}[48;5;235m ");
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

        // Decode image
        let img = ImageReader::new(Cursor::new(data))
            .with_guessed_format()?
            .decode()?;

        // Get terminal width (assume 80 if can't determine)
        let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);

        // Resize to terminal width, preserve aspect ratio
        let resized = if img.width() > term_width as u32 {
            img.resize(
                term_width as u32,
                u32::MAX,
                image::imageops::FilterType::Lanczos3,
            )
        } else {
            img
        };

        // Re-encode as PNG
        let mut png_data = Vec::new();
        resized.write_to(&mut Cursor::new(&mut png_data), image::ImageFormat::Png)?;

        // Render using kitty protocol
        Ok(self.render_kitty_image(&png_data))
    }

    fn render_kitty_image(&self, png_data: &[u8]) -> String {
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
                // First chunk: include format and transmission parameters
                output.push_str(&format!("\x1b_Gf=100,a=T,m={};{}\x1b\\", m, chunk));
            } else {
                // Continuation chunks
                output.push_str(&format!("\x1b_Gm={};{}\x1b\\", m, chunk));
            }
        }

        // Add newline after image
        output.push('\n');
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

        let mut i = start;
        while i + marker_len <= chars.len() {
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
