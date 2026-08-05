//! CLI-level tests for logging flags (`--verbose`, `--debug-log`).
//!
//! These run the real binary in a subprocess rather than calling
//! `logging::init` directly. The `log` logger is global and can only be
//! installed once per process, so in-process tests could not cover more than a
//! single configuration. Going through the binary also lets us assert the
//! property that matters most: diagnostics never reach stdout.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const EXE: &str = env!("CARGO_BIN_EXE_mdriver");

/// A unique temp path per test, so tests can run in parallel.
fn temp_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("mdriver-logtest-{}-{}", std::process::id(), name));
    path
}

fn write_file(path: &Path, contents: &str) {
    fs::write(path, contents).expect("write temp file");
}

fn run(args: &[&str]) -> Output {
    Command::new(EXE).args(args).output().expect("run mdriver")
}

/// An SVG that makes usvg emit a warning: the embedded image is not a valid
/// PNG, so usvg skips it and logs. The rect ensures the SVG still renders,
/// which is the silent-degradation case from issue #78.
const SVG_WITH_BAD_IMAGE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="200" height="60">
  <image x="0" y="0" width="50" height="50" xlink:href="data:image/png;base64,bm90YXBuZw=="/>
  <rect x="60" y="10" width="30" height="30" fill="blue"/>
</svg>
"#;

#[test]
fn no_diagnostics_without_flags() {
    let md = temp_path("quiet.md");
    write_file(&md, "![missing](/nonexistent/image.png)\n");

    let out = run(&[
        "--color=always",
        "--images",
        "kitty",
        md.to_str().expect("utf8 path"),
    ]);

    assert!(
        out.stderr.is_empty(),
        "expected silence by default, got stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn verbose_reports_our_own_fallbacks() {
    let md = temp_path("verbose.md");
    write_file(&md, "![missing](/nonexistent/image.png)\n");

    let out = run(&[
        "--color=always",
        "--images",
        "kitty",
        "--verbose",
        md.to_str().expect("utf8 path"),
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stderr.contains("/nonexistent/image.png"),
        "warning should name the failing image, got: {stderr}"
    );
    assert!(
        stderr.contains("alt text"),
        "warning should say what happened instead, got: {stderr}"
    );
}

/// The core of issue #78: usvg's own `log` warnings were being discarded
/// because no logger was installed.
#[test]
fn verbose_surfaces_usvg_warnings() {
    let svg = temp_path("bad-image.svg");
    write_file(&svg, SVG_WITH_BAD_IMAGE);
    let md = temp_path("usvg.md");
    write_file(&md, &format!("![d]({})\n", svg.to_str().expect("utf8")));

    let out = run(&[
        "--color=always",
        "--images",
        "kitty",
        "--verbose",
        md.to_str().expect("utf8 path"),
    ]);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stderr.contains("usvg"),
        "usvg diagnostics should be attributed to usvg, got: {stderr}"
    );
    assert!(
        stderr.contains("Skipped"),
        "usvg should report the content it skipped, got: {stderr}"
    );
}

/// Diagnostics must never mix into stdout: it carries the ANSI stream and
/// kitty image payloads, where a stray log line would corrupt the image.
#[test]
fn diagnostics_never_touch_stdout() {
    let svg = temp_path("stdout-purity.svg");
    write_file(&svg, SVG_WITH_BAD_IMAGE);
    let md = temp_path("stdout-purity.md");
    write_file(&md, &format!("![d]({})\n", svg.to_str().expect("utf8")));
    let md_arg = md.to_str().expect("utf8 path");
    let log = temp_path("stdout-purity.log");

    let quiet = run(&["--color=always", "--images", "kitty", md_arg]);
    let verbose = run(&["--color=always", "--images", "kitty", "--verbose", md_arg]);
    let logged = run(&[
        "--color=always",
        "--images",
        "kitty",
        "--debug-log",
        log.to_str().expect("utf8 path"),
        md_arg,
    ]);

    assert!(!quiet.stdout.is_empty(), "expected rendered output");
    assert_eq!(
        quiet.stdout, verbose.stdout,
        "--verbose must not alter stdout"
    );
    assert_eq!(
        quiet.stdout, logged.stdout,
        "--debug-log must not alter stdout"
    );
    // The warning we rely on for this test must actually have fired, or the
    // comparison above proves nothing.
    assert!(
        !verbose.stderr.is_empty(),
        "expected a warning on stderr to make this test meaningful"
    );
}

#[test]
fn debug_log_writes_trace_records_to_file() {
    let svg = temp_path("debuglog.svg");
    write_file(&svg, SVG_WITH_BAD_IMAGE);
    let md = temp_path("debuglog.md");
    write_file(&md, &format!("![d]({})\n", svg.to_str().expect("utf8")));
    let log = temp_path("debuglog.log");

    let out = run(&[
        "--color=always",
        "--images",
        "kitty",
        "--debug-log",
        log.to_str().expect("utf8 path"),
        md.to_str().expect("utf8 path"),
    ]);

    assert!(
        out.stderr.is_empty(),
        "--debug-log alone should stay off stderr, got: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let contents = fs::read_to_string(&log).expect("debug log should exist");
    assert!(
        contents.contains("Skipped"),
        "debug log should capture usvg warnings, got: {contents}"
    );
    // Records are stamped with elapsed time and level for bug reports.
    assert!(
        contents.contains("WARN"),
        "records should include a level, got: {contents}"
    );
    assert!(
        contents.contains("ms"),
        "records should include elapsed time, got: {contents}"
    );
}

#[test]
fn debug_log_truncates_existing_file() {
    let log = temp_path("truncate.log");
    write_file(&log, "stale content from a previous run\n");
    let md = temp_path("truncate.md");
    write_file(&md, "# hello\n");

    run(&[
        "--color=always",
        "--debug-log",
        log.to_str().expect("utf8 path"),
        md.to_str().expect("utf8 path"),
    ]);

    let contents = fs::read_to_string(&log).expect("debug log should exist");
    assert!(
        !contents.contains("stale content"),
        "debug log should be truncated, got: {contents}"
    );
}

#[test]
fn unopenable_debug_log_is_an_error() {
    let md = temp_path("unopenable.md");
    write_file(&md, "# hello\n");

    let out = run(&[
        "--debug-log",
        "/nonexistent-directory/mdriver.log",
        md.to_str().expect("utf8 path"),
    ]);

    assert!(
        !out.status.success(),
        "an explicitly requested log path that cannot be opened should fail"
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("debug log"),
        "error should mention the debug log"
    );
}

#[test]
fn debug_log_requires_a_path() {
    let out = run(&["--debug-log"]);

    assert!(!out.status.success(), "missing argument should fail");
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("--debug-log"),
        "error should name the flag"
    );
}

#[test]
fn short_verbose_flag_is_accepted() {
    let md = temp_path("shortflag.md");
    write_file(&md, "![missing](/nonexistent/image.png)\n");

    let out = run(&[
        "--color=always",
        "--images",
        "kitty",
        "-v",
        md.to_str().expect("utf8 path"),
    ]);

    assert!(
        String::from_utf8_lossy(&out.stderr).contains("/nonexistent/image.png"),
        "-v should behave like --verbose"
    );
}

#[test]
fn logging_flags_appear_in_help() {
    let out = run(&["--help"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("--verbose"),
        "--verbose should be documented"
    );
    assert!(
        stdout.contains("--debug-log"),
        "--debug-log should be documented"
    );
}
