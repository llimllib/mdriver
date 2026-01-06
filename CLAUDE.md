# CLAUDE.md - AI Assistant Context

This file provides context for AI assistants (like Claude) working on this codebase.

## Project Overview

**mdriver** is a streaming markdown printer that renders GitHub Flavored Markdown to the terminal with ANSI escape codes. The critical requirement is **incremental emission**: blocks must be emitted immediately once parsed, not buffered until the entire document is complete.

### Why This Is Interesting

Most markdown renderers are document-oriented: they parse the entire input and then render. This project requires **streaming** behavior where:
- Input arrives in arbitrary chunks (could be byte-by-byte, line-by-line, or paragraph-by-paragraph)
- Blocks emit as soon as they're complete (heading after `\n`, paragraph after `\n\n`, code block after closing fence)
- Incomplete blocks buffer without emitting
- The parser must maintain state across `feed()` calls

This is useful for real-time rendering of markdown as it's typed or received over a network.

## Code Quality Requirements

**CRITICAL**: This project maintains zero-tolerance for compiler warnings and linting errors.

### Mandatory Checks Before Committing

1. **Code Formatting**
   ```bash
   cargo fmt
   ```
   Must be run before committing. The CI checks that code is properly formatted with `cargo fmt -- --check`.

2. **No Compiler Warnings**
   ```bash
   cargo build
   cargo build --release
   ```
   Must complete with zero warnings. If warnings appear, fix them immediately.

3. **No Clippy Errors**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
   Must pass with zero errors. Clippy runs with `-D warnings` which treats warnings as errors.

4. **All Tests Pass**
   ```bash
   cargo test
   ```
   All conformance tests must pass.

### Common Clippy Fixes

- **Unused variables**: Remove them or prefix with `_`
- **Unused imports**: Remove them
- **Manual string stripping**: Use `strip_prefix()` instead of `starts_with()` + slicing
- **Unnecessary `mut`**: Remove `mut` if variable is never mutated
- **Dead code**: Add `#[allow(dead_code)]` only if keeping for future use

### When to Allow Warnings

Only use `#[allow(...)]` when:
- Code is intentionally kept for future features (e.g., `info` field for syntax highlighting)
- The lint is a false positive (rare, document why)

**Never commit code without running `cargo fmt`, or with warnings or clippy errors.**

## Test-Driven Development Approach

**IMPORTANT**: This project uses **conformance tests** that define exact expected behavior. The test suite was created BEFORE implementation.

### Test Philosophy

The conformance tests serve three purposes:
1. **Specification**: Define exactly how streaming should work
2. **Verification**: Ensure implementation matches spec
3. **Regression prevention**: Prevent streaming behavior from breaking

### Running Tests

```bash
# All tests (currently failing - parser is stub)
cargo test

# Specific categories
cargo test test_block_fixtures      # Basic block types
cargo test test_streaming_fixtures  # Streaming behavior
cargo test test_ansi_fixtures       # Terminal formatting
cargo test test_complex_fixtures    # Real-world scenarios
```

## Test Fixture Format

Tests are TOML files in `tests/fixtures/`. Each test specifies:

```toml
name = "test-name"
description = "What this tests"

[[chunks]]
input = "markdown input"
emit = "expected output"  # Empty string "" means "no emission yet"
```

### ANSI Escape Codes in Tests

**CRITICAL**: Use `\u001b` NOT `\x1b` in TOML files!

```toml
# ✅ CORRECT
emit = "\u001b[1mbold\u001b[0m"

# ❌ WRONG - TOML doesn't support \x escape
emit = "\x1b[1mbold\x1b[0m"
```

Common ANSI codes:
- **Bold**: `\u001b[1m...\u001b[0m`
- **Italic**: `\u001b[3m...\u001b[0m`
- **Blue heading**: `\u001b[1;34m...\u001b[0m`
- **Code background**: `\u001b[48;5;235m...\u001b[0m`

## GitHub Flavored Markdown Specification

**CRITICAL**: The authoritative specification for parsing behavior is in `gfmspec.md`.

### When to Consult the Spec

**Always reference `gfmspec.md` when**:
- Implementing a new block type (headings, lists, code blocks, tables, etc.)
- Implementing inline formatting (emphasis, links, code spans, etc.)
- Handling edge cases or ambiguous input
- Unsure about parsing precedence or rules
- Debugging why a test expects certain output

### How to Use the Spec

The GFM spec is organized by feature. Use the Read tool to look up specific sections:

**Block-level structures**:
- ATX headings (lines starting with `#`)
- Fenced code blocks (` ``` `)
- Paragraphs and blank lines
- Lists (ordered and unordered)
- Block quotes
- Tables
- Thematic breaks

**Inline structures**:
- Emphasis and strong emphasis (`*` and `**`)
- Code spans (`` ` ``)
- Links and images
- Autolinks
- Strikethrough (GFM extension)

**Examples from spec**:
- Search for specific examples (e.g., "Example 32" in the spec)
- Look at edge cases to understand boundary conditions
- Check how nesting and precedence work

### Parsing Strategy

The spec describes the **final output** of parsing, not the algorithm. For our streaming parser:

1. **Read the spec** to understand what output should look like
2. **Design state machine** to detect when blocks complete
3. **Implement incrementally** - our tests are based on spec behavior
4. **Handle edge cases** as defined in spec examples

### Important Spec Sections for Streaming

**Block boundary detection** is key for streaming:
- Blank lines often terminate blocks
- Some blocks (code fences) need explicit closing
- Some blocks (paragraphs) can be interrupted by other blocks
- Understanding these rules is critical for knowing when to emit

**Container blocks** (blockquotes, lists) can contain other blocks:
- May need nested state tracking
- Our initial implementation can skip these if too complex

## Architecture

### Core API

```rust
pub struct StreamingParser {
    buffer: String,  // Accumulates incomplete blocks
    // TODO: Add parser state
}

impl StreamingParser {
    /// Feed a chunk of markdown
    /// Returns any completed blocks as formatted output
    pub fn feed(&mut self, chunk: &str) -> String;

    /// Flush remaining buffered content
    pub fn flush(&mut self) -> String;
}
```

### Key Implementation Challenges

1. **State Management**: Parser must track what type of block it's currently building
   - In heading? In paragraph? In code block? In list?
   - Need state machine or parser state tracking

2. **Block Boundary Detection**:
   - Heading: Complete after `\n`
   - Paragraph: Complete after `\n\n` (blank line)
   - Code block: Complete after closing ` ``` `
   - List: Complete after blank line or different block type
   - Blockquote: Complete when exited

3. **Incremental Parsing**: Can't use traditional single-pass parsers
   - Input is chunked arbitrarily
   - Must handle partial blocks
   - Examples:
     - Chunk 1: `"#"` → No emission (heading incomplete)
     - Chunk 2: `" Hello"` → No emission (still no newline)
     - Chunk 3: `"\n"` → Emit heading now!

4. **ANSI Formatting**: Convert markdown to terminal codes
   - Inline: `**bold**` → `\u001b[1mbold\u001b[0m`
   - Block: Different formatting for headings, code, etc.
   - Must preserve semantic structure in terminal output

## Implementation Strategy

### Recommended Approach

1. **Start Simple**: Make basic tests pass first
   - Heading emission (test: `tests/fixtures/blocks/heading.toml`)
   - Paragraph emission (test: `tests/fixtures/blocks/paragraph.toml`)

2. **State Machine**: Build a state tracker
   ```rust
   enum ParserState {
       Ready,           // Not in any block
       InHeading,
       InParagraph,
       InCodeBlock { language: Option<String> },
       InList,
       // etc.
   }
   ```

3. **Chunk Processing**: Process incoming chunks character-by-character or line-by-line
   - Detect block boundaries
   - Emit when complete
   - Buffer when incomplete

4. **Consider Using pulldown-cmark**:
   - Excellent markdown parser, but designed for complete documents
   - You may need to wrap it or use it differently for streaming
   - Alternatively, implement a custom streaming parser

### Potential Libraries

- `pulldown-cmark`: Full markdown parser (may need adaptation)
- `termion` or `crossterm`: Terminal handling
- `syntect`: Syntax highlighting for code blocks

## File Structure

```
mdriver/
├── Cargo.toml
├── README.md           # User-facing documentation
├── CLAUDE.md          # This file - AI assistant context
├── src/
│   └── lib.rs         # StreamingParser (currently stub)
├── tests/
│   ├── conformance.rs           # Test runner
│   ├── common/
│   │   ├── mod.rs
│   │   └── fixture_loader.rs   # Loads TOML fixtures
│   └── fixtures/
│       ├── blocks/              # Basic block tests
│       ├── streaming/           # Incremental emission tests
│       ├── ansi/                # Formatting tests
│       └── complex/             # Integration tests
```

## Adding New Tests

When adding features, add tests FIRST:

1. Create `.toml` file in appropriate `tests/fixtures/` directory
2. Define chunks that test the streaming behavior
3. Specify exact expected output with ANSI codes
4. Run tests to see them fail
5. Implement feature to make tests pass

Example test structure:
```toml
name = "new-feature"
description = "Tests new markdown feature"

[[chunks]]
input = "partial input"
emit = ""  # Not complete yet

[[chunks]]
input = " complete\n\n"
emit = "formatted output\n"  # Now it emits
```

## Common Pitfalls

1. **TOML Escape Sequences**: Always use `\u001b` for ESC, not `\x1b`
2. **Test Directory Structure**: Tests in `tests/` are integration tests, need `mod common` not `mod helpers`
3. **Streaming vs Document**: Don't assume complete input - handle partial blocks
4. **Empty Emissions**: Tests explicitly check for NO emission with `emit = ""`
5. **ANSI Reset**: Always reset with `\u001b[0m` after formatting

## Current Status

- ✅ **Test Infrastructure**: Complete and working (8 tests)
- ✅ **Test Categories**: Blocks, streaming, ANSI, complex
- ✅ **Test Runner**: Loads TOML, feeds chunks, validates output
- ❌ **Parser Implementation**: Stub only (all tests fail)

## Next Implementation Steps

1. **Basic Block Detection** (src/lib.rs):
   - Track parser state
   - Detect heading completion (after `\n`)
   - Emit formatted heading with ANSI codes
   - Pass `tests/fixtures/blocks/heading.toml`

2. **Paragraph Support**:
   - Detect paragraph completion (after `\n\n`)
   - Pass `tests/fixtures/blocks/paragraph.toml`

3. **Code Blocks**:
   - Detect opening/closing fences
   - Buffer content
   - Emit only when closed
   - Pass `tests/fixtures/blocks/code_block.toml`

4. **Incremental Emission**:
   - Ensure blocks emit separately
   - Pass `tests/fixtures/streaming/incremental_emit.toml`

5. **Inline Formatting**:
   - Parse `**bold**`, `*italic*`, `` `code` ``
   - Generate ANSI codes
   - Pass `tests/fixtures/ansi/inline_formatting.toml`

## Questions to Consider

When implementing, think about:

1. **How much to buffer?**
   - Need enough context to parse correctly
   - But emit as soon as possible

2. **Error handling?**
   - What if invalid markdown?
   - Should malformed input emit or buffer?

3. **Performance?**
   - Byte-by-byte feeding should work but might be slow
   - Can you optimize without breaking streaming?

4. **GFM Extensions?**
   - Tables, task lists, strikethrough
   - Add tests first, then implement

## Testing Your Implementation

The test output is very helpful:

```
✗ heading-basic
  Heading should emit after newline is received
  Chunk 4 failed:
  Input: "\n"
  Expected: "\u{1b}[1;34m# Hello\u{1b}[0m\n"
  Actual: ""
```

This tells you:
- Which test failed (`heading-basic`)
- What it's testing (description)
- Which chunk failed (4th chunk)
- What was fed (`"\n"`)
- What should have been emitted (formatted heading with ANSI)
- What actually was emitted (nothing)

## Philosophy

This project is about **streaming** and **incremental rendering**, not just markdown parsing. The conformance tests encode this philosophy: they verify not just final output, but the timing and chunking of emissions.

Keep this principle in mind: **Emit as soon as you can, not when you must.**
