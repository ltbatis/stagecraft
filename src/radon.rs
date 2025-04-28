use std::process::Command;
use which::which;
use regex::Regex;

/// Struct to hold complexity analysis for a function or method
#[derive(Debug)]
pub struct FunctionComplexity {
    pub file: String,
    pub kind: String,     // Function, Method, Class
    pub line: usize,
    pub name: String,
    pub grade: String,
    pub score: usize,
}

/// Analyze a staged Python file using radon and return parsed results.
pub fn analyze_file(file_path: &str) -> Vec<FunctionComplexity> {
    if which("radon").is_err() {
        eprintln!("❌ Error: radon is not installed or not found in PATH.");
        eprintln!("👉 Please install it using: pip install radon");
        return vec![];
    }

    let output = Command::new("radon")
        .args(["cc", "-s", file_path])
        .output()
        .expect("Failed to execute radon");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("⚠️ Radon returned an error:\n{}", stderr);
        return vec![];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim().is_empty() {
        eprintln!("⚠️ Radon analysis produced no output for file: {}", file_path);
        return vec![];
    }

    parse_radon_output(file_path, &stdout)
}

/// Parse the raw output from radon into a list of FunctionComplexity
fn parse_radon_output(file_path: &str, output: &str) -> Vec<FunctionComplexity> {
    let mut results = Vec::new();

    let re = Regex::new(r"^\s*(?P<kind>[FMC])\s+(?P<line>\d+):\d+\s+(?P<name>[\w\.]+)\s+-\s+(?P<grade>[A-F])\s+\((?P<score>\d+)\)").unwrap();

    for line in output.lines() {
        if let Some(caps) = re.captures(line) {
            let kind = match &caps["kind"] {
                "F" => "Function",
                "M" => "Method",
                "C" => "Class",
                _ => "Unknown",
            };

            let fc = FunctionComplexity {
                file: file_path.to_string(),
                kind: kind.to_string(),
                line: caps["line"].parse().unwrap_or(0),
                name: caps["name"].to_string(),
                grade: caps["grade"].to_string(),
                score: caps["score"].parse().unwrap_or(0),
            };

            results.push(fc);
        }
    }

    results
}
