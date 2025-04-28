use std::process::Command;

/// Analyze the given Python file using radon.
/// Returns the raw output as a String.
pub fn analyze_file(file_path: &str) -> Option<String> {
    let output = Command::new("radon")
        .args(["cc", "-s", file_path])
        .output()
        .expect("Failed to execute radon");

    if !output.status.success() {
        eprintln!("Error: Failed to analyze file: {}", file_path);
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(stdout.to_string())
}
