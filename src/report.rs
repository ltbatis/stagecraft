use crate::radon::FunctionComplexity;
use colored::*;
use std::collections::HashMap;

pub fn generate_report(results: Vec<FunctionComplexity>) {
    if results.is_empty() {
        println!("{}", "⚠️  No complexity data found.".yellow().bold());
        return;
    }

    println!();
    println!("{}", "📋 Stagecraft Complexity Report".bold().underline().blue());
    println!("{}", "════════════════════════════════════════════════════".blue());

    // Group results by file
    let mut grouped: HashMap<String, Vec<FunctionComplexity>> = HashMap::new();
    for func in results {
        grouped.entry(func.file.clone()).or_default().push(func);
    }

    let total_files = grouped.len();
    let mut total_functions = 0;

    // Print report for each file
    for (file, mut functions) in grouped {
        println!();
        println!("{} {}", "📄 File:".bold().cyan(), file.bold().cyan());
        println!("{}", "────────────────────────────────────────────────────".cyan());

        total_functions += functions.len();

        functions.sort_by(|a, b| b.score.cmp(&a.score));

        for (i, func) in functions.iter().enumerate() {
            let grade_colored = match func.grade.as_str() {
                "A" => func.grade.green().bold(),
                "B" | "C" => func.grade.yellow().bold(),
                "D" | "E" | "F" => func.grade.red().bold(),
                _ => func.grade.normal().bold(),
            };

            println!(
                "  {}. [{}] {} ({} - {}) [Line {}]",
                i + 1,
                func.kind.bold(),
                func.name.bold(),
                grade_colored,
                func.score.to_string().bold(),
                func.line
            );
        }

        println!("{}", "────────────────────────────────────────────────────".cyan());

        // Worst function per file
        if let Some(worst) = functions.first() {
            let worst_grade_colored = match worst.grade.as_str() {
                "A" => worst.grade.green().bold(),
                "B" | "C" => worst.grade.yellow().bold(),
                "D" | "E" | "F" => worst.grade.red().bold(),
                _ => worst.grade.normal().bold(),
            };

            println!(
                "{} {} ({}) - Complexity: {} [Line {}]",
                "🚨 Worst Function:".bold().red(),
                worst.name.bold(),
                worst_grade_colored,
                worst.score.to_string().bold(),
                worst.line
            );
        }

        // Average complexity score per file
        let total_score: usize = functions.iter().map(|f| f.score).sum();
        let average_score = total_score as f64 / functions.len() as f64;

        let avg_colored = if average_score <= 5.0 {
            format!("{:.2}", average_score).green().bold()
        } else if average_score <= 10.0 {
            format!("{:.2}", average_score).yellow().bold()
        } else {
            format!("{:.2}", average_score).red().bold()
        };

        println!("{} {}", "📊 Average Complexity Score:".bold().blue(), avg_colored);
        println!("{}", "════════════════════════════════════════════════════".cyan());
    }

    println!();

    // Final Summary
    println!("{}", "📄 Stagecraft Final Summary".bold().underline().blue());
    println!("{}", "────────────────────────────────────────────────────".blue());
    println!(
        "{} {} | {} {}",
        "🗂️ Files analyzed:".bold(),
        total_files.to_string().bold().green(),
        "🔎 Functions scanned:".bold(),
        total_functions.to_string().bold().green()
    );
    println!("{}", "────────────────────────────────────────────────────".blue());
    println!();

    println!("{}", "✅ Stagecraft analysis completed successfully!".green().bold());
}
