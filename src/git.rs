use std::process::Command;

/// Get a list of staged Python files.
pub fn get_staged_python_files() -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .output()
        .expect("Failed to execute git command");

    if !output.status.success() {
        eprintln!("Error: Failed to retrieve staged files from Git.");
        return vec![];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .filter(|line| line.ends_with(".py"))
        .map(|line| line.to_string())
        .collect()
}
