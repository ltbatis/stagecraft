use crate::radon::FunctionComplexity;

pub fn generate_report(mut results: Vec<FunctionComplexity>) {
    // Sort by score descending
    results.sort_by(|a, b| b.score.cmp(&a.score));

    println!("📋 Stagecraft Complexity Report");
    println!("────────────────────────────────────────────────────");

    for (i, func) in results.iter().enumerate() {
        println!(
            "{}. [{}] {} ({} - {}) [{}:{}]",
            i + 1,
            func.kind,
            func.name,
            func.grade,
            func.score,
            func.file,
            func.line
        );
    }

    println!("────────────────────────────────────────────────────");

    if let Some(worst) = results.first() {
        println!(
            "🚨 Worst Function: {} ({}) - Complexity: {} [Line {}]",
            worst.name, worst.grade, worst.score, worst.line
        );
    }

    println!("✅ Stagecraft analysis completed successfully!");
}
