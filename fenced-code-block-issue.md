# Bug: Fenced code blocks don't interrupt paragraphs

## Summary

mdriver fails to recognize fenced code blocks (` ``` `) when they appear immediately after paragraph text without a preceding blank line. Per the [CommonMark spec](https://spec.commonmark.org/0.31.2/#fenced-code-blocks), code fences **can** interrupt paragraphs.

## Reproduction

```bash
# BROKEN: no blank line before code fence
echo '**1. Built-in Name Shadowing**
```python
# Bad:
filter = DevOpsLabelFilter()

# Good:
filter_instance = DevOpsLabelFilter()
```' | mdriver --color always

# Output (visible text only):
# **1. Built-in Name Shadowing**
# `python
# # Bad:
# filter = DevOpsLabelFilter()
#
# # Good:
#
# filter_instance = DevOpsLabelFilter()
# `
```

```bash
# WORKS: blank line before code fence
echo '**1. Built-in Name Shadowing**

```python
# Bad:
filter = DevOpsLabelFilter()

# Good:
filter_instance = DevOpsLabelFilter()
```' | mdriver --color always

# Output: properly syntax-highlighted code block
```

Simplest repro:

```bash
echo 'Some text.
```python
x = 1
```' | mdriver --color always
```

## Where it happens in the code

`src/lib.rs`, `handle_in_paragraph()` (line 504). This method only checks three things that can end a paragraph:

1. A blank line → emits the paragraph
2. A setext heading underline (`===` or `---`)
3. A table delimiter row

It does **not** check for fenced code block openings. The ` ```python ` line gets appended to the paragraph as regular text, where the backticks are partially consumed by inline code formatting, leaving a single visible backtick.

By contrast, the `Ready` state handler (line 414) correctly detects code fences via `self.parse_code_fence(trimmed)`.

## Fix

Add a code fence check to `handle_in_paragraph()`, before the "add line to paragraph" fallthrough. When a code fence is found, emit the current paragraph and transition to `InCodeBlock`:

```rust
fn handle_in_paragraph(&mut self, line: &str) -> Option<String> {
    let trimmed = line.trim_end_matches('\n');

    // Blank line completes paragraph
    if trimmed.is_empty() {
        return self.emit_current_block();
    }

    // NEW: Check for fenced code block opening (``` or ~~~)
    // Per CommonMark spec, code fences can interrupt paragraphs
    if let Some((info, fence, indent_offset)) = self.parse_code_fence(trimmed) {
        let output = self.emit_current_block();
        self.state = ParserState::InCodeBlock {
            info: info.clone(),
            fence: fence.clone(),
            indent_offset,
        };
        self.current_block = BlockBuilder::CodeBlock {
            lines: Vec::new(),
            info,
        };
        return output;
    }

    // Check if this is a setext heading underline
    // ... rest of existing code ...
```

Note: you may also want to check whether ATX headings, blockquotes, horizontal rules, and list items can interrupt paragraphs in `handle_in_paragraph()` — the CommonMark spec says [they can](https://spec.commonmark.org/0.31.2/#paragraphs), and the same pattern of "no blank line before block element" would hit the same bug for those too.

## How I found this

pr-review pipes LLM-generated markdown summaries to mdriver via stdin. The LLM frequently generates markdown like:

```
**1. Built-in Name Shadowing**
```python
# Bad:
filter = DevOpsLabelFilter()
```

(no blank line between the bold text and the code fence)

The stored session data (`~/.cache/pr-review/019ce87f-f507-797c-a545-b05cac8d187a/reports.json`) confirms the markdown is correct — the issue is entirely in mdriver's parsing.
