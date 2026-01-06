mod common;

use common::ConformanceTest;
use mdriver::StreamingParser;
use std::path::PathBuf;

/// Run a single conformance test
fn run_conformance_test(test: &ConformanceTest) -> Result<(), String> {
    let mut parser = StreamingParser::new();
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
fn test_block_fixtures() {
    run_tests_in_directory("blocks").unwrap();
}

#[test]
fn test_streaming_fixtures() {
    run_tests_in_directory("streaming").unwrap();
}

#[test]
fn test_ansi_fixtures() {
    run_tests_in_directory("ansi").unwrap();
}

#[test]
fn test_complex_fixtures() {
    run_tests_in_directory("complex").unwrap();
}
