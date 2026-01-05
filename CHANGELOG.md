# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-01-05

### Changed
- **Package renamed** from `mdstream` to `mdriver` for crates.io publication (the name `mdstream` was already taken)

### Added
- **Syntax Highlighting**: Code blocks now feature full syntax highlighting for 100+ languages using syntect
- **Configurable Themes**: Choose from multiple color schemes via `--theme` flag or `MDSTREAM_THEME` environment variable
  - Available themes: InspiredGitHub, Solarized (dark/light), base16-ocean.dark, base16-mocha.dark, and more
  - Use `--list-themes` to see all available themes
- **OSC8 Hyperlinks**: Markdown links `[text](url)` are converted to clickable terminal hyperlinks (OSC8 protocol)
- **Nested Inline Formatting**: Support for nested formatting like `**`bold code`**`
- **CLI Enhancements**:
  - `--help` flag with comprehensive usage documentation
  - File path argument support (e.g., `mdstream README.md`)
  - `--list-themes` to display available syntax highlighting themes
  - `--theme <THEME>` to specify syntax highlighting theme

### Fixed
- ANSI color bleeding after code blocks (added proper reset codes)
- Syntax highlighting state management for bash comments and multi-line code

### Changed
- Code blocks now use 4-space indentation (previously 1 space)
- Code block background removed in favor of syntax highlighting colors
- Default theme set to `base16-ocean.dark`

## [0.1.0] - 2026-01-04

### Added
- Initial release
- Streaming markdown parser with incremental emission
- Support for ATX headings (# through ######)
- Paragraph rendering with inline formatting
- Code blocks (fenced with ```)
- Unordered and ordered lists
- Inline formatting: **bold**, *italic*, `code`
- ANSI color output for terminal rendering
- Comprehensive conformance test suite (TOML-based fixtures)

[Unreleased]: https://github.com/llimllib/mdstream/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/llimllib/mdstream/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/llimllib/mdstream/releases/tag/v0.1.0
