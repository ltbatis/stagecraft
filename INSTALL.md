# 📦 Stagecraft Installation Guide

Follow these steps to install and use Stagecraft on a Linux system (Ubuntu/Debian based).

---

## 1. Install Rust

Stagecraft is written in Rust. You need to install Rust first.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # To check if installed correctly
```

---

## 2. Install Radon (Python package)

Stagecraft uses the `radon` tool to calculate cyclomatic complexity.

```bash
pip install radon
```

Check if Radon is installed correctly:

```bash
radon --help
```

If radon is not found, ensure your Python user bin path is in your system PATH (example: `~/.local/bin`).

---

## 3. Clone and Build Stagecraft

Clone the Stagecraft repository and install it locally:

```bash
git clone https://github.com/ltbatis/stagecraft.git
cd stagecraft
cargo install --path .
```

This will install the `stagecraft` binary to your Cargo bin path, usually `~/.cargo/bin`.

Make sure your `~/.cargo/bin` is in your PATH. If not, add it:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

Or if you use zsh:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

---

## 4. Quick Test

In any Git repository:

```bash
git add some_script.py
stagecraft analyze
```

Stagecraft will find the staged `.py` files and generate a colorful report!

✅ You're ready to use Stagecraft!

---

Made with 🧙‍♂️ magic and ☕ coffee.
