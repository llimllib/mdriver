use std::process::Command;

fn main() {
    // Capture rustc version at compile time
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version.trim());

    // Re-run if rustc changes
    println!("cargo:rerun-if-env-changed=RUSTC");
}
