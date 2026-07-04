
Role: Act as an elite Principal Software Engineer, an expert in Rust, Python,
and Open-Source Software (OSS) maintainership. You have successfully published
dozens of highly popular Rust-backed Python libraries to PyPI.

Context: I am preparing to publish version 1.0.0 of my library, lutufi, to PyPI.
It is a Rust-core library with Python bindings (likely using PyO3/Maturin). I
want this release to be absolutely bulletproof. I do not want any beginner
mistakes, memory leaks, compilation headaches for end-users, or missing
standards that would embarrass me on a v1.0 release.

Task: Perform a merciless, exhaustive, and comprehensive scan of the entire
lutufi codebase. Analyze the architecture, Rust-Python interoperability,
packaging configuration, tests, and documentation.

Please generate a highly structured, prioritized report covering the following
areas:

1. Rust-Python Interoperability (The Bindings):

  - GIL Management: Am I releasing the Python GIL (Python::allow_threads) during
    heavy Rust computations so I don't block Python's concurrency?
  - Error Handling: Are Rust Results and Errors being safely and idiomatically
    converted into native Python Exceptions, or will it just crash/panic the
    Python interpreter?
  - Data Passing: Are we avoiding unnecessary cloning between Rust and Python?
    (e.g., using memoryviews, PyBuffer, or zero-copy techniques where
    appropriate).

2. Packaging & PyPI Readiness (pyproject.toml & Cargo.toml):

  - Wheel Distribution: Review my build setup. Am I configured to build abi3
    wheels so that I don't have to compile separate wheels for
    Python 3.9, 3.10, etc.?
  - Metadata Checklist: Are the description, long_description (README), authors,
    URLs, license, and classifiers correctly formatted for PyPI?
  - Versioning: Is the version synchronized between Cargo.toml and
    pyproject.toml?

3. Developer Experience & Cleanliness:

  - .gitignore Mastery: Provide a robust, combined .gitignore for a Rust/Python
    hybrid project. Make sure it ignores target/, __pycache__, .venv,
    OS-specific junk, built artifacts (*.so, *.pyd), and packaging leftovers
    (dist/, build/, *.egg-info).
  - Type Hinting & Stubs: Python developers rely heavily on IDE autocomplete.
    Does my project generate or include .pyi stub files so lutufi has full
    Python type hinting?
  - Docstrings: Are my Rust docstrings being correctly exported to Python so
    that help(lutufi) works natively?

4. Code Quality & Robustness:

  - Scan the Rust code for unwrap(), expect(), or potential panics that could
    abruptly kill a user's Python process.
  - Point out any code smells, inefficient loops, or memory safety concerns.
  - Review the Python wrapper code (if any) for idiomatic Python guidelines
    (PEP 8).

5. CI/CD & Deployment Strategy:

  - Advise on my GitHub Actions (or CI) setup. If I don't have one, give me the
    exact workflow to use maturin-action to cross-compile wheels for Linux,
    macOS, and Windows (x86_64 and arm64). Crucial: End-users should not need a
    Rust compiler to pip install lutufi.

Output Format:

1.  🔴 Critical Blockers (Fix immediately before v1.0).
2.  🟡 Recommended Improvements (Best practices for a professional feel).
3.  🟢 The Ultimate .gitignore (Ready to copy-paste).
4.  🚀 The Pre-Flight Checklist (A step-by-step checklist of terminal commands to
    test, build, and upload to PyPI).

Be direct, highly technical, and leave no stone unturned. Tell me what I did
right, but more importantly, tell me what I did wrong. Let's make lutufi
world-class. 
>>> WRITE IT ALL IN A COMPREHENSIVE .MD REPORT. <<<
