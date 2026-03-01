# Contributor Governance: Lutufi

## Welcome
Lutufi is a multi-disciplinary effort. We welcome contributions from mathematicians, sociologists, economists, and software engineers.

## Our Philosophy
1. **Quality over Quantity:** We prefer small, well-tested, and well-documented PRs over large, monolithic changes.
2. **Mathematical Correctness:** Any change to the inference engine must include validation against known analytical ground truths.
3. **Respect for Domain Expertise:** We recognize that a sociologist might have different requirements for the API than a software engineer. We design for the intersection.

## Contribution Standards
- **Coding Style:** We follow PEP 8 for Python and established conventions for our core engine.
- **Testing:** No feature is complete without unit tests and, where applicable, probabilistic validation tests.
- **Documentation:** Every public function must be documented. Every new module must include a theoretical overview in the `docs/` directory.

## Governance Model
Lutufi currently follows a **Benevolent Dictator for Life (BDFL)** model, led by the original creator. As the community grows, we aim to transition to a more distributed governance model (e.g., a Steering Committee) to ensure long-term sustainability.

## Decision Making
Significant changes to the API or architecture should be proposed as a **Lutufi Enhancement Proposal (LEP)** and discussed openly in the repository's issues or discussions section.

## Ethical Alignment
Contributors are expected to adhere to the principles laid out in the `docs/governance/ETHICS.md`. We reserve the right to reject contributions that explicitly facilitate the misuse scenarios identified in that document.

## Credit & Attribution
All contributors will be credited in the `CONTRIBUTORS.md` file and in the library's metadata. We believe in "Citation as Currency" and will ensure that individual contributions are visible to the academic and professional communities.
