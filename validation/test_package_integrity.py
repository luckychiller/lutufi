"""Packaging and distribution integrity tests.

These tests catch the kind of release-blocking mistakes that don't show up
in functional tests: a version bump applied to ``pyproject.toml`` but not
``Cargo.toml`` (or vice versa), a license declaration that no longer matches
the ``LICENSE`` file, a documented repository URL that doesn't match the one
``cargo`` and ``pip`` will advertise, or a public export that silently
disappeared.

They are intentionally implemented with light regex parsing rather than a
TOML library, since ``tomllib`` is only available on Python >= 3.11 and this
project supports Python >= 3.9.
"""

import re
from pathlib import Path

import lutufi

REPO_ROOT = Path(__file__).resolve().parents[1]


def _read(path: str) -> str:
    return (REPO_ROOT / path).read_text(encoding="utf-8")


def _toml_value(text: str, key: str) -> str:
    match = re.search(rf'(?m)^{re.escape(key)}\s*=\s*"([^"]+)"', text)
    assert match, f"Could not find top-level key {key!r}"
    return match.group(1)


# ─── Version consistency ───────────────────────────────────────────────────────

def test_version_matches_pyproject_toml():
    pyproject_version = _toml_value(_read("pyproject.toml"), "version")
    assert lutufi.__version__ == pyproject_version


def test_version_matches_cargo_toml():
    cargo_version = _toml_value(_read("Cargo.toml"), "version")
    assert lutufi.__version__ == cargo_version


def test_version_is_pep440_compliant():
    """A version like '0.1.0-dev' is invalid PEP 440 and will be rejected
    or silently mangled by PyPI.
    """
    assert re.match(r"^\d+\.\d+\.\d+$", lutufi.__version__), (
        f"Version {lutufi.__version__!r} is not a plain X.Y.Z release version"
    )


# ─── License consistency ───────────────────────────────────────────────────────

def test_license_file_exists():
    license_path = REPO_ROOT / "LICENSE"
    assert license_path.exists(), "Missing top-level LICENSE file"


def test_pyproject_license_matches_license_file():
    pyproject_text = _read("pyproject.toml")
    license_match = re.search(r'license\s*=\s*\{\s*text\s*=\s*"([^"]+)"\s*\}', pyproject_text)
    assert license_match, "Could not find [project.license] in pyproject.toml"
    declared_license = license_match.group(1)

    license_text = _read("LICENSE")
    if "Apache" in declared_license:
        assert "Apache License" in license_text
    elif "MIT" in declared_license:
        assert "MIT License" in license_text


def test_pyproject_has_no_conflicting_license_classifier():
    """A v1.0 release declaring 'Apache-2.0' in [project.license] must not
    also carry an 'MIT License' classifier (or vice versa) - pip and PyPI
    treat these as separate, conflicting declarations.
    """
    pyproject_text = _read("pyproject.toml")
    license_match = re.search(r'license\s*=\s*\{\s*text\s*=\s*"([^"]+)"\s*\}', pyproject_text)
    declared_license = license_match.group(1)

    classifiers = re.findall(r'"License :: OSI Approved :: ([^"]+)"', pyproject_text)
    if "Apache" in declared_license:
        assert not any("MIT" in c for c in classifiers), classifiers
    elif "MIT" in declared_license:
        assert not any("Apache" in c for c in classifiers), classifiers


# ─── Repository URL consistency ────────────────────────────────────────────────

def test_repository_url_matches_cargo_toml():
    cargo_repo = _toml_value(_read("Cargo.toml"), "repository")

    pyproject_text = _read("pyproject.toml")
    urls_match = re.search(r'(?m)^Repository\s*=\s*"([^"]+)"', pyproject_text)
    assert urls_match, "Could not find Repository under [project.urls]"
    assert urls_match.group(1) == cargo_repo


def test_readme_and_install_reference_correct_repository():
    cargo_repo = _toml_value(_read("Cargo.toml"), "repository")
    repo_slug = cargo_repo.replace("https://github.com/", "")

    for doc in ("README.md", "INSTALL.md"):
        text = _read(doc)
        assert "github.com/lutufi/lutufi" not in text, (
            f"{doc} still references the placeholder 'lutufi/lutufi' repository"
        )
        assert repo_slug in text, f"{doc} does not reference {repo_slug}"


# ─── Public API surface ────────────────────────────────────────────────────────

def test_all_exports_are_importable_and_not_none():
    for name in lutufi.__all__:
        assert hasattr(lutufi, name), f"lutufi.__all__ lists {name!r} but it is not an attribute"
        assert getattr(lutufi, name) is not None, f"lutufi.{name} is None"


def test_native_extension_exposes_expected_symbols():
    """The compiled `_lutufi` extension must expose every symbol the pure
    Python layer (`models.py`, `inference.py`, `learning.py`) imports from
    it. A mismatch here means the Python wrapper will fail at import time.
    """
    from lutufi import _lutufi

    expected = [
        "__version__",
        "Variable",
        "Domain",
        "ValidationResult",
        "_RustBayesianNetwork",
        "_RustMarkovRandomField",
        "_RustDynamicBayesianNetwork",
        "_RustVariableEliminationEngine",
        "_RustJunctionTreeEngine",
        "_RustLBPEngine",
        "_RustMCMCEngine",
        "_RustVariationalEngine",
        "_RustParameterEstimator",
        "_RustStructureLearner",
    ]
    for name in expected:
        assert hasattr(_lutufi, name), f"_lutufi extension is missing expected symbol {name!r}"


def test_native_extension_version_matches_python_package():
    from lutufi import _lutufi

    assert _lutufi.__version__ == lutufi.__version__
