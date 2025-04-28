use std::process::Command;
use which::which;

/// Analyze the given Python file using radon.
/// Returns the raw output as a String.
pub fn analyze_file(file_path: &str) -> Option<String> {
    // Check if radon is installed
    if which("radon").is_err() {
        eprintln!("❌ Error: radon is not installed or not found in PATH.");
        eprintln!("👉 Please install it using: pip install radon");
        return None;
    }

    // Run radon command
    let output = Command::new("radon")
        .args(["cc", "-s", file_path])
        .output()
        .expect("Failed to execute radon");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("⚠️ Radon returned an error:\n{}", stderr);
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim().is_empty() {
        eprintln!("⚠️ Radon analysis produced no output for file: {}", file_path);
        return None;
    }

    Some(stdout.to_string())
}
