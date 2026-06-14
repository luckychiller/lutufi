# Installing Lutufi

## Quick Install (from source)

### Prerequisites

- **Python** 3.8 or later (3.11+ recommended)
- **Rust toolchain** 1.70 or later — [Install Rust](https://rustup.rs/)
- **maturin** — `pip install maturin`

### Installation Steps

```bash
# 1. Clone the repository
git clone https://github.com/luckychiller/lutufi.git
cd lutufi

# 2. (Recommended) Create a virtual environment
python -m venv venv
# On Windows:
venv\Scripts\activate
# On macOS/Linux:
source venv/bin/activate

# 3. Install lutufi in development mode
pip install maturin
maturin develop --release

# 4. Verify the installation
python -c "import lutufi; print(lutufi.__version__)"
```

### Development Installation

```bash
# Install with all optional dependencies
pip install -e ".[dev,visualization,docs]"

# Run the test suite
pytest validation/
```

## Platform-Specific Notes

### Linux

Ensure `build-essential`, `pkg-config`, and `libssl-dev` are installed:

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

### macOS

Install the Rust toolchain via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Xcode Command Line Tools are required:

```bash
xcode-select --install
```

### Windows

1. Install [Rust for Windows](https://www.rust-lang.org/tools/install) (use the default installation).
2. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) with the "Desktop development with C++" workload.
3. Ensure Rust is in your PATH: `rustc --version`

```powershell
# In PowerShell (as Administrator)
python -m venv venv
venv\Scripts\Activate.ps1
pip install maturin
maturin develop --release
python -c "import lutufi; print(lutufi.__version__)"
```

## Building Wheels

To build distributable wheels for all supported platforms:

```bash
pip install cibuildwheel
cibuildwheel --platform linux   # or "windows" or "macos"
```

Wheels are output to `wheelhouse/`.

## Troubleshooting

| Problem | Likely Cause | Solution |
|---------|-------------|----------|
| `ImportError: No module named lutufi._lutufi` | Native extension not built | Run `maturin develop --release` |
| `error: failed to run custom build command for 'pyo3'` | Missing Python headers | Install `python3-dev` (Linux) or reinstall Python |
| `link.exe failed` (Windows) | Missing Visual Studio Build Tools | Install VS Build Tools with C++ workload |
| Rust compile error | Outdated Rust | Run `rustup update` |

## Dependencies

### Runtime (Python)

- numpy >= 1.21.0
- scipy >= 1.7.0
- networkx >= 2.6.0
- pandas >= 1.3.0
- matplotlib >= 3.4.0

### Build (Rust)

- pyo3 0.22 (Python/Rust FFI)
- See `Cargo.toml` for full Rust dependency list.

## Verifying Your Installation

Run the validation test suite to verify everything works:

```bash
pytest validation/ -v
```

A successful installation should show all tests passing.

---

*For questions or issues, see: [github.com/luckychiller/lutufi/issues](https://github.com/luckychiller/lutufi/issues)*
