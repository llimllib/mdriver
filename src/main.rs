use mdriver::StreamingParser;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn print_help() {
    println!("mdriver - Streaming Markdown Printer");
    println!();
    println!("USAGE:");
    println!("    mdriver [OPTIONS] [FILE]");
    println!();
    println!("OPTIONS:");
    println!("    --help              Print this help message");
    println!("    --list-themes       List available syntax highlighting themes");
    println!("    --theme <THEME>     Use specified syntax highlighting theme");
    println!();
    println!("ARGS:");
    println!("    <FILE>              Markdown file to render (reads from stdin if not provided)");
    println!();
    println!("ENVIRONMENT:");
    println!("    MDRIVER_THEME       Default syntax highlighting theme (overridden by --theme)");
    println!();
    println!("EXAMPLES:");
    println!("    mdriver README.md");
    println!("    mdriver --theme \"Solarized (dark)\" README.md");
    println!("    cat file.md | mdriver");
    println!("    MDRIVER_THEME=\"InspiredGitHub\" mdriver file.md");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut theme: Option<String> = None;
    let mut file_path: Option<String> = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
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

    // Get theme from parameter, environment variable, or use default
    let theme = theme
        .or_else(|| env::var("MDRIVER_THEME").ok())
        .unwrap_or_else(|| "base16-ocean.dark".to_string());

    let mut parser = StreamingParser::with_theme(&theme);
    let mut buffer = [0u8; 4096];

    // Read from file or stdin
    let mut reader: Box<dyn Read> = if let Some(path) = file_path {
        Box::new(File::open(path)?)
    } else {
        Box::new(io::stdin())
    };

    // Read and process in chunks
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF
        }

        let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        let output = parser.feed(&chunk);
        print!("{}", output);
    }

    // Flush any remaining buffered content
    let output = parser.flush();
    print!("{}", output);

    Ok(())
}
