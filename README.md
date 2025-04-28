# 🧪 Stagecraft

Stagecraft is a command-line tool written in **Rust** to analyze the **cyclomatic complexity** of staged Python files in a Git repository.

It generates a beautiful and professional report highlighting:

- 🚨 Worst Functions per file
- 📊 Average Complexity Score per file
- 📋 Full complexity classification (Grades A–F) with colors
- 📄 Final summary of files and functions scanned

---

## 📦 Installation

**Clone the repository** and build the project manually:

```bash
git clone https://github.com/YOUR-USER/stagecraft.git
cd stagecraft
cargo build --release
```

**Run locally:**

```bash
cargo run -- analyze
```

**Or run the compiled binary:**

```bash
./target/release/stagecraft analyze
```

---

## ⚡ Features

- Detects staged Python `.py` files
- Calculates cyclomatic complexity using `radon`
- Colorful and clean terminal report
- Highlights worst functions and overall code health
- Supports multiple staged files at once
- ASCII logo and personality on startup ✨

---

## 🚀 Roadmap

- [ ] JSON report output
- [ ] Markdown report export
- [ ] Configurable thresholds for allowed complexity
- [ ] Add support for PySpark and SQL files
- [ ] CI/CD ready mode (silent/quiet flags)
- [ ] Plugin system for extensibility

---

## 🛠 Requirements

- **Rust** (latest stable version)
- **radon** Python package (installed globally):

```bash
pip install radon
```

---

## 📄 Example

When you run:

```bash
cargo run -- analyze
```

You will see:

```
📋 Stagecraft Complexity Report
════════════════════════════════════════════════════

📄 File: example.py
────────────────────────────────────────────────────
  1. [Function] very_complex_function (D - 22) [Line 40]
  2. [Method] AnotherClass.simple_method (A - 1) [Line 12]
────────────────────────────────────────────────────
🚨 Worst Function: very_complex_function (D) - Complexity: 22 [Line 40]
📊 Average Complexity Score: 11.50
════════════════════════════════════════════════════

📄 Stagecraft Final Summary
────────────────────────────────────────────────────
🗂️ Files analyzed: 1 | 🔎 Functions scanned: 2
────────────────────────────────────────────────────

✅ Stagecraft analysis completed successfully!
```

---

## 🤝 Contributing

Pull requests are welcome!
For major changes, please open an issue first to discuss what you would like to change.

---

## 📜 License

This project is licensed under the MIT License.

---

Made with 🧙 magic and ☕ coffee.
