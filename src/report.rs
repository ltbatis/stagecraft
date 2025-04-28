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
    // Worsst Function
    if let Some(worst) = results.first() {
        println!(
            "🚨 Worst Function: {} ({}) - Complexity: {} [Line {}]",
            worst.name, worst.grade, worst.score, worst.line
        );
    }


    println!("────────────────────────────────────────────────────");
    // Average complexity score
    let total_score: usize = results.iter().map(|f| f.score).sum();
    let average_score = total_score as f64 / results.len() as f64;

    println!("📊 Average Complexity Score: {:.2}", average_score);
    println!("");

    println!("✅ Stagecraft analysis completed successfully!");
}
