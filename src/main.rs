mod git;
mod radon;
mod report;

use clap::{Parser, Subcommand};

/// Stagecraft: Analyze your staged code with precision.
#[derive(Parser)]
#[command(name = "stagecraft")]
#[command(about = "Analyze staged Python files for code complexity and quality.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze staged Python files for cyclomatic complexity
    Analyze,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze => {
            println!("🔍 Starting staged file analysis...");

            let staged_files = git::get_staged_python_files();

            if staged_files.is_empty() {
                println!("✅ No staged Python files found.");
            } else {
                println!("📄 Staged Python files detected:");
                for file in &staged_files {
                    println!("- {}", file);
                }

                println!("\n📈 Running complexity analysis...\n");

                let mut all_results = Vec::new();

                for file in &staged_files {
                    println!("Analyzing file: {}\n", file);
                    let results = radon::analyze_file(file);
                    all_results.extend(results);
                }

                if all_results.is_empty() {
                    println!("⚠️ No complexity data found.");
                } else {
                    report::generate_report(all_results);
                }
            }
        }
    }
}
