use crate::radon::FunctionComplexity;
use colored::*;
use std::collections::HashMap;

pub fn generate_report(results: Vec<FunctionComplexity>) {
    if results.is_empty() {
        println!("⚠️ No complexity data found.");
        return;
    }

    println!("{}", "📋 Stagecraft Complexity Report".bold());
    println!("{}", "────────────────────────────────────────────────────");

    // Group results by file
    let mut grouped: HashMap<String, Vec<FunctionComplexity>> = HashMap::new();

    for func in results {
        grouped.entry(func.file.clone()).or_default().push(func);
    }

    // Print report for each file
    for (file, mut functions) in grouped {
        println!("\n{} {}\n{}", "📄 File:".bold(), file.bold(), "────────────────────────────────────────────────────".bold());

        // Sort functions inside each file
        functions.sort_by(|a, b| b.score.cmp(&a.score));

        for (i, func) in functions.iter().enumerate() {
            let grade_colored = match func.grade.as_str() {
                "A" => func.grade.green().bold(),
                "B" | "C" => func.grade.yellow().bold(),
                "D" | "E" | "F" => func.grade.red().bold(),
                _ => func.grade.normal().bold(),
            };

            println!(
                "{}. [{}] {} ({} - {}) [Line {}]",
                i + 1,
                func.kind,
                func.name,
                grade_colored,
                func.score,
                func.line
            );
        }

        println!("{}", "────────────────────────────────────────────────────".bold());

        // Worst function per file
        if let Some(worst) = functions.first() {
            let worst_grade_colored = match worst.grade.as_str() {
                "A" => worst.grade.green().bold(),
                "B" | "C" => worst.grade.yellow().bold(),
                "D" | "E" | "F" => worst.grade.red().bold(),
                _ => worst.grade.normal().bold(),
            };

            println!(
                "🚨 Worst Function: {} ({}) - Complexity: {} [Line {}]",
                worst.name, worst_grade_colored, worst.score, worst.line
            );
        }

        // Average complexity score per file
        let total_score: usize = functions.iter().map(|f| f.score).sum();
        let average_score = total_score as f64 / functions.len() as f64;

        println!("📊 Average Complexity Score: {:.2}", average_score);
        println!("{}", "────────────────────────────────────────────────────".bold());
    }

    println!("{}", "✅ Stagecraft analysis completed successfully!".green().bold());
}
