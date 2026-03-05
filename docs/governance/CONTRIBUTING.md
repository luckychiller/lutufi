# Contributor Governance for Lutufi

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Welcome](#welcome)
2. [Code of Conduct](#code-of-conduct)
3. [How to Contribute](#how-to-contribute)
4. [Development Setup](#development-setup)
5. [Contribution Workflow](#contribution-workflow)
6. [Commit Message Standards](#commit-message-standards)
7. [Code Review Process](#code-review-process)
8. [Testing Requirements](#testing-requirements)
9. [Documentation Requirements](#documentation-requirements)
10. [Intellectual Property](#intellectual-property)
11. [Recognition](#recognition)
12. [Governance Structure](#governance-structure)
13. [Security Issues](#security-issues)
14. [Ethics Considerations](#ethics-considerations)

---

## Welcome

### Welcoming Contributors

Lutufi is a multi-disciplinary effort that welcomes contributions from diverse backgrounds. We believe that the best software emerges from collaboration between experts in different domains.

**Who Can Contribute?**

- **Software Engineers**: Core library development, performance optimization, tooling
- **Mathematicians and Statisticians**: Algorithm development, theoretical foundations, numerical methods
- **Network Scientists**: Domain expertise in social, biological, and economic networks
- **Domain Experts**: Epidemiologists, economists, sociologists, political scientists
- **Technical Writers**: Documentation, tutorials, examples
- **Designers**: Visualization, user experience
- **Users**: Bug reports, feature requests, feedback

No contribution is too small. Whether you're fixing a typo in documentation or implementing a major new feature, your contribution is valued.

### Types of Contributions Needed

**Code Contributions**:
- Core inference engine improvements
- New algorithms and methods
- Performance optimizations
- Bug fixes
- Platform support (new operating systems, architectures)
- Language bindings (R, Julia, C++, etc.)

**Documentation Contributions**:
- API documentation improvements
- Tutorial development
- Example notebooks
- Theoretical explanations
- Translation to other languages

**Quality Contributions**:
- Test case development
- Bug reports and verification
- Performance benchmarking
- Security auditing

**Community Contributions**:
- Answering questions in forums
- Mentoring new contributors
- Speaking about Lutufi at conferences
- Writing blog posts and articles
- Organizing meetups and workshops

**Research Contributions**:
- Validation studies
- Comparative analyses
- New application domains
- Publication of results using Lutufi

---

## Code of Conduct

### Our Pledge

We as members, contributors, and leaders pledge to make participation in our community a harassment-free experience for everyone, regardless of age, body size, visible or invisible disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

We pledge to act and interact in ways that contribute to an open, welcoming, diverse, inclusive, and healthy community.

### Our Standards

**Examples of Expected Behavior**:
- Demonstrating empathy and kindness toward other people
- Being respectful of differing opinions, viewpoints, and experiences
- Giving and gracefully accepting constructive feedback
- Accepting responsibility and apologizing to those affected by our mistakes
- Focusing on what is best for the community and the project
- Respecting the expertise of contributors from different domains
- Acknowledging the limitations of one's own knowledge
- Engaging in good-faith debate about technical decisions

**Examples of Unacceptable Behavior**:
- The use of sexualized language or imagery, and sexual attention or advances of any kind
- Trolling, insulting or derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information, such as a physical or email address, without explicit permission
- Dismissing contributions based on the contributor's background rather than merit
- Gatekeeping based on credentials or institutional affiliation
- Sustained disruption of community discussions
- Other conduct which could reasonably be considered inappropriate in a professional setting

### Domain-Specific Respect

Lutufi brings together contributors from different disciplinary backgrounds:

**For Computer Scientists**: Respect that domain experts may use different terminology or have different priorities than software optimization.

**For Domain Experts**: Respect that implementation concerns (performance, maintainability) are valid technical constraints.

**For Mathematicians**: Respect that practical implementations may require approximations or simplifications of theoretical ideals.

**For Everyone**: Assume good faith. Academic and cultural backgrounds differ. Misunderstandings are usually just that—misunderstandings.

### Enforcement Responsibilities

Community leaders are responsible for clarifying and enforcing our standards of acceptable behavior and will take appropriate and fair corrective action in response to any behavior that they deem inappropriate, threatening, offensive, or harmful.

Community leaders have the right and responsibility to remove, edit, or reject comments, commits, code, wiki edits, issues, and other contributions that are not aligned with this Code of Conduct, and will communicate reasons for moderation decisions when appropriate.

### Scope

This Code of Conduct applies within all community spaces, and also applies when an individual is officially representing the community in public spaces. Examples of representing our community include using an official e-mail address, posting via an official social media account, or acting as an appointed representative at an online or offline event.

### Reporting Violations

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported to the community leaders responsible for enforcement at conduct@lutufi.org. All complaints will be reviewed and investigated promptly and fairly.

All community leaders are obligated to respect the privacy and security of the reporter of any incident.

### Enforcement Guidelines

Community leaders will follow these guidelines in determining the consequences for any action they deem in violation of this Code of Conduct:

**1. Correction**
- **Community Impact**: Use of inappropriate language or other behavior deemed unprofessional or unwelcome in the community.
- **Consequence**: A private, written warning from community leaders, providing clarity around the nature of the violation and an explanation of why the behavior was inappropriate. A public apology may be requested.

**2. Warning**
- **Community Impact**: A violation through a single incident or series of actions.
- **Consequence**: A warning with consequences for continued behavior. No interaction with the people involved, including unsolicited interaction with those enforcing the Code of Conduct, for a specified period of time. This includes avoiding interactions in community spaces as well as external channels like social media. Violating these terms may lead to a temporary or permanent ban.

**3. Temporary Ban**
- **Community Impact**: A serious violation of community standards, including sustained inappropriate behavior.
- **Consequence**: A temporary ban from any sort of interaction or public communication with the community for a specified period of time. No public or private interaction with the people involved, including unsolicited interaction with those enforcing the Code of Conduct, is allowed during this period. Violating these terms may lead to a permanent ban.

**4. Permanent Ban**
- **Community Impact**: Demonstrating a pattern of violation of community standards, including sustained inappropriate behavior, harassment of an individual, or aggression toward or disparagement of classes of individuals.
- **Consequence**: A permanent ban from any sort of public interaction within the community.

### Attribution

This Code of Conduct is adapted from the [Contributor Covenant](https://www.contributor-covenant.org), version 2.0, available at https://www.contributor-covenant.org/version/2/0/code_of_conduct.html.

---

## How to Contribute

### Step-by-Step for Different Contribution Types

#### Code Contributions

**1. Find or Create an Issue**
- Check existing issues for something you'd like to work on
- Create a new issue describing the bug or feature if one doesn't exist
- Wait for maintainer feedback before major efforts on new features

**2. Fork and Clone**
```bash
git clone https://github.com/YOUR_USERNAME/lutufi.git
cd lutufi
```

**3. Set Up Development Environment**
- Follow the [Development Setup](#development-setup) instructions
- Ensure tests pass before making changes

**4. Create a Branch**
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

**5. Make Changes**
- Write code following project conventions
- Add tests for new functionality
- Update documentation as needed

**6. Commit Changes**
- Follow [commit message standards](#commit-message-standards)
- Make focused, atomic commits

**7. Push and Create Pull Request**
```bash
git push origin feature/your-feature-name
```
- Create PR against the main repository
- Fill out the PR template completely
- Reference any related issues

**8. Respond to Review**
- Address reviewer feedback
- Make requested changes
- Engage constructively in discussion

#### Documentation Contributions

**1. Identify Documentation Needs**
- Missing documentation for existing features
- Clarification of complex concepts
- Translation to other languages
- New tutorials or examples

**2. Follow Documentation Style**
- Use clear, accessible language
- Include code examples
- Follow existing formatting conventions
- Cite sources for theoretical claims

**3. Submit via Pull Request**
- Documentation changes follow the same workflow as code
- Include screenshots for UI documentation
- Test code examples to ensure they work

#### Bug Reports

**Effective Bug Reports Include**:
- Clear description of the problem
- Steps to reproduce
- Expected vs. actual behavior
- Environment details (OS, Python version, Lutufi version)
- Minimal code example demonstrating the issue
- Relevant error messages and stack traces

**Before Submitting**:
- Search existing issues to avoid duplicates
- Test with the latest development version
- Verify the issue is truly a bug, not expected behavior

#### Feature Requests

**Effective Feature Requests Include**:
- Clear description of the proposed feature
- Rationale—why is this feature needed?
- Use cases and examples
- Possible implementation approaches
- Willingness to contribute implementation (if applicable)

**Before Submitting**:
- Check if similar features have been proposed or rejected
- Consider whether the feature aligns with project scope
- Discuss in forums for feedback before formal proposal

---

## Development Setup

### Getting the Development Environment Running

#### Prerequisites

**Required**:
- Python 3.9 or higher
- Git
- C++ compiler (for building extensions)

**Recommended**:
- Virtual environment manager (venv, conda, or pyenv)
- IDE with Python support (VS Code, PyCharm)
- Docker (for containerized development)

#### Setup Steps

**1. Clone the Repository**
```bash
git clone https://github.com/wasswalutufi/lutufi.git
cd lutufi
```

**2. Create Virtual Environment**
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

**3. Install Development Dependencies**
```bash
pip install -e ".[dev]"
```

This installs Lutufi in editable mode along with:
- Testing frameworks (pytest, hypothesis)
- Linting tools (ruff, mypy)
- Documentation tools (sphinx, myst-parser)
- Development utilities (pre-commit, tox)

**4. Install Pre-commit Hooks**
```bash
pre-commit install
```

**5. Verify Installation**
```bash
pytest tests/ -x  # Run tests to verify setup
```

### Directory Structure

```
lutufi/
  src/                    # Rust core
    core/
      models/
      representation/
      inference/
      learning/
      io/
    ffi/                  # PyO3 FFI layer
  python/                 # Installable Python package
    lutufi/
      __init__.py
      models.py
      inference.py
      learning.py
      io.py
  tests/                  # Rust tests (cargo test)
    unit/
    integration/
    ground_truth/         # Analytical solutions, Rust
  benches/                # Rust benchmarks (cargo bench)
  examples/               # Python examples via bindings
    epidemiology/
    finance/
    social/
    intelligence/
    validation/           # Examples that assert correctness
  bindings/               # Future R, Julia
  docs/
  Cargo.toml
  pyproject.toml
```

### Development Tools

**Linting and Formatting**:
```bash
ruff check src/           # Check for style issues
ruff format src/          # Auto-format code
mypy src/                 # Type checking
```

**Testing**:
```bash
pytest tests/unit/        # Run unit tests
pytest tests/integration/ # Run integration tests
pytest --cov=lutufi      # Run with coverage
```

**Documentation**:
```bash
cd docs/
make html                 # Build HTML documentation
make doctest             # Run doctests
```

### Troubleshooting

**Build Failures**: Ensure you have a C++ compiler installed and Python development headers.

**Import Errors**: Verify you're in the virtual environment and installed with `pip install -e`.

**Test Failures**: Some tests may require additional dependencies. Check test output for missing dependencies.

---

## Contribution Workflow

### Fork, Branch, Commit, Pull Request, Review, Merge

#### Fork

Create a personal fork of the Lutufi repository on GitHub. This is your own copy where you can make changes without affecting the main project.

#### Branch

Create a branch for your work:
- `feature/description` for new features
- `fix/description` for bug fixes
- `docs/description` for documentation changes
- `refactor/description` for code refactoring

Keep branches focused on a single logical change.

#### Commit

Make focused, atomic commits:
- Each commit should represent a single logical change
- Commit messages should follow [conventional commits format](#commit-message-standards)
- Commit frequently as you work

#### Pull Request

When ready, push your branch and create a pull request:

**PR Title**: Use conventional commit format (e.g., "feat: add loopy belief propagation")

**PR Description**:
- What changes were made
- Why the changes were made
- Reference to related issues (Fixes #123)
- Testing performed
- Breaking changes (if any)

**PR Checklist**:
- [ ] Tests pass locally
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] Code follows style guidelines
- [ ] Commit messages follow conventions
- [ ] No merge conflicts

#### Review

**For Contributors**:
- Respond to reviewer comments promptly
- Be open to feedback and suggestions
- Ask questions if feedback is unclear
- Make requested changes in new commits
- Re-request review when ready

**For Reviewers**:
- Review within reasonable timeframe (target: 5 business days)
- Be constructive and respectful in feedback
- Distinguish between required changes and suggestions
- Approve when satisfied, request changes when not

#### Merge

Maintainers will merge approved PRs:
- Squash merge for single logical changes
- Regular merge for PRs with intentionally separate commits
- Delete branch after merge
- Close related issues

### Continuous Integration

All PRs must pass CI checks:
- **Tests**: All tests must pass on supported Python versions
- **Linting**: Code must pass ruff and mypy checks
- **Documentation**: Documentation must build without errors
- **License**: All files must have appropriate license headers

---

## Commit Message Standards

### Conventional Commits Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages.

**Format**:
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, semicolons, etc.)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Code change that improves performance
- **test**: Adding or correcting tests
- **chore**: Changes to build process or auxiliary tools

### Scopes

Optional scope indicating the area of change:
- `core`: Core inference engine
- `networks`: Network representations
- `inference`: Inference algorithms
- `learning`: Learning algorithms
- `viz`: Visualization
- `docs`: Documentation
- `tests`: Test suite
- `build`: Build system

### Examples

```
feat(inference): implement loopy belief propagation

Add support for loopy belief propagation on graphs with cycles.
Uses damping to ensure convergence.

Closes #234
```

```
fix(core): correct normalization in factor operations

Factors were not being properly normalized during marginalization,
leading to incorrect probability distributions in edge cases.

Fixes #456
```

```
docs(tutorials): add epidemiological modeling tutorial

New tutorial demonstrating SIR model implementation using
Lutufi for contact network analysis.
```

### Breaking Changes

Indicate breaking changes with `!` after type/scope or with `BREAKING CHANGE:` in footer:

```
feat(api)!: redesign Network class interface

BREAKING CHANGE: Network class constructor signature changed.
Migration guide: https://docs.lutufi.org/migration/v2
```

### Best Practices

- Use imperative mood ("add" not "added")
- Keep first line under 72 characters
- Reference issues in body or footer
- Explain what and why, not how (code shows how)

---

## Code Review Process

### Who Reviews

**Maintainers**: Core maintainers (currently Wasswa Lutufi Sebbanja) review all PRs.

**Domain Experts**: Domain experts may be requested to review contributions in their area:
- Algorithm changes: Network science or statistics experts
- Performance changes: Systems engineering experts
- API changes: Core maintainers

**Community**: Community members are welcome to review PRs even if they don't have merge authority.

### What to Expect

**Timeline**:
- Initial acknowledgment: Within 2 business days
- First review: Within 5 business days
- Follow-up reviews: Within 2 business days

**What Reviewers Look For**:
- Correctness: Does the code work as intended?
- Tests: Are there adequate tests?
- Documentation: Is the code documented?
- Style: Does it follow project conventions?
- Performance: Are there performance concerns?
- Security: Are there security implications?
- Ethics: Does it align with ethical guidelines?

### Review Expectations

**For Small PRs** (bug fixes, documentation):
- Single reviewer approval sufficient
- Focus on correctness and style

**For Medium PRs** (new features, refactoring):
- Single maintainer approval required
- May request domain expert review
- Focus on design and maintainability

**For Large PRs** (architectural changes, major features):
- Multiple reviewers recommended
- May require design document review
- Focus on alignment with project direction

### Handling Review Feedback

**If You Disagree**:
- Explain your reasoning respectfully
- Ask questions to understand reviewer perspective
- Seek compromise when possible
- Escalate to BDFL if fundamental disagreement

**If You Need Time**:
- Let reviewers know you're working on changes
- It's okay to say "I'll address this next week"
- Mark comments as resolved when addressed

---

## Testing Requirements

### Tests Required for Contributions

All code contributions must include appropriate tests.

#### Unit Tests

Required for:
- New functions and methods
- Bug fixes (regression test)
- Edge cases and error handling

Location: `tests/unit/`

```python
def test_factor_multiplication():
    """Test that factor multiplication produces correct result."""
    f1 = Factor(variables=['A', 'B'], values=...)
    f2 = Factor(variables=['B', 'C'], values=...)
    result = f1 * f2
    expected = Factor(variables=['A', 'B', 'C'], values=...)
    assert result == expected
```

#### Integration Tests

Required for:
- Feature integration
- End-to-end workflows
- Cross-module interactions

Location: `tests/integration/`

#### Property-Based Tests

Encouraged for:
- Mathematical operations
- Invariants that should always hold
- Fuzzing for edge cases

Using [Hypothesis](https://hypothesis.readthedocs.io/):

```python
from hypothesis import given, strategies as st

@given(st.lists(st.integers(min_value=0), min_size=1))
def test_probabilities_sum_to_one(probabilities):
    """Probabilities in a distribution should sum to 1."""
    normalized = normalize(probabilities)
    assert abs(sum(normalized) - 1.0) < 1e-10
```

#### Performance Tests

Required for:
- Performance-critical algorithms
- Optimizations claiming performance improvements

Location: `benchmarks/`

### Test Coverage

**Target Coverage**: Minimum 80% code coverage

**Critical Paths**: Core inference algorithms should have near-100% coverage

**Coverage Reports**: Generated in CI and visible in PR checks

### Running Tests

```bash
# All tests
pytest

# Specific test file
pytest tests/unit/test_factor.py

# Specific test
pytest tests/unit/test_factor.py::test_factor_multiplication

# With coverage
pytest --cov=lutufi --cov-report=html

# Parallel execution
pytest -n auto
```

### Test Quality Guidelines

- Tests should be independent (no ordering dependencies)
- Tests should be deterministic (no random failures)
- Tests should be fast (milliseconds per test)
- Tests should be readable (clear arrange-act-assert structure)

---

## Documentation Requirements

### Docs Required for New Features

**All New Features Must Include**:

1. **API Documentation**
   - Docstrings for all public functions/classes
   - Type hints
   - Examples in docstrings

2. **User Documentation**
   - Description of the feature
   - Usage examples
   - When to use (and when not to use)

3. **What's New Entry**
   - Brief description for changelog

**Complex Features May Also Require**:
- Tutorial or guide
- Theoretical background
- Performance considerations
- Migration guide (if breaking change)

### Documentation Standards

#### Docstring Format

We use [Google-style docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings):

```python
def infer_posterior(network: Network, evidence: Dict[str, Any]) -> Distribution:
    """Perform posterior inference given evidence.
    
    Uses variable elimination to compute posterior distributions
    for all non-evidence variables in the network.
    
    Args:
        network: The Bayesian network to perform inference on.
        evidence: Dictionary mapping variable names to observed values.
        
    Returns:
        Distribution object containing posterior probabilities.
        
    Raises:
        ValueError: If evidence contains variables not in the network.
        InferenceError: If inference fails to converge.
        
    Example:
        >>> network = load_network('example.bn')
        >>> evidence = {'A': True, 'B': 42}
        >>> posterior = infer_posterior(network, evidence)
        >>> print(posterior['C'])
        {True: 0.7, False: 0.3}
    """
```

#### Tutorial Structure

Tutorials should follow this structure:
1. **Introduction**: What will be learned
2. **Prerequisites**: What knowledge is assumed
3. **Setup**: Code to get started
4. **Step-by-step**: Incremental development
5. **Complete example**: Full working code
6. **Exercises**: Suggestions for further exploration
7. **References**: Links to related resources

#### Example Quality

Examples in documentation should:
- Be complete and runnable
- Use realistic (but simple) data
- Include expected output
- Follow best practices

### Building Documentation

```bash
cd docs/
make html
# Output in _build/html/
```

Documentation is built automatically in CI and deployed to the documentation site.

---

## Intellectual Property

### DCO (Developer Certificate of Origin)

We use the Developer Certificate of Origin (DCO) to manage contribution rights.

**Sign-off Requirement**: All commits must include a sign-off:

```bash
git commit -s -m "feat: add new feature"
```

This adds a line to the commit message:
```
Signed-off-by: Your Name <your.email@example.com>
```

**What Sign-off Means**:
By signing off, you certify:
```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.


Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

### CLA (Contributor License Agreement)

For significant contributions, we may request a Contributor License Agreement:

**Individual CLA**: For individual contributors
**Corporate CLA**: For contributions made as part of employment

This provides additional legal clarity for both contributors and the project.

### Copyright

- Contributors retain copyright to their contributions
- By contributing, you license your work under Apache 2.0
- Include copyright notice in significant new files

### License Headers

All source files should include the Apache 2.0 license header:

```python
# Copyright 2026 [Your Name or Organization] and contributors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
```

---

## Recognition

### How Contributors Are Credited

**CONTRIBUTORS.md**: All contributors are listed in CONTRIBUTORS.md

**Release Notes**: Significant contributions are acknowledged in release notes

**Documentation**: Major contributors to specific features are acknowledged in relevant documentation

**Academic Citation**: Academic contributors can cite their contributions in publications

### Recognition Levels

**First Contribution**: Welcome and acknowledgment

**Regular Contributor**: Listed as contributor, early access to new features

**Significant Contributor**: Named in release notes, input on roadmap

**Core Contributor**: Commit access, recognized as project leader

**Maintainer**: Decision-making authority, governance role

### Hall of Fame

Outstanding contributions are recognized in the project Hall of Fame:
- Major architectural contributions
- Sustained high-quality contributions
- Exceptional community support
- Significant documentation efforts

### Academic Credit

Academic contributors can:
- List contributions on CVs
- Cite contributions in publications
- Request recommendation letters for significant contributions

---

## Governance Structure

### Decision-Making Process

**Current Model**: Benevolent Dictator for Life (BDFL)

Currently, Wasswa Lutufi Sebbanja serves as BDFL with final decision-making authority.

**Decision Levels**:

1. **Trivial Decisions** (typo fixes, documentation clarifications):
   - Any maintainer can decide
   - No formal process required

2. **Routine Decisions** (bug fixes, small features):
   - Community input welcome
   - BDFL or designated maintainer decides

3. **Significant Decisions** (new features, API changes):
   - Discussion in issues/PRs
   - Community feedback sought
   - BDFL decides after consideration

4. **Major Decisions** (architecture changes, governance):
   - Formal proposal (LEP)
   - Community discussion period
   - BDFL decides with community input

### Lutufi Enhancement Proposals (LEPs)

Major changes should be proposed as LEPs:

**Structure**:
- Summary
- Motivation
- Detailed design
- Alternatives considered
- Backwards compatibility
- Timeline

**Process**:
1. Draft LEP as issue or PR to docs/leps/
2. Community discussion period (minimum 2 weeks)
3. Revision based on feedback
4. Decision by BDFL
5. Implementation if approved

### BDFL Model

**Current BDFL**: Wasswa Lutufi Sebbanja

**BDFL Responsibilities**:
- Final decision-making authority
- Setting project direction
- Resolving disputes
- Representing the project
- Ensuring ethical alignment

**BDFL Limitations**:
- Expected to follow community input
- Expected to act in project's best interest
- Expected to delegate where appropriate
- Can be replaced if not serving project well

### Future Governance Evolution

As the community grows, we aim to transition to more distributed governance:

**Phase 1 (Current)**: BDFL model

**Phase 2** (Community size ~20 regular contributors):
- Add committers with domain expertise
- BDFL delegates routine decisions

**Phase 3** (Community size ~50 regular contributors):
- Steering committee for major decisions
- Working groups for specific domains
- BDFL as tie-breaker and visionary

**Phase 4** (Mature project):
- Foundation or nonprofit governance
- Elected steering committee
- Clear succession planning

### Governance Principles

Regardless of structure, governance will prioritize:
- **Transparency**: Decisions made openly
- **Inclusivity**: Diverse voices heard
- **Merit**: Contributions valued over credentials
- **Sustainability**: Long-term project health
- **Ethics**: Alignment with ethical framework

---

## Security Issues

### Separate Process for Security Vulnerabilities

Security vulnerabilities should **not** be reported through public GitHub issues.

**Reporting Channel**: security@lutufi.org

**What to Include**:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)
- Your disclosure timeline preferences

### Security Response Process

**1. Acknowledgment** (within 48 hours):
- Confirm receipt of report
- Assign CVE if appropriate

**2. Assessment** (within 1 week):
- Verify vulnerability
- Assess severity (CVSS scoring)
- Determine affected versions

**3. Remediation**:
- Develop fix
- Test fix thoroughly
- Prepare security advisory

**4. Disclosure**:
- Coordinate with reporter on disclosure timeline
- Target: 90 days from report to public disclosure
- Release patch
- Publish security advisory
- Credit reporter (unless anonymous)

### Security Best Practices for Contributors

- Never commit secrets or credentials
- Use parameterized queries (no SQL injection)
- Validate all inputs
- Be cautious with deserialization
- Follow OWASP guidelines
- Report potential vulnerabilities even if unsure

### Security Hall of Fame

Security researchers who report vulnerabilities responsibly are recognized in our Security Hall of Fame.

---

## Ethics Considerations

### Reviewing Contributions for Ethical Alignment

All contributions are reviewed for alignment with Lutufi's [ethical framework](ETHICS.md).

**What We Review**:

- **Feature Design**: Could this feature facilitate misuse?
- **Examples**: Do examples demonstrate responsible use?
- **Documentation**: Are ethical considerations documented?
- **Tests**: Are edge cases that could cause harm tested?

**Red Flags**:

- Features explicitly designed for surveillance
- Examples using sensitive data without safeguards
- Documentation that ignores privacy implications
- Optimizations that only benefit harmful use cases

**Green Flags**:

- Privacy-preserving implementations
- Documentation addressing ethical use
- Examples with synthetic data
- Features supporting transparency and accountability

### Ethical Review Process

**Self-Assessment**: Contributors should assess their own contributions against ETHICS.md

**Automated Checks**: CI includes checks for:
- Hardcoded credentials or sensitive data
- References to problematic use cases
- Missing documentation

**Reviewer Assessment**: Reviewers evaluate ethical implications

**Escalation**: Uncertain cases are escalated to BDFL or ethics committee

### Contributing Ethical Analysis

Contributions to governance documents are welcome:
- Improvements to ETHICS.md
- New misuse scenarios for MISUSE_ANALYSIS.md
- Case studies of ethical use
- Documentation of ethical considerations for specific domains

---

## Conclusion

Lutufi thrives on community contributions. Whether you're fixing a typo, implementing a new algorithm, or helping other users, your contribution matters.

**Key Reminders**:
- Be respectful and constructive
- Follow the process for your contribution type
- Include tests and documentation
- Sign off on your commits
- Align with our ethical framework
- Ask questions when unsure

**Getting Help**:
- GitHub Discussions for questions
- Issue trackers for bugs and features
- Email for private concerns

Thank you for contributing to Lutufi!

---

*This document is part of Lutufi's governance framework. For questions about contributing, contact contributors@lutufi.org*

*Last updated: March 2026*
