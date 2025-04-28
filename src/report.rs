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
    println!("✅ Stagecraft analysis completed successfully!");
}
