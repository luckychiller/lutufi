# License Selection Rationale for Lutufi

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Why Apache 2.0](#why-apache-20)
2. [Alternative Licenses Considered](#alternative-licenses-considered)
3. [Patent Protection](#patent-protection)
4. [Commercial Use](#commercial-use)
5. [Academic Use](#academic-use)
6. [Government Use](#government-use)
7. [Attribution Requirements](#attribution-requirements)
8. [What the License Does NOT Do](#what-the-license-does-not-do)
9. [Compatibility with Other Licenses](#compatibility-with-other-licenses)
10. [Version Control and Licensing](#version-control-and-licensing)
11. [Future License Changes](#future-license-changes)
12. [Legal Review](#legal-review)

---

## Why Apache 2.0

The Apache License 2.0 was selected for Lutufi after careful consideration of the library's goals, intended users, and the broader ecosystem in which it will operate. This section details the reasoning behind this choice.

### The Core Philosophy

Lutufi is designed to bridge the gap between Bayesian network inference and network analysis, serving a diverse community of academic researchers, data scientists, government analysts, and commercial practitioners. The license must support this diversity while protecting the interests of contributors and users.

The Apache 2.0 license aligns with several core principles:

**1. Maximum Accessibility**: Lutufi's mission requires that the library be accessible to the broadest possible audience. This includes academics at resource-constrained institutions, government analysts working on public safety, startups building innovative applications, and large enterprises integrating Lutufi into their infrastructure.

**2. Patent Protection**: In domains like machine learning, network analysis, and probabilistic computing, patent litigation is a significant risk. The Apache 2.0 license's explicit patent grant protects contributors from frivolous lawsuits and protects users from patent-based threats to their use of the software.

**3. Institutional Trust**: The Apache 2.0 license is the standard for major infrastructure projects (TensorFlow, Kubernetes, Hadoop, Spark). Using this license signals that Lutufi is a serious, production-ready project suitable for institutional adoption.

**4. Attribution Preservation**: The attribution requirements ensure that the "Lutufi" name travels with the work, building recognition for the project and ensuring users can identify the source of the software they are using.

### Detailed Rationale

#### Balancing Freedom and Protection

The Apache 2.0 license strikes an optimal balance between user freedom and contributor protection:

**For Users**:
- Freedom to use Lutufi for any purpose, including commercial applications
- Freedom to modify and customize the software
- Freedom to distribute original or modified versions
- Freedom to combine Lutufi with other software
- Confidence that patent rights are addressed

**For Contributors**:
- Protection through the patent retaliation clause
- Confidence that their contributions will be attributed
- Assurance that derivative works will also be open and attributed
- Clear terms that reduce legal uncertainty

#### Enabling the Full Spectrum of Applications

Lutufi is designed for applications spanning:

- **Academic Research**: From computational sociology to epidemiology
- **Public Interest**: Public health, financial stability monitoring, counter-terrorism
- **Commercial Innovation**: Risk modeling, fraud detection, recommendation systems
- **Government Applications**: Intelligence analysis, infrastructure protection, regulatory oversight

The Apache 2.0 license imposes minimal restrictions on these applications, ensuring that Lutufi can serve its intended diverse community without artificial barriers.

#### Patent Considerations

Network analysis and Bayesian inference are active areas of patent activity. Large technology companies, financial institutions, and even patent trolls hold patents that could theoretically cover aspects of Lutufi's functionality.

The Apache 2.0 license addresses this through:

1. **Patent Grant**: Contributors explicitly grant patent rights to users (Section 3)
2. **Patent Retaliation**: Users who initiate patent litigation lose their patent rights under the license (Section 3)
3. **Defensive Protection**: The retaliation clause discourages frivolous patent litigation against the community

This protection is particularly important for:
- Individual contributors who cannot afford patent litigation
- Small companies building on Lutufi
- Academic institutions with limited legal resources
- International users who may face patent threats in multiple jurisdictions

#### Attribution and Recognition

Scientific and academic software depends on attribution for its sustainability. Researchers cite software they use, building the reputation of the project and its contributors. The Apache 2.0 license's attribution requirements (Section 4) ensure:

- The Lutufi name is preserved in derivative works
- Contributors receive credit for their work
- Users can identify the source of the software
- The academic citation chain is maintained

This is particularly important for academic software where "citation is currency" and proper attribution enables career advancement for contributors.

---

## Alternative Licenses Considered

Several alternative licenses were evaluated before selecting Apache 2.0. This section explains why each was rejected.

### MIT License

**Description**: The MIT License is a permissive free software license originating at the Massachusetts Institute of Technology. It is simple and widely used.

**Text**: The MIT license is short and straightforward, granting permission to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software.

**Why It Was Considered**:
- Maximum simplicity and brevity
- Wide familiarity in the open-source community
- Very permissive, imposing minimal restrictions

**Why It Was Rejected**:

1. **No Patent Protection**: The MIT license does not address patent rights. In the domains where Lutufi operates, this leaves contributors and users vulnerable to patent litigation. Contributors could inadvertently contribute patented technology and later sue users, or patent trolls could threaten the community.

2. **Attribution Weakness**: While the MIT license requires preserving copyright notices, it is less explicit about attribution in derivative works compared to Apache 2.0.

3. **Lack of Defensive Measures**: The MIT license lacks the patent retaliation clause that discourages frivolous litigation against the community.

**Conclusion**: While the MIT license's simplicity is appealing, the lack of patent protection is a significant vulnerability for a project in Lutufi's domains.

### GNU General Public License (GPL)

**Description**: The GPL is a copyleft license that requires derivative works to be distributed under the same license terms. It is designed to ensure that software remains free and open.

**Versions Considered**: GPL v2 and GPL v3

**Why It Was Considered**:
- Strong protection of user freedoms
- Large existing community of GPL-licensed software
- Philosophical alignment with ensuring software remains open

**Why It Was Rejected**:

1. **Institutional Adoption Barriers**: Many organizations, particularly in government and certain commercial sectors, have policies against using GPL-licensed software. This includes:
   - Intelligence agencies with concerns about license compliance in classified environments
   - Commercial entities building proprietary products
   - Government contractors with complex licensing requirements

2. **Academic Research Constraints**: Some academic institutions have policies that make GPL compliance difficult, particularly around collaboration with industry partners.

3. **Integration Challenges**: The GPL's copyleft provisions create uncertainty when integrating with non-GPL software, potentially limiting Lutufi's utility as a component in larger systems.

4. **Mission Conflict**: Lutufi's mission of serving diverse communities (including those building proprietary applications) is not well-served by GPL's restrictions.

**Conclusion**: The GPL's copyleft provisions would create adoption barriers that conflict with Lutufi's goal of maximum accessibility.

### GNU Lesser General Public License (LGPL)

**Description**: The LGPL is a compromise between the GPL and permissive licenses. It allows linking with proprietary software while maintaining copyleft for modifications to the library itself.

**Why It Was Considered**:
- Allows commercial use more readily than GPL
- Still provides copyleft protection for the library itself
- Compromise between philosophical and practical considerations

**Why It Was Rejected**:

1. **Complexity**: The LGPL's provisions regarding dynamic vs. static linking, derivative works, and aggregation create legal complexity that can deter adoption.

2. **Ambiguity in Network Analysis Context**: For a Python library like Lutufi, the distinction between "linking" and "using" is less clear than for C libraries, creating uncertainty about LGPL compliance.

3. **Remaining Adoption Barriers**: While less restrictive than GPL, LGPL still creates some institutional hesitancy and compliance concerns.

4. **Unnecessary Copyleft**: The Apache 2.0 license provides sufficient protection for contributors while being more permissive than LGPL.

**Conclusion**: LGPL's added complexity provides little benefit over Apache 2.0 for Lutufi's use case while creating unnecessary adoption barriers.

### BSD Licenses (2-Clause and 3-Clause)

**Description**: The BSD licenses are permissive licenses similar to MIT but with additional provisions regarding attribution and (in 3-Clause) endorsement.

**Why They Were Considered**:
- Permissive like MIT
- Widely used and understood
- 3-Clause BSD includes non-endorsement language

**Why They Were Rejected**:

1. **No Patent Protection**: Like MIT, BSD licenses do not address patent rights, leaving the community vulnerable.

2. **Attribution Provisions**: While BSD licenses require attribution, Apache 2.0's provisions are more comprehensive and explicit.

3. **Institutional Preference**: Apache 2.0 has become the preferred license for major open-source projects, particularly those with institutional adoption goals.

**Conclusion**: Apache 2.0 provides better protection than BSD licenses without significant additional restrictions.

### Proprietary / Commercial License

**Description**: Retaining proprietary rights and offering commercial licenses to paying customers.

**Why It Was Considered**:
- Potential revenue generation
- Maximum control over the software
- Ability to offer different terms to different users

**Why It Was Rejected**:

1. **Mission Conflict**: Proprietary licensing conflicts with Lutufi's mission of democratizing access to advanced network analysis capabilities. Many potential users (academic researchers, resource-constrained institutions, public interest organizations) would be unable to afford licenses.

2. **Adoption Barriers**: Proprietary software faces significant adoption barriers in academic and government contexts where open-source is preferred or required.

3. **Community Building**: Open-source licensing is essential for building a contributor community. Proprietary software rarely attracts volunteer contributors.

4. **Innovation and Improvement**: Open-source development with community contributions typically produces higher quality software than closed development.

5. **Transparency Requirements**: For a tool with potential surveillance applications, open-source provides crucial transparency and accountability.

6. **Dual-Licensing Complexity**: Dual-licensing (open-source + commercial) adds complexity and can create confusion about which terms apply.

**Conclusion**: Proprietary licensing is fundamentally incompatible with Lutufi's mission and goals.

### Creative Commons Licenses

**Description**: Creative Commons licenses are designed for creative works, not software.

**Why It Was Not Seriously Considered**:
Creative Commons explicitly discourages using their licenses for software. CC licenses are not designed for code and do not address software-specific concerns like source code distribution, patent rights, or linking.

**Conclusion**: Not appropriate for software projects.

### Mozilla Public License (MPL)

**Description**: The MPL is a file-level copyleft license that requires modifications to MPL-licensed files to be shared under MPL, but allows combination with proprietary code.

**Why It Was Considered**:
- Middle ground between GPL and permissive licenses
- File-level copyleft provides some protection
- Compatible with proprietary combination

**Why It Was Rejected**:

1. **Complexity**: The file-level copyleft creates complexity in understanding license obligations, particularly for larger projects.

2. **Less Familiar**: MPL is less widely used than Apache 2.0, creating uncertainty for institutional adopters.

3. **Insufficient Advantage**: The file-level copyleft provides limited additional protection over Apache 2.0 while adding complexity.

**Conclusion**: Apache 2.0 provides a better balance of protection and simplicity.

---

## Patent Protection

### The Patent Problem in Software

Software patents pose significant risks to open-source projects and their users:

**1. Submarine Patents**: Patents may exist that cover aspects of the software without the developers' knowledge. These "submarine patents" can surface years after development, threatening users and contributors.

**2. Patent Trolls**: Entities that acquire patents solely to extract licensing fees through litigation pose ongoing threats to successful open-source projects.

**3. Defensive Patents**: Large companies accumulate patent portfolios that can be used offensively against competitors using open-source software.

**4. Contributor Patents**: Contributors may hold patents that cover their contributions and could later assert them against users.

### How Apache 2.0 Addresses Patent Risks

**Section 3: Grant of Patent License**

The Apache 2.0 license includes an explicit patent grant:

> "Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable (except as stated in this section) patent license to make, have made, use, offer to sell, sell, import, and otherwise transfer the Work..."

This means:
- Contributors cannot later sue users for patent infringement related to their contributions
- Users have explicit rights to use patented technology contributed to the project
- The license is perpetual and irrevocable (with one exception)

**Section 3: Patent Litigation Termination**

The license also includes a patent retaliation clause:

> "If You institute patent litigation against anyone... alleging that the Work or a Contribution incorporated within the Work constitutes direct or contributory patent infringement, then any patent licenses granted to You under this License for that Work shall terminate..."

This means:
- Users who sue over patents related to Lutufi lose their patent rights under the license
- This discourages frivolous patent litigation against the community
- It provides a defensive mechanism to protect the community

### Implications for Lutufi

Given Lutufi's domains (Bayesian networks, social network analysis, influence modeling), patent risks include:

- Probabilistic inference algorithms
- Network propagation models
- Specific optimization techniques
- Data structures for sparse networks

The Apache 2.0 patent provisions protect:

**Individual Contributors**: An academic who contributes a novel algorithm cannot later patent it and sue commercial users.

**Commercial Users**: A startup building on Lutufi is protected from patent claims by contributors.

**The Community**: The retaliation clause discourages patent trolls from targeting Lutufi users.

### Limitations of Patent Protection

Important limitations to understand:

**1. Third-Party Patents**: The Apache 2.0 license only addresses patents held by contributors. It does not protect against patents held by third parties who did not contribute to Lutufi.

**2. Independent Development**: The license does not prevent others from independently developing and patenting similar technology.

**3. Patent Quality**: The license does not guarantee that contributed technology is non-infringing. It only addresses patents held by contributors.

**4. Jurisdictional Variation**: Patent law varies by jurisdiction, and the Apache 2.0 patent provisions may have different effects in different countries.

### Best Practices for Patent Risk Management

In addition to the Apache 2.0 provisions, Lutufi adopts the following practices:

**Prior Art Documentation**: Comprehensive documentation of algorithms and their sources establishes prior art that may prevent subsequent patenting.

**Open Development**: Public development and discussion of algorithms creates a public record that supports prior art claims.

**Patent Policy**: Contributors are encouraged not to file patents on Lutufi-related innovations, or to license any such patents broadly.

**Legal Monitoring**: The project monitors patent filings in relevant domains to identify potential threats.

---

## Commercial Use

### Why Commercial Use Is Permitted

The Apache 2.0 license explicitly permits commercial use, and this is intentional and important for Lutufi.

### The Value of Commercial Use

**Innovation and Investment**: Commercial use drives investment in improving and extending Lutufi. Companies using Lutufi for commercial applications have incentives to:
- Contribute bug fixes and improvements
- Fund development of new features
- Support the project financially
- Hire contributors to work on Lutufi

**Sustainability**: A healthy ecosystem of commercial users provides resources that sustain the project's long-term development.

**Adoption and Impact**: Commercial use extends Lutufi's reach and impact. Applications built by companies reach users who might not directly use the library themselves.

**Real-World Testing**: Commercial deployments subject Lutufi to rigorous testing at scale, identifying issues that might not surface in academic use.

### Addressing Concerns About Commercial Use

**Concern**: Commercial companies will take Lutufi, build proprietary products, and give nothing back.

**Response**: While this is possible, experience shows that successful commercial users typically contribute back:
- Bug fixes and performance improvements
- Documentation and examples
- Financial support
- Employment of core contributors

The Apache 2.0 attribution requirements ensure that even proprietary products must acknowledge Lutufi, building the project's reputation.

**Concern**: Commercial use will create competition with open-source versions.

**Response**: Commercial products built on Lutufi typically add value through integration, support, and domain-specific applications rather than competing directly with the open-source library.

**Concern**: Commercial users will hoard improvements.

**Response**: The Apache 2.0 license requires that distributed derivative works include attribution and license information. While it does not require sharing modifications, in practice, maintaining a private fork is often more expensive than contributing upstream.

### Implications of Commercial Use Permission

**No Royalties**: Commercial users do not pay royalties for using Lutufi.

**No Usage Restrictions**: There are no restrictions on the type of commercial applications (with the exception of ethical prohibitions documented separately).

**No Support Obligations**: The license disclaims any obligation to provide support to commercial users (or any users).

**Attribution Required**: Commercial products must include appropriate attribution to Lutufi.

### Commercial Use Guidelines

While the license permits commercial use, commercial users are expected to:

1. **Comply with the Ethical Framework**: Commercial use must align with the ethical guidelines in ETHICS.md
2. **Provide Attribution**: Include Lutufi attribution in products and documentation
3. **Consider Contributing**: Evaluate opportunities to contribute improvements back to the project
4. **Support the Ecosystem**: Consider financial or in-kind support for Lutufi development
5. **Respect Trademarks**: Do not use the Lutufi name in ways that suggest endorsement without permission

---

## Academic Use

### Why Permissive Licensing Helps Academic Adoption

Academic research has unique requirements that the Apache 2.0 license supports effectively.

### Academic Benefits of Apache 2.0

**Publication Requirements**: Academic researchers must publish their work. The Apache 2.0 license permits publication of research using Lutufi without licensing complications.

**Collaboration**: Researchers collaborate across institutions and with industry partners. Apache 2.0's permissive terms facilitate these collaborations without licensing barriers.

**Grant Compliance**: Many research grants require or prefer open-source licensing of resulting software. Apache 2.0 satisfies these requirements.

**Citation and Attribution**: The attribution requirements support academic norms of citation and credit, helping build the reputations of contributors.

**No Commercial Restrictions**: Researchers can spin off companies based on their research using Lutufi without license conflicts.

**Institutional Acceptance**: Apache 2.0 is widely accepted by university technology transfer offices and legal departments.

### Academic Use Cases

**Research Software**: Researchers can build Lutufi into research software without licensing concerns.

**Teaching**: Lutufi can be used in courses and educational materials.

**Reproducibility**: The open-source nature of Lutufi supports reproducible research—others can access and verify the exact software used in published research.

**Methodological Development**: Researchers developing new methods can extend Lutufi and share their extensions under compatible terms.

### Academic Citation

Academic users are expected to cite Lutufi in publications:

```
Sebbanja, W. L. (2026). Lutufi: A Library for Bayesian Network Analysis 
of Social and Economic Networks. [Software]. 
https://github.com/wasswalutufi/lutufi
```

(Note: Formal citation format will be established as the project matures)

Proper citation:
- Acknowledges the work of contributors
- Helps track academic impact of the project
- Supports funding applications for continued development
- Enables other researchers to find and use the software

### Academic Contributions

Academic contributions to Lutufi are welcomed and valued:

**Algorithm Implementation**: Implementing published algorithms with proper attribution
**Documentation**: Contributing theoretical explanations and domain knowledge
**Testing**: Providing test cases and validation datasets
**Bug Reports**: Identifying and reporting issues
**Feature Development**: Developing new capabilities

Academic contributors retain the right to publish their work while contributing to the open-source project.

---

## Government Use

### Why Apache 2.0 Works for Government/Intelligence Users

Government and intelligence community (IC) users have specific requirements that Apache 2.0 addresses effectively.

### Government Adoption Requirements

**No Cost**: Government budgets are constrained. Apache 2.0 imposes no licensing fees.

**No Export Control Issues**: Apache 2.0 does not create export control complications (though see EXPORT_CONTROL.md for Lutufi-specific considerations).

**Auditability**: The open-source nature allows security auditing required for government use.

**Modification Rights**: Government users can modify the software for specific operational needs.

**Vendor Independence**: No dependency on a single vendor's licensing decisions.

Apache 2.0 addresses all of these requirements.

### Intelligence Community Considerations

The IC has specific requirements for software used in operational contexts:

**Security**: Source code must be auditable for security vulnerabilities.

**Modification**: Operational needs often require modifications that cannot wait for vendor updates.

**Integration**: Must integrate with existing classified systems.

**No External Dependencies**: License terms must not create dependencies on external entities.

Apache 2.0 addresses all of these requirements.

### Classification and Licensing

Unique considerations for classified use:

**Distribution in Classified Environments**: Apache 2.0 permits distribution in classified environments, provided the license terms are followed.

**Attribution in Classified Products**: Attribution requirements apply even to classified products, though attribution may be internal rather than public.

**Modifications**: Modifications made for classified use remain subject to Apache 2.0 terms.

### Government Contributions

Government agencies can contribute to Lutufi:

**Public Contributions**: Unclassified improvements can be contributed back to the public repository.

**Government-Funded Development**: Development funded by government grants or contracts can be contributed under Apache 2.0 (subject to grant terms).

**Collaboration**: Government researchers can collaborate with academic and commercial contributors.

### Government Ethics and Legal Compliance

Government use of Lutufi must comply with:

**Legal Authority**: Use must be authorized by applicable law (surveillance statutes, intelligence authorizations, etc.)

**Human Rights**: Use must respect human rights obligations

**Oversight**: Use must be subject to appropriate oversight

**Transparency**: Where possible and legally permissible, use should be transparent

These requirements are addressed in the ethical framework (ETHICS.md) and are in addition to license terms.

---

## Attribution Requirements

### How Apache 2.0 Requires Credit

The Apache 2.0 license includes specific attribution requirements that ensure Lutufi and its contributors receive appropriate credit.

### Section 4: Redistribution

The license states:

> "You must give any other recipients of the Work or Derivative Works a copy of this License; and... You must cause any modified files to carry prominent notices stating that You changed the files..."

### Specific Attribution Requirements

**1. License Inclusion**: Any distribution of Lutufi or derivative works must include a copy of the Apache 2.0 license.

**2. NOTICE File**: If a NOTICE file is included with the original work, derivative works must include a readable copy of the attribution notices contained within that file.

**3. Modified Files**: Any modified files must carry prominent notices stating that changes were made.

**4. No Endorsement**: Derivative works must not claim endorsement by the original authors without specific permission.

### Lutufi's NOTICE File

Lutufi includes a NOTICE file containing:

```
Lutufi
Copyright 2026 Wasswa Lutufi Sebbanja and contributors

This product includes software developed by Wasswa Lutufi Sebbanja
and the Lutufi contributors (https://github.com/wasswalutufi/lutufi).
```

This NOTICE must be preserved in derivative works.

### Attribution in Different Contexts

**Academic Publications**: Academic users should cite Lutufi in publications using the software.

**Commercial Products**: Commercial products must include Lutufi attribution in documentation, about screens, or other appropriate locations.

**Web Applications**: Web applications using Lutufi should include attribution in documentation or about pages.

**Internal Use**: Internal use within organizations does not require public attribution, though the license terms still apply.

**Modified Versions**: Modified versions must indicate what changes were made while preserving original attribution.

### Why Attribution Matters

**Credit**: Contributors deserve credit for their work.

**Provenance**: Users should know the source of the software they are using.

**Quality**: Attribution helps users identify the quality and reliability of software.

**Community**: Attribution builds the Lutufi community and reputation.

**Sustainability**: Recognition encourages continued contribution and support.

---

## What the License Does NOT Do

Understanding the limitations of the Apache 2.0 license is essential for informed use of Lutufi.

### No Warranty

**Section 7: Disclaimer of Warranty**

> "Unless required by applicable law or agreed to in writing, Licensor provides the Work (and each Contributor provides its Contributions) on an 'AS IS' BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND..."

**What This Means**:
- Lutufi is provided without warranty of any kind
- There is no guarantee that Lutufi will work for your specific purpose
- There is no guarantee that Lutufi is free of bugs or defects
- There is no guarantee that Lutufi is secure or suitable for any particular use case

**Implications**: Users must evaluate Lutufi for their specific needs and assume all risks associated with its use. Commercial support may be available from third parties, but is not provided by the license.

### No Liability

**Section 8: Limitation of Liability**

> "In no event and under no legal theory... shall any Contributor be liable to You for damages..."

**What This Means**:
- Contributors are not liable for damages arising from use of Lutufi
- This includes direct, indirect, special, incidental, and consequential damages
- This applies even if contributors were advised of the possibility of such damages

**Implications**: Users cannot sue contributors for damages caused by Lutufi, even if those damages result from bugs, security vulnerabilities, or other issues. Users who require liability protection must obtain insurance or support contracts from third parties.

### No Control Over Use

**What This Means**:
- The license does not restrict how Lutufi is used
- The license does not prevent use for harmful or unethical purposes
- The license does not provide mechanisms to enforce ethical guidelines

**Implications**: While Lutufi's governance documents (ETHICS.md, MISUSE_ANALYSIS.md) establish ethical expectations, the Apache 2.0 license does not enforce them. Enforcement of ethical guidelines relies on community norms, institutional oversight, and legal mechanisms outside the license.

### No Trademark Grant

**Section 6: Trademarks**

> "This License does not grant permission to use the trade names, trademarks, service marks, or product names of the Licensor..."

**What This Means**:
- The license does not grant rights to use the "Lutufi" name as a trademark
- Derivative works cannot claim to be "official" Lutufi without permission
- The Lutufi logo and branding cannot be used without permission

**Implications**: Users can use Lutufi and refer to it by name factually, but cannot imply endorsement or create confusion about the source of derivative works.

### No Patent Protection from Third Parties

**What This Means**:
- The license only addresses patents held by contributors
- Third parties who did not contribute to Lutufi may still hold patents that cover aspects of the software
- The license does not protect against patent infringement claims by non-contributors

**Implications**: Users may still face patent risks from entities who did not contribute to Lutufi. Due diligence regarding third-party patents remains the user's responsibility.

### No Guarantee of Compatibility

**What This Means**:
- The license does not guarantee that Lutufi is compatible with any particular system or standard
- The license does not guarantee that Lutufi will continue to be maintained or developed
- The license does not guarantee that future versions will be backward compatible

**Implications**: Users should evaluate compatibility and maintenance expectations independently.

---

## Compatibility with Other Licenses

### How Apache 2.0 Combines with Other Open Source

License compatibility determines whether Lutufi can be combined with software under other licenses.

### Apache 2.0 Compatibility Overview

**Compatible with Apache 2.0**:
- MIT License
- BSD (2-clause and 3-clause)
- ISC License
- Python Software Foundation License
- zlib License
- Apache 2.0 (obviously)

**Compatible with Restrictions**:
- LGPL v2.1/v3 (must keep LGPL components separate)
- MPL (file-level copyleft)

**Incompatible**:
- GPL v2 (without "or later" clause)
- Proprietary licenses (cannot combine with open-source obligations)

### Combining Lutufi with Other Software

**MIT/BSD Libraries**: Lutufi can freely incorporate MIT or BSD-licensed libraries. The resulting combination is under Apache 2.0.

**LGPL Libraries**: Lutufi can use LGPL libraries as dependencies. The LGPL components remain under LGPL, while Lutufi code remains under Apache 2.0.

**GPL Libraries**: Lutufi cannot incorporate GPL v2 code. GPL v3 code can be used only if the entire combination is distributed under GPL v3, which would conflict with Lutufi's Apache 2.0 license.

### Using Lutufi in Other Projects

**In MIT/BSD Projects**: Projects under MIT or BSD licenses can use Lutufi. The combination is subject to both licenses' terms.

**In GPL Projects**: GPL projects can use Lutufi. The Apache 2.0 license is compatible with GPL v3 (and GPL v2 with "or later" clause).

**In Proprietary Projects**: Proprietary projects can use Lutufi, provided they comply with Apache 2.0 attribution requirements.

### Dependency Management

Lutufi's dependencies are carefully selected to maintain license compatibility:

- Core dependencies are under permissive licenses (MIT, BSD, Apache 2.0)
- Optional dependencies with copyleft licenses are clearly documented
- Users are responsible for understanding the license implications of their specific dependency configuration

### License Compliance Resources

For complex scenarios, consult:
- [Apache Software Foundation License Compatibility](https://www.apache.org/legal/resolved.html)
- [Open Source Initiative License Information](https://opensource.org/licenses)
- Legal counsel for specific situations

---

## Version Control and Licensing

### How the License Applies to Contributions

The Apache 2.0 license governs all contributions to Lutufi, ensuring a consistent legal framework for the project.

### Contribution Licensing

**By Contributing**: Anyone contributing to Lutufi agrees to license their contributions under Apache 2.0. This is typically acknowledged through:
- Signed Contributor License Agreements (CLAs)
- Developer Certificate of Origin (DCO) sign-offs
- Explicit statements in pull requests

**What This Means**:
- Contributors retain copyright to their contributions
- Contributors grant the broad permissions specified in Apache 2.0
- Contributors provide the patent grant specified in Apache 2.0
- The project can distribute contributions under Apache 2.0

### Copyright Notices

**Individual Copyrights**: Contributors retain copyright to their individual contributions. Copyright notices should be preserved in file headers.

**Project Copyright**: The project as a whole is copyrighted by Wasswa Lutufi Sebbanja and the Lutufi contributors.

**Copyright Headers**: Source files include headers indicating copyright and license:

```python
# Copyright 2026 Wasswa Lutufi Sebbanja and contributors
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

### Version-Specific Licensing

**All Versions Under Apache 2.0**: All versions of Lutufi, from the initial release forward, are licensed under Apache 2.0.

**Future License Changes**: See [Future License Changes](#future-license-changes) for discussion of potential future licensing decisions.

**Retroactive Application**: The Apache 2.0 license applies to all code in the repository, including historical commits.

### Forks and Derivatives

**Forking**: Anyone can fork Lutufi under the Apache 2.0 license terms. Forks must maintain attribution and license compliance.

**Derivatives**: Derivative works must comply with Apache 2.0 terms, including attribution and license inclusion.

**Re-licensing**: Derivative works cannot change the license of Lutufi code they incorporate. New code added in derivatives can be under a different license, but the Lutufi-derived portions remain under Apache 2.0.

---

## Future License Changes

### Possibility and Process

While Lutufi is currently licensed under Apache 2.0, circumstances may arise that warrant consideration of license changes.

### When License Changes Might Be Considered

**Significant Legal Changes**: Changes in law that affect the validity or enforceability of Apache 2.0.

**New Requirements**: Emerging requirements from the community or stakeholders that Apache 2.0 cannot accommodate.

**Compatibility Issues**: Fundamental incompatibility with essential dependencies or use cases.

**Governance Evolution**: Changes in project governance that necessitate different licensing approaches.

### Process for License Changes

Any license change would follow a rigorous process:

**1. Proposal**: A detailed proposal outlining the rationale, proposed new license, and expected impacts.

**2. Community Consultation**: Extended period for community input and discussion.

**3. Contributor Consent**: Effort to obtain consent from all significant contributors (or determine that their contributions can be relicensed).

**4. Legal Review**: Comprehensive legal review of the proposed change.

**5. Decision**: Decision by project governance (currently BDFL, potentially steering committee in future).

**6. Transition Period**: Grace period for users to adapt to new terms.

**7. Implementation**: Execution of the license change with clear documentation.

### Constraints on License Changes

**Historical Code**: Code already released under Apache 2.0 remains under those terms forever. License changes only apply to future versions.

**Contributor Rights**: Contributors who object to a license change may have their contributions removed from future versions if their consent cannot be obtained.

**Community Trust**: License changes can damage community trust. Any change would require overwhelming justification.

### No Current Plans for Change

As of this writing, there are **no plans** to change Lutufi's license from Apache 2.0. The license was selected after careful consideration and is expected to serve the project's needs for the foreseeable future.

---

## Legal Review

### Recommendation for Legal Review

**This document is not legal advice.** It explains the reasoning behind Lutufi's licensing choice and provides information about the Apache 2.0 license, but it does not constitute legal counsel.

### When to Seek Legal Review

Organizations using Lutufi should consider legal review in the following situations:

**Commercial Products**: Before integrating Lutufi into commercial products, especially if those products will be distributed.

**Government Contracts**: Before using Lutufi in government contracts, particularly in classified or sensitive contexts.

**Patent Concerns**: If you have patents that might relate to Lutufi's functionality, or if you are concerned about patent risks.

**License Compliance Programs**: As part of establishing or maintaining open-source license compliance programs.

**Derivative Works**: Before creating and distributing derivative works based on Lutufi.

**Custom Modifications**: Before making significant modifications for internal use, to understand any obligations that might arise if those modifications are later distributed.

**Multi-Licensing Scenarios**: If considering dual-licensing or offering Lutufi under different terms.

### Resources for Legal Review

**Open Source Program Offices**: Many organizations have internal open-source program offices (OSPOs) that can provide guidance.

**Legal Counsel**: Consult with attorneys experienced in open-source software licensing.

**Open Source Foundations**: Organizations like the Apache Software Foundation, Linux Foundation, and Open Source Initiative can provide resources and guidance.

**Community**: The Lutufi community can share experiences and approaches, though not legal advice.

### Open Source Compliance Tools

Several tools can help with license compliance:

- **FOSSology**: Open source license compliance toolkit
- **ScanCode**: Code scanning tool for license detection
- **Black Duck**: Commercial open-source security and compliance management
- **Snyk**: Security and license compliance scanning

These tools can help identify license obligations and ensure compliance.

### Disclaimer

**THE INFORMATION IN THIS DOCUMENT IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND.** Lutufi's authors and contributors are not attorneys, and this document does not constitute legal advice. Users should consult with qualified legal counsel for advice specific to their situation.

---

## Conclusion

The Apache License 2.0 was selected for Lutufi after careful consideration of the project's goals, community needs, and the broader ecosystem. It provides:

- **Maximum accessibility** for diverse users including academics, government analysts, and commercial developers
- **Patent protection** for contributors and users in a patent-active domain
- **Attribution preservation** ensuring recognition for contributors
- **Institutional trust** through adoption of a standard, widely-accepted license
- **Permissive terms** that enable innovation and broad adoption

While no license is perfect for all situations, Apache 2.0 provides the optimal balance for Lutufi's mission of democratizing access to advanced network analysis capabilities.

Users are encouraged to understand the license terms, comply with attribution requirements, and seek legal counsel when appropriate. The open-source community depends on mutual respect for license terms to sustain the commons of shared software.

---

*This document is part of Lutufi's governance framework. For the full license text, see the LICENSE file in the repository root.*

*Last updated: March 2026*
