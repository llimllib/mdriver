use mdriver::StreamingParser;
use std::env;
use std::fs::File;
use std::io::{self, ErrorKind, IsTerminal, Read, Write};

fn print_version() {
    println!("mdriver {}", env!("CARGO_PKG_VERSION"));
    println!("rustc: {}", env!("RUSTC_VERSION"));
}

fn print_help() {
    println!("mdriver - Streaming Markdown Printer");
    println!();
    println!("USAGE:");
    println!("    mdriver [OPTIONS] [FILE]");
    println!();
    println!("OPTIONS:");
    println!("    --version, -V       Print version information");
    println!("    --help              Print this help message");
    println!("    --list-themes       List available syntax highlighting themes");
    println!("    --theme <THEME>     Use specified syntax highlighting theme");
    println!("    --images <PROTOCOL> Enable image rendering (protocols: kitty)");
    println!("    --width <N>         Set output width for line wrapping (default: min(terminal width, 80))");
    println!("    --color <WHEN>      When to use colors: auto, always, never (default: auto)");
    println!();
    println!("ARGS:");
    println!("    <FILE>              Markdown file to render (reads from stdin if not provided)");
    println!();
    println!("ENVIRONMENT:");
    println!("    MDRIVER_THEME       Default syntax highlighting theme (overridden by --theme)");
    println!("    MDRIVER_WIDTH       Default output width (overridden by --width)");
    println!();
    println!("EXAMPLES:");
    println!("    mdriver README.md");
    println!("    mdriver --theme \"Solarized (dark)\" README.md");
    println!("    mdriver --images kitty document.md");
    println!("    mdriver --width 100 document.md");
    println!("    mdriver --color=always README.md | less -R");
    println!("    cat file.md | mdriver");
    println!("    MDRIVER_THEME=\"InspiredGitHub\" mdriver file.md");
}

/// Color output mode
#[derive(Clone, Copy, PartialEq)]
enum ColorMode {
    Auto,   // Color if stdout is a terminal
    Always, // Always use color
    Never,  // Never use color (pass through unchanged)
}

/// Run the main logic, returning a Result for error handling
fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut theme: Option<String> = None;
    let mut width: Option<usize> = None;
    let mut image_protocol = mdriver::ImageProtocol::None;
    let mut color_mode = ColorMode::Auto;
    let mut file_path: Option<String> = None;
    let mut i = 1;

    while i < args.len() {
        // Handle --color=value syntax
        if args[i].starts_with("--color=") {
            let value = &args[i]["--color=".len()..];
            match value {
                "auto" => color_mode = ColorMode::Auto,
                "always" => color_mode = ColorMode::Always,
                "never" => color_mode = ColorMode::Never,
                _ => {
                    eprintln!(
                        "Error: Unknown color mode '{}'. Use auto, always, or never.",
                        value
                    );
                    eprintln!("Run 'mdriver --help' for usage information");
                    std::process::exit(1);
                }
            }
            i += 1;
            continue;
        }

        match args[i].as_str() {
            "--version" | "-V" => {
                print_version();
                return Ok(());
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--list-themes" => {
                println!("Available syntax highlighting themes:");
                for theme_name in StreamingParser::list_themes() {
                    println!("  {}", theme_name);
                }
                return Ok(());
            }
            "--theme" => {
                if i + 1 < args.len() {
                    theme = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --theme requires a theme name");
                    eprintln!("Run 'mdriver --help' for usage information");
                    std::process::exit(1);
                }
            }
            "--width" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(w) if w > 0 => {
                            width = Some(w);
                            i += 2;
                        }
                        _ => {
                            eprintln!("Error: --width requires a positive integer");
                            eprintln!("Run 'mdriver --help' for usage information");
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: --width requires a number");
                    eprintln!("Run 'mdriver --help' for usage information");
                    std::process::exit(1);
                }
            }
            "--images" => {
                if i + 1 < args.len() {
                    match args[i + 1].as_str() {
                        "kitty" => image_protocol = mdriver::ImageProtocol::Kitty,
                        protocol => {
                            eprintln!("Error: Unknown image protocol '{}'", protocol);
                            eprintln!("Supported protocols: kitty");
                            eprintln!("Run 'mdriver --help' for usage information");
                            std::process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --images requires a protocol name");
                    eprintln!("Run 'mdriver --help' for usage information");
                    std::process::exit(1);
                }
            }
            "--color" => {
                if i + 1 < args.len() {
                    match args[i + 1].as_str() {
                        "auto" => color_mode = ColorMode::Auto,
                        "always" => color_mode = ColorMode::Always,
                        "never" => color_mode = ColorMode::Never,
                        value => {
                            eprintln!(
                                "Error: Unknown color mode '{}'. Use auto, always, or never.",
                                value
                            );
                            eprintln!("Run 'mdriver --help' for usage information");
                            std::process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --color requires a mode (auto, always, never)");
                    eprintln!("Run 'mdriver --help' for usage information");
                    std::process::exit(1);
                }
            }
            arg if !arg.starts_with('-') => {
                file_path = Some(arg.to_string());
                i += 1;
            }
            unknown => {
                eprintln!("Error: Unknown option '{}'", unknown);
                eprintln!("Run 'mdriver --help' for usage information");
                std::process::exit(1);
            }
        }
    }

    // Determine if we should use color/formatting
    let use_color = match color_mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => io::stdout().is_terminal(),
    };

    let mut buffer = [0u8; 4096];

    // Read from file or stdin
    let mut reader: Box<dyn Read> = if let Some(path) = file_path {
        Box::new(File::open(path)?)
    } else {
        Box::new(io::stdin())
    };

    let mut stdout = io::stdout().lock();

    if use_color {
        // Get theme from parameter, environment variable, or use default
        let theme = theme
            .or_else(|| env::var("MDRIVER_THEME").ok())
            .unwrap_or_else(|| "base16-ocean.dark".to_string());

        // Get width from parameter, environment variable, or use default
        let width = width.or_else(|| env::var("MDRIVER_WIDTH").ok().and_then(|s| s.parse().ok()));

        let mut parser = if let Some(w) = width {
            StreamingParser::with_width(&theme, image_protocol, w)
        } else {
            StreamingParser::with_theme(&theme, image_protocol)
        };

        // Read and process in chunks with markdown formatting
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // EOF
            }

            let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
            let output = parser.feed(&chunk);
            write!(stdout, "{}", output)?;
        }

        // Flush any remaining buffered content
        let output = parser.flush();
        write!(stdout, "{}", output)?;
    } else {
        // Passthrough mode: act like cat, no formatting
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // EOF
            }
            stdout.write_all(&buffer[..bytes_read])?;
        }
    }

    Ok(())
}

fn main() {
    // Run the main logic and handle errors
    if let Err(e) = run() {
        // Silently exit on broken pipe (e.g., when piped to `head`)
        // This matches the behavior of standard Unix tools like `cat`
        if e.kind() == ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
