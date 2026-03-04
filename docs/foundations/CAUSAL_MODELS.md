# Structural Causal Models and Do-Calculus

---

**Document Version:** 1.0
**Status:** Working Draft — Research Phase
**Author:** Wasswa Lutufi Sebbanja
**Last Updated:** March 2026
**License:** Apache 2.0

---

> *"Causation is not a thing that exists in the world, but a category that we impose upon the world to make sense of it."*
> — David Hume
>
> *"You are smarter than your data. Data do not understand causes and effects; humans do."*
> — Judea Pearl

---

## Table of Contents

1. [Introduction: The Causation Problem](#1-introduction-the-causation-problem)
2. [Historical Development](#2-historical-development)
3. [Structural Causal Models (SCMs)](#3-structural-causal-models-scms)
4. [The Do-Operator](#4-the-do-operator)
5. [The Three Rules of Do-Calculus](#5-the-three-rules-of-do-calculus)
6. [The Adjustment Formula (Back-Door Criterion)](#6-the-adjustment-formula-back-door-criterion)
7. [The Front-Door Criterion](#7-the-front-door-criterion)
8. [Instrumental Variables](#8-instrumental-variables)
9. [Identifiability](#9-identifiability)
10. [Counterfactuals](#10-counterfactuals)
11. [Mediation Analysis](#11-mediation-analysis)
12. [Causal Discovery](#12-causal-discovery)
13. [Potential Outcomes Framework](#13-potential-outcomes-framework)
14. [Causation in Social Networks](#14-causation-in-social-networks)
15. [Causation in Economic Networks](#15-causation-in-economic-networks)
16. [How Lutufi Implements Causal Reasoning](#16-how-lutufi-implements-causal-reasoning)
17. [Key References](#17-key-references)

---

## 1. Introduction: The Causation Problem

### 1.1 Why Correlation ≠ Causation

The distinction between correlation and causation is the central problem of empirical science. Two variables can be correlated for three fundamentally different reasons:

1. **$X$ causes $Y$:** Changes in $X$ produce changes in $Y$. Smoking causes cancer.
2. **$Y$ causes $X$:** The causal direction is reversed. A company's stock price rising causes media attention, not vice versa.
3. **A common cause $Z$ produces both $X$ and $Y$:** Neither causes the other. Shoe size correlates with reading ability in children because age (the common cause) affects both.

Observational data — data collected by passively observing a system — cannot, in general, distinguish between these three scenarios. The observed statistical relationship $P(Y \mid X)$ is the same regardless of the causal mechanism. Yet the implications for action are entirely different: if smoking causes cancer, prohibiting smoking reduces cancer. If the correlation is due to a genetic factor that causes both smoking behavior and cancer susceptibility, prohibiting smoking has no effect on cancer rates.

### 1.2 Simpson's Paradox: A Worked Example

**Simpson's paradox** dramatically illustrates the danger of confusing correlation with causation. Consider a clinical trial for a new drug, with the following data:

**Overall data:**

| | Recovered | Not Recovered | Recovery Rate |
|---|-----------|---------------|---------------|
| Drug | 20 | 20 | 50% |
| No Drug | 16 | 24 | 40% |

The drug appears effective: 50% recovery vs. 40% without the drug.

**Stratified by gender:**

| **Males** | Recovered | Not Recovered | Recovery Rate |
|-----------|-----------|---------------|---------------|
| Drug | 18 | 12 | 60% |
| No Drug | 7 | 3 | 70% |

| **Females** | Recovered | Not Recovered | Recovery Rate |
|-------------|-----------|---------------|---------------|
| Drug | 2 | 8 | 20% |
| No Drug | 9 | 21 | 30% |

Within males, the drug is *harmful*: 60% vs. 70%. Within females, the drug is *also harmful*: 20% vs. 30%. Yet overall, the drug appears *beneficial*!

**Resolution.** This paradox arises because gender is a **confounder**: it affects both the treatment assignment (males are more likely to take the drug) and the outcome (males have higher baseline recovery rates). The correct causal conclusion requires adjusting for the confounder — stratifying by gender reveals the true harmful effect of the drug.

**Crucially**, whether to adjust depends on the causal structure — not just the data. If gender is a confounder (common cause of treatment and outcome), we should adjust. If gender is a mediator (the drug affects outcomes partly through gender-linked mechanisms), adjusting could be incorrect. The data alone cannot tell us which; we need a causal model.

This is precisely the problem that structural causal models and do-calculus solve: they provide a formal, rigorous framework for determining when and how to adjust for variables to estimate causal effects from observational data.

### 1.3 The Fundamental Problem of Causal Inference

The fundamental problem of causal inference (Holland, 1986) is that we can never observe the same unit under both treatment and control. If patient $i$ receives the drug, we observe their outcome under treatment but never their outcome without it. The counterfactual outcome — what *would have* happened — is inherently unobservable.

This means that individual causal effects $Y_i(1) - Y_i(0)$ (the difference between the treated and untreated potential outcomes for unit $i$) are never directly observable. Causal inference is possible only at the population level (average causal effects) and only under assumptions that link the observed data to the unobserved counterfactuals.

---

## 2. Historical Development

### 2.1 Wright's Path Analysis (1920s–1930s)

The modern mathematization of causal reasoning begins with **Sewall Wright** (1889–1988), a population geneticist at the University of Chicago. In a series of papers starting in 1920, Wright developed **path analysis** — a method for decomposing correlations between variables into direct and indirect causal effects using directed graphs and structural equations.

Wright's basic framework: given a system of equations $X_i = \sum_j \beta_{ij} X_j + \epsilon_i$ and a directed graph showing which variables directly affect which, the total correlation between any two variables can be decomposed as a sum over all directed paths connecting them. Each path's contribution is the product of the path coefficients (the $\beta_{ij}$) along the path.

Wright's work was largely ignored by statisticians for decades, partly because it conflated causal and statistical reasoning in ways that were not clearly justified, and partly because the dominant statistical paradigm (Fisherian frequentism) was explicitly anti-causal. Wright's contributions were rediscovered and formalized by Pearl (2000), who showed that path analysis was an early, incomplete version of structural causal modeling.

### 2.2 Rubin's Potential Outcomes (1970s–1980s)

**Donald Rubin** developed the **potential outcomes framework** (also called the Rubin Causal Model or the Neyman-Rubin model) starting in the 1970s. Building on earlier ideas by Neyman (1923) and Fisher (1935), Rubin formalized causal inference as a missing data problem.

For each unit $i$ and each treatment level $t$, there is a **potential outcome** $Y_i(t)$ — the outcome that would be observed if unit $i$ were assigned to treatment $t$. The causal effect of treatment $t = 1$ versus $t = 0$ for unit $i$ is $Y_i(1) - Y_i(0)$.

The framework's key assumption is the **Stable Unit Treatment Value Assumption (SUTVA)**: each unit's potential outcome depends only on its own treatment assignment, not on the treatment assignments of other units. SUTVA is critical for defining individual causal effects and is pervasively violated in network settings.

Rubin's framework provides a clean language for defining causal estimands (ATE, ATT, ATU), stating assumptions (ignorability, overlap), and designing estimation strategies (matching, propensity score methods, instrumental variables). However, it is weaker than the structural causal model framework in important respects:

- It does not represent the causal mechanism (only the counterfactual outcomes).
- It cannot express conditional causal effects or mediating pathways as naturally.
- It does not provide a systematic identification procedure for arbitrary graphs.

### 2.3 Pearl's Structural Causal Models (1990s–2000s)

**Judea Pearl** (1936–2024), a computer scientist at UCLA, developed the framework that unifies and extends the earlier approaches. In a series of papers and two landmark books — *Probabilistic Reasoning in Intelligent Systems* (1988) and *Causality: Models, Reasoning, and Inference* (2000, 2nd edition 2009) — Pearl built the theory of **structural causal models (SCMs)**, introduced the **do-operator** and **do-calculus**, and proved the completeness of his identification framework.

Pearl's key contributions:
1. **The distinction between seeing and doing.** Formalizing the difference between $P(Y \mid X = x)$ (conditioning/observation) and $P(Y \mid \text{do}(X = x))$ (intervention).
2. **The do-calculus.** Three rules that transform interventional expressions into observational ones, whenever the transformation is possible.
3. **The completeness theorem.** Do-calculus is complete — any identifiable causal effect can be identified using the three rules.
4. **The ID algorithm.** An algorithmic procedure for identifying causal effects from arbitrary graphs with latent confounders.
5. **Counterfactual semantics.** A formal definition of counterfactuals within the SCM framework, enabling reasoning about what would have happened under different conditions.

Pearl was awarded the Turing Award in 2011 for these and related contributions to artificial intelligence.

### 2.4 The "Causal Revolution"

Pearl, along with collaborators and intellectual allies (Spirtes, Glymour, Scheines, Robins, Richardson, Tian, Shpitser, Bareinboim, and many others), catalyzed what is sometimes called the **causal revolution** — a paradigm shift in statistics, epidemiology, social science, and AI toward formal causal reasoning. The revolution's key tenets:

- Causal questions cannot be answered with statistical tools alone; they require causal models.
- Directed graphs are essential for representing and reasoning about causal structure.
- The distinction between correlation and causation is not philosophical but mathematical.
- Formal identification theory determines exactly when causal effects can be estimated from observational data.

This revolution is ongoing. Many domains — including social network analysis and economics — are still assimilating these ideas.

---

## 3. Structural Causal Models (SCMs)

### 3.1 Formal Definition

A **Structural Causal Model (SCM)** is a quadruple $\mathcal{M} = (\mathbf{U}, \mathbf{V}, \mathcal{F}, P(\mathbf{U}))$ where:

- **$\mathbf{U}$** is a set of **exogenous** (background) variables. These are determined by factors outside the model. Their values are not explained by the model.

- **$\mathbf{V} = \{V_1, V_2, \ldots, V_n\}$** is a set of **endogenous** variables. Each is determined by a structural equation involving other endogenous variables and exogenous variables.

- **$\mathcal{F} = \{f_1, f_2, \ldots, f_n\}$** is a set of **structural equations** (also called **mechanisms**). Each equation specifies the value of one endogenous variable as a function of its direct causes:

$$
V_i = f_i(\text{Pa}(V_i), U_i)
$$

where $\text{Pa}(V_i) \subseteq \mathbf{V} \setminus \{V_i\}$ is the set of endogenous parents of $V_i$, and $U_i \subseteq \mathbf{U}$ is the set of exogenous variables that directly affect $V_i$.

- **$P(\mathbf{U})$** is a probability distribution over the exogenous variables. This distribution, together with the structural equations, determines the joint distribution over all endogenous variables.

### 3.2 The Causal Graph

The **causal graph** $\mathcal{G}$ of an SCM $\mathcal{M}$ is a directed graph with:
- One node for each endogenous variable $V_i$.
- A directed edge $V_j \to V_i$ whenever $V_j \in \text{Pa}(V_i)$ — that is, $V_j$ appears in the structural equation for $V_i$.

For the model to be well-defined, the causal graph must be a **Directed Acyclic Graph (DAG)** — there can be no cyclic chains of causation. This ensures that every endogenous variable has a unique value for any setting of the exogenous variables.

When exogenous variables $U_i$ and $U_j$ are correlated (share a common source), this is represented in the causal graph by a **bidirected edge** $V_i \leftrightarrow V_j$ (dashed or double-headed arrow), indicating the presence of a latent common cause.

### 3.3 Exogenous vs. Endogenous Variables

**Exogenous variables** represent the background conditions, noise, or omitted factors. They are:
- Not causally explained by the model.
- May be correlated with each other (latent confounders).
- Determine the "individuality" of each unit — two units with different exogenous values may respond differently to the same intervention.

**Endogenous variables** are the variables of interest — the ones we observe, intervene on, or reason about. They are completely determined by their structural equations once the exogenous values are fixed.

### 3.4 Worked Example: Education, Income, and Social Network Position

Consider a social network model with three endogenous variables:

- **$E$ (Education):** Years of education.
- **$N$ (Network Centrality):** A measure of social network position (e.g., betweenness centrality).
- **$I$ (Income):** Annual income.

And three exogenous variables $U_E$, $U_N$, $U_I$ representing individual talent, family background, and luck respectively.

**Structural equations:**
$$
E = f_E(U_E) = \beta_0 + U_E
$$
$$
N = f_N(E, U_N) = \gamma_0 + \gamma_1 E + U_N
$$
$$
I = f_I(E, N, U_I) = \delta_0 + \delta_1 E + \delta_2 N + U_I
$$

**Causal graph:**
```
E → N → I
 \       ↑
  \ ----→
```

(Education has a direct effect on income and an indirect effect through network centrality.)

**Exogenous distribution:** $U_E, U_N, U_I$ are jointly distributed (potentially correlated, representing latent common causes).

From this SCM, we can compute:
- The **observational distribution** $P(I \mid E = e)$: the expected income of people who happen to have education level $e$.
- The **interventional distribution** $P(I \mid \text{do}(E = e))$: the expected income if we *forced* everyone to have education level $e$.
- The **counterfactual** $P(I_{E=e'} \mid E = e, I = i)$: the expected income if person with education $e$ and income $i$ had instead received education $e'$.

These three quantities are generally different and answer fundamentally different questions.

### 3.5 The Causal Hierarchy (Ladder of Causation)

Pearl identifies three levels of causal reasoning, often called the **ladder of causation** or **causal hierarchy:**

**Level 1 — Association (Seeing):** $P(Y \mid X)$. What is the probability of $Y$ given that I observe $X$? This is statistical reasoning — computable from observational data alone. No causal model needed.

**Level 2 — Intervention (Doing):** $P(Y \mid \text{do}(X = x))$. What is the probability of $Y$ if I intervene to set $X = x$? This requires a causal model. It cannot, in general, be computed from observational data without causal assumptions.

**Level 3 — Counterfactual (Imagining):** $P(Y_x \mid X = x', Y = y')$. Given that I observed $X = x'$ and $Y = y'$, what would $Y$ have been if $X$ had been $x$ instead? This requires the full SCM, including the exogenous variables and structural equations.

Each level strictly subsumes the one below — interventional queries cannot be answered from associational data alone, and counterfactual queries cannot be answered from interventional data alone (Bareinboim et al., 2022).

---

## 4. The Do-Operator

### 4.1 Formal Definition

The **do-operator** $\text{do}(X = x)$ represents an **external intervention** that forces variable $X$ to take value $x$, overriding the natural mechanism that would otherwise determine $X$'s value.

Formally, in an SCM $\mathcal{M}$, the intervention $\text{do}(X = x)$ produces a new model $\mathcal{M}_x$ (the **mutilated model**) obtained by:

1. Replacing the structural equation $X = f_X(\text{Pa}(X), U_X)$ with the constant equation $X = x$.
2. Leaving all other structural equations unchanged.

The interventional distribution is:

$$
P(Y = y \mid \text{do}(X = x)) = P_{\mathcal{M}_x}(Y = y)
$$

— the probability of $Y = y$ in the mutilated model.

### 4.2 Do(X = x) vs. X = x

The distinction between $\text{do}(X = x)$ and $X = x$ is the mathematical formalization of "doing" vs. "seeing":

- **$P(Y \mid X = x)$** — "seeing" or conditioning: What is the distribution of $Y$ among units where $X$ happened to be $x$, including any selection effects?
- **$P(Y \mid \text{do}(X = x))$** — "doing" or intervening: What is the distribution of $Y$ when $X$ is forced to be $x$ by external action, removing all natural causes of $X$?

These differ whenever $X$ and $Y$ share a common cause (confounder). If $Z \to X$ and $Z \to Y$, then conditioning on $X = x$ implicitly selects for certain values of $Z$, biasing the estimate of $Y$. Intervening on $X$ breaks the link $Z \to X$, removing this bias.

### 4.3 Graph Mutilation

Graphically, $\text{do}(X = x)$ corresponds to **mutilating** the causal graph: removing all edges **into** $X$ (severing $X$ from its causes) and setting $X = x$. All other edges remain.

**Before intervention:** $Z \to X \to Y$ with $Z \to Y$ (the confounder).
**After $\text{do}(X = x)$:** $X = x$ (constant), $Z \not\to X$ (edge removed), $X \to Y$ (unchanged), $Z \to Y$ (unchanged).

### 4.4 The Truncated Factorization Formula

The interventional distribution has a clean expression in terms of the pre-intervention model. For an SCM with causal graph $\mathcal{G}$ over variables $\mathbf{V} = \{V_1, \ldots, V_n\}$, and intervention $\text{do}(X = x)$ where $X \subseteq \mathbf{V}$:

$$
P(\mathbf{v} \setminus \mathbf{x} \mid \text{do}(X = x)) = \prod_{i : V_i \notin X} P(V_i \mid \text{Pa}(V_i)) \bigg|_{X = x}
$$

This is the product of the conditional distributions for all non-intervened variables, evaluated with $X$ fixed to $x$. The factors $P(X_i \mid \text{Pa}(X_i))$ for the intervened variables are **removed** (truncated) — because the intervention replaces their natural mechanism.

**Comparison with the observational distribution:**

$$
P(\mathbf{v}) = \prod_{i=1}^{n} P(V_i \mid \text{Pa}(V_i))
$$

The interventional distribution differs only in the omission of the factors for the intervened variables.

### 4.5 Worked Examples

**Example 1: Simple confounder.** Consider the graph $Z \to X \to Y$ with $Z \to Y$.

The observational distribution:
$$
P(Y \mid X = x) = \sum_z P(Y \mid X = x, Z = z) P(Z = z \mid X = x)
$$

Note that $P(Z \mid X)$ reflects the backward association from $X$ to $Z$ — knowing $X$ tells us about $Z$.

The interventional distribution:
$$
P(Y \mid \text{do}(X = x)) = \sum_z P(Y \mid X = x, Z = z) P(Z = z)
$$

The intervention replaces $P(Z \mid X)$ with $P(Z)$ — because intervening on $X$ destroys the information that $X$ carries about $Z$.

**Example 2: Mediator.** Consider the chain $X \to M \to Y$.

$$
P(Y \mid \text{do}(X = x)) = \sum_m P(Y \mid M = m) P(M = m \mid X = x)
$$

Here, the mechanism from $X$ to $M$ is preserved (we're not intervening on $M$), and the intervention simply fixes $X$. There is no confounding, so $P(Y \mid \text{do}(X = x)) = P(Y \mid X = x)$ — the observational and interventional distributions are identical.

---

## 5. The Three Rules of Do-Calculus

Pearl's **do-calculus** consists of three inference rules that allow interventional distributions to be transformed. Given a causal graph $\mathcal{G}$ over variables $\mathbf{V}$ with latent confounders, the rules specify when observations and interventions can be added, removed, or exchanged.

### 5.1 Notation

Let $X$, $Y$, $Z$, $W$ be disjoint sets of variables. Let $\mathcal{G}_{\overline{X}}$ denote the graph $\mathcal{G}$ with all incoming edges to $X$ removed ("overbar" = incoming edges deleted). Let $\mathcal{G}_{\underline{X}}$ denote the graph with all outgoing edges from $X$ removed ("underbar" = outgoing edges deleted). Let $\mathcal{G}_{\overline{X} \underline{Z}}$ denote both modifications simultaneously.

### 5.2 Rule 1: Insertion/Deletion of Observations

$$
P(Y \mid \text{do}(X), Z, W) = P(Y \mid \text{do}(X), W)
$$

if $(Y \perp\!\!\!\perp Z \mid X, W)_{\mathcal{G}_{\overline{X}}}$

**Interpretation:** If $Y$ and $Z$ are d-separated in the manipulated graph $\mathcal{G}_{\overline{X}}$ (where incoming edges to $X$ are removed) given $X$ and $W$, then observing $Z$ is irrelevant for predicting $Y$ under intervention $\text{do}(X)$. The observation $Z$ can be added to or removed from the conditioning set without changing the result.

This rule generalizes the standard probabilistic rule that conditionally independent variables can be dropped from conditioning.

### 5.3 Rule 2: Action/Observation Exchange

$$
P(Y \mid \text{do}(X), \text{do}(Z), W) = P(Y \mid \text{do}(X), Z, W)
$$

if $(Y \perp\!\!\!\perp Z \mid X, W)_{\mathcal{G}_{\overline{X} \underline{Z}}}$

**Interpretation:** If $Y$ and $Z$ are d-separated in $\mathcal{G}_{\overline{X} \underline{Z}}$ (incoming edges to $X$ removed, outgoing edges from $Z$ removed), then the intervention $\text{do}(Z)$ can be replaced by the observation $Z$ (or vice versa). This rule is crucial — it allows converting interventions into observations, which can then be estimated from data.

The condition involves checking d-separation in the graph where $Z$'s causal effects are severed ($\underline{Z}$). If, after severing $Z$'s outgoing edges, $Z$ carries no information about $Y$ beyond what $X$ and $W$ provide, then it does not matter whether $Z$ was forced or merely observed.

### 5.4 Rule 3: Insertion/Deletion of Actions

$$
P(Y \mid \text{do}(X), \text{do}(Z), W) = P(Y \mid \text{do}(X), W)
$$

if $(Y \perp\!\!\!\perp Z \mid X, W)_{\mathcal{G}_{\overline{X} \overline{Z(S)}}}$

where $Z(S)$ is the set of nodes in $Z$ that are not ancestors of any node in $W$ in $\mathcal{G}_{\overline{X}}$.

**Interpretation:** If intervening on $Z$ has no effect on $Y$ (given $X$ and $W$ and in the appropriately modified graph), the intervention $\text{do}(Z)$ can be removed entirely. This is the rule that eliminates unnecessary interventions from the expression.

### 5.5 Completeness Theorem

**Theorem (Huang and Valtorta, 2006; Shpitser and Pearl, 2006).** The three rules of do-calculus are **complete**: any interventional distribution that is identifiable from the causal graph and observational data can be identified using a finite sequence of applications of Rules 1–3.

This means that do-calculus is not merely a set of useful tricks — it captures the full power of causal identification from graphical models. If do-calculus cannot identify a causal effect, the effect is genuinely non-identifiable from observational data given the graph.

### 5.6 Proof Strategy

The proofs of the three rules follow a common pattern:

1. Express the interventional/observational distribution in terms of the structural equations and the truncated factorization.
2. Show that the d-separation condition in the modified graph implies that certain terms in the factorization cancel or simplify.
3. Conclude that the two sides of the equation are equal.

The completeness proof (Shpitser and Pearl, 2006) is more involved: it shows that for any non-identifiable causal effect, there exist two distinct SCMs with the same causal graph and the same observational distribution but different interventional distributions — making identification impossible.

---

## 6. The Adjustment Formula (Back-Door Criterion)

### 6.1 The Problem

Given a causal graph with a treatment $X$ and an outcome $Y$, we want to compute $P(Y \mid \text{do}(X))$ from observational data. The **back-door criterion** identifies a set of variables $\mathbf{Z}$ that, when adjusted for (conditioned on), removes all confounding bias.

### 6.2 Definition of the Back-Door Criterion

A set of variables $\mathbf{Z}$ satisfies the **back-door criterion** relative to $(X, Y)$ in a DAG $\mathcal{G}$ if:

1. No node in $\mathbf{Z}$ is a descendant of $X$.
2. $\mathbf{Z}$ blocks every path between $X$ and $Y$ that has an arrow into $X$ (a "back-door" path).

Condition 1 prevents adjusting for variables that are affected by the treatment (post-treatment variables), which could introduce bias. Condition 2 ensures that all confounding paths — paths that create a spurious association between $X$ and $Y$ — are blocked.

### 6.3 The Adjustment Formula

If $\mathbf{Z}$ satisfies the back-door criterion relative to $(X, Y)$, then:

$$
P(Y \mid \text{do}(X = x)) = \sum_z P(Y \mid X = x, Z = z) P(Z = z)
$$

For continuous variables:

$$
P(Y \mid \text{do}(X = x)) = \int P(Y \mid X = x, Z = z) P(Z = z) \, dz
$$

This is the **adjustment formula** (also called the **back-door adjustment**). It computes the causal effect by stratifying on the confounders $\mathbf{Z}$, computing the effect within each stratum, and averaging over the strata weighted by their marginal probability.

**Derivation from do-calculus:** The adjustment formula follows from Rules 2 and 3 of do-calculus, applied to the specific graphical structure where $\mathbf{Z}$ satisfies the back-door criterion.

### 6.4 Worked Example: Social Network and Health

Consider a study of whether social network centrality ($X$) affects health outcomes ($Y$). The causal graph includes:

- $\text{SES} \to X$ (socioeconomic status affects centrality)
- $\text{SES} \to Y$ (socioeconomic status affects health)
- $X \to Y$ (centrality affects health — the effect we want to estimate)

```
SES → X → Y
 \         ↑
  \ ------→
```

SES is a confounder — it creates a back-door path from $X$ to $Y$: $X \leftarrow \text{SES} \to Y$.

**Check back-door criterion for $\mathbf{Z} = \{\text{SES}\}$:**
1. SES is not a descendant of $X$. ✓
2. $\{$SES$\}$ blocks all back-door paths from $X$ to $Y$. The only back-door path is $X \leftarrow \text{SES} \to Y$, and SES is on this path. ✓

**Adjustment formula:**
$$
P(Y \mid \text{do}(X = x)) = \sum_{\text{ses}} P(Y \mid X = x, \text{SES} = \text{ses}) \cdot P(\text{SES} = \text{ses})
$$

This gives us the causal effect of centrality on health, unconfounded by socioeconomic status.

### 6.5 Graphical Criterion for Valid Adjustment Sets

Not all adjustment sets are equally good in practice. The **O-set** (optimal adjustment set; Henckel et al., 2022) minimizes the asymptotic variance of the adjusted estimator. Lutufi computes both valid adjustment sets and the optimal one.

The set of **all valid adjustment sets** for $(X, Y)$ in graph $\mathcal{G}$ can be characterized graphically:
- A set $\mathbf{Z}$ is a valid adjustment set if and only if it satisfies the back-door criterion.
- The **minimal** valid adjustment set is the smallest such set (fewest variables).
- The **maximal** valid adjustment set includes all non-descendants of $X$ that are ancestors of $X$, $Y$, or of any nodes on a back-door path.

### 6.6 When the Back-Door Criterion Fails

The back-door criterion requires that a valid set $\mathbf{Z}$ exists among the observed variables. It fails when:

- All back-door paths contain unobserved confounders. Example: $U \to X$, $U \to Y$ with $U$ latent. No observed variable blocks the back-door path $X \leftarrow U \to Y$.
- The only blocking sets include descendants of $X$ (which violate condition 1).

When the back-door criterion fails, the front-door criterion, instrumental variables, or the general ID algorithm may still identify the causal effect.

---

## 7. The Front-Door Criterion

### 7.1 Definition

A set of variables $\mathbf{M}$ satisfies the **front-door criterion** relative to $(X, Y)$ in a DAG $\mathcal{G}$ if:

1. $\mathbf{M}$ blocks all directed paths from $X$ to $Y$. ($\mathbf{M}$ completely mediates the effect of $X$ on $Y$.)
2. There is no unblocked back-door path from $X$ to $\mathbf{M}$.
3. All back-door paths from $\mathbf{M}$ to $Y$ are blocked by $X$.

### 7.2 The Front-Door Adjustment Formula

If $\mathbf{M}$ satisfies the front-door criterion:

$$
P(Y \mid \text{do}(X = x)) = \sum_m P(M = m \mid X = x) \sum_{x'} P(Y \mid M = m, X = x') P(X = x')
$$

This formula computes the causal effect through the mediator, without ever needing to observe the confounder.

### 7.3 Worked Example: Smoking, Tar, and Cancer

The classic example (Pearl, 2000): suppose smoking ($X$) causes cancer ($Y$) only through tar deposits in the lungs ($M$), and there is an unobserved genetic factor ($U$) that predisposes both smoking and cancer:

```
U (unobserved)
↓           ↓
X → M → Y
```

(Edges: $U \to X$, $U \to Y$, $X \to M$, $M \to Y$. $U$ is latent.)

The back-door criterion fails: the path $X \leftarrow U \to Y$ cannot be blocked because $U$ is unobserved.

**Check front-door criterion for $\mathbf{M} = \{M\}$:**
1. $M$ blocks all directed paths from $X$ to $Y$: the only directed path is $X \to M \to Y$, and $M$ is on it. ✓
2. No unblocked back-door path from $X$ to $M$: the only back-door path would go through $U$, but $U \to Y$ and $U$ is not a parent of $M$, so there is no back-door path from $X$ to $M$. ✓
3. All back-door paths from $M$ to $Y$ are blocked by $X$: the path $M \leftarrow X \leftarrow U \to Y$ is blocked by $X$. ✓

**Front-door formula:**
$$
P(Y \mid \text{do}(X = x)) = \sum_m P(M = m \mid X = x) \sum_{x'} P(Y \mid M = m, X = x') P(X = x')
$$

This gives us the causal effect of smoking on cancer through tar deposits, despite the unobserved genetic confounder.

### 7.4 When the Front-Door Criterion Applies

The front-door criterion is applicable in situations where:
- The causal effect is fully mediated through an observed set of mediators.
- There are no unblocked back-door paths from $X$ to the mediators.
- The treatment $X$ blocks all back-door paths from the mediators to the outcome.

These conditions are relatively restrictive, but the front-door criterion is invaluable when they hold — it enables identification in cases where no back-door set exists.

---

## 8. Instrumental Variables

### 8.1 Definition

An **instrumental variable (IV)** $Z$ for the effect of $X$ on $Y$ is a variable that satisfies:

1. **Relevance:** $Z$ affects $X$. ($Z$ is not independent of $X$.)
2. **Exclusion restriction:** $Z$ affects $Y$ only through $X$. (No direct effect of $Z$ on $Y$.)
3. **Independence:** $Z$ is independent of any confounders of $X$ and $Y$.

Graphically, in a causal graph with latent confounder $U$:

```
Z → X → Y
      ↑   ↑
      U ---→
```

$Z$ affects $X$ (arrow from $Z$ to $X$), $Z$ has no direct path to $Y$ except through $X$, and $Z$ is independent of $U$.

### 8.2 IV Estimation

Using an instrumental variable, the causal effect of $X$ on $Y$ can be identified even in the presence of unmeasured confounders. For linear models:

$$
\text{Causal effect of } X \text{ on } Y = \frac{\text{Cov}(Z, Y)}{\text{Cov}(Z, X)}
$$

This is the **Wald estimator**. For more complex settings, **two-stage least squares (2SLS)** is used:

**Stage 1:** Regress $X$ on $Z$ to get the predicted values $\hat{X}$.
$$
\hat{X} = \hat{\alpha}_0 + \hat{\alpha}_1 Z
$$

**Stage 2:** Regress $Y$ on $\hat{X}$.
$$
Y = \beta_0 + \beta_1 \hat{X} + \epsilon
$$

The coefficient $\beta_1$ is the estimated causal effect.

### 8.3 Connection to Econometric IV Estimation

The graphical/structural notion of instrumental variables unifies and clarifies the econometric tradition. Economists have used IV estimation since the 1920s (Wright, 1928 — the same Sewall Wright as §2.1) to handle endogeneity. The structural causal model framework provides:

- A **formal definition** of what makes a valid instrument (the graphical conditions).
- A **testable implication**: the exogeneity of $Z$ implies $Z \perp\!\!\!\perp Y \mid \text{do}(X)$, which can be partially tested.
- **Conditions for non-identification**: when no valid instrument exists, the causal effect is not identifiable by IV methods.

### 8.4 Challenges with Weak Instruments

When the instrument $Z$ has only a weak effect on $X$ (the "relevance" condition is barely satisfied), IV estimates become unreliable:

- Large standard errors.
- Bias toward the OLS estimate.
- Sensitivity to violations of the exclusion restriction.

The **first-stage F-statistic** is the standard diagnostic: $F > 10$ is the traditional threshold for a "strong" instrument (Staiger and Stock, 1997).

---

## 9. Identifiability

### 9.1 When Causal Effects Can Be Computed from Observational Data

A causal effect $P(Y \mid \text{do}(X = x))$ is **identifiable** from observational data in a causal model $\mathcal{M}$ if:

$$
P_{\mathcal{M}_1}(Y \mid \text{do}(X = x)) = P_{\mathcal{M}_2}(Y \mid \text{do}(X = x))
$$

for every pair of SCMs $\mathcal{M}_1, \mathcal{M}_2$ that share the same causal graph and the same observational distribution $P(\mathbf{V})$.

In other words, the causal effect is identifiable if it can be uniquely determined from the combination of the causal graph structure and the observational distribution — regardless of the specific parameter values.

### 9.2 The ID Algorithm (Shpitser and Pearl, 2006)

The **ID algorithm** is a complete algorithm for determining whether $P(\mathbf{Y} \mid \text{do}(\mathbf{X} = \mathbf{x}))$ is identifiable for arbitrary disjoint sets $\mathbf{X}$ and $\mathbf{Y}$ in a causal graph $\mathcal{G}$ with latent confounders (represented as bidirected edges).

**The algorithm (simplified):**

1. **If $\mathbf{X} = \emptyset$:** Return $\sum_{\mathbf{v} \setminus \mathbf{y}} P(\mathbf{v})$ (marginalize the observational distribution).

2. **If all ancestors of $\mathbf{Y}$ in $\mathcal{G}$ exclude some variables in $\mathbf{V}$:** Remove non-ancestral variables and recurse on the reduced graph.

3. **Compute the c-components (confounded components):** A c-component is a maximal set of nodes connected by bidirected edges. Each c-component represents a group of variables sharing latent common causes.

4. **If $\mathcal{G}$ has a single c-component containing all variables:** Attempt to decompose using the recursive structure. If $\mathbf{X}$ is not in any separating set, the effect may be non-identifiable.

5. **If $\mathcal{G}$ has multiple c-components:** Express $P(\mathbf{Y} \mid \text{do}(\mathbf{X}))$ as a sum-product of c-component factors, each of which is recursively identified.

6. **If a c-component of the treatment $\mathbf{X}$ is a subset of a c-component containing $\mathbf{Y}$:** Return FAIL (non-identifiable).

**Completeness:** If the ID algorithm returns FAIL, the causal effect is provably non-identifiable — there exist two SCMs with the same graph and observational distribution but different causal effects.

### 9.3 Non-Identification Results

When identification fails, it means that no amount of observational data (no matter how large) can determine the causal effect. The graph structure alone determines identifiability — it is a property of the model class, not the data.

**Example of non-identifiability:** The "bow arc" graph:

```
X → Y
↑     ↑
U ----→
```

($U$ is latent, causing both $X$ and $Y$ directly.) No back-door adjustment is possible (there is no observed confounder to condition on), no front-door criterion applies (no mediator), and no instrument is available. The ID algorithm correctly returns FAIL.

**Hedges.** A **hedge** (Shpitser and Pearl, 2006) is a pair of c-forests in the graph that provides a **witness** for non-identifiability. When the ID algorithm returns FAIL, the hedge structure can be extracted to explain *why* the effect is not identifiable and to characterize the range of possible causal effects consistent with the data.

### 9.4 Partial Identification

When full identification is impossible, **partial identification** provides bounds on the causal effect. For binary treatment and outcome, the **natural bounds** are:

$$
\max_z P(Y = 1, X = 1 \mid Z = z) - P(X = 0 \mid Z = z) \leq P(Y(1) = 1) \leq \min_z P(Y = 1 \mid Z = z) + P(X = 1 \mid Z = z)
$$

Tighter bounds can be obtained by combining multiple partial identification strategies (Manski, 1990; Balke and Pearl, 1997).

---

## 10. Counterfactuals

### 10.1 Formal Definition in the SCM Framework

A **counterfactual** statement has the form: "If $X$ had been $x$, $Y$ would have been $y$." In the SCM framework, counterfactuals are formalized using the **abduction-action-prediction** procedure.

Given an SCM $\mathcal{M} = (\mathbf{U}, \mathbf{V}, \mathcal{F}, P(\mathbf{U}))$ and observed evidence $\mathbf{E} = \mathbf{e}$, the counterfactual $Y_x$ (the value $Y$ would take if $X$ were set to $x$) is defined as:

$$
Y_x(\mathbf{u}) = Y_{\mathcal{M}_x}(\mathbf{u})
$$

— the value of $Y$ in the mutilated model $\mathcal{M}_x$, evaluated at the specific exogenous values $\mathbf{u}$.

The counterfactual distribution, conditional on evidence, is:

$$
P(Y_x = y \mid \mathbf{E} = \mathbf{e}) = \sum_{\mathbf{u}} P(\mathbf{u} \mid \mathbf{E} = \mathbf{e}) \cdot \mathbf{1}[Y_{\mathcal{M}_x}(\mathbf{u}) = y]
$$

### 10.2 The Three-Step Procedure

Computing counterfactuals follows three steps:

**Step 1 — Abduction.** Given the evidence $\mathbf{E} = \mathbf{e}$, compute the posterior distribution over the exogenous variables:

$$
P(\mathbf{U} \mid \mathbf{E} = \mathbf{e})
$$

This "abduces" the likely background conditions that gave rise to the observed evidence. For deterministic structural equations, this may pin down the exogenous values exactly; for stochastic equations, it provides a posterior distribution.

**Step 2 — Action.** Modify the model to reflect the hypothetical intervention $\text{do}(X = x)$:

$$
\mathcal{M} \to \mathcal{M}_x
$$

Replace the equation for $X$ with $X = x$, keeping all other equations and the updated (posterior) distribution over exogenous variables.

**Step 3 — Prediction.** In the modified model $\mathcal{M}_x$, compute the distribution of $Y$ using the posterior exogenous distribution:

$$
P(Y_x \mid \mathbf{E} = \mathbf{e}) = \sum_{\mathbf{u}} P(\mathbf{u} \mid \mathbf{E} = \mathbf{e}) \cdot \mathbf{1}[Y_{\mathcal{M}_x}(\mathbf{u}) = y]
$$

### 10.3 Probabilities of Causation

Counterfactuals enable defining and computing **probabilities of causation** — the probability that a specific cause was responsible for a specific effect:

**Probability of Necessity (PN):**
$$
PN = P(Y_0 = 0 \mid X = 1, Y = 1)
$$

"Given that the treatment was applied and the effect occurred, what is the probability that the effect would not have occurred without the treatment?" This is the question asked in legal liability: was the defendant's action a necessary cause of the harm?

**Probability of Sufficiency (PS):**
$$
PS = P(Y_1 = 1 \mid X = 0, Y = 0)
$$

"Given that the treatment was not applied and the effect did not occur, what is the probability that the effect would have occurred if the treatment had been applied?" This is the question asked in policy evaluation: would the intervention have been sufficient to produce the desired outcome?

**Probability of Necessity and Sufficiency (PNS):**
$$
PNS = P(Y_1 = 1, Y_0 = 0)
$$

The probability that the treatment was both necessary and sufficient for the effect.

**Bounds on PN and PS** (Tian and Pearl, 2000): Even when PN and PS are not point-identified, tight bounds can be derived from observational and experimental data:

$$
\max\left(0, \frac{P(Y = 1 \mid X = 1) - P(Y = 1 \mid X = 0)}{P(Y = 1 \mid X = 1)}\right) \leq PN \leq \min\left(1, \frac{P(Y = 0 \mid X = 0)}{P(Y = 1 \mid X = 1)}\right)
$$

---

## 11. Mediation Analysis

### 11.1 Direct and Indirect Effects

In many causal analyses, we want to decompose the total effect of $X$ on $Y$ into a **direct effect** (not through any mediator) and an **indirect effect** (through a mediator $M$):

$$
X \to M \to Y \quad \text{(indirect)}
$$
$$
X \to Y \quad \text{(direct)}
$$

### 11.2 Controlled Direct Effect (CDE)

The **Controlled Direct Effect** of $X$ on $Y$, controlling $M$ at value $m$:

$$
CDE(m) = P(Y \mid \text{do}(X = 1), \text{do}(M = m)) - P(Y \mid \text{do}(X = 0), \text{do}(M = m))
$$

This measures the effect of $X$ on $Y$ when $M$ is forcibly held at value $m$. It requires intervening on both $X$ and $M$.

### 11.3 Natural Direct and Indirect Effects

**Natural Direct Effect (NDE):**

$$
NDE = P(Y_{1, M_0}) - P(Y_{0, M_0})
$$

The effect of changing $X$ from 0 to 1, while keeping $M$ at the value it would **naturally** take if $X$ were 0. This isolates the direct pathway $X \to Y$.

**Natural Indirect Effect (NIE):**

$$
NIE = P(Y_{1, M_1}) - P(Y_{1, M_0})
$$

The effect of changing $M$ from its natural value under $X = 0$ to its natural value under $X = 1$, while holding $X$ at 1. This isolates the indirect pathway $X \to M \to Y$.

**Total effect decomposition:**

$$
TE = NDE + NIE \quad \text{(on the risk difference scale, for binary outcomes)}
$$

$$
TE = NDE \times NIE \quad \text{(on the ratio scale)}
$$

### 11.4 Identification of Natural Effects

Natural direct and indirect effects are **counterfactual** quantities — they involve the simultaneous consideration of two different treatment levels (the treatment $X$ is set to one value while the mediator is at the level it would take under a different treatment value). This makes them harder to identify than simple interventional effects.

**Sufficient conditions for identification (Pearl, 2001; Robins and Greenland, 1992):**

The NDE and NIE are identifiable if:
1. There is no confounding of the $X \to Y$ relationship (possibly conditional on covariates).
2. There is no confounding of the $M \to Y$ relationship (possibly conditional on $X$ and covariates).
3. There is no confounding of the $X \to M$ relationship.
4. There is no mediator-outcome confounder that is affected by $X$.

Under these conditions (the "sequential ignorability" assumption of Imai et al., 2010):

$$
NDE = \sum_m \left[ P(Y \mid X = 1, M = m) - P(Y \mid X = 0, M = m) \right] P(M = m \mid X = 0)
$$

### 11.5 Path-Specific Effects

When there are multiple mediating pathways ($X \to M_1 \to Y$, $X \to M_2 \to Y$, $X \to Y$), **path-specific effects** decompose the total effect along each pathway. The effect along a specific set of paths $\pi$ is:

$$
\text{PSE}_\pi = P(Y_{x, M_{\pi}(x), M_{\bar{\pi}}(x')}) - P(Y_{x', M_{\pi}(x'), M_{\bar{\pi}}(x')})
$$

where $M_\pi$ are mediators on pathways in $\pi$ and $M_{\bar{\pi}}$ are mediators on the remaining pathways. Path-specific effects are generally not identifiable from observational data without strong assumptions.

### 11.6 Worked Example: Network Centrality and Economic Outcomes

Consider the causal model:

$$
\text{Education} (X) \to \text{Network Centrality} (M) \to \text{Income} (Y)
$$
$$
\text{Education} (X) \to \text{Income} (Y)
$$

The total effect of education on income operates through two pathways:
1. **Direct:** More education directly increases income (through human capital).
2. **Indirect through network centrality:** More education increases network centrality (through professional contacts), which in turn increases income (through job referrals, information access).

Using the NDE/NIE decomposition:
- NDE: The income gain from education holding network position fixed — pure human capital effect.
- NIE: The income gain from the network improvement caused by education — the "social capital" effect.

This decomposition is directly relevant to policy: if the indirect effect is large, interventions that improve network position (mentorship programs, professional networking) may be as effective as education itself.

---

## 12. Causal Discovery

### 12.1 Learning Causal Structure from Data vs. Imposing It

In many applications, the causal graph is not known a priori. **Causal discovery** (also called **causal structure learning**) aims to learn the causal graph from data, possibly combined with background knowledge.

Two fundamentally different approaches exist:

**Constraint-based methods:** Identify the causal graph by testing conditional independence relationships in the data and finding a graph consistent with those relationships.

**Score-based methods:** Define a score function that measures how well a causal graph explains the data, penalized by complexity, and search for the highest-scoring graph.

**Functional methods:** Exploit asymmetries in the data-generating process (e.g., additive noise models) to determine causal direction.

### 12.2 The PC Algorithm for Causal Structure

The **PC algorithm** (Spirtes and Glymour, 1991; Spirtes, Glymour, and Scheines, 1993/2000) is the foundational constraint-based algorithm for causal discovery:

**Phase 1 — Skeleton discovery:**
1. Start with a complete undirected graph.
2. For each pair $(X_i, X_j)$, test $X_i \perp\!\!\!\perp X_j \mid \mathbf{S}$ for conditioning sets $\mathbf{S}$ of increasing size, drawn from the adjacencies of $X_i$ or $X_j$.
3. If a separating set $\mathbf{S}$ is found (the CI test does not reject independence at level $\alpha$), remove the edge between $X_i$ and $X_j$ and record $\mathbf{S}$ as the separating set.

**Phase 2 — V-structure identification:**
For each triple $X_i - X_k - X_j$ where $X_i$ and $X_j$ are non-adjacent:
- If $X_k$ is NOT in the separating set of $X_i$ and $X_j$, orient as $X_i \to X_k \leftarrow X_j$ (v-structure/collider).

**Phase 3 — Orientation propagation (Meek's rules):**
Apply four rules that orient additional edges without creating new v-structures or directed cycles:
- **R1:** If $X_i \to X_k - X_j$ and $X_i$ is non-adjacent to $X_j$, orient as $X_k \to X_j$.
- **R2:** If $X_i \to X_k \to X_j$ and $X_i - X_j$, orient as $X_i \to X_j$.
- **R3:** If $X_i - X_k \to X_j$, $X_i - X_l \to X_j$, and $X_k$ is non-adjacent to $X_l$, orient as $X_i \to X_j$.
- **R4:** If $X_i - X_k \to X_l \to X_j$ and $X_i - X_j$, orient as $X_i \to X_j$.

**Output:** A **CPDAG** (Completed Partially Directed Acyclic Graph) representing the Markov equivalence class — the set of all DAGs that encode the same conditional independence relationships.

**Correctness:** Under the faithfulness and Markov assumptions, the PC algorithm recovers the correct CPDAG in the large-sample limit.

**Complexity:** $O(n^{d+2})$ in the worst case, where $n$ is the number of variables and $d$ is the maximum degree. For sparse graphs ($d \ll n$), this is polynomial.

### 12.3 FCI for Latent Variables

The **Fast Causal Inference (FCI)** algorithm (Spirtes, Meek, and Richardson, 1999; updated by Zhang, 2008) extends PC to handle **latent confounders** and **selection bias** — two critical issues in social network data where many variables are unobserved.

FCI outputs a **Partial Ancestral Graph (PAG)** using four edge types:
- $X \to Y$: $X$ is a definite cause of $Y$.
- $X \leftrightarrow Y$: There is a latent common cause of $X$ and $Y$.
- $X \circ\!\!\!\to Y$: Either $X \to Y$ or $X \leftrightarrow Y$ (the algorithm cannot distinguish).
- $X \circ\!\!\!-\!\!\!\circ Y$: The relationship is undetermined.

FCI is essential for social network analysis because many variables (personality traits, cultural background, private information) are latent confounders that affect both network formation and behavior.

### 12.4 Additive Noise Models

**Additive Noise Models (ANMs)** (Hoyer et al., 2009; Peters et al., 2014) exploit the asymmetry of the causal mechanism to determine causal direction between two variables.

If $Y = f(X) + N$ where $N \perp\!\!\!\perp X$ (the noise is independent of the cause), then under certain conditions (e.g., $f$ is nonlinear and $N$ is non-Gaussian), the reverse model $X = g(Y) + N'$ with $N' \perp\!\!\!\perp Y$ does not hold. This asymmetry identifies the causal direction.

**For Gaussian variables with linear relationships:** The direction is not identifiable (the model is symmetric). This is the well-known limitation of applying Gaussianity and linearity together — the world looks the same in both causal directions.

**LiNGAM (Linear Non-Gaussian Acyclic Model)** (Shimizu et al., 2006): If the causal relationships are linear but the noise terms are non-Gaussian, the causal direction and the full DAG are identifiable from observational data alone. This uses Independent Component Analysis (ICA) to recover the structural equations.

---

## 13. Potential Outcomes Framework

### 13.1 Rubin's Notation

In the **potential outcomes** framework (Neyman, 1923; Rubin, 1974), each unit $i$ has a set of potential outcomes $\{Y_i(t)\}_{t \in \mathcal{T}}$, one for each treatment level $t$. The observed outcome for unit $i$ assigned to treatment $T_i = t$ is:

$$
Y_i^{\text{obs}} = Y_i(T_i)
$$

This is the **consistency** or **SUTVA** assumption: the observed outcome equals the potential outcome corresponding to the assigned treatment.

### 13.2 Causal Estimands

**Average Treatment Effect (ATE):**
$$
\tau_{\text{ATE}} = E[Y(1) - Y(0)] = E[Y(1)] - E[Y(0)]
$$

The expected difference in outcomes between treatment ($t = 1$) and control ($t = 0$) across the entire population.

**Average Treatment Effect on the Treated (ATT):**
$$
\tau_{\text{ATT}} = E[Y(1) - Y(0) \mid T = 1]
$$

The expected treatment effect among those who actually received treatment.

**Average Treatment Effect on the Untreated (ATU):**
$$
\tau_{\text{ATU}} = E[Y(1) - Y(0) \mid T = 0]
$$

**Conditional Average Treatment Effect (CATE):**
$$
\tau(x) = E[Y(1) - Y(0) \mid X = x]
$$

The treatment effect as a function of covariates $X$ — allowing for treatment effect heterogeneity.

### 13.3 SUTVA and Its Importance

The **Stable Unit Treatment Value Assumption (SUTVA)** states:

1. **No interference:** Unit $i$'s potential outcome $Y_i(t)$ depends only on $i$'s own treatment $t$, not on any other unit's treatment: $Y_i(t_1, \ldots, t_n) = Y_i(t_i)$.

2. **No hidden versions of treatment:** There is only one version of each treatment level. Two units assigned to $t = 1$ receive the same treatment.

SUTVA is the foundation of the potential outcomes framework, but it is **systematically violated** in network settings (see Section 14).

### 13.4 Connection to the SCM Framework

Pearl (2000, 2009) showed that the potential outcomes framework is a special case of the SCM framework:

**Translation from SCM to potential outcomes:** The potential outcome $Y_i(x)$ in the potential outcomes framework corresponds to the counterfactual $Y_{X=x}(u_i)$ in the SCM framework — the value of $Y$ in the mutilated model $\mathcal{M}_x$ for unit $i$ with exogenous values $u_i$.

**Translation from potential outcomes to SCM:** Any potential outcome model can be expressed as an SCM by defining structural equations that map treatments and exogenous variables to outcomes.

The SCM framework is strictly more expressive:
- It can express conditional independencies between potential outcomes under different treatments.
- It provides a graphical language for determining identifiability.
- It supports mediation analysis, path-specific effects, and other decompositions that are cumbersome in the potential outcomes notation.

The potential outcomes framework's strengths:
- Cleaner notation for simple treatment-control comparisons.
- Natural connection to experimental design (randomized controlled trials).
- Emphasis on missing data interpretation (the fundamental problem of causal inference).

### 13.5 Ignorability and Its Connection to the Back-Door Criterion

**Strong ignorability** (Rosenbaum and Rubin, 1983): Treatment assignment is strongly ignorable given covariates $\mathbf{Z}$ if:

$$
Y(0), Y(1) \perp\!\!\!\perp T \mid \mathbf{Z}
$$

and $0 < P(T = 1 \mid \mathbf{Z} = z) < 1$ for all $z$ (the overlap condition).

Pearl (2009) showed that strong ignorability is equivalent to the back-door criterion: $\mathbf{Z}$ satisfies the back-door criterion relative to $(T, Y)$ if and only if treatment is ignorable given $\mathbf{Z}$ (for distributions that are Markov and faithful with respect to the graph).

---

## 14. Causation in Social Networks

Social networks present unique challenges for causal inference that go beyond standard settings. The core issues are network interference, the conflation of homophily and influence, and the reflection problem.

### 14.1 Network Interference (SUTVA Violations)

In social networks, SUTVA is violated by design: one person's treatment affects another's outcome. If person $A$ is vaccinated, person $B$ (who is connected to $A$) benefits through reduced exposure — even if $B$ is not vaccinated.

**Formal expression.** Let $\mathbf{T} = (T_1, \ldots, T_n)$ be the treatment vector for all $n$ individuals. Under network interference:

$$
Y_i = Y_i(\mathbf{T}) \neq Y_i(T_i)
$$

Person $i$'s outcome depends on the entire treatment vector, not just their own treatment.

**Exposure mapping.** To make the problem tractable, many approaches assume that $i$'s outcome depends on a summary of others' treatments mediated by the network:

$$
Y_i(\mathbf{T}) = Y_i(T_i, g_i(\mathbf{T}_{-i}))
$$

where $g_i$ is an **exposure mapping** — a function that summarizes the treatment exposure from $i$'s network neighborhood. Common choices:
- $g_i = \sum_{j \in \text{Nb}(i)} T_j$ (total treated neighbors)
- $g_i = \frac{\sum_{j \in \text{Nb}(i)} T_j}{|\text{Nb}(i)|}$ (proportion of treated neighbors)

**Causal effects under interference.** New estimands replace the standard ATE:
- **Direct effect:** The effect of treating $i$ while holding neighbors' treatments fixed.
- **Indirect (spillover) effect:** The effect of treating $i$'s neighbors while holding $i$'s treatment fixed.
- **Total effect:** The combined direct and indirect effect.
- **Overall effect:** The effect of a policy that changes everyone's treatment simultaneously.

### 14.2 Peer Effects vs. Homophily

The most vexed problem in social network causal inference is distinguishing **peer effects** (genuine causal influence between connected individuals) from **homophily** (the tendency for similar individuals to form connections).

**Peer effects (contagion):** Person $A$'s behavior causally affects person $B$'s behavior through their social tie. $A$ adopts a new technology, which causes $B$ to adopt it.

**Homophily (selection):** Persons $A$ and $B$ form a tie because they are similar. Their similar behavior (both adopting the technology) is due to their shared characteristics, not mutual influence.

These produce the same observational signature — correlated behaviors among connected individuals — but have completely different policy implications. If peer effects dominate, targeting influential individuals (network interventions) is effective. If homophily dominates, network interventions are ineffective; individual-level interventions are needed.

**Identification strategies:**
- **Temporal data:** If $A$ adopts before $B$, this provides evidence for influence from $A$ to $B$ (though temporal precedence is not sufficient for causation).
- **Natural experiments:** Exogenous shocks to individuals (e.g., random assignment to dormitories in Sacerdote, 2001) provide instruments for network position.
- **Structural models:** Specifying a full structural causal model of network formation and behavior allows formal identification analysis (Snijders, 2001; SAOMs).

### 14.3 Contagion vs. Confounding

Christakis and Fowler's influential studies (2007, 2008) on the "spread" of obesity, smoking, and happiness through social networks sparked intense debate. Shalizi and Thomas (2011) showed that homophily and latent confounding are generically indistinguishable from contagion in observational network data, absent strong structural assumptions.

**The fundamental challenge:** In the causal graph:

- **Contagion:** $Y_A^{(t)} \to Y_B^{(t+1)}$ (A's behavior at time $t$ causes B's behavior at time $t+1$).
- **Homophily:** $H \to Y_A$, $H \to Y_B$, $H \to N_{AB}$ (shared latent trait $H$ causes both behaviors and the network tie).
- **Common environment:** $E \to Y_A$, $E \to Y_B$ (shared environmental exposure causes both behaviors).

These three mechanisms produce observationally equivalent patterns: correlated behaviors among connected individuals.

### 14.4 The Manski Reflection Problem

Manski's (1993) reflection problem (introduced in the CONDITIONAL_INDEPENDENCE document) has deep implications for causal identification in networks. In the linear-in-means model:

$$
Y_i = \alpha + \beta \bar{Y}_{-i} + \gamma \mathbf{X}_i + \delta \bar{\mathbf{X}}_{-i} + \epsilon_i
$$

where $\bar{Y}_{-i}$ and $\bar{\mathbf{X}}_{-i}$ are peer group averages, $\beta$ (endogenous effect), $\gamma$ (direct effect of own characteristics), and $\delta$ (exogenous or contextual effect) are not separately identifiable without restrictions. This is because the simultaneity of $Y_i$ and $\bar{Y}_{-i}$ creates a perfect reflection — the system's equilibrium is consistent with any combination of $\beta$ and $\delta$ that produces the same reduced-form relationship.

**Solutions:**
- **Network structure variation** (Bramoullé et al., 2009): In a network (as opposed to a group), the intransitivity of connections provides instruments. Friends-of-friends who are not friends provide exogenous variation in peer exposure.
- **Temporal variation:** Dynamic models where influence propagates with a lag.
- **Exogenous interventions:** Randomly assigned treatments that affect network peers.

---

## 15. Causation in Economic Networks

### 15.1 Policy Evaluation

Economic networks — interbank lending, supply chains, trade networks — are targets of policy interventions. Causal reasoning determines whether a proposed policy will have the intended effect.

**Example: Bank bailout.** If Bank $A$ is failing, will bailing it out prevent contagion to Bank $B$? This requires estimating:

$$
P(\text{B fails} \mid \text{do}(\text{A rescued})) \quad \text{vs.} \quad P(\text{B fails} \mid \text{do}(\text{A not rescued}))
$$

The causal graph must include: interbank exposures (who lends to whom), common exposures (shared portfolio risks), and the contagion mechanism (through direct defaults or through market confidence).

### 15.2 Intervention Analysis

**Central bank interventions.** When a central bank intervenes in the financial system (e.g., quantitative easing, interest rate changes, emergency lending facilities), the effects propagate through the economic network. Causal analysis asks:

- What is the **direct effect** of the intervention on the targeted institution?
- What is the **spillover effect** on other institutions through the network?
- What are the **unintended consequences** (e.g., moral hazard) of the intervention?

These correspond to direct, indirect, and path-specific causal effects in the SCM framework.

### 15.3 Sanctions Modeling

International economic sanctions operate through network disruption. When country $A$ sanctions country $B$:

- **Direct effect:** Trade between $A$ and $B$ decreases.
- **Indirect effect:** Countries connected to both $A$ and $B$ may reroute trade, partially compensating for the sanction.
- **Third-party effects:** Countries that depend on $B$ for critical imports are indirectly affected.

The causal model for sanctions must represent the entire trade network, including substitution effects and strategic responses. Lutufi's network-native causal reasoning is designed for exactly this type of analysis.

### 15.4 Regulatory Impact Assessment

Financial regulations (capital requirements, liquidity ratios, transaction taxes) have network-mediated effects:

- **Direct effect:** The regulated institution changes its behavior.
- **Competitive effect:** Other institutions adjust their behavior in response.
- **Network effect:** Changes in one institution's risk profile propagate through the interconnected system.

Standard regulatory impact assessments often ignore the network effects, leading to underestimates of both the costs and benefits of regulation. Lutufi's causal framework provides a principled approach to incorporating these network-mediated effects.

---

## 16. How Lutufi Implements Causal Reasoning

### 16.1 SCM Representation

Lutufi provides a first-class representation for structural causal models:

- **Structural equations** are specified as functional relationships between variables. Lutufi supports linear equations ($V_i = \sum_j \beta_{ij} V_j + U_i$), nonlinear equations (specified as arbitrary functions), and discrete mechanisms (specified as conditional probability tables under the causal interpretation).

- **The causal graph** is maintained as a DAG with possible bidirected edges (representing latent confounders). The graph is stored separately from the structural equations, allowing the same graph to have different parameterizations.

- **Exogenous variables** have user-specified distributions. By default, exogenous variables are assumed independent (no latent confounders); bidirected edges must be explicitly added.

- **Causal vs. statistical designation.** Every model in Lutufi carries a flag indicating whether it is a **causal model** (edges represent mechanisms) or a **statistical model** (edges represent statistical dependencies). Causal operations (do-calculus, counterfactuals) are only available on causal models. This prevents users from accidentally making causal claims from purely associational analyses.

### 16.2 Do-Calculus Engine

Lutufi implements the do-calculus as a **symbolic reasoning engine** that applies the three rules iteratively:

**Input:** A causal query $P(Y \mid \text{do}(X), W)$ and a causal graph $\mathcal{G}$.

**Process:**
1. Parse the query into a normal form.
2. Apply Rules 1–3 in a systematic search, attempting to reduce all $\text{do}()$ operators to conditioning (observational) operations.
3. At each step, check the graphical conditions for each rule using d-separation algorithms on the appropriate modified graphs ($\mathcal{G}_{\overline{X}}$, $\mathcal{G}_{\underline{X}}$, etc.).
4. If all $\text{do}()$ operators are eliminated, the result is an observational expression — a formula computable from data.
5. If the search exhausts all possible rule applications without eliminating all $\text{do}()$ operators, invoke the ID algorithm for a definitive answer.

**Output:** Either an **identification formula** (an expression in terms of observational distributions) or a **non-identification certificate** (a proof that the query is not identifiable, with the hedge structure explaining why).

### 16.3 Identifiability Checker

Lutufi implements the **ID algorithm** (Shpitser and Pearl, 2006) and its extensions:

- **`model.is_identifiable(Y, do_X)`** — Checks whether $P(Y \mid \text{do}(X))$ is identifiable given the causal graph.
- **`model.identify(Y, do_X)`** — Returns the identification formula if the effect is identifiable, or a non-identifiability certificate otherwise.
- **`model.find_adjustment_set(X, Y)`** — Searches for a valid back-door adjustment set and returns it if one exists. Also returns the optimal adjustment set (minimizing asymptotic variance).
- **`model.find_instrument(X, Y)`** — Searches for valid instrumental variables.
- **`model.bounds(Y, do_X)`** — When the effect is not point-identifiable, returns bounds using partial identification methods.

### 16.4 Counterfactual Engine

For computing counterfactuals, Lutufi implements the three-step abduction-action-prediction procedure:

1. **Abduction:** Given observed evidence, compute the posterior distribution over exogenous variables using Bayesian inference (exact for small models, approximate for large models).
2. **Action:** Modify the structural equations to reflect the hypothetical intervention.
3. **Prediction:** Compute the distribution of the outcome in the modified model using the posterior exogenous distribution.

For linear Gaussian models, all three steps have closed-form solutions. For general models, Lutufi uses MCMC sampling over the exogenous variables.

### 16.5 Causal Discovery Module

Lutufi provides causal discovery algorithms adapted to the network setting:

- **PC algorithm** for structure learning from observational data.
- **FCI algorithm** for structure learning with latent confounders.
- **LiNGAM** for causal direction identification under non-Gaussianity.
- **Network-adapted variants** that incorporate network structure as prior knowledge (e.g., connected nodes are more likely to have a causal relationship than disconnected ones).

### 16.6 Integration with Network Analysis

Lutufi's unique contribution is the integration of causal reasoning with network analysis:

- **Network-mediated causal effects:** Compute causal effects that propagate through the network, respecting the network structure.
- **Network interference adjustment:** Account for SUTVA violations using exposure mappings derived from the network topology.
- **Peer effect estimation:** Identify and estimate peer effects using network-based instrumental variables (Bramoullé et al., 2009).
- **Causal centrality:** Define and compute a "causal centrality" measure — how much intervening on a single node affects the global state of the network. This combines causal inference (the do-operator) with network science (centrality measures).

---

## 17. Key References

1. **Pearl, J.** (2009). *Causality: Models, Reasoning, and Inference* (2nd ed.). Cambridge University Press. — The foundational text on structural causal models, do-calculus, and the formal theory of causal inference. Essential reading.

2. **Pearl, J.** (2000). *Causality: Models, Reasoning, and Inference* (1st ed.). Cambridge University Press. — The original edition that launched the causal revolution.

3. **Pearl, J., Glymour, M. & Jewell, N. P.** (2016). *Causal Inference in Statistics: A Primer*. Wiley. — An accessible introduction to causal inference from the SCM perspective, with worked examples and exercises.

4. **Spirtes, P., Glymour, C. & Scheines, R.** (2000). *Causation, Prediction, and Search* (2nd ed.). MIT Press. — Introduced the PC and FCI algorithms for causal structure learning. The constraint-based complement to Pearl's work.

5. **Rubin, D. B.** (1974). "Estimating Causal Effects of Treatments in Randomized and Nonrandomized Studies." *Journal of Educational Psychology*, 66(5), 688–701. — The foundational paper for the potential outcomes framework.

6. **Shpitser, I. & Pearl, J.** (2006). "Identification of Joint Interventional Distributions in Recursive Semi-Markovian Causal Models." In *Proceedings of AAAI 2006*, 1219–1226. — The ID algorithm for complete identification of causal effects with latent confounders.

7. **Huang, Y. & Valtorta, M.** (2006). "Pearl's Calculus of Intervention Is Complete." In *Proceedings of UAI 2006*, 217–224. — Proved the completeness of do-calculus.

8. **Robins, J. M.** (1986). "A New Approach to Causal Inference in Mortality Studies with a Sustained Exposure Period — Application to Control of the Healthy Worker Survivor Effect." *Mathematical Modelling*, 7(9–12), 1393–1512. — Introduced the G-computation formula and the foundations of modern causal epidemiology.

9. **Imbens, G. W. & Rubin, D. B.** (2015). *Causal Inference for Statistics, Social, and Biomedical Sciences: An Introduction*. Cambridge University Press. — The comprehensive textbook from the potential outcomes perspective.

10. **Tian, J. & Pearl, J.** (2002). "On the Identification of Causal Effects." Technical Report R-290-L, UCLA Cognitive Systems Laboratory. — Extended the ID algorithm and provided a complete graphical characterization of identifiability.

11. **Manski, C. F.** (1993). "Identification of Endogenous Social Effects: The Reflection Problem." *Review of Economic Studies*, 60(3), 531–542. — The fundamental challenge of separating social influence from confounding in network data.

12. **Bramoullé, Y., Djebbari, H. & Fortin, B.** (2009). "Identification of Peer Effects through Social Networks." *Journal of Econometrics*, 150(1), 41–55. — Showed how network structure provides instruments for identifying peer effects.

13. **Shalizi, C. R. & Thomas, A. C.** (2011). "Homophily and Contagion Are Generically Confounded in Observational Social Network Studies." *Sociological Methods & Research*, 40(2), 211–239. — Proved that contagion and homophily are not generally distinguishable from observational data.

14. **Shimizu, S., Hoyer, P. O., Hyvärinen, A. & Kerminen, A.** (2006). "A Linear Non-Gaussian Acyclic Model for Causal Discovery." *Journal of Machine Learning Research*, 7, 2003–2030. — Introduced LiNGAM for causal discovery under non-Gaussianity.

15. **Peters, J., Janzing, D. & Schölkopf, B.** (2017). *Elements of Causal Inference: Foundations and Learning Algorithms*. MIT Press. — Modern treatment of causal discovery from a machine learning perspective, including additive noise models.

16. **Imai, K., Keele, L. & Yamamoto, T.** (2010). "Identification, Inference and Sensitivity Analysis for Causal Mediation Effects." *Statistical Science*, 25(1), 51–71. — Formalized mediation analysis in the potential outcomes framework with the sequential ignorability assumption.

17. **Bareinboim, E. & Pearl, J.** (2016). "Causal Inference and the Data-Fusion Problem." *Proceedings of the National Academy of Sciences*, 113(27), 7345–7352. — Extended causal identification to settings where data comes from multiple heterogeneous sources.

---

*"We do not see the world as it is; we see it as our models allow. The causal model is the lens through which correlation becomes explanation, prediction becomes intervention, and data becomes wisdom."*

---

**End of Document — Structural Causal Models and Do-Calculus v1.0**
