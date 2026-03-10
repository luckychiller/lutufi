import os
import lutufi

def test_version():
    """Verify the package version string."""
    assert lutufi.__version__ == "0.1.0-dev"

def test_package_import():
    """Verify the package can be imported."""
    import lutufi
    assert lutufi.__name__ == "lutufi"

def test_directory_structure():
    """Verify critical directories exist."""
    required_dirs = [
        "src",
        "python",
        "tests",
        "docs",
        "examples",
        "benches",
    ]
    for d in required_dirs:
        assert os.path.isdir(d), f"Directory {d} is missing"

def test_ground_truth_exists():
    """Verify ground truth directory and some files exist."""
    assert os.path.isdir("tests/ground_truth")
    assert os.path.isfile("tests/ground_truth/chain.json")
    assert os.path.isfile("tests/ground_truth/asia.json")
