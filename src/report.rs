use crate::radon::FunctionComplexity;
use colored::*; // Import colored

pub fn generate_report(mut results: Vec<FunctionComplexity>) {
    // Sort by score descending
    results.sort_by(|a, b| b.score.cmp(&a.score));

    println!("{}", "📋 Stagecraft Complexity Report".bold());
    println!("{}", "────────────────────────────────────────────────────".bold());

    for (i, func) in results.iter().enumerate() {
        let grade_colored = match func.grade.as_str() {
            "A" => func.grade.green().bold(),
            "B" | "C" => func.grade.yellow().bold(),
            "D" | "E" | "F" => func.grade.red().bold(),
            _ => func.grade.normal().bold(),
        };

        println!(
            "{}. [{}] {} ({} - {}) [{}:{}]",
            i + 1,
            func.kind,
            func.name,
            grade_colored,
            func.score,
            func.file,
            func.line
        );
    }

    println!("{}", "────────────────────────────────────────────────────".bold());

    // Worst Function
    if let Some(worst) = results.first() {
        let worst_grade_colored = match worst.grade.as_str() {
            "A" => worst.grade.green().bold(),
            "B" | "C" => worst.grade.yellow().bold(),
            "D" | "E" | "F" => worst.grade.red().bold(),
            _ => worst.grade.normal().bold(),
        };

        println!(
            "{} {} ({}) - Complexity: {} [Line {}]",
            "🚨 Worst Function:".bold(),
            worst.name,
            worst_grade_colored,
            worst.score,
            worst.line
        );
    }

    println!("{}", "────────────────────────────────────────────────────".bold());

    // Average complexity score
    let total_score: usize = results.iter().map(|f| f.score).sum();
    let average_score = total_score as f64 / results.len() as f64;

    println!("{} {:.2}", "📊 Average Complexity Score:".bold(), average_score);
    println!();

    println!("{}", "✅ Stagecraft analysis completed successfully!".bold().green());
}
