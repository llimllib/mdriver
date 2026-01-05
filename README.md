# mdriver - Streaming Markdown Printer

[![CI](https://github.com/llimllib/mdstream/actions/workflows/ci.yml/badge.svg)](https://github.com/llimllib/mdstream/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/mdriver.svg)](https://crates.io/crates/mdriver)

A streaming markdown printer for the terminal that renders GitHub Flavored Markdown with ANSI escape codes. The key feature is **incremental emission**: blocks are emitted immediately once parsed, not waiting for the entire document.

## Installation

### From crates.io (Recommended)

```bash
cargo install mdriver
```

### From Pre-built Binaries

Download the latest release for your platform from the [GitHub Releases](https://github.com/llimllib/mdstream/releases) page:

- **Linux**: `mdriver-x86_64-unknown-linux-gnu.tar.gz` or `mdriver-x86_64-unknown-linux-musl.tar.gz`
- **macOS**: `mdriver-x86_64-apple-darwin.tar.gz` (Intel) or `mdriver-aarch64-apple-darwin.tar.gz` (Apple Silicon)

Extract and add to your PATH:
```bash
tar xzf mdriver-*.tar.gz
sudo mv mdriver /usr/local/bin/
```

### From Source

```bash
git clone https://github.com/llimllib/mdstream.git
cd mdstream
cargo build --release
```

The binary will be available at `target/release/mdriver`.

## Usage

```bash
# Read from file
mdriver README.md

# Pipe markdown from a file
cat document.md | mdriver

# Pipe from echo
echo "# Hello World" | mdriver

# Redirect from file
mdriver < document.md

# Use a specific syntax highlighting theme
mdriver --theme "InspiredGitHub" README.md

# Set default theme via environment variable
MDSTREAM_THEME="Solarized (dark)" mdriver README.md

# List available themes
mdriver --list-themes

# Show help
mdriver --help
```

### Example

```bash
$ echo "# Demo\n\nThis is **bold** and *italic*." | mdriver
```

Output (with ANSI colors in your terminal):

- **# Demo** (in blue and bold)
- This is **bold** and _italic_

## Features

- ✅ **Streaming**: Renders markdown incrementally as it arrives
- ✅ **ATX Headings**: `# Heading` with blue/bold formatting
- ✅ **Paragraphs**: Text blocks with inline formatting
- ✅ **Code Blocks**: Fenced blocks with ` ``` ` and syntax highlighting
- ✅ **Lists**: Unordered (`-`) and ordered (`1.`) lists
- ✅ **Inline Formatting**: `**bold**`, `*italic*`, `` `code` `` with nested support
- ✅ **Hyperlinks**: `[text](url)` converted to clickable OSC8 terminal links
- ✅ **Syntax Highlighting**: 100+ languages supported with customizable themes
- ✅ **ANSI Colors**: Beautiful terminal output with 24-bit true color
- ✅ **Zero Warnings**: Strict clippy linting, no compiler warnings

## Syntax Highlighting Themes

mdstream uses the [syntect](https://github.com/trishume/syntect) library for syntax highlighting, supporting 100+ languages with customizable color themes.

### Available Themes

Use `mdstream --list-themes` to see all available themes. Popular options include:

- **InspiredGitHub** - Bright, vibrant colors inspired by GitHub's syntax highlighting
- **Solarized (dark)** - The classic Solarized dark color scheme
- **Solarized (light)** - Solarized optimized for light backgrounds
- **base16-ocean.dark** - Calm oceanic colors (default)
- **base16-mocha.dark** - Warm mocha tones
- **base16-eighties.dark** - Retro 80s aesthetic

### Setting a Theme

There are three ways to configure the theme (in order of precedence):

1. **Command-line flag**: `mdstream --theme "InspiredGitHub" file.md`
2. **Environment variable**: `export MDSTREAM_THEME="Solarized (dark)"`
3. **Default**: `base16-ocean.dark`

### Example

```bash
# Use InspiredGitHub theme
mdstream --theme "InspiredGitHub" README.md

# Set environment variable for persistent default
export MDSTREAM_THEME="Solarized (dark)"
mdstream README.md

# Combine with piping
MDSTREAM_THEME="base16-mocha.dark" cat file.md | mdstream
```

## Conformance Test Suite

This project uses a comprehensive conformance test suite to verify streaming behavior, markdown parsing, and ANSI formatting.

### Test Structure

Tests are written as TOML fixture files that specify:

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

### Writing New Tests

1. Create a `.toml` file in the appropriate `tests/fixtures/` subdirectory
2. Define test name and description
3. Add chunks with input and expected emissions
4. Use `\u001b` for ESC character in ANSI codes

Example ANSI codes:

- Bold: `\u001b[1m...\u001b[0m`
- Italic: `\u001b[3m...\u001b[0m`
- Color: `\u001b[1;34m...\u001b[0m` (bold blue)
- Background: `\u001b[48;5;235m...\u001b[0m`

### Current Status

**All Systems Operational** ✅

- ✅ **Parser Implementation**: Complete with full streaming support
- ✅ **Test Suite**: 8 conformance tests - all passing
- ✅ **CLI**: Working binary for command-line usage
- ✅ **Code Quality**: Zero compiler warnings, zero clippy errors
- ✅ **Documentation**: Comprehensive CLAUDE.md for AI assistants

**Test Coverage**:
- Block types: heading, paragraph, code block, list
- Streaming: incremental emission, block boundaries
- Formatting: inline ANSI codes (bold, italic, code)
- Complex: mixed document scenarios

### Future Enhancements

Potential areas for expansion:

1. Additional GFM features (tables, task lists, strikethrough)
2. Blockquotes and nested structures
3. Terminal width awareness and text wrapping
4. Performance benchmarks for large documents
5. Image rendering using terminal graphics protocols

## Project Structure

```
mdstream/
├── Cargo.toml
├── README.md
├── CLAUDE.md              # AI assistant context and guidelines
├── gfmspec.md             # GitHub Flavored Markdown specification
├── .clippy.toml           # Clippy linting configuration
├── src/
│   ├── lib.rs             # StreamingParser implementation
│   └── main.rs            # CLI binary
└── tests/
    ├── conformance.rs     # Test runner
    ├── common/
    │   ├── mod.rs
    │   └── fixture_loader.rs
    └── fixtures/
        ├── blocks/        # 4 tests: heading, paragraph, code_block, list
        ├── streaming/     # 2 tests: incremental_emit, block_boundaries
        ├── ansi/          # 1 test: inline_formatting
        └── complex/       # 1 test: mixed_document
```

## Design Philosophy

1. **Streaming First**: Blocks emit immediately when complete, enabling real-time rendering
2. **Test-Driven**: Comprehensive test suite defines expected behavior before implementation
3. **Exact Output**: Tests verify exact ANSI codes, not just content
4. **Incremental Testing**: Tests verify streaming property, not just final output
5. **Zero Tolerance**: No compiler warnings, no clippy errors - strict code quality standards

## Development

### Code Quality Standards

This project maintains strict code quality requirements:

```bash
# All must pass before committing:
cargo build                                              # No warnings
cargo build --release                                    # No warnings
cargo clippy --all-targets --all-features -- -D warnings # No errors
cargo test                                               # All tests pass
```

See `CLAUDE.md` for comprehensive development guidelines and best practices.

### Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests first (TDD approach)
4. Implement feature to pass tests
5. Ensure all quality checks pass
6. Submit pull request

### Key Files

- **`CLAUDE.md`**: Comprehensive guide for AI assistants and developers
- **`gfmspec.md`**: GitHub Flavored Markdown specification (authoritative source)
- **`.clippy.toml`**: Linting configuration
- **`tests/fixtures/`**: Conformance test cases in TOML format

## License

MIT
