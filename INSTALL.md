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

## 3. Build Stagecraft

Clone the Stagecraft repository and build it:

```bash
git clone https://github.com/YOUR-USER/stagecraft.git
cd stagecraft
cargo build --release
```

The executable will be located at:

```bash
./target/release/stagecraft
```

---

## 4. Install Stagecraft Globally (optional)

To use Stagecraft from anywhere, copy the binary to `/usr/local/bin`:

```bash
sudo cp ./target/release/stagecraft /usr/local/bin/stagecraft
```

Now you can run Stagecraft globally:

```bash
stagecraft analyze
```

---

## 5. Quick Test

In any Git repository:

```bash
git add some_script.py
stagecraft analyze
```

Stagecraft will find the staged `.py` files and generate a report!

✅ You're ready to use Stagecraft!

---

Made with 🧙‍♂️ magic and ☕ coffee.
