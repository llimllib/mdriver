use unicode_width::UnicodeWidthStr;

/// Streaming markdown parser that emits formatted blocks incrementally
pub struct StreamingParser {
    buffer: String,
    state: ParserState,
    current_block: BlockBuilder,
}

#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    Ready,
    InParagraph,
    InCodeBlock { info: String, fence: String },
    InList,
}

#[derive(Debug, Clone)]
enum BlockBuilder {
    None,
    Paragraph { lines: Vec<String> },
    CodeBlock {
        lines: Vec<String>,
        #[allow(dead_code)]
        info: String,  // Language info for future syntax highlighting
    },
    List { items: Vec<String> },
}

impl StreamingParser {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            state: ParserState::Ready,
            current_block: BlockBuilder::None,
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

        output
    }

    fn process_line(&mut self, line: &str) -> Option<String> {
        match &self.state {
            ParserState::Ready => self.handle_ready_state(line),
            ParserState::InParagraph => self.handle_in_paragraph(line),
            ParserState::InCodeBlock { .. } => self.handle_in_code_block(line),
            ParserState::InList => self.handle_in_list(line),
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
        if let Some((info, fence)) = self.parse_code_fence(trimmed) {
            self.state = ParserState::InCodeBlock { info: info.clone(), fence: fence.clone() };
            self.current_block = BlockBuilder::CodeBlock { lines: Vec::new(), info };
            return None;
        }

        // Check for list item (- or digit.)
        if self.is_list_item(trimmed) {
            self.state = ParserState::InList;
            self.current_block = BlockBuilder::List { items: vec![trimmed.to_string()] };
            return None;
        }

        // Otherwise, start a paragraph
        self.state = ParserState::InParagraph;
        self.current_block = BlockBuilder::Paragraph { lines: vec![trimmed.to_string()] };
        None
    }

    fn handle_in_paragraph(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Blank line completes paragraph
        if trimmed.is_empty() {
            return self.emit_current_block();
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
        if let ParserState::InCodeBlock { fence, .. } = &self.state {
            if trimmed.starts_with(fence) && trimmed.trim() == fence.trim() {
                // Closing fence - emit the block
                return self.emit_current_block();
            }
        }

        // Add line to code block
        if let BlockBuilder::CodeBlock { lines, .. } = &mut self.current_block {
            lines.push(trimmed.to_string());
        }
        None
    }

    fn handle_in_list(&mut self, line: &str) -> Option<String> {
        let trimmed = line.trim_end_matches('\n');

        // Blank line completes list
        if trimmed.is_empty() {
            return self.emit_current_block();
        }

        // Check if it's another list item
        if self.is_list_item(trimmed) {
            if let BlockBuilder::List { items } = &mut self.current_block {
                items.push(trimmed.to_string());
            }
            return None;
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

    fn parse_code_fence(&self, line: &str) -> Option<(String, String)> {
        if let Some(rest) = line.strip_prefix("```") {
            let fence = "```".to_string();
            let info = rest.trim().to_string();
            Some((info, fence))
        } else if let Some(rest) = line.strip_prefix("~~~") {
            let fence = "~~~".to_string();
            let info = rest.trim().to_string();
            Some((info, fence))
        } else {
            None
        }
    }

    fn is_list_item(&self, line: &str) -> bool {
        // Unordered list: starts with "- "
        if line.starts_with("- ") {
            return true;
        }

        // Ordered list: starts with digit(s) followed by "." and space
        if let Some(dot_pos) = line.find('.') {
            if dot_pos > 0 && dot_pos < line.len() - 1 {
                let before_dot = &line[..dot_pos];
                let after_dot = &line[dot_pos + 1..];
                if before_dot.chars().all(|c| c.is_ascii_digit()) && after_dot.starts_with(' ') {
                    return true;
                }
            }
        }

        false
    }

    fn emit_current_block(&mut self) -> Option<String> {
        let block = std::mem::replace(&mut self.current_block, BlockBuilder::None);
        self.state = ParserState::Ready;

        match block {
            BlockBuilder::None => None,
            BlockBuilder::Paragraph { lines } => Some(self.format_paragraph(&lines)),
            BlockBuilder::CodeBlock { lines, .. } => Some(self.format_code_block(&lines)),
            BlockBuilder::List { items } => Some(self.format_list(&items)),
        }
    }

    fn format_heading(&self, level: usize, text: &str) -> String {
        let formatted_text = self.format_inline(text);
        // Heading: blue and bold, with line break after for spacing
        format!("\u{001b}[1;34m{} {}\u{001b}[0m\n\n", "#".repeat(level), formatted_text)
    }

    fn format_paragraph(&self, lines: &[String]) -> String {
        let text = lines.join(" ");
        let formatted_text = self.format_inline(&text);
        format!("{}\n\n", formatted_text)
    }

    fn format_code_block(&self, lines: &[String]) -> String {
        let mut output = String::new();

        // Find the maximum display width (accounting for Unicode characters)
        let max_width = lines.iter()
            .map(|l| {
                let with_space = format!(" {}", l);
                with_space.width()  // Use unicode-width for proper display width
            })
            .max()
            .unwrap_or(1);

        // Each line: leading space + content, pad to max_width for uniform visual width
        for line in lines {
            let content_with_lead = format!(" {}", line);
            let display_width = content_with_lead.width();
            let padding = max_width.saturating_sub(display_width);
            output.push_str(&format!("\u{001b}[48;5;235m{}{}\u{001b}[0m\n", content_with_lead, " ".repeat(padding)));
        }

        // Add blank line after code block for spacing
        output.push('\n');
        output
    }

    fn format_list(&self, items: &[String]) -> String {
        let mut output = String::new();
        for item in items {
            // Extract the content after the marker
            let content = if let Some(rest) = item.strip_prefix("- ") {
                rest
            } else if let Some(dot_pos) = item.find(". ") {
                &item[dot_pos + 2..]
            } else {
                item
            };

            let formatted_content = self.format_inline(content);
            output.push_str(&format!("  • {}\n", formatted_content));
        }
        // Add blank line after list for spacing
        output.push('\n');
        output
    }

    fn format_inline(&self, text: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check for **bold**
            if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
                if let Some(end) = self.find_closing("**", &chars, i + 2) {
                    result.push_str("\u{001b}[1m");
                    result.extend(&chars[i + 2..end]);
                    result.push_str("\u{001b}[0m");
                    i = end + 2;
                    continue;
                }
            }

            // Check for *italic*
            if chars[i] == '*' {
                if let Some(end) = self.find_closing("*", &chars, i + 1) {
                    result.push_str("\u{001b}[3m");
                    result.extend(&chars[i + 1..end]);
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
                    result.push_str("\u{001b}[1m");
                    result.extend(&chars[i + 2..end]);
                    result.push_str("\u{001b}[0m");
                    i = end + 2;
                    continue;
                }
            }

            // Check for _italic_ (underscore variant)
            if chars[i] == '_' {
                if let Some(end) = self.find_closing("_", &chars, i + 1) {
                    result.push_str("\u{001b}[3m");
                    result.extend(&chars[i + 1..end]);
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
