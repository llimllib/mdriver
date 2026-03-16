# Mermaid Diagram Rendering Design

**Date:** 2026-03-16

## Overview

Render mermaid code blocks as inline terminal images (via Kitty protocol) instead of syntax-highlighted code, using the `mermaid-rs-renderer` crate for SVG generation.

## Behavior

- **Image mode enabled** (`ImageProtocol::Kitty` or future protocols): mermaid code blocks are rendered as images via `mermaid-rs-renderer::render()` → SVG → existing `process_image()` pipeline.
- **Image mode disabled** (`ImageProtocol::None`): mermaid code blocks display as normal syntax-highlighted code (no change).
- **Render failure** (parse error, unsupported diagram, timeout): fall back to syntax-highlighted code block. Silent — no error messages shown to user.

## Timeout

A hardcoded 5-second timeout prevents complex diagrams from blocking the streaming output:

```rust
const MERMAID_RENDER_TIMEOUT: Duration = Duration::from_secs(5);
```

Rendering runs on a spawned thread; if it doesn't complete within the timeout, the mermaid source is shown as a regular code block.

## Dependency

```toml
mermaid-rs-renderer = { version = "0.2", default-features = false }
```

SVG-only — no CLI, no PNG feature. Reuses mdriver's existing `resvg` for SVG→raster conversion.

## Code Changes

1. **`Cargo.toml`**: Add `mermaid-rs-renderer` dependency.
2. **`src/lib.rs`**: In `format_code_block()`, intercept `info == "mermaid"` when `image_protocol != None`. Attempt render with timeout, fall back on failure.

## Pipeline

```
mermaid source text
  → mermaid_rs_renderer::render() → SVG string
  → SVG bytes → render_svg() → DynamicImage
  → process_image() → Kitty protocol escape sequences
```
