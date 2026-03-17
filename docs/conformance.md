# Test Suite

This project uses a comprehensive conformance test suite to verify streaming behavior, markdown parsing, and ANSI formatting, and unit tests to verify application-specific behavior.

## Conformance Test Structure

Conformance tests are written as TOML fixture files that specify:

- **Input chunks**: Markdown arriving incrementally (simulating streaming)
- **Expected emissions**: What should be output after each chunk (empty string if block incomplete)
- **Raw ANSI codes**: Actual escape sequences for exact terminal output matching

### Test Format Example

```toml
name = "heading-basic"
description = "Heading should emit after newline is received"

[[chunks]]
input = "#"
emit = ""

[[chunks]]
input = " Hello"
emit = ""

[[chunks]]
input = "\n"
emit = "\u001b[1;34m# Hello\u001b[0m\n"
```

**Key Points**:

- Each `[[chunks]]` represents a piece of markdown fed to the parser
- `input`: The markdown chunk
- `emit`: Expected terminal output (empty `""` means no emission yet)
- ANSI codes use `\u001b` format (TOML Unicode escape)

### Test Categories

Tests are organized in `tests/fixtures/`:

1. **`blocks/`** - Individual block types (headings, paragraphs, code blocks, lists)
2. **`streaming/`** - Incremental emission and block boundary detection
3. **`ansi/`** - ANSI escape sequence formatting (bold, italic, colors)
4. **`complex/`** - Real-world documents with mixed block types

### Running Tests

```bash
# Run all conformance tests
cargo test

# Run specific test category
cargo test test_block_fixtures
cargo test test_streaming_fixtures
cargo test test_ansi_fixtures
cargo test test_complex_fixtures

# Run with verbose output
cargo test -- --nocapture
```

### Test Output

When tests fail, you see clear diagnostics:

```
Running 4 tests from blocks...
  ✗ heading-basic
    Heading should emit after newline is received
    Chunk 4 failed:
  Input: "\n"
  Expected: "\u{1b}[1;34m# Hello\u{1b}[0m\n"
  Actual: ""
```

### Writing New Conformance Tests

1. Create a `.toml` file in the appropriate `tests/fixtures/` subdirectory
2. Define test name and description
3. Add chunks with input and expected emissions
4. Use `\u001b` for ESC character in ANSI codes

Example ANSI codes:

- Bold: `\u001b[1m...\u001b[0m`
- Italic: `\u001b[3m...\u001b[0m`
- Color: `\u001b[1;34m...\u001b[0m` (bold blue)
- Background: `\u001b[48;5;235m...\u001b[0m`

## Unit tests

The unit tests live in `tests/unit.rs`
