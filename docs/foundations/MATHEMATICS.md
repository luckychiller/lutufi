# Mathematical Foundations of Lutufi

### A Unified Probabilistic Framework for Social and Economic Network Reasoning

**Document Version:** 1.0
**Status:** Working Draft — Research Phase
**Author:** Wasswa Lutufi Sebbanja
**Last Updated:** March 2026

---

> *"All models are wrong, but some are useful."*
> — George E. P. Box
>
> Lutufi is built on this principle. It does not claim to reveal absolute truth. It claims to reason carefully, quantify uncertainty honestly, and make the invisible structure of human systems legible.

---

## Table of Contents

1. [Introduction and Motivation](#1-introduction-and-motivation)
2. [Probabilistic Graphical Models — The Core Formalism](#2-probabilistic-graphical-models)
3. [Core Assumptions](#3-core-assumptions)
4. [D-Separation and Conditional Independence](#4-d-separation-and-conditional-independence)
5. [Markov Random Fields and Factor Graphs](#5-markov-random-fields-and-factor-graphs)
6. [The Unification Challenge](#6-the-unification-challenge)
7. [Inference Algorithms](#7-inference-algorithms)
8. [Learning from Data](#8-learning-from-data)
9. [Dynamic Bayesian Networks](#9-dynamic-bayesian-networks)
10. [Causal Modeling and Do-Calculus](#10-causal-modeling-and-do-calculus)
11. [Identifiability](#11-identifiability)
12. [Missing Data](#12-missing-data)
13. [Numerical Stability](#13-numerical-stability)
14. [Computational Complexity](#14-computational-complexity)
15. [Scope and Limitations](#15-scope-and-limitations)
16. [Key References](#16-key-references)

---

## 1. Introduction and Motivation

Social and economic systems are fundamentally networks of relationships. A financial market is a network of institutions connected by exposure and obligation. A community is a network of individuals connected by trust, communication, and influence. A supply chain is a network of organizations connected by dependency and transaction. These systems share a defining characteristic: **what happens at one node propagates — with uncertainty — to others.**

Existing tools handle parts of this problem. Graph libraries model the structure. Probabilistic programming languages model the uncertainty. Statistical tools model the data. But none of them unify structure, uncertainty, and temporal dynamics into a single coherent framework designed for the scale and messiness of real social and economic data.

Lutufi is built to fill this gap. Its mathematical foundations draw from three deeply interconnected fields:

- **Probabilistic Graphical Models (PGMs):** Formalizing how uncertainty is represented and propagated over structured systems.
- **Network Science:** Formalizing how relationships, influence, and dynamics are represented in connected systems.
- **Causal Inference:** Distinguishing between observing a pattern and understanding what produces it — the difference between correlation and mechanism.

This document establishes the mathematical foundations that make Lutufi's reasoning principled, its results interpretable, and its limitations honest.

---

## 2. Probabilistic Graphical Models

### 2.1 What is a Probabilistic Graphical Model?

A **Probabilistic Graphical Model (PGM)** is a mathematical framework that uses a graph to encode the conditional dependence structure of a joint probability distribution over a set of random variables. The graph makes explicit which variables are directly related and which are independent given certain observations.

Formally, let $\mathbf{X} = \{X_1, X_2, \dots, X_n\}$ be a set of random variables. Each variable $X_i$ takes values in a domain $\mathcal{X}_i$. The joint distribution $P(\mathbf{X})$ over all variables lives in a space that grows exponentially with $n$ — specifying it naively is computationally intractable. PGMs exploit conditional independence to factor this distribution into a product of smaller, manageable terms.

There are two primary families of PGMs:

- **Directed Graphical Models (Bayesian Networks):** Use a Directed Acyclic Graph (DAG) to encode asymmetric, causal-like dependencies.
- **Undirected Graphical Models (Markov Random Fields):** Use an undirected graph to encode symmetric, correlational dependencies.

Lutufi works with both families, and with **Factor Graphs** as a unifying representation.

### 2.2 Bayesian Networks

A **Bayesian Network** $\mathcal{B} = (\mathcal{G}, \Theta)$ consists of:

- A **DAG** $\mathcal{G} = (\mathbf{V}, \mathbf{E})$ where each node $V_i \in \mathbf{V}$ corresponds to a random variable $X_i$, and each directed edge $V_i \to V_j$ encodes a direct probabilistic influence of $X_i$ on $X_j$.
- A set of **parameters** $\Theta = \{\theta_{X_i | \text{Pa}(X_i)}\}$ specifying the **Conditional Probability Distribution (CPD)** of each variable given its parents.

The requirement that $\mathcal{G}$ is acyclic — containing no directed cycles — is a structural commitment: the dependencies it represents have a consistent causal ordering. This does not require that time flows in the graph; it requires that the influence relationships can be ordered without circular dependency.

### 2.3 The Chain Rule and Factorization Theorem

The central result of Bayesian network theory is the **factorization theorem**. Given a Bayesian network $\mathcal{B}$, the joint probability distribution over all variables factors as:

$$
P(X_1, X_2, \dots, X_n) = \prod_{i=1}^{n} P\!\left(X_i \mid \text{Pa}_{\mathcal{G}}(X_i)\right)
$$

where $\text{Pa}_{\mathcal{G}}(X_i)$ denotes the set of **parent nodes** of $X_i$ in the DAG $\mathcal{G}$.

**What this means in practice:** Instead of specifying a joint distribution over $n$ variables — which may require an exponentially large table — we specify $n$ smaller conditional distributions, one per variable. Each variable is only conditioned on its direct parents, not all variables. For a variable $X_i$ with $k$ parents, each binary, the CPD requires $2^k$ parameters rather than $2^n$. This is the computational leverage that makes inference tractable.

**A concrete example:** Consider a small social influence network with three nodes: Exposure $E$, Belief $B$, and Action $A$, with structure $E \to B \to A$. The full joint distribution is:

$$
P(E, B, A) = P(E) \cdot P(B \mid E) \cdot P(A \mid B)
$$

This captures the social intuition that exposure influences belief, and belief drives action — direct influence is local, but effects propagate through the chain.

### 2.4 Conditional Probability Tables and Distributions

For **discrete variables**, the CPD for $X_i$ is a **Conditional Probability Table (CPT)**: a table of values $P(X_i = x \mid \text{Pa}(X_i) = \mathbf{pa})$ for each combination of parent configurations $\mathbf{pa}$ and child states $x$.

For **continuous variables**, the CPD is typically a parametric distribution — most commonly a **linear Gaussian model**:

$$
P(X_i \mid \text{Pa}(X_i)) = \mathcal{N}\!\left(\mu_i + \sum_{j \in \text{Pa}(X_i)} w_{ij} X_j, \; \sigma_i^2\right)
$$

Lutufi supports both discrete and continuous CPDs, and hybrid networks that contain both.

---

## 3. Core Assumptions

The factorization theorem is not universally valid. It holds under two foundational assumptions that must be understood clearly, because violating them silently leads to incorrect inference.

### 3.1 The Local Markov Condition

**Statement:** Each variable $X_i$ is conditionally independent of its **non-descendants** in $\mathcal{G}$, given its parents $\text{Pa}(X_i)$.

Formally: $X_i \perp\!\!\!\perp \text{NonDesc}(X_i) \mid \text{Pa}(X_i)$

**What this means:** Once you know the direct causes of $X_i$ (its parents), knowing anything else about the rest of the graph that doesn't lie downstream of $X_i$ gives you no additional information about $X_i$. The parents *screen off* $X_i$ from its non-causal context.

**Why it can fail in social systems:** In human networks, variables that appear non-adjacent may be connected through unmeasured common causes (hidden confounders), through reciprocal influence that creates cycles, or through measurement processes that introduce spurious dependencies. Lutufi detects and flags potential Markov condition violations when the data implies dependencies the graph structure does not encode.

### 3.2 The Faithfulness Assumption

**Statement:** The conditional independence relationships present in the probability distribution $P$ are exactly those implied by d-separation in the graph $\mathcal{G}$. There are no "accidental" independencies — independencies that hold in $P$ but are not structurally implied by $\mathcal{G}$.

**Why faithfulness matters:** Without faithfulness, the graph could represent a distribution that is "accidentally" simpler than the graph suggests — parameters could cancel each other out in ways that create false independencies. Learning algorithms that assume faithfulness would then produce incorrect graphs.

**Faithfulness in social networks:** Faithfulness can fail when, for example, two pathways of influence precisely cancel each other — a positive influence through one route exactly offset by a negative influence through another. This is rare but not impossible in social and economic systems, particularly in competitive or adversarial settings. Lutufi's structure learning module includes diagnostics for potential faithfulness violations.

---

## 4. D-Separation and Conditional Independence

D-separation (Directional Separation) is the graphical criterion for determining whether two sets of variables are conditionally independent given a third set, by reading the structure of the DAG without any computation.

### 4.1 Formal Definition

Let $A$, $B$, and $C$ be disjoint sets of nodes in DAG $\mathcal{G}$. A **path** $\pi$ between a node in $A$ and a node in $B$ is **blocked** by $C$ if it contains either:

1. A chain $X \to Z \to Y$ or fork $X \leftarrow Z \to Y$ such that $Z \in C$ (the middle node is observed), or
2. A collider $X \to Z \leftarrow Y$ such that neither $Z$ nor any descendant of $Z$ is in $C$ (the collider and all its descendants are unobserved).

$A$ and $B$ are **d-separated** by $C$ in $\mathcal{G}$, written $A \perp_{\mathcal{G}} B \mid C$, if every path between $A$ and $B$ is blocked by $C$.

If $A$ and $B$ are d-separated by $C$, then under the Markov and faithfulness assumptions, $X_A \perp\!\!\!\perp X_B \mid X_C$ in $P$.

### 4.2 The Three Structural Patterns

Understanding d-separation requires understanding the three elemental path structures and their behavior under observation.

**Chains:** $X \to Z \to Y$

This represents a mediated influence: $X$ affects $Y$ only through intermediary $Z$. Observing $Z$ blocks the flow of information — once we know $Z$'s state, $X$ tells us nothing more about $Y$.

*Social example:* Exposure to misinformation $X$ affects voting behavior $Y$ through belief change $Z$. Controlling for (observing) the belief state blocks the indirect effect of exposure.

**Forks:** $X \leftarrow Z \to Y$

$Z$ is a common cause of both $X$ and $Y$. Without observing $Z$, $X$ and $Y$ appear correlated — not because one causes the other, but because both are caused by $Z$. Observing $Z$ removes this spurious correlation.

*Social example:* Socioeconomic status $Z$ causes both network position $X$ and health outcomes $Y$. The correlation between network position and health is confounded by status.

**Colliders:** $X \to Z \leftarrow Y$

$Z$ is a common effect of both $X$ and $Y$. This is the counterintuitive case: without observing $Z$, $X$ and $Y$ are independent. But *conditioning on* $Z$ — or any descendant of $Z$ — opens a path and creates a spurious dependency between $X$ and $Y$.

*Social example:* Being selected for a competitive program $Z$ requires either high academic performance $X$ or strong social connections $Y$. Among program participants (conditioning on $Z$), academic performance and social connections become negatively correlated — a purely statistical artifact of selection, not a real relationship. This is known as **Berkson's paradox** or **collider bias** and is one of the most common inferential errors in observational social research.

### 4.3 Why D-Separation Matters for Lutufi

Lutufi exposes d-separation as a first-class operation. Users can query: *"Given that we observe these variables, are these two groups of nodes independent?"* This is essential for:

- **Identifying valid adjustment sets** for causal estimation.
- **Detecting confounding** in network data.
- **Pruning inference** computations by identifying which parts of the network are irrelevant given current observations.
- **Designing studies** — knowing in advance which variables to measure to ensure identifiability.

---

## 5. Markov Random Fields and Factor Graphs

### 5.1 Markov Random Fields (MRFs)

An **Undirected Graphical Model**, or **Markov Random Field (MRF)**, represents a joint distribution using an undirected graph $\mathcal{H} = (\mathbf{V}, \mathbf{E})$. Instead of CPDs, the distribution is defined through **potential functions** (also called clique potentials) $\psi_C(\mathbf{X}_C)$ over **cliques** (fully connected subsets) of the graph:

$$
P(\mathbf{X}) = \frac{1}{Z} \prod_{C \in \mathcal{C}} \psi_C(\mathbf{X}_C)
$$

where $\mathcal{C}$ is the set of maximal cliques, and $Z = \sum_{\mathbf{x}} \prod_C \psi_C(\mathbf{x}_C)$ is the **partition function** — a normalizing constant that ensures the distribution sums to one.

**The partition function is the central computational challenge of MRFs.** Computing $Z$ exactly requires summing over all possible configurations of all variables — an exponential operation. Most inference methods for MRFs are concerned with computing or approximating quantities that depend on $Z$.

The **Markov property** for MRFs states that each variable is conditionally independent of all other variables given its immediate neighbors (the **Markov blanket**):

$$
X_i \perp\!\!\!\perp \mathbf{X}_{\mathbf{V} \setminus \{i\} \setminus \text{Nb}(i)} \mid X_{\text{Nb}(i)}
$$

This is well-suited to symmetric social relationships — friendship, co-authorship, co-membership — where there is no natural direction of influence.

### 5.2 Factor Graphs

A **Factor Graph** $\mathcal{F} = (\mathbf{V}, \mathbf{F}, \mathbf{E})$ is a bipartite graph with two types of nodes: **variable nodes** $\mathbf{V}$ and **factor nodes** $\mathbf{F}$. Edges connect variables to the factors that involve them.

The joint distribution is:

$$
P(\mathbf{X}) = \frac{1}{Z} \prod_{f \in \mathbf{F}} f(\mathbf{X}_f)
$$

where $\mathbf{X}_f$ denotes the variables connected to factor $f$.

**Why Factor Graphs are Lutufi's canonical representation:** Factor graphs generalize both Bayesian networks and MRFs. Every BN and every MRF can be converted to a factor graph. The **sum-product algorithm** (belief propagation) on factor graphs provides a single unified inference algorithm that applies to both directed and undirected models. This is the architectural choice that allows Lutufi to handle the full spectrum of network types without maintaining separate inference codepaths for each.

### 5.3 Converting Between Representations

**From BN to MRF (Moralization):**

To convert a DAG to an undirected graph (called the **moral graph**), Lutufi:

1. Adds undirected edges between all pairs of parents that share a common child (this is the "marrying" step — the origin of the term "moral graph").
2. Removes all edge directions.

**Critical information loss:** Moralization destroys the distinction between chains, forks, and colliders. A collider $X \to Z \leftarrow Y$ becomes an undirected triangle in the moral graph, which implies that $X$ and $Y$ are *not* independent given $Z$ — the opposite of the original BN semantics. This means moralized graphs encode *fewer* conditional independencies than the original DAG. Lutufi tracks which independencies are lost during moralization and reports them to the user.

---

## 6. The Unification Challenge

This is the central technical problem Lutufi solves.

Social networks violate the DAG assumption in fundamental ways:

**Cycles and reciprocal influence:** If person $A$ influences person $B$, and $B$ influences $A$, the DAG requirement is violated. But reciprocal influence is not an anomaly in social networks — it is the norm. Friendship, professional relationships, and market dynamics all involve feedback loops.

**Undirected relationships:** Many social ties — co-membership, similarity, proximity — are symmetric and directionless. Forcing direction on them misrepresents the underlying reality.

**Dynamic structure:** The network itself changes over time as relationships form, strengthen, weaken, and dissolve. A static DAG cannot represent this temporal evolution.

Lutufi's solution is a **multi-representation architecture**:

- Where the data has clear causal direction and acyclic structure, Lutufi uses BNs with exact or approximate inference.
- Where the data is symmetric or weakly cyclic, Lutufi converts to MRF/Factor Graph representation.
- Where the data is temporal, Lutufi uses Dynamic Bayesian Networks.
- Where cycles are unavoidable and approximate inference is acceptable, Lutufi uses Loopy Belief Propagation.
- Where the posterior is complex and sampling is required, Lutufi uses MCMC.

The user specifies the problem; Lutufi selects or assists in selecting the appropriate representation and inference strategy.

---

## 7. Inference Algorithms

Inference is the process of computing the posterior distribution $P(\mathbf{X}_Q \mid \mathbf{X}_E = \mathbf{e})$ — the probability distribution over **query variables** $\mathbf{X}_Q$ given observed values $\mathbf{e}$ for **evidence variables** $\mathbf{X}_E$.

### 7.1 Exact Inference: The Junction Tree Algorithm

**Applicable to:** DAGs and singly-connected graphs (trees and polytrees).

The **Junction Tree Algorithm** (also called the **Clique Tree Algorithm**) is the canonical algorithm for exact inference in Bayesian networks. It proceeds in three phases:

**Phase 1 — Triangulation:** The moral graph is *triangulated* by adding fill-in edges until no cycle of length 4 or more exists without a chord. This converts the graph into a *chordal* graph.

**Phase 2 — Clique Tree Construction:** The maximal cliques of the triangulated graph are organized into a tree structure (the **junction tree** or **clique tree**) such that the **running intersection property** holds: if a variable appears in two cliques, it appears in all cliques on the path between them in the tree.

**Phase 3 — Message Passing:** Probability messages are passed between adjacent cliques, first from leaves to root (**collect evidence**), then from root to leaves (**distribute evidence**). After two passes, all marginal and conditional distributions can be read from the clique beliefs.

**Complexity:** The computational cost is exponential in the **treewidth** $w$ of the graph — specifically $O(n \cdot k^{w+1})$ where $k$ is the maximum domain size. For sparse social networks with small treewidth, this is tractable. For dense networks, treewidth can be in the hundreds and exact inference becomes computationally infeasible.

**Guarantee:** When it terminates, the Junction Tree Algorithm gives **exact** marginal and joint distributions. There is no approximation error.

### 7.2 Belief Propagation on Trees

For **tree-structured** graphs (no cycles), the **sum-product algorithm** computes exact marginals through local message passing between variable and factor nodes. Each node passes a *message* to its neighbors summarizing its local information.

For a variable node $x_i$, the marginal belief is:

$$
b(x_i) \propto \prod_{f \in \text{Nb}(x_i)} \mu_{f \to x_i}(x_i)
$$

where $\mu_{f \to x_i}$ is the message from factor $f$ to variable $x_i$. Messages are computed in a single forward-backward pass, making BP on trees extremely efficient — $O(n)$ in the number of nodes.

### 7.3 Loopy Belief Propagation (LBP)

**Applicable to:** General graphs with cycles.

**Loopy Belief Propagation** applies the same sum-product message passing algorithm to graphs with cycles, despite the fact that the algorithm is only theoretically justified for trees. On graphs with cycles, messages cycle indefinitely; LBP runs until the messages converge (change by less than a tolerance $\epsilon$) or until a maximum iteration count is reached.

**Convergence:** LBP is **not guaranteed to converge** on cyclic graphs. In practice, it converges on most large, sparse social networks (which have many short cycles but weak long-range dependencies), but can oscillate indefinitely on dense or highly regular graphs.

**Correctness when convergent:** Even when LBP converges, the resulting beliefs are generally **approximations** of the true marginals. However, the quality of approximation is often remarkably good. LBP is known to give exact results on graphs with at most one cycle, and on **Gaussian** models (regardless of graph structure). For social networks, the approximation error is typically acceptable for practical decision-making.

**When Lutufi uses LBP:** For large-scale social networks (hundreds of thousands or millions of nodes) where exact inference is computationally infeasible, LBP is the default approximate inference method. Lutufi monitors convergence and reports the current message change magnitude so users can assess inference quality.

### 7.4 Variational Inference

**Applicable to:** Large-scale models where MCMC is too slow and LBP convergence is unreliable.

**Variational inference** converts the inference problem — computing an intractable posterior $P(\mathbf{X}_H \mid \mathbf{X}_E)$ — into an **optimization problem**. It posits a family of simpler, tractable distributions $\mathcal{Q}$ and finds the member $Q^* \in \mathcal{Q}$ that minimizes the **Kullback-Leibler divergence** from the true posterior:

$$
Q^*(\mathbf{X}_H) = \arg\min_{Q \in \mathcal{Q}} D_{\text{KL}}\!\left(Q(\mathbf{X}_H) \;\|\; P(\mathbf{X}_H \mid \mathbf{X}_E)\right)
$$

**Mean Field Variational Inference** restricts $\mathcal{Q}$ to fully factorized distributions:

$$
Q(\mathbf{X}_H) = \prod_{i} Q_i(X_i)
$$

This assumption — that latent variables are independent in the approximate posterior — is the "mean field" assumption. It is often violated in social networks where latent community memberships and hidden influences are strongly correlated. Lutufi reports the **Evidence Lower Bound (ELBO)** as a diagnostic for approximation quality; a tighter ELBO (closer to zero KL divergence) indicates a better approximation.

**Why variational inference matters for Lutufi:** For models with millions of nodes and latent community structure, neither exact inference nor MCMC scales. Variational inference — particularly **Stochastic Variational Inference (SVI)**, which processes minibatches of nodes — scales to datasets that no other method handles.

### 7.5 Markov Chain Monte Carlo (MCMC)

**Applicable to:** Complex posterior distributions that cannot be approximated analytically.

MCMC methods construct a **Markov chain** whose stationary distribution is the target posterior $P(\mathbf{X}_H \mid \mathbf{X}_E)$. By running the chain long enough and collecting samples, we can estimate any quantity of interest from the posterior.

**Gibbs Sampling:** At each step, samples one variable at a time from its **conditional distribution** given all other current values. Requires that all full conditional distributions $P(X_i \mid \mathbf{X}_{-i}, \mathbf{X}_E)$ are known in closed form — which is true for many conjugate Bayesian network models. Efficient for high-dimensional models but can mix slowly when variables are highly correlated.

**Metropolis-Hastings (MH):** A more general algorithm that proposes a new state from a proposal distribution, then accepts or rejects the proposal with a probability that ensures detailed balance. Does not require full conditionals in closed form — applicable to any model where the unnormalized posterior can be evaluated.

**Practical considerations in Lutufi:**

- MCMC requires a **burn-in period** during which the chain converges to the stationary distribution. Lutufi discards burn-in samples automatically.
- **Mixing** — how quickly the chain explores the posterior — is assessed through diagnostics including the **Gelman-Rubin statistic** (comparing multiple chains) and **effective sample size**.
- For large social networks, standard MCMC is computationally prohibitive. Lutufi supports **Hamiltonian Monte Carlo (HMC)** for continuous models, which exploits gradient information to make larger, more efficient proposals.

---

## 8. Learning from Data

In many real applications, neither the graph structure nor the parameters are known in advance. They must be learned from observed data.

### 8.1 Parameter Learning

**Given:** A fixed graph structure $\mathcal{G}$ and a dataset $\mathcal{D} = \{\mathbf{x}^{(1)}, \mathbf{x}^{(2)}, \dots, \mathbf{x}^{(N)}\}$ of $N$ complete observations.

**Goal:** Estimate the parameters $\Theta$ of the CPDs.

**Maximum Likelihood Estimation (MLE):**

Maximize the log-likelihood of the data:

$$
\hat{\Theta}_{\text{MLE}} = \arg\max_\Theta \sum_{m=1}^N \log P(\mathbf{x}^{(m)} \mid \mathcal{G}, \Theta)
$$

Due to the factorization theorem, this decomposes into independent optimization problems, one per variable. For discrete CPTs, the MLE estimate is simply the empirical frequency:

$$
\hat{P}(X_i = x \mid \text{Pa}(X_i) = \mathbf{pa}) = \frac{\text{count}(X_i = x, \text{Pa}(X_i) = \mathbf{pa})}{\text{count}(\text{Pa}(X_i) = \mathbf{pa})}
$$

**Limitation:** MLE fails when data is sparse — if a particular parent configuration $\mathbf{pa}$ never appears in the training data, the estimate is undefined or zero, which causes problems downstream in inference (zero probability propagates as zero everywhere).

**Bayesian Parameter Estimation:**

Places a **prior distribution** over parameters and computes the posterior given data. For discrete CPTs, the natural conjugate prior is the **Dirichlet distribution**:

$$
P(\theta_{X_i | \mathbf{pa}}) = \text{Dir}(\alpha_{x_1|\mathbf{pa}}, \alpha_{x_2|\mathbf{pa}}, \dots)
$$

The posterior (and the predictive distribution used in Lutufi) is also Dirichlet, with hyperparameters updated by observed counts:

$$
P(\theta_{X_i | \mathbf{pa}} \mid \mathcal{D}) = \text{Dir}(\alpha_{x_1|\mathbf{pa}} + N_{x_1|\mathbf{pa}}, \; \alpha_{x_2|\mathbf{pa}} + N_{x_2|\mathbf{pa}}, \dots)
$$

where $N_{x|\mathbf{pa}}$ is the count of $X_i = x, \text{Pa}(X_i) = \mathbf{pa}$ in $\mathcal{D}$.

The **BDeu (Bayesian Dirichlet equivalent uniform)** prior — a specific choice of Dirichlet hyperparameters — is Lutufi's default for structure learning scoring.

**Expectation-Maximization (EM) for Latent Variables:**

When some variables are **latent** (never directly observed) or data has missing entries, MLE and Bayesian estimation cannot be applied directly. The **EM algorithm** addresses this by iterating between two steps:

- **E-step (Expectation):** Given current parameters $\Theta^{(t)}$, compute the expected sufficient statistics of the latent variables using belief propagation:

$$
Q(\Theta | \Theta^{(t)}) = \mathbb{E}_{\mathbf{X}_H | \mathbf{X}_E, \Theta^{(t)}}\!\left[\log P(\mathbf{X}, \Theta)\right]
$$

- **M-step (Maximization):** Update parameters to maximize $Q$:

$$
\Theta^{(t+1)} = \arg\max_\Theta Q(\Theta | \Theta^{(t)})
$$

EM is guaranteed to increase the likelihood at each iteration and converges to a **local maximum** — not necessarily the global one. Lutufi runs multiple random restarts to mitigate this.

### 8.2 Structure Learning

Structure learning — inferring the graph $\mathcal{G}$ from data $\mathcal{D}$ — is computationally harder than parameter learning. The space of DAGs over $n$ variables is **superexponential** in $n$ (it grows faster than $2^{n^2}$). Exact search is infeasible for $n > 5$.

Lutufi supports two families of structure learning methods:

**Score-Based Methods:**

Define a scoring function $S(\mathcal{G}, \mathcal{D})$ that measures how well the graph explains the data, penalized by complexity, then search for the highest-scoring graph.

- **BIC (Bayesian Information Criterion):** $S_{\text{BIC}}(\mathcal{G}, \mathcal{D}) = \log P(\mathcal{D} \mid \hat{\Theta}, \mathcal{G}) - \frac{\log N}{2} |\Theta_\mathcal{G}|$, where $|\Theta_\mathcal{G}|$ is the number of free parameters.
- **BDeu Score:** A Bayesian score that integrates over parameters (marginalizes them out), avoiding overfitting. Preferred when $N$ is small or data is sparse.
- **Greedy Equivalence Search (GES)** (Chickering 2002): Searches over **Markov equivalence classes** of DAGs rather than individual DAGs, reducing search space. Provably correct in the large-sample limit under faithfulness. Lutufi's default structure learning algorithm.

**Constraint-Based Methods:**

Use statistical tests of conditional independence — such as the $G$-test or kernel-based tests for continuous data — to determine which edges are absent from the true graph, then reconstruct a graph consistent with the observed independencies.

- **PC Algorithm** (Spirtes, Glymour, Scheines): The foundational constraint-based algorithm. Starts with a complete graph and removes edges where the empirical test finds independence. Provably correct under faithfulness and the Markov condition.
- **FCI Algorithm** (Fast Causal Inference): An extension of PC that handles **latent confounders** — variables that are causally relevant but never observed. Particularly important for social network data where many relevant factors are unmeasured.

**The Role of Domain Knowledge in Structure Learning:**

Structure learning algorithms often produce multiple statistically equivalent graphs. Domain knowledge is essential for selecting among them. Lutufi provides an interface for experts to specify **forbidden edges** (relationships that cannot exist), **required edges** (relationships known to exist), and **ordering constraints** (variable $A$ cannot be a descendant of variable $B$).

---

## 9. Dynamic Bayesian Networks

Social and economic networks are not static. Relationships form and dissolve. Beliefs update. Markets move. Influence propagates over time. **Dynamic Bayesian Networks (DBNs)** extend the Bayesian network framework to model temporal processes.

### 9.1 Formal Structure of a DBN

A DBN represents the joint distribution over variables across **time slices** $t = 0, 1, 2, \dots, T$. The state at time $t$ is denoted $\mathbf{X}^{(t)} = \{X_1^{(t)}, X_2^{(t)}, \dots, X_n^{(t)}\}$.

A DBN is defined by two components:

**Prior Distribution:** $P(\mathbf{X}^{(0)})$ — the distribution over the initial state.

**Transition Model:** $P(\mathbf{X}^{(t)} \mid \mathbf{X}^{(t-1)})$ — the conditional distribution of the current state given the previous state.

Under the **first-order Markov assumption**, the current state depends only on the immediately preceding state, not on any earlier history:

$$
P(\mathbf{X}^{(t)} \mid \mathbf{X}^{(0)}, \dots, \mathbf{X}^{(t-1)}) = P(\mathbf{X}^{(t)} \mid \mathbf{X}^{(t-1)})
$$

The full joint distribution over a trajectory of length $T$ is:

$$
P(\mathbf{X}^{(0:T)}) = P(\mathbf{X}^{(0)}) \prod_{t=1}^T P(\mathbf{X}^{(t)} \mid \mathbf{X}^{(t-1)})
$$

### 9.2 The Observation Model

In most real applications, the true state $\mathbf{X}^{(t)}$ is **not fully observed**. Instead, we observe evidence $\mathbf{E}^{(t)}$ that is generated from the hidden state:

$$
P(\mathbf{E}^{(t)} \mid \mathbf{X}^{(t)})
$$

This separates what is real (the hidden state) from what is measurable (the evidence). In a social network context, the true influence relationships are hidden; what we observe are behaviors, statements, and transactions.

### 9.3 Special Cases: HMMs and Kalman Filters

**Hidden Markov Models (HMMs)** are the discrete, univariate special case of DBNs. A single discrete hidden state $X^{(t)} \in \{1, \dots, K\}$ transitions according to a matrix $A_{ij} = P(X^{(t)} = j \mid X^{(t-1)} = i)$, and generates a discrete or continuous observation $E^{(t)}$ according to an emission distribution $B_k(e) = P(E^{(t)} = e \mid X^{(t)} = k)$.

HMMs are used extensively in sequence modeling — detecting phases of market behavior, stages of opinion formation, or states in an influence campaign.

**Kalman Filters** are the continuous, linear-Gaussian special case of DBNs. Both the transition and observation models are linear with Gaussian noise:

$$
\mathbf{X}^{(t)} = A\mathbf{X}^{(t-1)} + \mathbf{w}^{(t)}, \quad \mathbf{w}^{(t)} \sim \mathcal{N}(\mathbf{0}, Q)
$$

$$
\mathbf{E}^{(t)} = C\mathbf{X}^{(t)} + \mathbf{v}^{(t)}, \quad \mathbf{v}^{(t)} \sim \mathcal{N}(\mathbf{0}, R)
$$

The Kalman Filter computes the exact posterior $P(\mathbf{X}^{(t)} \mid \mathbf{E}^{(1:t)})$ analytically in closed form. It is widely used in financial network state estimation.

### 9.4 Inference in DBNs

**Filtering:** Computing $P(\mathbf{X}^{(t)} \mid \mathbf{E}^{(1:t)})$ — the current hidden state given all observations up to now. This is online inference, updating the belief state as each new observation arrives.

**Smoothing:** Computing $P(\mathbf{X}^{(t)} \mid \mathbf{E}^{(1:T)})$ for $t < T$ — the hidden state at a past time given all observations including future ones. More accurate than filtering but requires the full observation sequence.

**Prediction:** Computing $P(\mathbf{X}^{(t+k)} \mid \mathbf{E}^{(1:t)})$ for $k > 0$ — the future hidden state given current observations.

The **Forward-Backward Algorithm** performs exact filtering and smoothing in $O(T \cdot K^2)$ time for HMMs, where $K$ is the number of hidden states. For general DBNs, the interface network (the two-slice temporal BN) is unrolled and inference is performed using standard BN algorithms.

### 9.5 Non-Stationary DBNs

Standard DBNs assume the transition model $P(\mathbf{X}^{(t)} \mid \mathbf{X}^{(t-1)})$ is **time-homogeneous** — the same transition dynamics apply at every time step. Real social and economic networks violate this assumption constantly. The network structure changes. New edges appear. Old ones disappear. The influence dynamics of a market in crisis are different from those in stability.

Lutufi supports **non-stationary DBNs** where the transition model itself can be parameterized by time or by higher-level regime variables. This enables modeling of phase transitions — the qualitative shifts in network dynamics that accompany crises, elections, and other discontinuous events.

---

## 10. Causal Modeling and Do-Calculus

Probabilistic inference answers the question: *"Given that I observe $X$, what do I expect to observe about $Y$?"* This is **associational** reasoning — it quantifies correlation, not causation.

Many of the most important questions in social, economic, and policy research are **interventional**: *"If I change $X$, what effect will that have on $Y$?"* These questions cannot be answered from observational data alone, no matter how much data we have. They require a **causal model**.

### 10.1 Structural Causal Models (SCMs)

A **Structural Causal Model** $\mathcal{M} = (\mathbf{U}, \mathbf{V}, \mathcal{F}, P(\mathbf{U}))$ consists of:

- **Exogenous variables** $\mathbf{U}$: Background factors determined outside the model, with distribution $P(\mathbf{U})$.
- **Endogenous variables** $\mathbf{V}$: Variables whose values are determined by the model.
- **Structural equations** $\mathcal{F} = \{f_i\}$: Each equation $X_i = f_i(\text{Pa}(X_i), U_i)$ specifies the **mechanism** by which $X_i$ is generated from its direct causes.

**Critical distinction from standard BNs:** In a BN, edges represent probabilistic dependence. In an SCM, edges represent **direct causal mechanisms**. The graph of an SCM is called a **Directed Acyclic Causal Graph (DACG)**. Confusing the two leads to incorrect causal conclusions.

### 10.2 The Do-Operator and Interventional Distributions

Pearl's **do-operator** formalizes the distinction between observation and intervention.

**Observational distribution:** $P(Y \mid X = x)$ — the distribution of $Y$ among units where $X$ happened to be $x$. This is affected by confounding — the common causes of $X$ and $Y$.

**Interventional distribution:** $P(Y \mid \text{do}(X = x))$ — the distribution of $Y$ when $X$ is **forced** to value $x$ by external intervention, regardless of its natural causes.

In the SCM framework, computing $P(Y \mid \text{do}(X = x))$ corresponds to constructing a **mutilated model** $\mathcal{M}_x$: remove all edges into $X$, set $X = x$, and compute the resulting distribution of $Y$.

**A policy example:** In a network of financial institutions, $P(\text{Bank B fails} \mid \text{Bank A fails})$ is the observational probability — affected by the common macroeconomic conditions that may cause both to fail. $P(\text{Bank B fails} \mid \text{do(Bank A fails)})$ is the interventional probability — asking what happens if Bank A is exogenously forced into failure, regardless of the economic climate. Only the latter answers the question of whether Bank A's failure causally propagates to Bank B.

### 10.3 The Rules of Do-Calculus

Pearl's **do-calculus** provides three rules that, together, allow any interventional distribution to be reduced to observational distributions — provided the reduction is possible given the graph structure. The three rules govern when:

1. Observations of additional variables can be added or deleted from conditioning.
2. Observations can be replaced by interventions or vice versa.
3. Interventions on variables can be deleted.

Each rule applies under specific graph-theoretic conditions. Together they are **complete** — any causal query that can be identified from a given graph can be identified using these three rules.

Lutufi implements do-calculus as a symbolic reasoning system, allowing users to specify a causal query and the available data, and returns either an identification formula (the observational expression that computes the causal effect) or a non-identification result (proof that the effect cannot be determined from observational data alone).

### 10.4 Causal vs. Statistical Graphs

**A graph used in a BN is not automatically a causal graph.** A BN can represent any joint distribution using any DAG that encodes the same conditional independencies — including graphs that have no causal interpretation. A causal graph makes a stronger commitment: each edge asserts a direct causal mechanism, and the absence of an edge asserts the absence of such a mechanism.

Lutufi requires users to explicitly designate whether a model is **causal** or **statistical**. Causal operations (do-calculus, counterfactual queries) are only available on models designated as causal. This prevents users from accidentally making causal claims from purely statistical models.

---

## 11. Identifiability

### 11.1 The Problem

A causal effect $P(Y \mid \text{do}(X))$ is **identifiable** from observational data $P(\mathbf{V})$ if it can be uniquely computed from the observational distribution and the causal graph $\mathcal{G}$ — regardless of the specific numerical values of the parameters.

Identifiability can fail when there are **hidden confounders** — latent variables that are common causes of both the treatment $X$ and the outcome $Y$, but are not observed. In social networks, hidden confounders are everywhere: unmeasured individual traits, shared environment effects, selection processes.

### 11.2 The ID Algorithm

The **ID algorithm** (Shpitser and Pearl, 2006) is a complete algorithm for determining identifiability of arbitrary interventional distributions in the presence of hidden variables, given a causal graph with **bidirected edges** (representing hidden confounding).

Lutufi exposes the ID algorithm as a first-class operation. Given a query $P(Y \mid \text{do}(X))$ and a causal graph, Lutufi either:

- Returns an **identification formula** — an expression in terms of observational distributions that computes the causal effect, or
- Returns a **non-identification certificate** — a proof that the query cannot be uniquely determined from the available data.

### 11.3 Parametric Non-Identifiability

Even in the absence of hidden confounders, models with **latent variables** can be non-identifiable at the level of specific parameter values. For example, in a mixture model with $K$ components, permuting the component labels produces the same distribution — the labels are not identifiable. Lutufi checks for and reports potential parametric non-identifiability in latent variable models.

---

## 12. Missing Data

### 12.1 Why Missing Data is Central to Lutufi

In social and economic network analysis, **complete data is the exception, not the rule.** Survey non-response, censored transactions, covert relationships, and measurement limitations all produce incomplete network data. How missing data is handled determines whether inference results are valid or systematically biased.

### 12.2 The Three Mechanisms of Missingness

Following the taxonomy of Rubin (1976), missingness falls into three categories with fundamentally different implications:

**Missing Completely at Random (MCAR):**

The probability of a value being missing is independent of both the observed and unobserved data:

$$
P(M_{ij} = 1 \mid X_{\text{obs}}, X_{\text{miss}}) = P(M_{ij} = 1)
$$

where $M_{ij}$ is an indicator that the value of variable $j$ for unit $i$ is missing. Under MCAR, the observed data is a simple random sample of the complete data, and standard inference on observed data is unbiased.

**Missing at Random (MAR):**

The probability of missingness depends only on observed variables, not on the missing values themselves:

$$
P(M_{ij} = 1 \mid X_{\text{obs}}, X_{\text{miss}}) = P(M_{ij} = 1 \mid X_{\text{obs}})
$$

Under MAR, standard complete-case analysis is biased, but valid inference is possible if the missingness mechanism is accounted for. EM and multiple imputation are valid under MAR.

**Missing Not at Random (MNAR):**

The probability of missingness depends on the missing values themselves:

$$
P(M_{ij} = 1 \mid X_{\text{obs}}, X_{\text{miss}}) \neq P(M_{ij} = 1 \mid X_{\text{obs}})
$$

**MNAR is the dominant case in social and economic network data.** High-status individuals are less likely to respond to surveys about their network connections. Covert relationships are precisely those that actors are least likely to report. Financial institutions conceal the most risky exposures. Under MNAR, no general estimation method is valid without modeling the missingness mechanism explicitly.

### 12.3 Lutufi's Approach to MNAR

Lutufi treats missing observations as **latent variables** and incorporates the missingness mechanism into the model. For network data:

- Missing edges are represented as unobserved binary variables with prior distributions informed by the observed network structure (e.g., homophily patterns, degree distribution).
- The EM algorithm estimates both the complete-data parameters and the posterior distribution over missing values simultaneously.
- Lutufi reports the fraction of missing data and a diagnostic for which missingness mechanism is most consistent with the observed data pattern.

### 12.4 Network-Specific Missingness

Beyond standard statistical missingness, network data has **structural missingness** — the possibility that entire nodes are absent from the observed network because they are unknown to the researcher. Standard missing data theory addresses missing values within observed units; network science requires additional tools for **network reconstruction** from partial observations. Lutufi's network reconstruction module, built on Bayesian inference over random graph models, addresses this problem explicitly.

---

## 13. Numerical Stability

Probabilistic inference involves multiplying many probabilities together. In large networks, this causes **floating-point underflow** — the product of many small numbers becomes smaller than the smallest representable floating-point number ($\approx 10^{-308}$ for 64-bit floats), and is rounded to zero. Once a probability becomes zero, all subsequent computations involving it are corrupted.

### 13.1 Log-Space Arithmetic

The standard remedy is to compute in **log-space**, replacing multiplications with additions:

$$
\log P(X_1, \dots, X_n) = \sum_{i=1}^n \log P(X_i \mid \text{Pa}(X_i))
$$

Additions in log-space correspond to multiplications in probability-space and do not suffer from underflow. All of Lutufi's internal inference computations are performed in log-space. Probabilities are only converted back to standard space when returning results to the user.

The **log-sum-exp** trick handles additions of probabilities (required for marginalization) stably:

$$
\log\sum_i e^{a_i} = a^* + \log\sum_i e^{a_i - a^*}, \quad a^* = \max_i a_i
$$

By factoring out the largest term, the remaining sum involves values in $[0, 1]$, which are well-represented by floating-point numbers.

### 13.2 Sparse Representations

Conditional Probability Tables grow exponentially with the number of parents. A variable with 20 binary parents has a CPT with $2^{20} \approx 10^6$ entries. In practice, most entries in real-world CPTs are either zero (impossible configurations) or very small.

Lutufi uses **sparse tensor representations** for CPTs, storing only non-zero entries. For structured CPDs (like noisy-OR or context-specific independence models), Lutufi uses compact parametric representations rather than full explicit tables, reducing memory requirements by several orders of magnitude.

### 13.3 Precision in Variational and MCMC Inference

Variational inference involves optimization that can get stuck in regions where gradients vanish (related to the **vanishing gradient problem** in deep networks). Lutufi uses **natural gradient methods** for variational optimization, which are less sensitive to the scaling of the probability space and converge more reliably.

For MCMC, Lutufi monitors **autocorrelation** in the chain (which degrades effective sample size) and the **acceptance rate** in Metropolis-Hastings (which should be between 0.2 and 0.5 for efficient mixing). Automatic tuning of proposal distributions is applied to maintain this range.

---

## 14. Computational Complexity

### 14.1 The Fundamental Result

Exact marginal inference in Bayesian Networks is **#P-complete** — strictly harder than NP. The #P-completeness result means that, unless the polynomial hierarchy collapses (considered highly unlikely), there is no algorithm that can solve all instances of exact inference in polynomial time.

**Intuitively:** Computing a marginal probability requires summing over all configurations of all unobserved variables — a sum over an exponentially large space.

### 14.2 Treewidth and the Complexity of Exact Inference

The **treewidth** $w(\mathcal{G})$ of a graph $\mathcal{G}$ is a measure of how "tree-like" the graph is. Trees have treewidth 1. The treewidth determines the complexity of exact inference through the Junction Tree Algorithm: $O(n \cdot k^{w+1})$, where $k$ is the maximum domain size.

**Practical guidance on treewidth:**

- Treewidth $\leq 15$: Generally tractable for exact inference on modern hardware.
- Treewidth $15$–$50$: Marginal tractability; approximate methods preferred.
- Treewidth $> 50$: Exact inference infeasible; approximate methods required.

Social networks with strong community structure (high clustering coefficient) tend to have high treewidth because the dense intra-community edges form large cliques. Lutufi computes a **treewidth estimate** when a model is loaded and recommends the appropriate inference algorithm automatically.

### 14.3 Complexity of Learning

**Parameter learning** (with complete data and fixed structure): $O(N \cdot n)$ — linear in data and variables. Efficient.

**Structure learning (score-based, exact):** NP-hard. The number of DAGs over $n$ nodes is superexponential.

**Structure learning (score-based, GES):** $O(n^2 \cdot N)$ per iteration, polynomial in practice for sparse graphs.

**Structure learning (constraint-based, PC):** $O(n^2 \cdot N)$ for sparse graphs with bounded maximum degree. Can be prohibitively slow for dense networks.

### 14.4 Scalability Architecture

Lutufi's scalability strategy is organized in three tiers:

**Tier 1 — Small networks (< 1,000 nodes):** Exact inference with Junction Tree. Full parameter and structure learning.

**Tier 2 — Medium networks (1,000–100,000 nodes):** Loopy Belief Propagation or Mean Field Variational Inference. Score-based structure learning with greedy search. Sparse CPT representations.

**Tier 3 — Large networks (> 100,000 nodes):** Stochastic Variational Inference with mini-batching. Constraint-based structure learning on sampled subnetworks. Distributed computation support.

---

## 15. Scope and Limitations

Lutufi is a tool for rigorous probabilistic reasoning. It is not an oracle. The following limitations are not deficiencies to be overcome — they are inherent to the mathematics and must be understood by every user.

**Model misspecification:** All inference results are conditional on the graph structure being correct. If the true causal structure of the social system differs from the specified graph — through incorrect edge directions, missing edges, or missing variables — the inference results will be systematically wrong. Lutufi does not check whether the model is correct; it computes the implications of the model the user provides. Sensitivity analysis and structure learning diagnostics are provided to probe model assumptions.

**Black Swan events:** The probability framework assumes that all possible outcomes are represented in the model's state space. Events that fall entirely outside this space — truly novel, unprecedented outcomes — are assigned probability zero by any model. Lutufi does not protect against unknown unknowns.

**Approximate inference error:** LBP, variational inference, and finite MCMC chains all introduce approximation error. For LBP, this error has no general bound. For variational inference, the ELBO provides a lower bound on the log-marginal likelihood but does not bound the error in individual marginals. For MCMC, increasing the number of samples reduces error asymptotically but any finite run has finite precision. Lutufi reports diagnostics for approximation quality but cannot guarantee the accuracy of any specific approximate inference result.

**Causal claims require causal models:** Probabilistic inference from a statistical graph does not support causal conclusions. Only models explicitly designated as causal SCMs support do-calculus and interventional queries. Using observational inference to answer causal questions — even from a well-fitted model — is invalid.

**Faithfulness can fail:** Structure learning and d-separation reasoning both rely on the faithfulness assumption. In social systems with strong competing pathways of influence, faithfulness can fail, causing structure learning algorithms to recover incorrect graphs.

**Scale of MNAR:** Lutufi's MNAR mitigation methods reduce but do not eliminate bias from non-random missingness. When the missingness mechanism is strongly related to unobserved variables, residual bias may be substantial.

---

## 16. Key References

- **Pearl, J. (1988).** *Probabilistic Reasoning in Intelligent Systems: Networks of Plausible Inference.* Morgan Kaufmann. — The foundational text on Bayesian networks, belief propagation, and d-separation.
- **Lauritzen, S. L., & Spiegelhalter, D. J. (1988).** Local computations with probabilities on graphical structures and their application to expert systems. *Journal of the Royal Statistical Society, Series B, 50*(2), 157–224. — The original Junction Tree Algorithm paper.
- **Koller, D., & Friedman, N. (2009).** *Probabilistic Graphical Models: Principles and Techniques.* MIT Press. — The comprehensive modern treatment of PGMs; the primary reference for Lutufi's mathematical foundations.
- **Murphy, K. P. (2002).** *Dynamic Bayesian Networks: Representation, Inference and Learning.* PhD Dissertation, UC Berkeley. — The defining reference for DBNs, HMMs, and Kalman Filters in the PGM framework.
- **Pearl, J. (2000).** *Causality: Models, Reasoning, and Inference.* Cambridge University Press. — The foundational text on structural causal models, do-calculus, and causal identifiability.
- **Shpitser, I., & Pearl, J. (2006).** Identification of joint interventional distributions in recursive semi-Markovian causal models. *Proceedings of AAAI 2006.* — The ID algorithm for causal identifiability with hidden variables.
- **Wainwright, M. J., & Jordan, M. I. (2008).** Graphical models, exponential families, and variational inference. *Foundations and Trends in Machine Learning, 1*(1–2), 1–305. — The authoritative treatment of variational inference and its connections to graphical models.
- **Chickering, D. M. (2002).** Optimal structure identification with greedy search. *Journal of Machine Learning Research, 3*, 507–554. — The GES algorithm for score-based structure learning.
- **Spirtes, P., Glymour, C., & Scheines, R. (2000).** *Causation, Prediction, and Search* (2nd ed.). MIT Press. — The PC and FCI algorithms for constraint-based structure learning.
- **Rubin, D. B. (1976).** Inference and missing data. *Biometrika, 63*(3), 581–592. — The MCAR/MAR/MNAR taxonomy.
- **Dempster, A. P., Laird, N. M., & Rubin, D. B. (1977).** Maximum likelihood from incomplete data via the EM algorithm. *Journal of the Royal Statistical Society, Series B, 39*(1), 1–38. — The original EM algorithm paper.
- **Newman, M. E. J. (2010).** *Networks: An Introduction.* Oxford University Press. — The standard reference text on network science, bridging the gap between graph theory and empirical social/economic network analysis.
- **Barber, D. (2012).** *Bayesian Reasoning and Machine Learning.* Cambridge University Press. — A modern unified treatment available freely online; particularly strong on factor graphs and approximate inference.

---

*This document is a living reference. It will be updated as Lutufi's mathematical foundations are refined, extended, and validated against empirical applications. Corrections, additions, and challenges from domain experts are welcomed.*

---

**End of Document — Mathematical Foundations of Lutufi v1.0**
