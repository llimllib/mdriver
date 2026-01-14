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

mod img_tag {
    use super::*;

    // When ImageProtocol::None, img tags should output markdown-style ![alt](src)

    #[test]
    fn test_img_self_closing_with_space() {
        let p = parser();
        let result = p.format_inline(r#"<img src="https://example.com/image.png" />"#);
        assert_eq!(result, "![](https://example.com/image.png)");
    }

    #[test]
    fn test_img_self_closing_no_space() {
        let p = parser();
        let result = p.format_inline(r#"<img src="https://example.com/image.png"/>"#);
        assert_eq!(result, "![](https://example.com/image.png)");
    }

    #[test]
    fn test_img_void_element() {
        // HTML5 void element - no closing tag or trailing slash
        let p = parser();
        let result = p.format_inline(r#"<img src="https://example.com/image.png">"#);
        assert_eq!(result, "![](https://example.com/image.png)");
    }

    #[test]
    fn test_img_with_alt() {
        let p = parser();
        let result =
            p.format_inline(r#"<img src="https://example.com/image.png" alt="My Image"/>"#);
        assert_eq!(result, "![My Image](https://example.com/image.png)");
    }

    #[test]
    fn test_img_with_alt_void_element() {
        let p = parser();
        let result = p.format_inline(r#"<img src="https://example.com/image.png" alt="My Image">"#);
        assert_eq!(result, "![My Image](https://example.com/image.png)");
    }

    #[test]
    fn test_img_single_quoted_attrs() {
        let p = parser();
        let result =
            p.format_inline(r#"<img src='https://example.com/image.png' alt='Alt Text'/>"#);
        assert_eq!(result, "![Alt Text](https://example.com/image.png)");
    }

    #[test]
    fn test_img_case_insensitive_tag() {
        let p = parser();
        let result = p.format_inline(r#"<IMG src="https://example.com/image.png"/>"#);
        assert_eq!(result, "![](https://example.com/image.png)");
    }

    #[test]
    fn test_img_case_insensitive_attrs() {
        let p = parser();
        let result = p.format_inline(r#"<img SRC="https://example.com/image.png" ALT="Test"/>"#);
        assert_eq!(result, "![Test](https://example.com/image.png)");
    }

    #[test]
    fn test_img_with_other_attrs() {
        let p = parser();
        let result = p.format_inline(
            r#"<img src="https://example.com/image.png" width="200" alt="Logo" style="padding: 10px"/>"#,
        );
        assert_eq!(result, "![Logo](https://example.com/image.png)");
    }

    #[test]
    fn test_img_inside_div() {
        let p = parser();
        let result =
            p.format_inline(r#"<div><img src="https://example.com/image.png" alt="Test"/></div>"#);
        assert_eq!(result, "![Test](https://example.com/image.png)");
    }

    #[test]
    fn test_img_no_src_returns_empty() {
        let p = parser();
        let result = p.format_inline(r#"<img alt="No Source"/>"#);
        // When no src attribute, should output empty (tag is skipped)
        assert_eq!(result, "");
    }

    #[test]
    fn test_img_inline_with_text() {
        let p = parser();
        let result = p.format_inline(r#"Here is an image: <img src="https://example.com/image.png" alt="pic"/> and more text"#);
        assert_eq!(
            result,
            "Here is an image: ![pic](https://example.com/image.png) and more text"
        );
    }

    #[test]
    fn test_multiple_img_tags() {
        let p = parser();
        let result = p.format_inline(
            r#"<img src="https://example.com/a.png" alt="A"/> and <img src="https://example.com/b.png" alt="B"/>"#,
        );
        assert_eq!(
            result,
            "![A](https://example.com/a.png) and ![B](https://example.com/b.png)"
        );
    }

    #[test]
    fn test_img_with_local_path() {
        let p = parser();
        let result = p.format_inline(r#"<img src="./images/logo.png" alt="Logo"/>"#);
        assert_eq!(result, "![Logo](./images/logo.png)");
    }
}

mod image_inside_link {
    use super::*;

    #[test]
    fn test_html_img_inside_link() {
        // This was a bug: image inside link wasn't being processed through format_inline
        let p = parser();
        let result = p.format_inline(
            r#"[<img src="https://example.com/logo.png" alt="Logo"/>](https://example.com)"#,
        );
        // The img should be converted to ![alt](src) format, wrapped in a link
        let stripped = strip_ansi(&result);
        assert_eq!(stripped, "![Logo](https://example.com/logo.png)");
        // Should have OSC8 hyperlink codes
        assert!(result.contains("\x1b]8;;https://example.com\x1b\\"));
    }

    #[test]
    fn test_html_img_inside_link_with_attributes() {
        let p = parser();
        let result = p.format_inline(
            r#"[<img src="http://example.com/img.png" width="200" alt="My Image" style="padding: 10px"/>](https://example.com)"#,
        );
        let stripped = strip_ansi(&result);
        assert_eq!(stripped, "![My Image](http://example.com/img.png)");
    }

    #[test]
    fn test_markdown_image_inside_link() {
        // [![alt](img-src)](link-url) pattern
        // Note: This is a known limitation - nested markdown brackets don't parse correctly.
        // The parser finds the first `]` instead of the matching one, so this becomes
        // a link with text "![Badge" and URL from the image src.
        // For proper nested image-in-link, use HTML: [<img src="..." alt="..."/>](url)
        let p = parser();
        let result =
            p.format_inline("[![Badge](https://example.com/badge.svg)](https://example.com)");
        let stripped = strip_ansi(&result);
        // Current behavior: parses as link with text "![Badge" and URL "https://example.com/badge.svg"
        assert_eq!(stripped, "![Badge](https://example.com)");
    }

    #[test]
    fn test_text_and_img_inside_link() {
        let p = parser();
        let result = p.format_inline(
            r#"[Click here <img src="https://example.com/icon.png" alt="icon"/>](https://example.com)"#,
        );
        let stripped = strip_ansi(&result);
        assert_eq!(stripped, "Click here ![icon](https://example.com/icon.png)");
    }

    #[test]
    fn test_bold_inside_link() {
        // Verify other inline formatting inside links also works
        let p = parser();
        let result = p.format_inline("[**bold text**](https://example.com)");
        let stripped = strip_ansi(&result);
        assert_eq!(stripped, "bold text");
        // Should have bold formatting
        assert!(result.contains("\x1b[1m"));
    }

    #[test]
    fn test_code_inside_link() {
        let p = parser();
        let result = p.format_inline("[`code`](https://example.com)");
        let stripped = strip_ansi(&result);
        // Code adds spaces around content
        assert!(stripped.contains("code"));
        // Should have code formatting (background color)
        assert!(result.contains("\x1b[38;5;167;48;5;235m"));
    }
}

mod html_entities {
    use super::*;

    // Essential XML entities
    #[test]
    fn test_amp_entity() {
        let p = parser();
        let result = p.format_inline("Tom &amp; Jerry");
        assert_eq!(result, "Tom & Jerry");
    }

    #[test]
    fn test_lt_entity() {
        let p = parser();
        let result = p.format_inline("5 &lt; 10");
        assert_eq!(result, "5 < 10");
    }

    #[test]
    fn test_gt_entity() {
        let p = parser();
        let result = p.format_inline("10 &gt; 5");
        assert_eq!(result, "10 > 5");
    }

    #[test]
    fn test_quot_entity() {
        let p = parser();
        let result = p.format_inline("He said &quot;hello&quot;");
        assert_eq!(result, "He said \"hello\"");
    }

    #[test]
    fn test_apos_entity() {
        let p = parser();
        let result = p.format_inline("It&apos;s great");
        assert_eq!(result, "It's great");
    }

    // Whitespace
    #[test]
    fn test_nbsp_entity() {
        let p = parser();
        let result = p.format_inline("Hello&nbsp;World");
        assert_eq!(result, "Hello\u{00A0}World");
    }

    // Typographic entities
    #[test]
    fn test_ndash_entity() {
        let p = parser();
        let result = p.format_inline("pages 10&ndash;20");
        assert_eq!(result, "pages 10–20");
    }

    #[test]
    fn test_mdash_entity() {
        let p = parser();
        let result = p.format_inline("Wait&mdash;what?");
        assert_eq!(result, "Wait—what?");
    }

    #[test]
    fn test_hellip_entity() {
        let p = parser();
        let result = p.format_inline("To be continued&hellip;");
        assert_eq!(result, "To be continued…");
    }

    #[test]
    fn test_curly_quotes() {
        let p = parser();
        let result = p.format_inline("&ldquo;Hello&rdquo; and &lsquo;hi&rsquo;");
        assert_eq!(result, "\u{201C}Hello\u{201D} and \u{2018}hi\u{2019}");
    }

    #[test]
    fn test_bull_entity() {
        let p = parser();
        let result = p.format_inline("Item &bull; Item");
        assert_eq!(result, "Item • Item");
    }

    // Symbols
    #[test]
    fn test_copy_entity() {
        let p = parser();
        let result = p.format_inline("&copy; 2024");
        assert_eq!(result, "© 2024");
    }

    #[test]
    fn test_reg_entity() {
        let p = parser();
        let result = p.format_inline("Brand&reg;");
        assert_eq!(result, "Brand®");
    }

    #[test]
    fn test_trade_entity() {
        let p = parser();
        let result = p.format_inline("Product&trade;");
        assert_eq!(result, "Product™");
    }

    #[test]
    fn test_deg_entity() {
        let p = parser();
        let result = p.format_inline("90&deg;");
        assert_eq!(result, "90°");
    }

    #[test]
    fn test_math_entities() {
        let p = parser();
        let result = p.format_inline("5 &plusmn; 2, 3 &times; 4, 10 &divide; 2");
        assert_eq!(result, "5 ± 2, 3 × 4, 10 ÷ 2");
    }

    // Fractions
    #[test]
    fn test_fraction_entities() {
        let p = parser();
        let result = p.format_inline("&frac14; + &frac12; = &frac34;");
        assert_eq!(result, "¼ + ½ = ¾");
    }

    // Currency
    #[test]
    fn test_currency_entities() {
        let p = parser();
        let result = p.format_inline("&cent; &pound; &euro; &yen;");
        assert_eq!(result, "¢ £ € ¥");
    }

    // Arrows
    #[test]
    fn test_arrow_entities() {
        let p = parser();
        let result = p.format_inline("&larr; &rarr; &uarr; &darr;");
        assert_eq!(result, "← → ↑ ↓");
    }

    // Numeric entities (decimal)
    #[test]
    fn test_numeric_decimal_entity() {
        let p = parser();
        let result = p.format_inline("&#169; &#8212;");
        assert_eq!(result, "© —");
    }

    // Numeric entities (hex)
    #[test]
    fn test_numeric_hex_entity() {
        let p = parser();
        let result = p.format_inline("&#x00A9; &#x2014;");
        assert_eq!(result, "© —");
    }

    #[test]
    fn test_numeric_hex_uppercase() {
        let p = parser();
        let result = p.format_inline("&#X00A9;");
        assert_eq!(result, "©");
    }

    // Entity without semicolon (common in wild markdown)
    #[test]
    fn test_entity_without_semicolon() {
        let p = parser();
        // The space after &nbsp should be preserved
        let result = p.format_inline("Hello&nbsp world");
        assert_eq!(result, "Hello\u{00A0} world");
    }

    // Unknown entity should be preserved
    #[test]
    fn test_unknown_entity_preserved() {
        let p = parser();
        let result = p.format_inline("Hello &unknown; world");
        assert_eq!(result, "Hello &unknown; world");
    }

    // Entity mixed with markdown formatting
    #[test]
    fn test_entity_with_bold() {
        let p = parser();
        let result = p.format_inline("**Tom &amp; Jerry**");
        assert!(result.contains("\x1b[1m")); // bold
        assert_eq!(strip_ansi(&result), "Tom & Jerry");
    }

    // Multiple entities in sequence
    #[test]
    fn test_multiple_entities() {
        let p = parser();
        let result = p.format_inline("&lt;&lt; &amp;&amp; &gt;&gt;");
        assert_eq!(result, "<< && >>");
    }

    // Edge case: ampersand alone
    #[test]
    fn test_ampersand_alone() {
        let p = parser();
        let result = p.format_inline("Tom & Jerry");
        assert_eq!(result, "Tom & Jerry");
    }

    // Edge case: ampersand at end of string
    #[test]
    fn test_ampersand_at_end() {
        let p = parser();
        let result = p.format_inline("Test &");
        assert_eq!(result, "Test &");
    }
}
