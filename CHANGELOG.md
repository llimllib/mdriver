# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2026-01-15

- feat: auto-populate changelog and add shellcheck CI (#19)
- feat: add SVG image support via resvg (#18)
- fix: preserve image aspect ratio by letting Kitty calculate row count (#17)
- fix: prevent image markdown from being broken by line wrapping (#16)
- fix: use configured width for horizontal rules instead of terminal detection (#15)
- feat: strip HTML comments from markdown output (#14)
- feat: add citation-style reference link support (#13)
- feat: fix image-inside-link bug and add parallel image prefetching (#12)
- feat: add HTML <img> tag support and fix kitty image sizing (#11)
- feat: add HTML entity support (#10)
- feat: add red foreground color to inline code for better visibility (#9)

## [0.8.0] - 2026-01-12

## [0.8.0] - 2026-01-12

- Add `--version` flag

## [0.7.0] - 2026-01-12

## [0.6.1] - 2026-01-12

### Added

- release script

## [0.6.1] - 2026-01-12

### Fixed

- Exclude build artifacts from crates.io package

## [0.6.0] - 2026-01-12

### Fixed

- Restore heading color after inline formatting

## [0.5.1] - 2026-01-12

### Fixed

- Restore heading color after inline formatting

## [0.5.0] - 2026-01-12

### Added

- **Kitty Graphics Protocol**: Image rendering support for terminals that support the Kitty graphics protocol
- **Horizontal Rules**: Support for thematic breaks (horizontal rules) using `---`, `***`, or `___`
- **JSX and TSX Syntax Highlighting**: Enhanced syntax highlighting for React files
- **HTML Tag Support**: Inline HTML tags are now supported in markdown
- **Output Width Control**: New `--width` option with word-aware line wrapping
- **Two-face Syntax Highlighting**: Switched to two-face library for extended language support

### Fixed

- Code fence indentation handling
- Multi-paragraph list items with blank lines
- OSC8 hyperlinks in line width calculation

### Changed

- Hyperlinks now styled with blue color and underline
- Refactored unit tests from lib.rs to tests/unit.rs

## [0.4.4] - 2026-01-06

- parse link titles
- support setext headings
- support hard line breaks
- support nested lists
- support strikethrough
- support blockquotes

## [0.4.3] - 2026-01-05

### Fixed

- Removed legacy `mdstream` symlink that was breaking cargo publish

## [0.4.2] - 2026-01-05

### Fixed

- Added `--allow-dirty` flag to cargo publish to handle downloaded artifact directories

## [0.4.1] - 2026-01-05

### Fixed

- Removed x86_64-unknown-linux-musl target from release workflow due to onig_sys compilation issues

## [0.4.0] - 2026-01-05

### Changed

- Renamed `MDSTREAM_THEME` environment variable to `MDRIVER_THEME` for consistency
- Updated all documentation to use `mdriver` instead of `mdstream`

### Fixed

- Updated GitHub Actions workflows to use non-deprecated action versions (v4/v6)

## [0.3.0] - 2026-01-05

### Added

- **GitHub Flavored Markdown Tables**: Full GFM table support with Unicode box-drawing characters
  - Column alignment support (left `:---`, center `:---:`, right `---:`)
  - Inline markdown formatting within table cells (bold, italic, code, links)
  - ANSI-aware column width calculation for proper alignment
  - Paragraph promotion pattern to detect tables on delimiter row
- Comprehensive table test coverage with 4 new fixtures

## [0.2.0] - 2026-01-05

### Changed

- **Package renamed** from `mdstream` to `mdriver` for crates.io publication (the name `mdstream` was already taken)

### Added

- **Syntax Highlighting**: Code blocks now feature full syntax highlighting for 100+ languages using syntect
- **Configurable Themes**: Choose from multiple color schemes via `--theme` flag or `MDRIVER_THEME` environment variable
  - Available themes: InspiredGitHub, Solarized (dark/light), base16-ocean.dark, base16-mocha.dark, and more
  - Use `--list-themes` to see all available themes
- **OSC8 Hyperlinks**: Markdown links `[text](url)` are converted to clickable terminal hyperlinks (OSC8 protocol)
- **Nested Inline Formatting**: Support for nested formatting like `**`bold code`**`
- **CLI Enhancements**:
  - `--help` flag with comprehensive usage documentation
  - File path argument support (e.g., `mdriver README.md`)
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
- Inline formatting: **bold**, _italic_, `code`
- ANSI color output for terminal rendering
- Comprehensive conformance test suite (TOML-based fixtures)

[Unreleased]: https://github.com/llimllib/mdriver/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/llimllib/mdriver/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/llimllib/mdriver/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/llimllib/mdriver/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/llimllib/mdriver/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/llimllib/mdriver/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/llimllib/mdriver/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/llimllib/mdriver/compare/v0.4.4...v0.5.0
[0.4.4]: https://github.com/llimllib/mdriver/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/llimllib/mdriver/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/llimllib/mdriver/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/llimllib/mdriver/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/llimllib/mdriver/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/llimllib/mdriver/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/llimllib/mdriver/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/llimllib/mdriver/releases/tag/v0.1.0
