mod common;

use common::ConformanceTest;
use mdriver::StreamingParser;
use std::path::PathBuf;

/// Run a single conformance test
fn run_conformance_test(test: &ConformanceTest) -> Result<(), String> {
    let mut parser = if let Some(width) = test.width {
        StreamingParser::with_width("base16-ocean.dark", mdriver::ImageProtocol::None, width)
    } else {
        StreamingParser::new()
    };
    let mut chunk_num = 0;

    for chunk in &test.chunks {
        chunk_num += 1;
        let actual_emit = parser.feed(&chunk.input);

        if actual_emit != chunk.emit {
            return Err(format!(
                "Chunk {} failed:\n  Input: {:?}\n  Expected: {:?}\n  Actual: {:?}",
                chunk_num, chunk.input, chunk.emit, actual_emit
            ));
        }
    }

    // Flush any remaining buffered content
    let final_output = parser.flush();
    if !final_output.is_empty() {
        return Err(format!(
            "Unexpected output during flush:\n  Output: {:?}",
            final_output
        ));
    }

    Ok(())
}

/// Load and run all tests in a directory
fn run_tests_in_directory(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(dir);

    if !fixtures_dir.exists() {
        // Directory doesn't exist yet, skip
        return Ok(());
    }

    let tests = ConformanceTest::load_from_directory(&fixtures_dir)?;

    if tests.is_empty() {
        println!("No tests found in {}", dir);
        return Ok(());
    }

    println!("\nRunning {} tests from {}...", tests.len(), dir);

    let mut passed = 0;
    let mut failed = 0;

    for test in &tests {
        match run_conformance_test(test) {
            Ok(_) => {
                println!("  ✓ {}", test.name);
                passed += 1;
            }
            Err(e) => {
                println!("  ✗ {}", test.name);
                println!("    {}", test.description);
                println!("    {}", e);
                failed += 1;
            }
        }
    }

    println!("\n{} passed, {} failed", passed, failed);

    if failed > 0 {
        Err(format!("{} tests failed", failed).into())
    } else {
        Ok(())
    }
}

#[test]
fn test_block_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    run_tests_in_directory("blocks")
}

#[test]
fn test_streaming_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    run_tests_in_directory("streaming")
}

#[test]
fn test_ansi_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    run_tests_in_directory("ansi")
}

#[test]
fn test_complex_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    run_tests_in_directory("complex")
}

#[test]
fn test_wrapping_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    run_tests_in_directory("wrapping")
}
