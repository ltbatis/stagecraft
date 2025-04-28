# 🧪 Stagecraft

Stagecraft is a command-line tool written in **Rust** to analyze the **cyclomatic complexity** of staged Python files in a Git repository.

It generates a beautiful and professional report highlighting:

- 🚨 Worst Functions per file
- 📊 Average Complexity Score per file
- 📋 Full complexity classification (Grades A–F) with colors
- 📄 Final summary of files and functions scanned

---

## 📥 Installation Guide

Please see the detailed [Installation Guide](./INSTALL.md) for full setup instructions.

---

## 📦 Quick Installation (short version)

```bash
git clone https://github.com/ltbatis/stagecraft.git
cd stagecraft
cargo install --path .
stagecraft analyze
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
- **radon** Python package

---

## 📄 Example

When you run:

```bash
stagecraft analyze
```

You will see a colorful report of your staged Python files.

---

## 🤝 Contributing

Pull requests are welcome!

---

## 📜 License

This project is licensed under the MIT License.

---

Made with 🧙 magic and ☕ coffee.
