//! Unit tests for StreamingParser internal functionality

use mdriver::{ImageProtocol, StreamingParser};

fn parser() -> StreamingParser {
    StreamingParser::new()
}

/// Strip ANSI codes for easier assertion in tests.
/// Handles both CSI sequences (\x1b[...m) and OSC sequences (\x1b]...\\)
fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\x1b' {
            i += 1;
            if i >= chars.len() {
                break;
            }
            if chars[i] == '[' {
                // CSI sequence - skip until 'm'
                while i < chars.len() && chars[i] != 'm' {
                    i += 1;
                }
                i += 1; // skip 'm'
            } else if chars[i] == ']' {
                // OSC sequence - skip until ST (\x1b\\)
                while i < chars.len() {
                    if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '\\' {
                        i += 2;
                        break;
                    }
                    i += 1;
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

mod html_tags {
    use super::*;

    #[test]
    fn test_em_tag() {
        let p = parser();
        let result = p.format_inline("Hello <em>world</em>!");
        assert!(result.contains("\x1b[3m")); // italic
        assert!(result.contains("\x1b[0m")); // reset
        assert_eq!(strip_ansi(&result), "Hello world!");
    }

    #[test]
    fn test_i_tag() {
        let p = parser();
        let result = p.format_inline("Hello <i>italic</i>!");
        assert!(result.contains("\x1b[3m")); // italic
        assert_eq!(strip_ansi(&result), "Hello italic!");
    }

    #[test]
    fn test_strong_tag() {
        let p = parser();
        let result = p.format_inline("Hello <strong>bold</strong>!");
        assert!(result.contains("\x1b[1m")); // bold
        assert_eq!(strip_ansi(&result), "Hello bold!");
    }

    #[test]
    fn test_b_tag() {
        let p = parser();
        let result = p.format_inline("Hello <b>bold</b>!");
        assert!(result.contains("\x1b[1m")); // bold
        assert_eq!(strip_ansi(&result), "Hello bold!");
    }

    #[test]
    fn test_u_tag() {
        let p = parser();
        let result = p.format_inline("Hello <u>underline</u>!");
        assert!(result.contains("\x1b[4m")); // underline
        assert_eq!(strip_ansi(&result), "Hello underline!");
    }

    #[test]
    fn test_s_tag() {
        let p = parser();
        let result = p.format_inline("Hello <s>strikethrough</s>!");
        assert!(result.contains("\x1b[9m")); // strikethrough
        assert_eq!(strip_ansi(&result), "Hello strikethrough!");
    }

    #[test]
    fn test_strike_tag() {
        let p = parser();
        let result = p.format_inline("Hello <strike>strikethrough</strike>!");
        assert!(result.contains("\x1b[9m")); // strikethrough
        assert_eq!(strip_ansi(&result), "Hello strikethrough!");
    }

    #[test]
    fn test_del_tag() {
        let p = parser();
        let result = p.format_inline("Hello <del>deleted</del>!");
        assert!(result.contains("\x1b[9m")); // strikethrough
        assert_eq!(strip_ansi(&result), "Hello deleted!");
    }

    #[test]
    fn test_code_tag() {
        let p = parser();
        let result = p.format_inline("Hello <code>code</code>!");
        assert!(result.contains("\x1b[38;5;167;48;5;235m")); // red foreground, dark background
        assert_eq!(strip_ansi(&result), "Hello  code !");
    }

    #[test]
    fn test_anchor_tag_with_href() {
        let p = parser();
        let result = p.format_inline(r#"Click <a href="https://example.com">here</a>!"#);
        // Should contain OSC8 hyperlink
        assert!(result.contains("\x1b]8;;https://example.com\x1b\\"));
        assert!(result.contains("\x1b[34;4m")); // blue underline
        assert_eq!(strip_ansi(&result), "Click here!");
    }

    #[test]
    fn test_anchor_tag_single_quotes() {
        let p = parser();
        let result = p.format_inline(r#"Click <a href='https://example.com'>here</a>!"#);
        assert!(result.contains("\x1b]8;;https://example.com\x1b\\"));
        assert_eq!(strip_ansi(&result), "Click here!");
    }

    #[test]
    fn test_anchor_tag_no_href() {
        let p = parser();
        let result = p.format_inline("Click <a>here</a>!");
        // Should just format the inner content without hyperlink
        assert!(!result.contains("\x1b]8;;"));
        assert_eq!(strip_ansi(&result), "Click here!");
    }

    #[test]
    fn test_nested_tags() {
        let p = parser();
        let result = p.format_inline("Hello <b><i>bold italic</i></b>!");
        assert!(result.contains("\x1b[1m")); // bold
        assert!(result.contains("\x1b[3m")); // italic
        assert_eq!(strip_ansi(&result), "Hello bold italic!");
    }

    #[test]
    fn test_unknown_tag_stripped() {
        let p = parser();
        let result = p.format_inline("Hello <span>content</span>!");
        // Unknown tags should be stripped but content preserved
        assert_eq!(strip_ansi(&result), "Hello content!");
    }

    #[test]
    fn test_self_closing_br() {
        let p = parser();
        let result = p.format_inline("Line 1<br/>Line 2");
        assert_eq!(result, "Line 1\nLine 2");
    }

    #[test]
    fn test_case_insensitive_tags() {
        let p = parser();
        let result = p.format_inline("Hello <STRONG>bold</STRONG>!");
        assert!(result.contains("\x1b[1m")); // bold
        assert_eq!(strip_ansi(&result), "Hello bold!");
    }

    #[test]
    fn test_tag_with_attributes() {
        let p = parser();
        let result = p.format_inline(r#"Hello <span class="foo">content</span>!"#);
        // Unknown tag with attributes should still work
        assert_eq!(strip_ansi(&result), "Hello content!");
    }

    #[test]
    fn test_unclosed_tag_preserved() {
        let p = parser();
        let result = p.format_inline("Hello <em>world");
        // Unclosed tag should be preserved as-is
        assert_eq!(result, "Hello <em>world");
    }

    #[test]
    fn test_less_than_not_tag() {
        let p = parser();
        let result = p.format_inline("5 < 10 and 10 > 5");
        // Standalone < should be preserved
        assert_eq!(result, "5 < 10 and 10 > 5");
    }

    #[test]
    fn test_html_mixed_with_markdown() {
        let p = parser();
        let result = p.format_inline("**bold** and <em>italic</em>");
        assert!(result.contains("\x1b[1m")); // bold from markdown
        assert!(result.contains("\x1b[3m")); // italic from HTML
        assert_eq!(strip_ansi(&result), "bold and italic");
    }

    #[test]
    fn test_pre_tag() {
        let p = parser();
        let result = p.format_inline("<pre>code block</pre>");
        assert!(result.contains("\x1b[38;5;167;48;5;235m")); // red foreground, dark background
    }
}

mod extract_href {
    use super::*;

    #[test]
    fn test_double_quoted_href() {
        let p = parser();
        let result = p.extract_href(r#"a href="https://example.com""#);
        assert_eq!(result, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_single_quoted_href() {
        let p = parser();
        let result = p.extract_href(r#"a href='https://example.com'"#);
        assert_eq!(result, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_href_with_spaces() {
        let p = parser();
        let result = p.extract_href(r#"a  href = "https://example.com" "#);
        assert_eq!(result, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_no_href() {
        let p = parser();
        let result = p.extract_href("a class=\"link\"");
        assert_eq!(result, None);
    }

    #[test]
    fn test_href_case_insensitive() {
        let p = parser();
        let result = p.extract_href(r#"a HREF="https://example.com""#);
        assert_eq!(result, Some("https://example.com".to_string()));
    }
}

mod strip_ansi_tests {
    use super::*;

    #[test]
    fn test_strip_basic_sgr() {
        let p = parser();
        let text = "\x1b[1mbold\x1b[0m";
        assert_eq!(p.strip_ansi(text), "bold");
    }

    #[test]
    fn test_strip_osc8_hyperlink() {
        let p = parser();
        // OSC8 hyperlink format: \x1b]8;;URL\x1b\\ VISIBLE_TEXT \x1b]8;;\x1b\\
        let text = "\x1b]8;;https://example.com\x1b\\link text\x1b]8;;\x1b\\";
        assert_eq!(p.strip_ansi(text), "link text");
    }

    #[test]
    fn test_strip_osc8_with_styling() {
        let p = parser();
        // Hyperlink with blue underline styling
        let text = "\x1b]8;;https://example.com\x1b\\\x1b[34;4mlink text\x1b[0m\x1b]8;;\x1b\\";
        assert_eq!(p.strip_ansi(text), "link text");
    }

    #[test]
    fn test_strip_mixed_content() {
        let p = parser();
        // Text with a hyperlink in the middle
        let text =
            "Click \x1b]8;;https://example.com\x1b\\\x1b[34;4mhere\x1b[0m\x1b]8;;\x1b\\ to continue";
        assert_eq!(p.strip_ansi(text), "Click here to continue");
    }

    #[test]
    fn test_strip_long_url() {
        let p = parser();
        // Long URL that would mess up line width calculations
        let text =
            "\x1b]8;;https://facebook.github.io/jsx/specification/very/long/path\x1b\\JSX specification\x1b]8;;\x1b\\";
        assert_eq!(p.strip_ansi(text), "JSX specification");
    }
}

mod wrap_text_tests {
    use super::*;

    fn parser_with_width(width: usize) -> StreamingParser {
        StreamingParser::with_width("base16-ocean.dark", ImageProtocol::None, width)
    }

    #[test]
    fn test_wrap_plain_text() {
        let p = parser_with_width(40);
        let text = "This is a simple sentence that needs wrapping";
        let result = p.wrap_text(text, "", "");
        // Should wrap at width 40
        assert!(result.lines().all(|line| line.len() <= 40));
    }

    #[test]
    fn test_wrap_with_hyperlink_visible_width() {
        let p = parser_with_width(50);
        // Create text with a hyperlink - URL is long but visible text is short
        let text = "Check the \x1b]8;;https://facebook.github.io/jsx/specification\x1b\\\x1b[34;4mJSX specification\x1b[0m\x1b]8;;\x1b\\ for details";
        let result = p.wrap_text(text, "", "");

        // Visible text is "Check the JSX specification for details" = 40 chars
        // Should fit on one line at width 50
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 1, "Should fit on one line. Got: {:?}", lines);
    }

    #[test]
    fn test_wrap_hyperlink_not_counted_in_width() {
        let p = parser_with_width(30);
        // The visible text "Click here now" is 14 chars
        // The URL is very long but should not count toward width
        let text = "Click \x1b]8;;https://example.com/very/long/path/that/would/exceed/width\x1b\\\x1b[34;4mhere\x1b[0m\x1b]8;;\x1b\\ now";
        let result = p.wrap_text(text, "", "");

        // Should fit on one line since visible text is only 14 chars
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(
            lines.len(),
            1,
            "Short visible text should fit. Got: {:?}",
            lines
        );
    }

    #[test]
    fn test_wrap_multiple_hyperlinks() {
        let p = parser_with_width(60);
        // Two hyperlinks in the same text
        let text = "See \x1b]8;;https://example1.com\x1b\\\x1b[34;4mlink one\x1b[0m\x1b]8;;\x1b\\ and \x1b]8;;https://example2.com\x1b\\\x1b[34;4mlink two\x1b[0m\x1b]8;;\x1b\\ for more";
        let result = p.wrap_text(text, "", "");

        // Visible: "See link one and link two for more" = 34 chars
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 1, "Should fit on one line. Got: {:?}", lines);
    }

    #[test]
    fn test_wrap_preserves_hyperlink_sequence() {
        let p = parser_with_width(80);
        let text = "\x1b]8;;https://example.com\x1b\\\x1b[34;4mclick me\x1b[0m\x1b]8;;\x1b\\";
        let result = p.wrap_text(text, "", "");

        // The OSC8 sequences should be preserved
        assert!(result.contains("\x1b]8;;https://example.com\x1b\\"));
        assert!(result.contains("\x1b]8;;\x1b\\"));
    }

    #[test]
    fn test_wrap_with_indent_and_hyperlink() {
        let p = parser_with_width(50);
        let text =
            "This has a \x1b]8;;https://example.com\x1b\\\x1b[34;4mlink\x1b[0m\x1b]8;;\x1b\\";
        let result = p.wrap_text(text, "  \u{2022} ", "    ");

        // Should start with the first indent
        assert!(result.starts_with("  \u{2022} "));
    }

    #[test]
    fn test_wrap_real_example_jsx_spec() {
        // This mimics the actual example.md content that was causing issues
        let p = parser_with_width(80);
        let text = "I think I originally didn't implement it even though it's part of the \x1b]8;;https://facebook.github.io/jsx/\x1b\\\x1b[34;4mJSX specification\x1b[0m\x1b]8;;\x1b\\ because it previously didn't work in TypeScript";
        let result = p.wrap_text(text, "", "");

        // Check that lines are reasonably balanced (not ragged)
        let lines: Vec<&str> = result.lines().collect();
        for line in &lines {
            let visible = strip_ansi(line);
            // Each line should be close to 80 chars (or less for last line)
            assert!(
                visible.chars().count() <= 80,
                "Line too long: {} chars",
                visible.chars().count()
            );
        }
    }
}
