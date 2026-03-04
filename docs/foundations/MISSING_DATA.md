# Missing Data Theory

**Document Version 1.0**  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Why Missing Data is Central to Lutufi](#why-missing-data-is-central-to-lutufi)
2. [Rubin's Taxonomy](#rubins-taxonomy)
3. [Impact on Network Analysis](#impact-on-network-analysis)
4. [Complete Case Analysis](#complete-case-analysis)
5. [Single Imputation](#single-imputation)
6. [Multiple Imputation](#multiple-imputation)
7. [EM Algorithm for Missing Data](#em-algorithm-for-missing-data)
8. [Full Information Maximum Likelihood](#full-information-maximum-likelihood)
9. [Handling MNAR in Networks](#handling-mnar-in-networks)
10. [Missing Node Problem](#missing-node-problem)
11. [Missing Edge Problem](#missing-edge-problem)
12. [Informative Missingness](#informative-missingness)
13. [Sensitivity Analysis for Missing Data](#sensitivity-analysis-for-missing-data)
14. [Censoring and Truncation](#censoring-and-truncation)
15. [Best Practices in Network Research](#best-practices-in-network-research)
16. [How Lutufi Handles Missing Data](#how-lutufi-handles-missing-data)
17. [Key References](#key-references)

---

## Why Missing Data is Central to Lutufi

Missing data is not an inconvenience in social and economic network analysis—it is the norm. Unlike carefully controlled experiments where missingness is rare and random, network data collection operates in messy, uncontrolled environments where missingness is pervasive, systematic, and often informative. Understanding and properly handling missing data is therefore fundamental to Lutufi's mission of unifying Bayesian networks with social and economic network analysis.

### Sources of Missing Data in Networks

**Survey Non-Response:**
In social network surveys, respondents may refuse to answer questions about their relationships, particularly regarding sensitive ties (illicit activities, romantic relationships, conflicts). Non-response rates of 20-50% are common, and higher for certain question types.

**Network Boundary Specification:**
The "network boundary problem"—deciding which nodes to include—inevitably leads to missing nodes. A network defined as "employees of Company X" excludes relevant external contacts; a friendship network defined by a school roster misses out-of-school relationships.

**Covert Relationships:**
Certain network relationships are intentionally hidden: criminal associations, covert political alliances, undisclosed financial ties. These edges are structurally missing, and their absence conveys information.

**Censored Data:**
Temporal network data is often censored—we observe the network up to time $t$ but not beyond. Cascades may be incomplete, relationships may form after observation ends, and long-term outcomes remain unknown.

**Measurement Limitations:**
Network data from digital sources (email logs, mobile phones, social media) captures only a subset of interactions and may miss offline relationships entirely.

### Consequences of Improper Handling

Improper handling of missing data leads to:
- **Biased parameter estimates:** Systematic missingness distorts learned relationships
- **Invalid inference:** Standard errors and confidence intervals become unreliable
- **Incorrect structure:** Missing data can induce spurious edges or hide true ones
- **Selection bias:** The observed network is not representative of the true network
- **Attenuation of effects:** Missing data reduces statistical power

The consequences are particularly severe in network analysis because missingness affects both local properties (node attributes) and global structure (connectivity, paths, communities).

---

## Rubin's Taxonomy

Donald Rubin's 1976 taxonomy classifies missing data mechanisms by their relationship to observed and unobserved values. This classification determines which analysis methods are appropriate.

### Formal Notation

Let $X = (X_{ij})$ be the complete data matrix where $X_{ij}$ is the value of variable $j$ for unit $i$. Define the **missingness indicator**:

$$M_{ij} = \begin{cases} 1 & \text{if } X_{ij} \text{ is missing} \\ 0 & \text{if } X_{ij} \text{ is observed} \end{cases}$$

The observed data is $X_{obs} = \{X_{ij} : M_{ij} = 0\}$ and missing data is $X_{miss} = \{X_{ij} : M_{ij} = 1\}$.

The missingness mechanism is characterized by the conditional distribution $P(M \mid X) = P(M \mid X_{obs}, X_{miss})$.

### MCAR: Missing Completely at Random

**Definition:** Data are **MCAR** if:

$$P(M \mid X) = P(M)$$

The probability of missingness is independent of all data, observed or unobserved.

**Equivalent:** $M \perp X$ (missingness is independent of the data).

**Example:** In a network survey, 10% of respondents are randomly selected for follow-up interviews due to budget constraints. Missingness depends only on random selection, not on network properties.

**Consequences:**
- Observed data is a random sample of complete data
- Complete case analysis is unbiased (though inefficient)
- Any valid analysis method works

**Testing:** Little's MCAR test compares means of observed variables across missingness patterns. Rejection suggests non-MCAR.

### MAR: Missing at Random

**Definition:** Data are **MAR** if:

$$P(M \mid X) = P(M \mid X_{obs})$$

Missingness depends only on observed data, not on unobserved values.

**Note:** "Random" here refers to conditional independence from $X_{miss}$, not to randomness in the common sense.

**Example:** In a friendship network survey, students are less likely to report friendships if they have few reported friendships themselves. Missingness depends on observed degree, not on the unobserved friendships themselves.

**Consequences:**
- Observed data is not a random sample
- Complete case analysis is generally biased
- Likelihood-based methods (EM, FIML, multiple imputation) give valid inference if the model for $P(X)$ is correct
- The missingness mechanism is **ignorable** for likelihood inference (though not for sampling-based inference)

### MNAR: Missing Not at Random

**Definition:** Data are **MNAR** if:

$$P(M \mid X) = P(M \mid X_{obs}, X_{miss})$$

Missingness depends on unobserved values, even after conditioning on observed data.

**Example:** High-status individuals in a corporate network are less likely to report their connections (status-driven non-response). Missingness depends on the unobserved status/prestige. Or, in a criminal network, individuals with more criminal ties are less likely to respond to surveys about associates.

**Consequences:**
- Standard methods are biased
- Joint modeling of data and missingness mechanism is required
- Inference depends on untestable assumptions about $P(M \mid X)$
- Sensitivity analysis is essential

### Their Implications for Inference

| Mechanism | $P(M \mid X)$ | Valid Methods | Ignorable? |
|-----------|---------------|---------------|------------|
| MCAR | $P(M)$ | Any | Yes |
| MAR | $P(M \mid X_{obs})$ | Likelihood, MI, EM | Yes (for likelihood) |
| MNAR | $P(M \mid X_{obs}, X_{miss})$ | Selection models, pattern mixture | No |

**Ignorability:** A missingness mechanism is **ignorable** if we can obtain valid parameter estimates for $P(X)$ without modeling $P(M \mid X)$. Under MAR, the likelihood factors as:

$$L(\theta \mid X_{obs}, M) = P(M \mid X_{obs}) \int P(X_{obs}, X_{miss} \mid \theta) dX_{miss}$$

If we only care about $\theta$, the missingness mechanism $P(M \mid X_{obs})$ can be ignored for likelihood inference.

### Worked Examples of Each

**Example 1 (MCAR):** A sensor network where 5% of nodes fail randomly due to hardware issues. The pattern of missing sensor readings is independent of the readings themselves or network position.

**Example 2 (MAR):** An email network where employees are less likely to archive emails if they receive fewer emails overall. Missingness depends on observed degree (number of received emails) but not on the content of missing emails.

**Example 3 (MNAR):** A drug trafficking network where individuals with the most connections to known traffickers avoid participating in surveys. Missingness depends on the unobserved number of criminal ties.

---

## Impact on Network Analysis

Missing data affects network analysis in ways that extend beyond standard statistical problems due to the interdependent nature of network structure.

### Network Boundary Problem

The **network boundary problem** asks: which nodes should be included in the network? Every boundary choice excludes potentially relevant actors, creating missing nodes.

**Types:**
- **Realist boundary:** Based on objective criteria (e.g., employees of Company X)
- **Nominalist boundary:** Based on shared attributes (e.g., "influential people in tech")
- **Relational boundary:** Based on connectivity (e.g., egocentric networks extending to 2-hop neighbors)

**Consequences:**
- Underestimation of centrality for boundary-spanning nodes
- Missed communities that span boundaries
- Biased degree distributions (truncation of high-degree nodes)

### Missing Nodes vs Missing Edges

**Missing Nodes:**
- Not all relevant actors are observed
- Affects global network properties (diameter, clustering, components)
- Cannot be imputed without strong assumptions

**Missing Edges:**
- Relationships between observed nodes are unknown
- Affects local network measures (degree, betweenness)
- Can be partially addressed through statistical models

**Node-level Missing Data:**
- Attributes of observed nodes are unknown
- Affects covariate analyses and community detection
- Can be handled with standard imputation methods

### Bias in Network Measures

Missing edges generally bias network measures:
- **Degree:** Underestimated (missing edges reduce observed degree)
- **Clustering coefficient:** Underestimated (fewer observed triangles)
- **Betweenness centrality:** Overestimated for nodes on observed paths; underestimated for nodes with many missing connections
- **Community structure:** Merged communities (missing inter-community edges) or fragmented communities (missing intra-community edges)

**Simulation Example:** In a network with 20% missing edges at random:
- Average degree reduced by ~20%
- Global clustering coefficient reduced by ~36% (since triangles require 3 edges)
- Giant component size may remain stable if network is dense

### Snowball Sampling Implications

**Snowball sampling** starts with seed nodes, interviews them, and follows their nominations. This creates specific missingness patterns:
- Nodes unreachable from seeds are missing entirely
- Within the sample, edges to unobserved nodes are missing
- Degree is censored (we know $d_{obs} \leq d_{true}$)

**Analysis implications:**
- Standard estimators are biased
- Specialized estimators (RDS, network scale-up) attempt correction
- Inference is conditional on the snowball sampling design

---

## Complete Case Analysis

Complete case analysis (listwise deletion) removes any case with any missing value. It is the default in many software packages but often problematic.

### When It Works (MCAR)

Under MCAR, complete case analysis is **unbiased** because the remaining cases are a random sample of the population. However, it is **inefficient**—variance increases because sample size is reduced.

**Efficiency loss:** If each variable has 10% missing independently, the complete case fraction is $(0.9)^p$ for $p$ variables. With 10 variables, only 35% of cases remain; with 20 variables, 12% remain.

### Bias Under MAR

Under MAR, complete case analysis is generally **biased**. The bias depends on the relationship between missingness and outcomes.

**Example:** In a regression of income on education, if high-income individuals are less likely to report income (MAR given other covariates), complete cases underrepresent high-income observations, biasing the slope downward.

### Inefficiency

Even when unbiased, complete case analysis discards information from incomplete cases. If missingness is scattered, most cases have some information but are discarded entirely.

**Relative efficiency:** For MCAR data with fraction $\pi$ missing, the efficiency of complete case analysis relative to full data is approximately $\pi$.

---

## Single Imputation

Single imputation replaces each missing value with a single plausible value. While convenient, it generally underestimates variance.

### Mean Imputation

Replace missing values with the variable mean:

$$\hat{x}_{ij} = \bar{x}_j = \frac{1}{n_{obs}} \sum_{i: x_{ij} \text{ obs}} x_{ij}$$

**Problems:**
- Underestimates variance (reduces spread)
- Distorts correlations (pulls correlations toward zero)
- Invalidates standard errors (appears more precise than it is)

### Regression Imputation

Predict missing values from observed variables using regression:

$$\hat{x}_{ij} = \hat{\beta}_0 + \sum_{k \neq j} \hat{\beta}_k x_{ik}$$

**Improvement over mean imputation:** Uses relationships among variables.

**Remaining problems:**
- Still underestimates variance (no residual error)
- Overstates relationships (correlations biased upward)
- Standard errors still invalid

### Limitations

Single imputation methods share common limitations:
1. **Underestimation of uncertainty:** No accounting for imputation uncertainty
2. **Distorted distributions:** Reduces variance and alters higher moments
3. **Biased inference:** Standard errors, confidence intervals, and p-values are incorrect
4. **Invalid for MNAR:** Cannot handle informative missingness

The fundamental problem: single imputation treats imputed values as observed, ignoring that they are estimated.

---

## Multiple Imputation

Multiple imputation (Rubin, 1987) addresses single imputation's limitations by creating multiple plausible datasets, analyzing each, and combining results.

### Rubin's Rules

**Procedure:**
1. **Imputation:** Create $m$ complete datasets by sampling from $P(X_{miss} \mid X_{obs})$
2. **Analysis:** Analyze each dataset separately, obtaining estimates $\hat{\theta}_k$ and standard errors $SE_k$
3. **Combination:** Combine using Rubin's rules

**Point estimate:**

$$\bar{\theta} = \frac{1}{m} \sum_{k=1}^m \hat{\theta}_k$$

**Variance decomposition:**

$$T = \bar{U} + \left(1 + \frac{1}{m}\right)B$$

where:
- **Within-imputation variance:** $\bar{U} = \frac{1}{m} \sum_{k=1}^m SE_k^2$
- **Between-imputation variance:** $B = \frac{1}{m-1} \sum_{k=1}^m (\hat{\theta}_k - \bar{\theta})^2$

The $(1 + 1/m)$ term adjusts for finite $m$.

### Combining Estimates

**Standard error:** $SE = \sqrt{T}$

**Degrees of freedom (Barnard-Rubin):**

$$\nu = \left(\frac{1}{m-1}\right) \frac{(1 + r^{-1})^2}{(1 + r^{-1})^2 + \frac{\nu_{com} + 1}{\nu_{com} + 3}}$$

where $r = (1 + 1/m)B/\bar{U}$ is the relative increase in variance due to missingness.

**Hypothesis tests:** Use t-statistics with $\nu$ degrees of freedom:

$$t = \frac{\bar{\theta} - \theta_0}{\sqrt{T}}$$

### Combining Variances

The total variance $T$ separates:
- **Sampling uncertainty:** Captured by $\bar{U}$ (variance if data were complete)
- **Missing data uncertainty:** Captured by $B$ (variance due to unknown true values)
- **Imputation uncertainty:** Captured by $B/m$ (variance due to finite imputations)

The **fraction of missing information** is $\lambda = \frac{B + B/m}{T}$, measuring the proportion of total uncertainty due to missing data.

### Proper Imputation

An imputation method is **proper** if:
1. It accounts for uncertainty in the imputation model
2. It creates draws from the correct predictive distribution
3. It preserves relationships among variables

**Bayesian proper imputation:**
1. Draw parameters from posterior: $\theta^* \sim P(\theta \mid X_{obs})$
2. Draw missing values from predictive: $X_{miss}^* \sim P(X_{miss} \mid X_{obs}, \theta^*)$

This accounts for both parameter uncertainty and inherent uncertainty in $X_{miss}$.

**Number of imputations:**
- Rule of thumb: $m = 5$ for moderate missingness, $m = 20$ for high missingness
- Efficiency: Relative efficiency $\approx (1 + \lambda/m)^{-1}$, so $m = 5$ gives >95% efficiency for $\lambda < 0.5$

---

## EM Algorithm for Missing Data

The Expectation-Maximization (EM) algorithm (Dempster, Laird, Rubin, 1977) finds maximum likelihood estimates with missing data through iterative expectation and maximization steps.

### E-Step and M-Step for Incomplete Data

**Notation:** Let $\mathbf{X}$ be complete data, $\mathbf{Y} = \mathbf{X}_{obs}$ observed data, $\mathbf{Z} = \mathbf{X}_{miss}$ missing data, and $\theta$ parameters.

**Complete data log-likelihood:**

$$\ell_c(\theta; \mathbf{X}) = \log P(\mathbf{X} \mid \theta)$$

**Observed data log-likelihood:**

$$\ell(\theta; \mathbf{Y}) = \log \int P(\mathbf{X} \mid \theta) d\mathbf{Z}$$

**E-Step:** Compute the expected complete-data log-likelihood given observed data and current parameter estimate $\theta^{(t)}$:

$$Q(\theta \mid \theta^{(t)}) = \mathbb{E}_{\mathbf{Z} \mid \mathbf{Y}, \theta^{(t)}}[\ell_c(\theta; \mathbf{X})]$$

**M-Step:** Maximize $Q$ to obtain new parameter estimates:

$$\theta^{(t+1)} = \arg\max_\theta Q(\theta \mid \theta^{(t)})$$

### Handling Missing Nodes in Networks

For network data with missing nodes or edges, the EM algorithm operates on the observed portion while integrating over the missing structure.

**Exponential Random Graph Models (ERGMs):**

For observed adjacency $\mathbf{Y}$ and missing entries $\mathbf{Z}$:

$$Q(\theta \mid \theta^{(t)}) = \mathbb{E}_{\mathbf{Z} \mid \mathbf{Y}, \theta^{(t)}}\left[\theta^T s(\mathbf{Y}, \mathbf{Z}) - \log \kappa(\theta)\right]$$

where $s(\cdot)$ are network statistics and $\kappa(\theta)$ is the normalizing constant.

**Challenges:**
- The expectation requires summing over all possible missing edge configurations (exponential in missing entries)
- Monte Carlo EM or variational approximations are needed

### Convergence Properties

**Monotonicity:** The observed-data likelihood never decreases:

$$\ell(\theta^{(t+1)}; \mathbf{Y}) \geq \ell(\theta^{(t)}; \mathbf{Y})$$

**Convergence:** Under regularity conditions, EM converges to a stationary point of the likelihood (local maximum or saddle point).

**Rate:** Linear convergence near the optimum. The convergence rate depends on the fraction of missing information.

**Standard errors:** EM does not directly provide standard errors. Methods include:
- Louis' formula (observed information matrix)
- Supplemented EM (SEM)
- Bootstrap

---

## Full Information Maximum Likelihood

Full Information Maximum Likelihood (FIML) maximizes the observed-data likelihood directly without explicit imputation.

### FIML Approach

For each case $i$, the contribution to the likelihood is:

$$L_i(\theta) = \int P(\mathbf{x}_i \mid \theta) d\mathbf{x}_{i,miss} = P(\mathbf{x}_{i,obs} \mid \theta)$$

The total log-likelihood is:

$$\ell(\theta) = \sum_{i=1}^n \log P(\mathbf{x}_{i,obs} \mid \theta)$$

**Optimization:** Directly maximize $\ell(\theta)$ using gradient-based methods (Newton-Raphson, Fisher scoring, L-BFGS).

### Direct Likelihood Approach

For multivariate normal data with missing values, the log-likelihood for case $i$ with observed variables indexed by $o_i$ is:

$$\ell_i = -\frac{|o_i|}{2}\log(2\pi) - \frac{1}{2}\log|\Sigma_{o_i}| - \frac{1}{2}(\mathbf{x}_{i,o_i} - \mu_{o_i})^T \Sigma_{o_i}^{-1}(\mathbf{x}_{i,o_i} - \mu_{o_i})$$

where $\mu_{o_i}$ and $\Sigma_{o_i}$ are the subvector and submatrix for observed variables.

**Advantages:**
- Uses all available data
- No imputation step
- Direct standard errors from observed information matrix

### Comparison to EM

| Aspect | EM | FIML |
|--------|-----|------|
| Output | Parameter estimates | Parameter estimates + SEs |
| Computational cost | Per-iteration cost may be lower | Each evaluation requires gradients |
| Convergence | Guaranteed ascent | Depends on optimizer |
| Missing patterns | Handles arbitrary patterns | Handles arbitrary patterns |
| Implementation | Often simpler | Requires derivatives |

For exponential family models with sufficient statistics, EM is natural. For complex models, FIML with automatic differentiation may be preferred.

---

## Handling MNAR in Networks

Missing Not at Random is the norm in network data, requiring explicit models of the missingness mechanism.

### Heckman's Selection Model

**Two-stage model:**
1. **Selection equation:** Model the probability of observation
2. **Outcome equation:** Model the outcome of interest

For network data, the selection model might predict response probability based on observed network position:

$$\text{Probit}(P(M_i = 0)) = \gamma_0 + \gamma_1 d_i^{obs} + \gamma_2 C_i$$

where $d_i^{obs}$ is observed degree and $C_i$ is centrality.

**Identification:** Requires exclusion restrictions (variables affecting selection but not outcome) or functional form assumptions.

### Pattern-Mixture Models

**Pattern-mixture models** stratify by missingness pattern and model outcomes within each pattern:

$$P(Y \mid X) = \sum_m P(Y \mid X, M = m) P(M = m \mid X)$$

**Advantages:**
- Explicitly models differences between observed and missing groups
- Sensitivity analysis through pattern-specific parameters

**Challenges:**
- Underidentified for patterns with no observed outcomes
- Requires identifying restrictions

### Selection Models

**Selection models** jointly model the outcome and missingness:

$$P(Y, M \mid X) = P(Y \mid X) P(M \mid Y, X)$$

The missingness model $P(M \mid Y, X)$ explicitly depends on (potentially unobserved) $Y$.

**Network application:** Model the probability of edge observation as a function of edge weight, node attributes, and dyadic characteristics:

$$\text{logit}(P(M_{ij} = 0)) = \alpha + \beta A_{ij} + \gamma |X_i - X_j|$$

where $A_{ij}$ is the (potentially unobserved) true edge existence.

### Sensitivity Analysis

Since MNAR models depend on unobservable quantities, **sensitivity analysis** is essential:

1. **Selection bias parameters:** Introduce sensitivity parameters representing the strength of dependence between missingness and outcomes
2. **Bounds analysis:** Derive bounds on estimates under extreme assumptions
3. **Multiple models:** Fit several MNAR models with different assumptions and compare results

**Example:** Let $\delta$ represent the difference in mean outcome between responders and non-responders. Estimate quantities for $\delta \in [-c, c]$ and report how conclusions vary.

### Explicit Modeling of Missingness Mechanism

**Joint modeling approach:**

$$P(X, M \mid \theta, \psi) = P(X \mid \theta) P(M \mid X, \psi)$$

Inference targets $P(X \mid \theta)$ while accounting for $P(M \mid X, \psi)$.

**Bayesian implementation:**

$$P(\theta \mid X_{obs}) \propto \int P(X_{obs}, X_{miss} \mid \theta) P(M \mid X_{obs}, X_{miss}, \psi) P(\theta) dX_{miss}$$

The missingness model $P(M \mid X, \psi)$ must be specified based on substantive knowledge.

---

## Missing Node Problem

Missing nodes are particularly challenging because they affect network structure in ways that cannot be observed.

### Network Reconstruction from Partial Observations

Given observed subgraph $G_{obs} = (V_{obs}, E_{obs})$, infer the full network $G_{full} = (V_{full}, E_{full})$ where $V_{obs} \subseteq V_{full}$.

**Approaches:**
1. **Actor-oriented models:** Model node presence/absence as part of the generative process
2. **Latent space models:** Infer positions of missing nodes in latent space
3. **Graphon estimation:** Estimate the underlying graphon from incomplete observations

**Challenges:**
- Unobserved nodes may connect multiple observed components
- Degree distribution is censored from below
- Global properties (diameter, clustering) are biased

### Bayesian Inference Over Random Graph Models

For random graph models (Erdős-Rényi, SBM), treat missing nodes as latent variables:

$$P(G_{obs} \mid \theta) = \sum_{V_{miss}} P(G_{obs} \mid V_{miss}, \theta) P(V_{miss} \mid \theta)$$

**MCMC approach:** Sample both parameters and missing node sets:
1. Propose addition/deletion of missing nodes
2. Propose connections between observed and proposed nodes
3. Accept/reject based on posterior

**Marginalization:** If the number of missing nodes is unknown, use reversible jump MCMC or Dirichlet process priors.

---

## Missing Edge Problem

Missing edges are more tractable than missing nodes because the node set is known. The problem reduces to inferring unobserved binary variables (edge existence).

### Tie Prediction

**Tie prediction** (or link prediction) estimates the probability of edge existence for unobserved dyads.

**Feature-based approaches:**
- **Topological features:** Common neighbors, Jaccard similarity, Adamic-Adar, Katz index
- **Latent features:** Node embeddings, matrix factorization
- **Attribute features:** Similarity of node attributes

**Model:**

$$P(A_{ij} = 1 \mid G_{obs}) = f(\text{features}(i, j))$$

**Evaluation:** Cross-validate by hiding observed edges and measuring prediction accuracy (AUC-ROC, AUC-PR).

### Link Prediction Algorithms

**Supervised link prediction:**

Train a classifier to distinguish existing edges from non-edges:

$$\hat{y}_{ij} = \text{Classifier}(s_{ij}^{(1)}, \ldots, s_{ij}^{(k)})$$

where $s_{ij}^{(k)}$ are similarity scores.

**Common similarity indices:**

| Index | Formula |
|-------|---------|
| Common Neighbors | $|\Gamma(i) \cap \Gamma(j)|$ |
| Jaccard | $\frac{|\Gamma(i) \cap \Gamma(j)|}{|\Gamma(i) \cup \Gamma(j)|}$ |
| Adamic-Adar | $\sum_{k \in \Gamma(i) \cap \Gamma(j)} \frac{1}{\log |\Gamma(k)|}$ |
| Katz | $\sum_{\ell=1}^{\infty} \beta^\ell |\text{paths}_{ij}^{(\ell)}|$ |
| Preferential Attachment | $|\Gamma(i)| \cdot |\Gamma(j)|$ |

### Stochastic Block Model Approaches

For networks with community structure, the Stochastic Block Model provides a principled framework for missing edge inference.

**Marginal edge probability:**

$$P(A_{ij} = 1 \mid A_{obs}) = \sum_{z_i, z_j} P(A_{ij} = 1 \mid z_i, z_j) P(z_i, z_j \mid A_{obs})$$

**Inference:**
1. Estimate block memberships $z$ from observed edges
2. Estimate block probabilities $P_{ab}$
3. Predict missing edges using posterior edge probabilities

**Advantage:** Accounts for community structure and provides uncertainty quantification.

---

## Informative Missingness

In network contexts, missingness itself often carries information about network properties.

### When Missingness is Informative

**Status-driven missingness:** High-status individuals may be less likely to respond to surveys about relationships. The fact that node $i$ did not respond suggests $i$ has high status.

**Relationship-driven missingness:** Individuals may be less likely to report sensitive relationships. Non-reporting of an edge suggests the relationship is of a particular type.

**Network position effects:** Isolated nodes may not participate in network surveys; highly connected nodes may be overwhelmed by survey requests.

### Modeling Informative Missingness

**Hierarchical models:**

$$P(M_i = 1 \mid \eta_i) = g(\eta_i)$$
$$\eta_i = f(\text{network position}_i, \text{attributes}_i)$$

where $\eta_i$ is a latent propensity for missingness.

**Joint models:** Simultaneously model the network formation process and the observation process:

$$P(G_{true}, G_{obs}, M) = P(G_{true}) P(M \mid G_{true}) P(G_{obs} \mid G_{true}, M)$$

**Censoring models:** When missingness takes the form of censoring (e.g., we observe $Y$ if $Y < c$), use survival analysis methods (Tobit models, Kaplan-Meier estimators).

---

## Sensitivity Analysis for Missing Data

Sensitivity analysis assesses how conclusions change under different missingness assumptions.

### Bounding Approach

**Worst-case bounds:** Without assumptions, the range of possible values for a statistic is often wide.

**Example:** For a mean with $n_{obs}$ observed values and $n_{miss}$ missing values:

$$\frac{n_{obs}\bar{x}_{obs} + n_{miss}x_{min}}{n} \leq \bar{x} \leq \frac{n_{obs}\bar{x}_{obs} + n_{miss}x_{max}}{n}$$

**Manski bounds:** For regression coefficients, derive bounds based on extreme assumptions about missing outcomes.

### Worst-Case Scenarios

**Extreme imputation:**
- Set all missing values to minimum: most pessimistic estimate
- Set all missing values to maximum: most optimistic estimate

**Impact assessment:** If conclusions are unchanged across extreme scenarios, missingness is not driving results. If conclusions change, more sophisticated analysis is needed.

### Distributional Assumptions

**Parametric sensitivity:** Assume missing values follow a distribution shifted from the observed:

$$X_{miss} \sim P(X - \delta)$$

where $\delta$ is a sensitivity parameter. Vary $\delta$ to assess robustness.

**Selection bias parameters:** In selection models, vary the coefficient linking missingness to outcomes to assess sensitivity.

---

## Censoring and Truncation

Network data often involves censoring (partial observation) or truncation (selective observation).

### Survival Analysis Approaches

**Censoring types:**
- **Right censoring:** Event has not occurred by observation end (e.g., tie has not formed)
- **Left censoring:** Event occurred before observation began
- **Interval censoring:** Event known to have occurred within an interval

**Kaplan-Meier estimator:** For time-to-event data with right censoring:

$$\hat{S}(t) = \prod_{t_i \leq t} \left(1 - \frac{d_i}{n_i}\right)$$

where $d_i$ is the number of events at time $t_i$ and $n_i$ is the number at risk.

**Cox proportional hazards:**

$$h(t \mid \mathbf{X}) = h_0(t) \exp(\boldsymbol{\beta}^T \mathbf{X})$$

Models the hazard (instantaneous event rate) as a function of covariates.

### Left/Right Censoring

**Right censoring in network formation:**
If we observe a network up to time $T$, edges that would form after $T$ are right-censored. Standard survival methods apply if we model edge formation as a time-to-event process.

**Left censoring:**
If we start observing an existing network, we may not know when existing ties formed. This is left censoring, requiring specialized methods or assumptions about the initial state.

### Interval Censoring

When event times are known only within intervals (e.g., surveys at times $t_1, t_2, \ldots$), we have interval-censored data.

**Turnbull estimator:** Nonparametric maximum likelihood for interval-censored data.

**Network application:** If we observe network snapshots at discrete times, edge formation times are interval-censored between observations.

---

## Best Practices in Network Research

Proper handling of missing data requires planning, transparency, and sensitivity analysis.

### Pre-Analysis Plans

**Design phase:**
- Anticipate missing data mechanisms
- Design data collection to minimize informative missingness
- Plan for follow-up of non-respondents

**Analysis plan:**
- Prespecify primary missing data method
- Define sensitivity analyses
- Set criteria for distinguishing MAR from MNAR scenarios

### Sensitivity Checks

**Standard practice:**
1. Report missing data rates by variable and by node
2. Compare observed vs. missing cases on observed variables
3. Conduct sensitivity analysis under MNAR assumptions
4. Compare results across multiple imputation methods

**Reporting:**
- State assumptions about missingness mechanism
- Describe imputation model and variables used
- Report fraction of missing information
- Discuss robustness of conclusions

### Reporting Standards

**Required elements:**
1. **Missing data rates:** Percent missing for each variable
2. **Missing data patterns:** Visualization of missingness structure
3. **Assumptions:** Explicit statement of MCAR/MAR/MNAR assumptions
4. **Methods:** Detailed description of handling method
5. **Sensitivity:** Results of sensitivity analyses
6. **Software:** Software and packages used

**Visualization:**
- Missingness pattern matrices
- Missingness by node attributes
- Imputation diagnostics (convergence of MI, distribution comparisons)

---

## How Lutufi Handles Missing Data

Lutufi provides comprehensive missing data handling tailored to network contexts.

### Latent Variable Representation

Lutufi represents missing data as latent variables in the probabilistic model:

$$P(X_{obs}, X_{miss}, Z \mid \theta) = P(X_{obs} \mid Z, \theta) P(X_{miss} \mid Z, \theta) P(Z \mid \theta)$$

where $Z$ represents latent structure (community memberships, latent positions) that explains both observed and missing data.

**Implementation:**
- Missing node attributes are additional latent variables
- Missing edges are binary latent variables
- Missing nodes require structural latent variables

### EM Implementation

Lutufi's EM module handles:
- **Gaussian networks:** Analytical E-step via conditional distributions
- **Discrete networks:** E-step via junction tree inference
- **Mixed networks:** Hybrid analytical/numerical integration
- **ERGM fitting:** Monte Carlo EM for exponential family network models

**Convergence diagnostics:**
- Likelihood monitoring
- Parameter change thresholds
- Maximum iteration limits

### Bayesian Missing Data Models

Lutufi's Bayesian inference module samples from the joint posterior:

$$P(\theta, X_{miss} \mid X_{obs}) \propto P(X_{obs}, X_{miss} \mid \theta) P(\theta)$$

**Gibbs sampling:**
1. Sample parameters: $\theta \sim P(\theta \mid X_{obs}, X_{miss})$
2. Sample missing values: $X_{miss} \sim P(X_{miss} \mid X_{obs}, \theta)$

**Advantages:**
- Natural uncertainty quantification
- Handles MNAR through explicit missingness models
- Integrates with Lutufi's Bayesian network inference

### Diagnostics for Missingness Mechanism

Lutufi provides diagnostic tools:
- **Little's MCAR test:** Chi-square test for MCAR
- **Missingness correlation analysis:** Correlation between missingness indicators and observed values
- **Sensitivity analysis framework:** Automated sensitivity analyses for key parameters
- **Imputation diagnostics:** Convergence checks for MCMC imputation

**Output:**
- Missing data summary statistics
- Mechanism assessment recommendations
- Warning flags for potential MNAR patterns

---

## Key References

1. Rubin, D. B. (1976). Inference and missing data. *Biometrika*, 63(3), 581-592. The foundational paper establishing the taxonomy of missing data mechanisms.

2. Little, R. J. A., & Rubin, D. B. (2002). *Statistical Analysis with Missing Data* (2nd ed.). Wiley. The comprehensive reference for missing data methodology.

3. Schafer, J. L. (1997). *Analysis of Incomplete Multivariate Data*. Chapman & Hall/CRC. Detailed treatment of multiple imputation and the EM algorithm.

4. Dempster, A. P., Laird, N. M., & Rubin, D. B. (1977). Maximum likelihood from incomplete data via the EM algorithm. *Journal of the Royal Statistical Society: Series B*, 39(1), 1-38. The original EM algorithm paper.

5. Schafer, J. L., & Graham, J. W. (2002). Missing data: Our view of the state of the art. *Psychological Methods*, 7(2), 147-177. Accessible overview of missing data methods.

6. White, I. R., Carlin, J. B., et al. (2008). Multiple imputation for missing data: What is it and what does it do? *International Journal of Epidemiology*, 37(6), 1669-1670. Practical introduction to multiple imputation.

7. Daniels, M. J., & Hogan, J. W. (2008). *Missing Data in Longitudinal Studies: Strategies for Bayesian Modeling and Sensitivity Analysis*. Chapman & Hall/CRC. Advanced treatment of longitudinal and sensitivity analysis methods.

8. Handcock, M. S., & Gile, K. J. (2010). Modeling social networks from sampled data. *Annals of Applied Statistics*, 4(1), 5-25. Missing data in network sampling.

9. Smith, J. A., & Moody, J. (2013). Structural effects of network sampling coverage I: Nodes missing at random. *Social Networks*, 35(4), 652-668. Impact of missing nodes on network measures.

10. Kossinets, G. (2006). Effects of missing data in social networks. *Social Networks*, 28(3), 247-268. Comprehensive simulation study of missing data effects in networks.

11. Heckman, J. J. (1979). Sample selection bias as a specification error. *Econometrica*, 47(1), 153-161. Selection models for MNAR data.

12. Robins, J. M., Rotnitzky, A., & Zhao, L. P. (1995). Analysis of semiparametric regression models for repeated outcomes in the presence of missing data. *Journal of the American Statistical Association*, 90(429), 106-121. Inverse probability weighting for missing data.

13. Newman, M. E. J. (2018). *Networks* (2nd ed.). Oxford University Press. Chapter on network sampling and missing data in networks.

14. Kolaczyk, E. D. (2009). *Statistical Analysis of Network Data: Methods and Models*. Springer. Section on network sampling designs and missing data.

15. Gómez-Rubio, V. (2020). Bayesian inference with INLA. *Chapman & Hall/CRC*. Chapter on missing data in hierarchical models.

---

*This document is part of the Lutufi Mathematical Foundations series. For related topics, see `BAYESIAN_NETWORKS.md`, `INFORMATION_THEORY.md`, and `STOCHASTIC_PROCESSES.md`.*
