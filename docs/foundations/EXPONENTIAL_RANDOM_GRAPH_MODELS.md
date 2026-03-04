# Exponential Random Graph Models (ERGMs)

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Introduction](#introduction)
2. [Why ERGMs](#why-ergms)
3. [Historical Development](#historical-development)
4. [The ERGM Framework](#the-ergm-framework)
5. [Markov Random Graphs](#markov-random-graphs)
6. [Social Circuit Dependence](#social-circuit-dependence)
7. [Common Network Statistics](#common-network-statistics)
8. [The Degeneracy Problem](#the-degeneracy-problem)
9. [Estimation Methods](#estimation-methods)
10. [Goodness of Fit](#goodness-of-fit)
11. [Model Specification](#model-specification)
12. [Temporal ERGMs (TERGMs)](#temporal-ergms-tergms)
13. [Bipartite ERGMs](#bipartite-ergms)
14. [Extensions](#extensions)
15. [Applications to Social and Economic Networks](#applications-to-social-and-economic-networks)
16. [How Lutufi Integrates ERGMs](#how-lutufi-integrates-ergms)
17. [Key References](#key-references)

---

## Introduction

Exponential Random Graph Models (ERGMs) represent a powerful statistical framework for analyzing and modeling network structure. Unlike descriptive network measures that characterize observed networks, ERGMs enable inference about the processes that generate network structure. They allow researchers to test hypotheses about network formation, predict tie formation, and simulate networks with specific structural properties.

The ERGM framework has become the dominant statistical approach in social network analysis, providing rigorous methods for understanding how local social processes—reciprocity, transitivity, popularity, homophily—combine to produce global network structure. This document presents the theoretical foundations, estimation methods, and applications of ERGMs, with particular attention to their integration with Bayesian approaches in the Lutufi library.

---

## Why ERGMs

### Statistical Models for Network Structure

Traditional network analysis provides descriptive statistics: density, clustering coefficients, degree distributions. These describe what a network looks like but not why it has that structure.

ERGMs answer the "why" by modeling the probability of network configurations based on local structural features. They enable:

**Hypothesis Testing:** Test whether specific structural features (e.g., triadic closure) significantly influence tie formation beyond what would be expected by chance.

**Parameter Estimation:** Quantify the strength of network formation mechanisms. A positive coefficient on triangles indicates that closed triples are more likely than open ones.

**Network Simulation:** Generate synthetic networks with specified structural properties for testing theories or designing interventions.

**Prediction:** Predict missing ties or future network evolution based on structural principles.

### Predicting Tie Formation

ERGMs predict tie formation using the principle of **structural equivalence**: nodes with similar network positions should have similar propensities to form ties.

The probability of a tie between nodes $i$ and $j$ depends on:
- Their attributes (homophily)
- Their existing ties (network effects)
- Their shared partners (triadic closure)
- Their degrees (popularity)

### Use in Social Science

ERGMs are particularly valuable in social science because they:

1. **Model endogeneity:** Network ties influence each other—friendship formation depends on existing friendships
2. **Handle dependence:** Traditional statistical models assume independence; ERGMs explicitly model dependence
3. **Separate effects:** Distinguish between homophily (birds of a feather flock together) and influence (friends become similar)
4. **Test theories:** Formalize social theories as statistical models and test against data

---

## Historical Development

### Holland & Leinhardt (1981) — p1 Models

The foundation of ERGMs emerged from Holland and Leinhardt's work on **dyadic independence models** for directed networks.

**The p1 Model:**
$$\log P(Y_{ij} = y_{ij}) = \lambda_{ij} + \rho y_{ij} + \alpha_i y_{ij} + \beta_j y_{ij}$$

where:
- $\rho$ is the density parameter (overall propensity for ties)
- $\alpha_i$ is the expansiveness of node $i$ (tendency to send ties)
- $\beta_j$ is the popularity of node $j$ (tendency to receive ties)

**Limitation:** Assumes dyads are independent, which contradicts the fundamental insight that network ties are interdependent.

### Frank & Strauss (1986) — Markov Graphs

Frank and Strauss introduced **Markov dependence:** two possible edges are dependent if they share a node.

**Markov Property:** The probability of a tie depends only on ties that share a node with it:
$$P(Y_{ij} = 1 | Y_{-ij}) = P(Y_{ij} = 1 | Y_{ik}, Y_{jk} : k \neq i, j)$$

**Homogeneous Markov Graphs:** Used sufficient statistics like:
- Number of edges
- Number of 2-stars
- Number of triangles

This allowed modeling transitivity: the probability of $Y_{ij} = 1$ increases with the number of shared partners.

### Wasserman & Pattison (1996) — General Formulation

Wasserman and Pattison generalized the framework to the modern ERGM form:

$$P(Y = y) = \frac{\exp\{\theta^T g(y)\}}{Z(\theta)}$$

This formulation:
- Allows arbitrary statistics $g(y)$
- Handles nodal attributes through covariates
- Provides a general exponential family framework

### Modern Developments (2000s-Present)

**Curved ERGMs (Hunter & Handcock, 2006):** Introduced geometrically weighted terms to solve degeneracy problems.

**Bergm (Bayesian ERGMs):** Enabled Bayesian inference for ERGMs, providing uncertainty quantification.

**TERGMs (Snijders et al., 2007):** Extended to temporal networks.

**BTERGMs (Krivitsky & Handcock, 2014):** Separated formation and dissolution for temporal networks.

**Computational Advances:** MCMC-MLE, stochastic approximation, and parallel computing made estimation feasible for larger networks.

---

## The ERGM Framework

### Exponential Family for Networks

ERGMs are exponential family distributions over the space of possible networks. For a random graph $Y$ on $n$ nodes, the probability of observing configuration $y$ is:

$$P(Y = y | \theta) = \frac{\exp\{\theta^T g(y)\}}{Z(\theta)}$$

where:
- $g(y) = (g_1(y), g_2(y), \ldots, g_p(y))^T$ is a vector of network statistics
- $\theta = (\theta_1, \theta_2, \ldots, \theta_p)^T$ are the natural parameters
- $Z(\theta) = \sum_{y' \in \mathcal{Y}} \exp\{\theta^T g(y')\}$ is the normalizing constant

**Network Space $\mathcal{Y}$:** The set of all possible networks on $n$ nodes. For undirected networks without self-loops:
$$|\mathcal{Y}| = 2^{n(n-1)/2}$$

This grows exponentially with network size, making exact computation infeasible for $n > 20$.

### Probability Distribution

The probability can be rewritten as:
$$P(Y = y | \theta) = \exp\left\{\theta^T g(y) - \psi(\theta)\right\}$$

where $\psi(\theta) = \log Z(\theta)$ is the log-partition function.

**Key Property:** The model assigns higher probability to networks with larger values of the sufficient statistics weighted by their parameters.

### Sufficient Statistics

**Definition:** Statistics $g(y)$ are sufficient if the conditional distribution $P(Y = y | g(Y) = g(y))$ does not depend on $\theta$.

**Interpretation:** $g(y)$ captures all the information about $\theta$ contained in the data.

**Common Sufficient Statistics:**
- $g_1(y) = \sum_{i<j} y_{ij}$ (number of edges)
- $g_2(y) = \sum_{i<j<k} y_{ij}y_{jk}y_{ki}$ (number of triangles)
- $g_3(y) = \sum_{i<j} y_{ij} x_{ij}$ (edges weighted by covariate)

### Natural Parameters

The natural parameters $\theta$ encode the strength and direction of network effects:

- **Positive $\theta_k$:** Networks with high $g_k(y)$ are more probable
- **Negative $\theta_k$:** Networks with low $g_k(y)$ are more probable
- **Zero $\theta_k$:** The statistic does not affect probability

**Example:** If $\theta_{edges} = -2$ and $\theta_{triangles} = 1$, the model prefers sparse networks with many triangles—exactly the structure observed in social networks.

---

## Markov Random Graphs

### Edge Independence Assumption

The simplest ERGMs assume edge independence (Bernoulli graphs):

$$P(Y = y) = \prod_{i<j} p^{y_{ij}} (1-p)^{1-y_{ij}}$$

This is equivalent to an ERGM with only the edge count statistic:
$$g_1(y) = \sum_{i<j} y_{ij}, \quad \theta_1 = \log\frac{p}{1-p}$$

**Erdős-Rényi Model:** This is a special case where all edges are independent and identically distributed.

### Dyadic Independence Models

**p1 Model (Holland & Leinhardt):**

For directed networks:
$$\log P(Y_{ij} = 1) = \theta + \alpha_i + \beta_j + \rho_{ij}$$

where $\rho_{ij}$ captures reciprocity.

**Sufficient Statistics:**
- Edge count
- Out-degree sequence
- In-degree sequence
- Number of mutual dyads

### Limitations and Degeneracy

**The Degeneracy Problem:** Markov graphs with triangle counts often place most probability mass on complete or empty graphs, even for moderate parameter values.

**Why This Happens:**
- Triangles create positive feedback: if $Y_{ij} = Y_{jk} = 1$, there's strong pressure for $Y_{ki} = 1$
- With moderate triangle parameters, this feedback leads to near-complete graphs
- With negative triangle parameters, it leads to near-empty graphs

**Mathematical Insight:** The exponential family has limited flexibility when using simple counts. The probability mass concentrates at extremes.

---

## Social Circuit Dependence

### Alternating k-Stars

Hunter (2007) introduced **alternating k-star statistics** to model degree distributions without degeneracy:

$$\text{AKS}_\lambda(y) = \sum_{k=2}^{n-1} (-1)^k \frac{S_k(y)}{\lambda^{k-2}}$$

where $S_k(y)$ is the number of k-stars.

**Interpretation:**
- Alternating signs prevent degeneracy
- Geometric weighting $\lambda^{-(k-2)}$ down-weights high-order stars
- Models skewed degree distributions (heavy tails)

**Parameter $\lambda$:** Controls the rate of alternation decay. Typically fixed at $\lambda = 2$.

### Alternating k-Triangles

**Alternating k-Triangle Statistic:**
$$\text{AKT}_\lambda(y) = \sum_{k=2}^{n-2} (-1)^k \frac{T_k(y)}{\lambda^{k-2}}$$

where $T_k(y)$ counts structures with $k$ triangles sharing a common edge.

**Interpretation:** Models transitivity and local clustering without the degeneracy of simple triangle counts.

### Geometrically Weighted Terms

**Geometrically Weighted Edgewise Shared Partners (GWESP):**

$$\text{GWESP}_\alpha(y) = e^{\alpha} \sum_{k=1}^{n-2} \left(1 - (1 - e^{-\alpha})^k\right) \text{ESP}_k(y)$$

where $\text{ESP}_k(y)$ counts edges with exactly $k$ shared partners.

**Interpretation:** The weight function $1 - (1 - e^{-\alpha})^k$ increases with $k$ but saturates, preventing runaway transitivity.

**Decay Parameter $\alpha$:** Controls how quickly the contribution saturates. Smaller $\alpha$ = faster saturation = less transitivity.

**Geometrically Weighted Degree (GWD):**

$$\text{GWD}_\alpha(y) = \sum_{k=0}^{n-1} e^{-\alpha k} D_k(y)$$

where $D_k(y)$ is the number of nodes with degree $k$.

---

## Common Network Statistics

### Edge Count

**Definition:** The total number of edges in the network.

$$g_{edges}(y) = \sum_{i<j} y_{ij}$$

**Interpretation:** Controls overall network density. Positive coefficient = denser networks preferred.

**Parameter:** $\theta_{edges}$ relates to density $d$ through:
$$d = \frac{1}{1 + e^{-\theta_{edges}}}$$ (for edge-only model)

### Mutual Ties (Directed Networks)

**Definition:** The number of reciprocal dyads.

$$g_{mutual}(y) = \sum_{i<j} y_{ij} y_{ji}$$

**Interpretation:** Models reciprocity—mutual ties are more (or less) likely than chance. Critical for social networks where relationships are typically reciprocal.

### Triangles

**Definition:** The number of closed triples.

$$g_{triangle}(y) = \sum_{i<j<k} y_{ij} y_{jk} y_{ki}$$

**Interpretation:** Models transitivity—friends of friends become friends. Core mechanism for clustering in social networks.

**Caution:** Simple triangle counts often cause degeneracy. Use GWESP instead.

### k-Stars

**Definition:** A k-star is a central node connected to $k$ other nodes.

$$S_k(y) = \sum_{i} \binom{d_i}{k}$$

where $d_i$ is the degree of node $i$.

**Interpretation:** k-stars relate to degree distribution moments. $S_2$ is related to variance of degree.

### Degree Distributions

**Degree Distribution Statistics:**

$$g_{degree(k)}(y) = \sum_{i} \mathbb{I}(d_i = k)$$

Models the exact degree distribution by counting nodes with each degree.

**Interpretation:** Allows arbitrary degree distributions, but uses many degrees of freedom.

### Geometrically Weighted Edgewise Shared Partners (GWESP)

**Definition:** See Social Circuit Dependence section.

**Interpretation:** Measures transitivity with saturation. An edge with many shared partners contributes marginally less than an edge with few shared partners.

**Parameter $\alpha$:** Typically fixed in estimation. Common values: $\alpha = 0.25$ (weak transitivity) to $\alpha = 0.69$ (strong transitivity).

### Homophily and Nodal Covariates

**Attribute-Based Edges:**

$$g_{homophily}(y, x) = \sum_{i<j} y_{ij} \mathbb{I}(x_i = x_j)$$

Counts edges between nodes with the same attribute value.

**Continuous Covariates:**

$$g_{covariate}(y, x) = \sum_{i<j} y_{ij} |x_i - x_j|$$

Models homophily on continuous attributes.

**Interaction Terms:**

$$g_{interaction}(y, x_1, x_2) = \sum_{i<j} y_{ij} x_{1i} x_{2j}$$

Models how multiple attributes jointly affect tie formation.

### Dyadic Covariates

**Edge Covariates:**

$$g_{dyad}(y, x) = \sum_{i<j} y_{ij} x_{ij}$$

where $x_{ij}$ is a dyad-level attribute (e.g., geographic distance, prior interaction).

**Interpretation:** Models exogenous drivers of tie formation. $\theta_{dyad} < 0$ for distance = geographic homophily.

---

## The Degeneracy Problem

### What is Degeneracy?

**Definition:** An ERGM is degenerate if the probability distribution places most mass on a small subset of network configurations—typically complete or empty graphs.

**Symptoms:**
- MCMC simulations produce only extreme graphs
- Parameter estimates diverge or fail to converge
- Model assigns high probability to unrealistic networks
- Standard errors explode

### Why Degeneracy Happens

**Mathematical Explanation:**

For an ERGM with triangle count statistic:
$$P(Y = y) \propto \exp\{\theta_1 g_{edges}(y) + \theta_2 g_{triangles}(y)\}$$

The number of triangles grows faster than the number of edges. For fixed $\theta_2 > 0$:
- Adding an edge that completes a triangle adds value
- This creates positive feedback loops
- The model prefers dense, highly clustered networks
- Small parameter changes lead to large probability shifts

**Phase Transition:** There exists a critical parameter value where the model abruptly shifts from favoring sparse to dense graphs.

### Implications for Estimation

**MCMC Challenges:**
- The Markov chain gets stuck in extreme states
- Poor mixing: the chain does not explore the network space effectively
- Convergence diagnostics fail

**Maximum Likelihood:**
- The likelihood surface is flat or multi-modal
- Standard optimization fails to find the global maximum
- Estimates are unstable and sensitive to starting values

### Solutions

**1. Curved ERGMs (Geometrically Weighted Terms):**

Use GWESP, GWD, and alternating statistics that saturate rather than growing linearly.

**Advantages:**
- Prevents runaway feedback
- Allows modeling transitivity without degeneracy
- Better statistical properties

**2. Constraints:**

Fix the number of edges (conditional ERGM):
$$P(Y = y | g_{edges}(y) = m) \propto \exp\{\theta^T g_{-edges}(y)\}$$

**3. Bayesian Approaches:**

Place priors on parameters that prevent extreme values:
$$p(\theta) = \mathcal{N}(0, \sigma^2)$$

**4. Regularization:**

Add penalty terms to the likelihood:
$$\ell_{reg}(\theta) = \ell(\theta) - \lambda ||\theta||^2$$

**5. Model Simplification:**

Remove problematic statistics. If triangles cause degeneracy, use 2-stars or GWESP instead.

---

## Estimation Methods

### Maximum Pseudolikelihood (MPLE)

**Idea:** Approximate the full likelihood by assuming conditional independence of edges given all others:

$$\ell_{PL}(\theta) = \sum_{i<j} \log P(Y_{ij} = y_{ij} | Y_{-ij})$$

**Conditional Logit Form:**

For undirected networks:
$$\text{logit } P(Y_{ij} = 1 | Y_{-ij}) = \theta^T \delta_{ij}(y)$$

where $\delta_{ij}(y) = g(y_{ij}^+) - g(y_{ij}^-)$ is the change statistic (difference in $g$ when toggling $y_{ij}$).

**Advantages:**
- Computationally fast: reduces to logistic regression
- No MCMC required
- Works well for large networks

**Disadvantages:**
- Biased for dependent data (violates independence assumption)
- Inconsistent estimates when dependence is strong
- Standard errors are too small

**When to Use:** Large networks where exact methods are infeasible; exploratory analysis.

### Markov Chain Monte Carlo Maximum Likelihood (MCMC-MLE)

**Algorithm:**

1. Generate MCMC samples from $P(Y | \theta^{(0)})$ for initial parameter guess $\theta^{(0)}$
2. Use importance sampling to estimate the likelihood ratio:
   $$\frac{Z(\theta)}{Z(\theta^{(0)})} = E_{\theta^{(0)}}\left[\exp\{ (\theta - \theta^{(0)})^T g(Y)\}\right]$$
3. Optimize the estimated likelihood
4. Iterate until convergence

**MCMC Sampling:**

The standard sampler uses Metropolis-Hastings:
1. Select a dyad $(i, j)$ uniformly at random
2. Propose toggling $y_{ij}$
3. Accept with probability $\min(1, \exp\{\theta^T \delta_{ij}(y)\})$

**Advantages:**
- Consistent and asymptotically efficient
- Valid standard errors
- Gold standard for ERGM estimation

**Disadvantages:**
- Computationally intensive
- Requires careful convergence diagnostics
- Can fail for degenerate models

**Convergence Diagnostics:**
- Geweke diagnostic
- Gelman-Rubin statistic (multiple chains)
- MCMC error estimates

### Stochastic Approximation

**Robbins-Monro Algorithm:**

Iteratively update parameters to solve the moment equation:
$$E_{\theta}[g(Y)] = g(y_{obs})$$

**Update Rule:**
$$\theta^{(t+1)} = \theta^{(t)} + a_t (g(y_{obs}) - g(Y^{(t)}))$$

where $Y^{(t)} \sim P(Y | \theta^{(t)})$ and $a_t$ is a decreasing step size.

**Advantages:**
- More stable than direct MCMC-MLE
- Handles large networks better
- Built into statnet software

**Disadvantages:**
- Slower convergence
- Requires tuning of step size sequence

### Bayesian Approaches

**Posterior Distribution:**

$$p(\theta | y) \propto P(y | \theta) p(\theta)$$

where $p(\theta)$ is the prior distribution.

**Common Priors:**
- Normal: $\theta \sim \mathcal{N}(0, \sigma^2 I)$
- Ridge: shrinks parameters toward zero
- Custom: encode theoretical expectations

**Posterior Sampling:**

Use MCMC to sample from the posterior:
1. Propose $\theta^* \sim q(\theta^* | \theta^{(t)})$
2. Accept with probability $\min(1, \alpha)$ where:
   $$\alpha = \frac{P(y | \theta^*) p(\theta^*) q(\theta^{(t)} | \theta^*)}{P(y | \theta^{(t)}) p(\theta^{(t)}) q(\theta^* | \theta^{(t)})}$$

**Advantages:**
- Natural uncertainty quantification
- Can incorporate prior knowledge
- More stable for degenerate models
- Provides full posterior distribution, not just point estimates

**Disadvantages:**
- Even more computationally intensive
- Requires specifying priors
- Posterior may be complex and multi-modal

---

## Goodness of Fit

### Comparing Model to Data

A well-fitting ERGM should reproduce the structural features of the observed network, not just those explicitly modeled.

**Procedure:**
1. Estimate model parameters $\hat{\theta}$
2. Simulate many networks from $P(Y | \hat{\theta})$
3. Compare simulated distributions to observed values

### T Statistics

For each network statistic $g_k$, compute:
$$T_k = \frac{g_k(y_{obs}) - E[g_k(Y)]}{\text{SD}[g_k(Y)]}$$

where expectation and standard deviation are from simulated networks.

**Interpretation:**
- $|T_k| < 2$: Model adequately captures statistic $k$
- $|T_k| > 2$: Model fails to capture statistic $k$

**Commonly Checked Statistics:**
- Degree distribution
- Geodesic distances
- Triad census
- Eigenvector centrality distribution

### Out-of-Sample Prediction

**Cross-Validation:**
1. Hold out some edges as test set
2. Estimate model on training edges
3. Predict test edges using estimated parameters
4. Evaluate with ROC, AUC, or log-likelihood

**Edge Prediction:**
The probability of edge $(i, j)$ is estimated by:
$$\hat{P}(Y_{ij} = 1 | Y_{-ij}) = \frac{1}{M} \sum_{m=1}^{M} \mathbb{I}(Y_{ij}^{(m)} = 1)$$

where $Y^{(m)}$ are MCMC samples.

### Posterior Predictive Checks

**Bayesian Approach:**

For posterior samples $\theta^{(1)}, \ldots, \theta^{(M)}$:
1. For each $\theta^{(m)}$, simulate network $Y^{(m)}$
2. Compute test statistic $T(Y^{(m)})$
3. Compare distribution of $T(Y^{(m)})$ to $T(y_{obs})$

**Posterior Predictive p-value:**
$$p_{PP} = \frac{1}{M} \sum_{m=1}^{M} \mathbb{I}(T(Y^{(m)}) \geq T(y_{obs}))$$

Extreme p-values (near 0 or 1) indicate model misspecification.

---

## Model Specification

### Choosing Statistics

**Theory-Driven:** Select statistics based on theoretical mechanisms:
- Social capital theory → triangles, closure
- Resource dependence → heterophily on resources
- Status competition → popularity effects

**Data-Driven:** Use exploratory analysis to identify important patterns:
- Plot degree distribution
- Examine clustering coefficient
- Look at attribute mixing patterns

** parsimony:** Prefer simpler models. Each statistic adds parameters and complexity.

### Model Selection (AIC, BIC)

**Akaike Information Criterion:**
$$AIC = -2\ell(\hat{\theta}) + 2p$$

**Bayesian Information Criterion:**
$$BIC = -2\ell(\hat{\theta}) + p \log n$$

where $p$ is the number of parameters and $n$ is the sample size (number of dyads).

**Model Selection:** Choose model with lowest AIC/BIC. BIC penalizes complexity more heavily.

### Stepwise Selection

**Forward Selection:**
1. Start with edge-only model
2. Add statistic that most improves fit
3. Continue until no improvement

**Backward Selection:**
1. Start with full model
2. Remove least significant statistic
3. Continue until all remaining are significant

**Caution:** Stepwise selection can overfit. Use cross-validation to validate.

### Theory-Driven Specification

**Example: Interorganizational Collaboration**

Theoretical mechanisms:
1. **Resource complementarity** → heterophily on organization type
2. **Prior ties** → edge covariate for past collaboration
3. **Transitivity** → GWESP (trusted partners introduce new partners)
4. **Status** → GWdegree (preferential attachment)

Model:
$$g(y) = (g_{edges}, g_{homophily}, g_{prior}, g_{GWESP}, g_{GWD})^T$$

---

## Temporal ERGMs (TERGMs)

### Dynamic Networks

TERGMs model network evolution as a discrete-time process:
$$Y^{(t)} \sim \text{ERGM}(\theta, Y^{(t-1)}, \ldots, Y^{(t-k)})$$

### Time Dependence

**First-Order Markov:**
$$P(Y^{(t)} | Y^{(1)}, \ldots, Y^{(t-1)}) = P(Y^{(t)} | Y^{(t-1)})$$

**Statistics Include:**
- Current network structure
- Structural changes from previous time step
- Stability: edges that persist
- Formation: new edges
- Dissolution: edges that end

### Stability and Change

**Edge Stability:**
$$g_{stability}(y^{(t)}, y^{(t-1)}) = \sum_{i<j} y_{ij}^{(t)} y_{ij}^{(t-1)}$$

Counts edges that persist from $t-1$ to $t$.

**Edge Formation:**
$$g_{formation}(y^{(t)}, y^{(t-1)}) = \sum_{i<j} y_{ij}^{(t)} (1 - y_{ij}^{(t-1)})$$

Counts new edges formed at time $t$.

**Interpretation:**
- Positive stability coefficient → ties persist over time
- Positive formation coefficient → new ties form easily
- Large stability relative to formation → stable networks

---

## Bipartite ERGMs

### Two-Mode Networks

Bipartite networks have two node types with edges only between types:
- Actors and events (affiliation networks)
- Firms and directors (board interlocks)
- Genes and diseases
- Users and items (recommendation systems)

### Actor-Event Networks

**Affiliation Networks:**

Nodes: $N_1$ (actors) and $N_2$ (events)
Edges: $y_{ij} = 1$ if actor $i$ participates in event $j$

**Projected Networks:**
- One-mode actor network: actors connected if they co-attend events
- One-mode event network: events connected if they share actors

**ERGMs for Affiliation Networks:**

Statistics include:
- Actor degree distribution (how many events each actor attends)
- Event degree distribution (how many actors each event has)
- 4-cycles (pairs of actors attending pairs of events together)
- Actor/event attributes

**Interpretation:**
- 4-cycle statistic models closure: if actors A and B both attend events X and Y, they are more likely to attend other events together

---

## Extensions

### Valued ERGMs

**Valued Networks:** Edges have weights, not just presence/absence.

**Model:**
$$P(Y = y) = \frac{\exp\{\theta^T g(y)\}}{Z(\theta)}$$

where $y_{ij} \in \{0, 1, \ldots, K\}$ or $y_{ij} \in \mathbb{R}^+$.

**Common Specifications:**
- Poisson reference: for count data
- Geometric reference: for positive integers
- Normal reference: for continuous weights

**Statistics:**
- Sum of edge values
- Weighted triangles
- Weighted degree distribution

### Multilevel ERGMs

**Hierarchical Structure:** Networks nested within groups (e.g., friendship networks within schools).

**Random Effects ERGM:**
$$P(Y^{(g)} = y^{(g)} | \theta, u_g) = \frac{\exp\{(\theta + u_g)^T g(y^{(g)})\}}{Z(\theta, u_g)}$$

where $u_g \sim \mathcal{N}(0, \Sigma)$ are group random effects.

**Interpretation:** Allows network parameters to vary across groups while borrowing strength through partial pooling.

### Hierarchical ERGMs

**Latent Structure:** Nodes have latent positions or memberships that affect tie probability.

**Latent Space Model:**
$$\text{logit } P(Y_{ij} = 1) = \theta^T g(y) - ||z_i - z_j||$$

where $z_i$ are latent positions.

**Combined with ERGM:**
$$P(Y = y | Z) = \frac{\exp\{\theta^T g(y) + \sum_{i<j} f(z_i, z_j) y_{ij}\}}{Z(\theta, Z)}$$

---

## Applications to Social and Economic Networks

### Interorganizational Networks

**Research Questions:**
- What drives alliance formation between firms?
- How does prior collaboration affect future partnerships?
- Do firms form transitive alliances (A partners with B, B with C → A with C)?

**Model Specification:**
- Edges: baseline propensity
- GWESP: transitivity in alliances
- Homophily: same industry, same geography
- Prior ties: edge covariate for past alliances
- Status: preferential attachment to central firms

**Findings:**
- Strong transitivity effects
- Homophily on industry and geography
- Prior ties strongly predict future ties
- Status effects moderate

### Trade Networks

**Research Questions:**
- How do existing trade relationships shape new ones?
- Does geographic distance remain important controlling for network structure?
- Are trade networks hierarchical or clustered?

**Model:**
- Edges: baseline trade propensity
- Mutual: reciprocity in trade
- GWESP: common trading partners
- Distance: dyadic covariate
- GDP: node covariates

**Insights:**
- Network effects explain much variation beyond gravity model variables
- Regional clustering through transitivity
- Distance effects persist but are mediated by network position

### Co-Authorship Networks

**Research Questions:**
- Do scientists cluster by discipline?
- Does productivity drive collaboration patterns?
- Are there status effects in scientific collaboration?

**Model:**
- Edges: baseline collaboration rate
- Homophily: same institution, same field
- GWESP: collaboration through common co-authors
- Degree effects: productive scientists collaborate more
- Prior ties: repeated collaboration

**Findings:**
- Strong field homophily
- High transitivity (scientists introduce collaborators)
- Productivity drives degree
- Persistent collaboration patterns

---

## How Lutufi Integrates ERGMs

### Structure Learning Priors

Lutufi uses ERGMs as **priors for Bayesian network structure learning**:

$$P(G) \propto \exp\{\theta^T g(G)\}$$

where $G$ is a Bayesian network structure.

**Advantages:**
1. **Incorporate domain knowledge:** If experts believe networks should be sparse and clustered, set $\theta_{edges} < 0$ and $\theta_{triangles} > 0$
2. **Regularize structure learning:** ERGM prior penalizes unlikely structures
3. **Guide search:** MCMC over structures samples preferentially from high-probability regions

**Integration:**
$$P(G | D) \propto P(D | G) P(G)$$

where $P(D | G)$ is the marginal likelihood and $P(G)$ is the ERGM prior.

### Hybrid Models with Bayesian Networks

**Combined Framework:**

Lutufi supports hybrid models where:
- Network structure follows ERGM prior
- Node variables follow Bayesian network conditional distributions
- The joint model is:

$$P(G, \Theta, X) = P(G) P(\Theta | G) P(X | G, \Theta)$$

**Example: Social Network with Attributes**

- $G$: Friendship network (ERGMs for structure)
- $\Theta$: Parameters of attribute dependencies
- $X$: Node attributes (e.g., smoking, drinking behaviors)

The joint model captures:
- How friendships form (network structure)
- How behaviors influence each other (Bayesian network)
- How friendships and behaviors co-evolve

### Statistical Testing Capabilities

Lutufi provides hypothesis testing for network structure:

**Test for Transitivity:**
$$H_0: \theta_{GWESP} = 0 \quad \text{vs} \quad H_1: \theta_{GWESP} > 0$$

**Likelihood Ratio Test:**
$$\Lambda = -2(\ell_{restricted} - \ell_{full}) \sim \chi^2_{df}$$

**Bayesian Testing:**
$$BF = \frac{P(D | H_1)}{P(D | H_0)}$$

**Model Comparison:**
- AIC/BIC for model selection
- Cross-validation for prediction assessment
- Posterior predictive checks for goodness of fit

### Lutufi ERGM Implementation

**Key Features:**

1. **Unified Interface:** ERGMs integrated with Bayesian networks, not separate
2. **Scalable Estimation:** Supports networks with thousands of nodes
3. **Flexible Statistics:** Users can define custom sufficient statistics
4. **Uncertainty Quantification:** Full Bayesian inference with posterior samples
5. **Model Diagnostics:** Automated goodness-of-fit checking

**Example Usage:**

```python
# Define ERGM prior for Bayesian network structure
from lutufi import ERGMPrior, BayesianNetwork

# Create ERGM prior with transitivity
prior = ERGMPrior(
    statistics=[
        EdgeCount(),
        GWESP(decay=0.5),
        GWD(decay=0.5)
    ],
    parameters=[-3.0, 1.0, -0.5]
)

# Use as prior for structure learning
model = BayesianNetwork(prior=prior)
model.fit(data)
```

---

## Key References

1. **Holland, P. W., & Leinhardt, S.** (1981). An exponential family of probability distributions for directed graphs. *Journal of the American Statistical Association*, 76(373), 33-50.

2. **Frank, O., & Strauss, D.** (1986). Markov graphs. *Journal of the American Statistical Association*, 81(395), 832-842.

3. **Wasserman, S., & Pattison, P.** (1996). Logit models and logistic regressions for social networks: I. An introduction to Markov graphs and p*. *Psychometrika*, 61(3), 401-425.

4. **Robins, G., Pattison, P., Kalish, Y., & Lusher, D.** (2007). An introduction to exponential random graph (p*) models for social networks. *Social Networks*, 29(2), 173-191.

5. **Snijders, T. A., Pattison, P. E., Robins, G. L., & Handcock, M. S.** (2006). New specifications for exponential random graph models. *Sociological Methodology*, 36(1), 99-153.

6. **Hunter, D. R.** (2007). Curved exponential family models for social networks. *Social Networks*, 29(2), 216-230.

7. **Hunter, D. R., & Handcock, M. S.** (2006). Inference in curved exponential family models for networks. *Journal of Computational and Graphical Statistics*, 15(3), 565-583.

8. **Goodreau, S. M.** (2007). Advances in exponential random graph (p*) models applied to a large social network. *Social Networks*, 29(2), 231-248.

9. **Handcock, M. S., Hunter, D. R., Butts, C. T., Goodreau, S. M., & Morris, M.** (2008). statnet: Software tools for the representation, visualization, analysis and simulation of network data. *Journal of Statistical Software*, 24(1), 1-11.

10. **Krivitsky, P. N.** (2012). Exponential-family random graph models for valued networks. *Electronic Journal of Statistics*, 6, 1100-1128.

11. **Krivitsky, P. N., & Handcock, M. S.** (2014). A separable model for dynamic networks. *Journal of the Royal Statistical Society: Series B*, 76(1), 29-46.

12. **Desmarais, B. A., & Cranmer, S. J.** (2012). Statistical mechanics of networks: Estimation and uncertainty. *Physica A*, 391(4), 1865-1876.

13. **Cranmer, S. J., & Desmarais, B. A.** (2011). Inferential network analysis with exponential random graph models. *Political Analysis*, 19(1), 66-86.

14. **Hunter, D. R., Goodreau, S. M., & Handcock, M. S.** (2008). Goodness of fit of social network models. *Journal of the American Statistical Association*, 103(481), 248-258.

15. **Schweinberger, M.** (2011). Instability, sensitivity, and degeneracy of discrete exponential families. *Journal of the American Statistical Association*, 106(496), 1361-1370.

---

*This document is part of the Lutufi documentation. For questions or contributions, please refer to the project's contribution guidelines.*
