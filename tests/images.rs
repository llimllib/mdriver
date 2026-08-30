//! CLI-level tests for when inline image escapes are suppressed.
//!
//! These run the real binary in a subprocess, which means stdout is a pipe
//! rather than a terminal — exactly the condition we want to assert on. The
//! per-condition logic lives in `mdriver::image_disable_reason` and is unit
//! tested directly, since covering the terminal case here would need a pty.

use std::io::Write;
use std::process::{Command, Output, Stdio};

const EXE: &str = env!("CARGO_BIN_EXE_mdriver");

/// A mermaid diagram renders as a kitty image when images are on and as text
/// when they are off, so it exercises the gate without needing an image file.
const MERMAID: &str = "```mermaid\nflowchart LR\n    A-->B-->C\n```\n";

fn run_with_stdin(args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(EXE)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run mdriver");
    child
        .stdin
        .as_mut()
        .expect("stdin is piped")
        .write_all(stdin.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("collect mdriver output")
}

#[test]
fn no_image_escapes_when_stdout_is_not_a_terminal() {
    let out = run_with_stdin(&["--color=always", "--images", "kitty"], MERMAID);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        !stdout.contains("\x1b_G"),
        "kitty escapes should not be written to a pipe, got: {:?}",
        &stdout[..stdout.len().min(200)]
    );
}

#[test]
fn suppressing_images_is_reported_under_verbose() {
    let out = run_with_stdin(
        &["--color=always", "--images", "kitty", "--verbose"],
        MERMAID,
    );
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stderr.contains("images: disabled"),
        "should explain why images were dropped, got: {stderr:?}"
    );
}

#[test]
fn suppressing_images_is_silent_without_verbose() {
    let out = run_with_stdin(&["--color=always", "--images", "kitty"], MERMAID);

    assert!(
        out.stderr.is_empty(),
        "should not warn unless asked, got: {:?}",
        String::from_utf8_lossy(&out.stderr)
    );
}
