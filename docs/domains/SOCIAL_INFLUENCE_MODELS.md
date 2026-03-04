# Social Influence Models

---

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Types of Social Influence](#2-types-of-social-influence)
3. [Simple Contagion: The SIR Analogy](#3-simple-contagion-the-sir-analogy)
4. [Complex Contagion](#4-complex-contagion)
5. [Granovetter's Threshold Model](#5-granovetters-threshold-model)
6. [The Linear Threshold Model](#6-the-linear-threshold-model)
7. [The Independent Cascade Model](#7-the-independent-cascade-model)
8. [Generalized Threshold Models](#8-generalized-threshold-models)
9. [Influence Maximization](#9-influence-maximization)
10. [Competitive Influence](#10-competitive-influence)
11. [Temporal Aspects of Influence](#11-temporal-aspects-of-influence)
12. [Network Structure Effects](#12-network-structure-effects)
13. [Empirical Studies](#13-empirical-studies)
14. [Applications](#14-applications)
15. [How Lutufi Implements Social Influence](#15-how-lutufi-implements-social-influence)
16. [Key References](#16-key-references)

---

## 1. Introduction

Social influence—the process by which individuals adjust their behavior, attitudes, or beliefs to align with those of others—is one of the fundamental forces shaping human societies. From the adoption of agricultural innovations to the spread of political movements, from the diffusion of linguistic features to the propagation of health behaviors, social influence underlies the collective dynamics that transform individual actions into large-scale social phenomena.

### 1.1 What Is Social Influence?

At its core, social influence involves a change in an individual's cognitions, attitudes, or behaviors brought about by the real, imagined, or implied presence of others. This definition, rooted in social psychology, captures several essential features:

- **Interdependence:** The individual's state depends on the states of others in their social environment.
- **Transmission:** Something (information, pressure, emotional contagion) flows through social ties.
- **Change:** Influence manifests as a deviation from what the individual would have done in isolation.

Social influence operates across multiple timescales—from the immediate mimicry of gestures in conversation to the gradual adoption of cultural norms over generations—and across multiple domains, including consumer behavior, political participation, health practices, and technological adoption.

### 1.2 Why Social Influence Matters

Understanding social influence is crucial for several reasons:

**Scientific:** Social influence processes illuminate how micro-level individual interactions generate macro-level collective outcomes. The emergence of consensus, polarization, cascades, and cultural evolution all depend on the mechanisms of social influence.

**Practical:** Organizations, policymakers, and marketers seek to understand influence to design effective interventions. Public health campaigns aim to promote beneficial behaviors; political strategists seek to mobilize voters; technology companies optimize viral growth.

**Ethical:** The same mechanisms that enable prosocial outcomes (charitable giving, public health compliance) can be exploited for manipulation and control. Understanding influence is prerequisite to both harnessing its benefits and guarding against its misuse.

### 1.3 Historical Context

#### 1.3.1 French's Formal Theory of Social Power

The mathematical study of social influence begins with **John R. P. French Jr.** (1956), who developed a formal theory of social power using graph-theoretic concepts. French represented a social group as a directed graph, where nodes are individuals and edges represent interpersonal influences. Each individual holds an initial position on some issue, and the group's power structure determines how these positions change through influence processes.

French distinguished several bases of power: reward power, coercive power, legitimate power, referent power, and expert power. Crucially, he showed that power depends not just on individual attributes but on position within the network. An individual who is central in the influence network can shape the group's final opinion even if they lack other forms of power.

#### 1.3.2 Katz and Lazarsfeld's Two-Step Flow

**Paul Lazarsfeld** and **Elihu Katz** (1955), in their study of voting behavior during the 1940 U.S. presidential election, discovered that mass media influence was mediated by interpersonal communication. Their "two-step flow of communication" model proposed that:

1. Information flows from mass media to "opinion leaders"—individuals who are particularly attentive to media and influential within their social circles.
2. Opinion leaders then transmit and interpret this information for their followers through personal influence.

This insight shifted the study of influence from a focus on direct media effects to an appreciation of the mediating role of social networks. It established that influence flows through network ties, not just through individual exposure to information sources.

#### 1.3.3 The Structural Revolution

The 1970s and 1980s saw a structural turn in influence research, led by **Mark Granovetter**, **Harrison White**, and others who emphasized that network structure itself shapes influence dynamics. Granovetter's (1973) "strength of weak ties" argument showed that novel information flows through weak acquaintanceship ties that bridge otherwise disconnected social circles. This structural perspective laid the groundwork for modern network-based models of influence.

### 1.4 The Network Perspective

The modern study of social influence treats influence as a process that unfolds on networks. Key insights from this perspective include:

- **Local interactions generate global patterns:** Macro-level adoption curves, cascade sizes, and opinion distributions emerge from micro-level influence processes between connected individuals.
- **Network structure matters profoundly:** The same influence mechanism can produce dramatically different outcomes depending on network topology. Clustered networks favor complex contagions; random networks favor simple contagions.
- **Position predicts influence:** Centrality, brokerage, and community membership determine who influences whom and how quickly influence spreads.
- **Dynamics are path-dependent:** Early adoptions and random fluctuations can send the system toward very different equilibria.

This document develops these insights through formal models, beginning with simple contagion and progressing through threshold models, cascade dynamics, influence maximization, and competitive influence. Throughout, we emphasize the mathematical foundations, empirical applications, and computational methods that enable practical analysis.

---

## 2. Types of Social Influence

Social psychologists have identified distinct mechanisms through which others affect individual behavior. These mechanisms have different informational requirements, different motivational bases, and different implications for network dynamics.

### 2.1 Informational Influence

**Informational influence** occurs when individuals use others' behavior or expressed beliefs as evidence about the state of the world. If many people are fleeing a building, one infers there is danger; if experts agree on a scientific claim, one has reason to accept it. Informational influence reflects rational belief updating: others' actions provide information that changes one's own beliefs.

**Characteristics:**
- Based on learning from others' signals or actions
- More powerful in ambiguous situations where direct evidence is scarce
- The influenced individual privately accepts the new belief
- Depends on the perceived expertise and reliability of the source

**Network implications:** Informational influence flows most readily through ties to knowledgeable others. In networks, this creates pathways for "social learning" where information aggregates as it spreads. However, informational influence can also lead to **information cascades** (Banerjee, 1992; Bikhchandani et al., 1992), where early adopters' choices heavily influence later adopters, potentially leading everyone to ignore their own private information.

### 2.2 Normative Influence

**Normative influence** (also called **social pressure** or **compliance**) occurs when individuals conform to others' expectations to gain social approval or avoid disapproval. Unlike informational influence, normative influence need not involve genuine belief change—the individual may publicly conform while privately maintaining their original view.

**Characteristics:**
- Driven by the desire for social acceptance or fear of rejection
- More powerful when the influencing group is important to the individual
- Can produce public compliance without private acceptance
- Reinforced by explicit or implicit sanctions

**Network implications:** Normative influence depends on the visibility of behavior within the network. In tightly-knit communities with dense mutual monitoring, normative pressures are strong. This explains why clustering often promotes behavior change: clustered networks provide redundant social pressure from multiple connected neighbors.

### 2.3 Identification and Imitation

**Identification** (or **referent influence**) occurs when individuals adopt behaviors or attitudes because they identify with the influencer and want to be like them. This mechanism underlies the influence of celebrities, role models, and charismatic leaders.

**Characteristics:**
- Based on attraction to or identification with the influencer
- Often operates through unconscious mimicry and emotional contagion
- The influenced individual internalizes the new attitude or behavior
- Strongest when the source is attractive, similar, or prestigious

**Network implications:** Identification-based influence depends on tie content, not just network structure. A single tie to a highly attractive or prestigious individual may be more influential than multiple ties to ordinary peers. This creates "influencer" dynamics where a small number of high-status nodes disproportionately shape network-wide outcomes.

### 2.4 Integration: Mixed Mechanisms in Practice

Real-world influence rarely operates through a single mechanism. A teenager might start vaping because friends do (normative), because they believe it's safe after seeing peers vape without immediate harm (informational), and because they admire a popular peer who vapes (identification). Comprehensive models of social influence must account for these mixed mechanisms.

The models presented in subsequent sections—threshold models, cascade models, and their extensions—can be interpreted as capturing different combinations of these influence types:
- **Threshold models** often emphasize informational or normative influence (need for reinforcement)
- **Independent cascade models** emphasize informational influence (single exposure sufficient)
- **Complex contagion models** emphasize normative influence (social pressure requires multiple sources)

---

## 3. Simple Contagion: The SIR Analogy

The simplest model of social influence treats adoption as analogous to disease infection: a single exposure from an "infected" (adopting) neighbor is sufficient for transmission. This **simple contagion** framework applies to behaviors that are easy to evaluate, low-risk, or require no social reinforcement.

### 3.1 The Basic SIR Model

The **SIR (Susceptible-Infected-Recovered)** model, originally developed for epidemiology (Kermack & McKendrick, 1927), has been widely applied to social contagion:

**States:**
- **S (Susceptible):** Has not adopted the behavior; can be influenced.
- **I (Infected/Active):** Has adopted the behavior; can influence others.
- **R (Recovered):** No longer adopts or influences; may have lost interest or become immune to further influence.

**Dynamics:**
- Infection rate: Each infected node infects each susceptible neighbor at rate $\beta$.
- Recovery rate: Each infected node recovers at rate $\gamma$.

In the network context, the SIR model is often simulated as follows:

```
Algorithm: SIR Simulation on Networks
Input: Graph G = (V, E), seed set S₀, infection probability β, recovery probability γ
Output: Final set of infected nodes

1. Initialize: I ← S₀, R ← ∅, S ← V \ S₀
2. While I ≠ ∅:
   a. For each node u in I:
      i. For each neighbor v in S:
         - With probability β: move v from S to I (new infection)
   b. For each node u in I:
      - With probability γ: move u from I to R (recovery)
3. Return R ∪ I (all ever-infected nodes)
```

### 3.2 Invasion Threshold

For simple contagion on networks, a key question is whether a small initial seed can trigger a large cascade. The **invasion threshold** (or **epidemic threshold**) depends on network topology:

For a configuration model network with degree distribution $P(k)$, the condition for a global epidemic is:

$$R_0 = \frac{\beta}{\gamma} \cdot \frac{\langle k^2 \rangle - \langle k \rangle}{\langle k \rangle} > 1$$

where $\langle k \rangle$ and $\langle k^2 \rangle$ are the mean and mean-squared degree, respectively. Networks with high degree variance (heavy-tailed distributions) have lower epidemic thresholds because high-degree "hub" nodes can sustain transmission.

### 3.3 Applications to Social Influence

Simple contagion models apply to social phenomena where:
- A single exposure is sufficient for adoption
- There is no social reinforcement requirement
- Adoption is often irreversible (no recovery)

**Examples:**
- **Information diffusion:** Hearing a news story once is usually sufficient to "know" it.
- **Viral content:** Viewing a meme or video once may prompt sharing.
- **Simple technology adoption:** Downloading an app may require only awareness.

However, many social behaviors exhibit **complex contagion** properties, requiring multiple exposures—addressed in the next section.

---

## 4. Complex Contagion

Not all social behaviors spread like diseases. For behaviors that are risky, costly, controversial, or difficult to evaluate, individuals often require **multiple independent exposures** before adopting. This phenomenon is termed **complex contagion** (Centola & Macy, 2007).

### 4.1 The Reinforcement Mechanism

In complex contagion, an individual's probability of adoption depends on the number (or fraction) of their neighbors who have already adopted. This creates a **reinforcement effect**: seeing multiple friends adopt provides stronger evidence or social pressure than seeing just one.

**Why some behaviors require complex contagion:**
- **Risk and uncertainty:** When outcomes are uncertain, multiple adopters provide independent evidence of safety or value.
- **Social proof:** Adoption by diverse others signals broad acceptability.
- **Coordination needs:** Some behaviors require coordinated adoption (e.g., communication platforms).
- **Normative pressure:** Social norms require seeing that "everyone is doing it" before conforming.

### 4.2 Network Effects on Complex Contagion

Centola and Macy (2007) demonstrated that complex contagion has very different network requirements than simple contagion:

| Property | Simple Contagion | Complex Contagion |
|----------|-----------------|-------------------|
| Optimal network | Random, small-world | Clustered, lattice-like |
| Role of bridges | Accelerate spread | May inhibit spread |
| Role of clustering | Slows spread | Enables spread |
| Long ties | Valuable | Often ineffective |

**Intuition:** In simple contagion, any path between adopters and non-adopters enables transmission. In complex contagion, a non-adopter needs multiple adopting neighbors. Random "long ties" connect nodes to distant parts of the network where neighbors don't know each other, failing to provide the local reinforcement complex contagion requires. Clustered networks, where neighbors of neighbors are also neighbors, enable the multiple exposures that drive complex contagion.

### 4.3 Empirical Evidence

Centola (2010) provided experimental evidence for these predictions using an online health behavior adoption study. Participants were placed in either:
- A **random network** (small-world structure with many long ties)
- A **clustered lattice network** (high clustering, no long ties)

The behavior required multiple adopting friends before adoption was possible (complex contagion). Results showed that:
- Adoption was more likely in the clustered lattice network
- Long ties in the random network failed to produce multiple exposures
- Clustering enabled the reinforcement necessary for complex contagion

This research overturned the intuition (from simple contagion) that network structures that facilitate rapid information spread also facilitate behavior change.

---

## 5. Granovetter's Threshold Model

**Mark Granovetter's** (1978) threshold model provides the foundational framework for understanding how individual decision rules aggregate to collective outcomes. The model shows that even small changes in the distribution of individual thresholds can produce dramatically different macro-level cascade sizes.

### 5.1 Formal Definition

Consider a population of $n$ individuals, each with a **threshold** $\theta_i \in [0, 1]$. Let $r(t)$ be the fraction of the population that has adopted by time $t$. Individual $i$ adopts when:

$$r(t) \geq \theta_i$$

That is, each individual adopts once the adoption rate in the population exceeds their personal threshold.

**Interpretation of thresholds:**
- $\theta_i = 0$: Innovator—adopts immediately without social influence
- $\theta_i = 0.5$: Majority adopter—adopts once majority has adopted
- $\theta_i = 1$: Never adopts (requires full adoption, impossible)

### 5.2 Collective Dynamics

The aggregate dynamics are determined by the **threshold distribution** $F(\theta)$, the cumulative distribution function of thresholds. Starting from an initial fraction $r_0$ of adopters, the adoption evolves according to:

$$r(t+1) = F(r(t))$$

This is a one-dimensional dynamical system. Fixed points occur where $r^* = F(r^*)$. The stability of fixed points determines whether cascades occur.

### 5.3 The Cascade Condition

A **cascade** (or **global adoption**) occurs when adoption spreads from an initial seed to a large fraction of the population. For this to happen:

1. The threshold distribution must satisfy $F(r) > r$ for some interval above the initial adoption level.
2. The initial seed must be large enough to push adoption into this interval.

**Key insight:** The shape of the threshold distribution matters enormously. Consider two distributions with the same mean threshold:
- **Low variance:** All thresholds clustered around the mean—hard to start cascades because few people have low thresholds to initiate, and those who do may not trigger those with slightly higher thresholds.
- **High variance:** Mixture of innovators (low thresholds) and late adopters (high thresholds)—cascades easier because innovators can trigger early majority, who trigger late majority.

### 5.4 Example: The Critical Mass Problem

Granovetter's classic example: Suppose we want collective action (e.g., joining a protest). Each person will join if they expect at least $\theta_i$ others to join. Let thresholds be uniformly distributed on $[0, 1]$.

The CDF is $F(r) = r$. Every point is a fixed point—cascades don't self-sustain. However, if there's a small fraction $\epsilon$ of people with threshold 0 (always protest) and the rest have thresholds uniformly on $[\epsilon, 1]$:

$$F(r) = \begin{cases} \epsilon + \frac{r-\epsilon}{1-\epsilon} & \text{if } r \geq \epsilon \\ 0 & \text{otherwise} \end{cases}$$

Now $F(r) > r$ for $r$ just above $\epsilon$, meaning a small seed can trigger a large cascade. This illustrates the **critical mass** phenomenon: a small committed minority can sometimes trigger widespread adoption.

### 5.5 Network Granovetter Model

The basic threshold model assumes a well-mixed population (mean-field approximation). The **network threshold model** generalizes this by making thresholds depend on local neighborhood adoption:

Individual $i$ adopts when:

$$\frac{|N_i^{adopted}|}{|N_i|} \geq \theta_i$$

where $N_i$ is the set of $i$'s neighbors and $N_i^{adopted}$ are the adopting neighbors. This local version can produce very different dynamics than the global version, especially when networks are clustered or have high variance in degree.

---

## 6. The Linear Threshold Model

The **Linear Threshold Model (LTM)**, formalized by Kempe, Kleinberg, and Tardos (2003), generalizes Granovetter's threshold model to weighted networks where different neighbors may have different influence strengths.

### 6.1 Formal Definition

Consider a directed graph $G = (V, E)$ with edge weights $w_{ij} \geq 0$ representing the influence of node $j$ on node $i$. Each node $i$ has a threshold $\theta_i \in [0, 1]$.

**Activation rule:** Node $i$ becomes active (adopts) when:

$$\sum_{j \in N_i^{active}} w_{ji} \geq \theta_i$$

where $N_i^{active}$ is the set of active in-neighbors of $i$.

**Constraints on weights:**
- $w_{ji} \geq 0$ (non-negative influence)
- $\sum_{j \in N_i} w_{ji} \leq 1$ (influences are bounded)

The threshold $\theta_i$ can be interpreted as the "resistance" of node $i$ to activation—higher thresholds require more cumulative influence.

### 6.2 The Activation Function

The LTM activation rule defines a step function:

$$f_i(x) = \begin{cases} 1 & \text{if } \sum_j w_{ji} x_j \geq \theta_i \\ 0 & \text{otherwise} \end{cases}$$

where $x_j \in \{0, 1\}$ indicates whether node $j$ is active. This is a linear classifier with threshold $\theta_i$.

**Relation to neural networks:** The LTM activation function is essentially a perceptron (Rosenblatt, 1958). The network of nodes with their weighted influences and threshold activation functions forms a network of perceptrons. This neural network analogy is more than superficial: the dynamics of influence propagation in LTMs share mathematical properties with neural network activation, and algorithms developed for one domain often transfer to the other.

### 6.3 Dynamics

Starting from an initial seed set $S_0$, the LTM evolves in discrete time steps:

```
Algorithm: Linear Threshold Model Dynamics
Input: Graph G = (V, E), weights w, thresholds θ, seed set S₀
Output: Final active set S∞

1. Initialize: A₀ ← S₀, t ← 0
2. Repeat until convergence:
   a. A_{t+1} ← A_t
   b. For each inactive node i ∉ A_t:
      i. Compute influence: I_i = Σ_{j ∈ A_t} w_{ji}
      ii. If I_i ≥ θ_i: Add i to A_{t+1}
   c. t ← t + 1
3. Return A_t
```

The process converges in at most $|V|$ steps because at least one node activates each iteration (until convergence).

### 6.4 Threshold Distribution

In many applications, thresholds are drawn from a distribution rather than assigned deterministically. Common choices include:
- **Uniform:** $\theta_i \sim U[0, 1]$
- **Fixed:** All $\theta_i = \theta$ (e.g., majority threshold $\theta = 0.5$)
- **Power-law:** Heavy-tailed distribution matching empirical observations

The choice of threshold distribution significantly affects cascade sizes and the effectiveness of seed selection.

### 6.5 Properties

**Monotonicity:** If a set $S$ activates node $i$, then any superset $S' \supset S$ also activates $i$. This property is crucial for the influence maximization analysis in Section 9.

**Submodularity:** The influence function $\sigma(S)$ = expected number of nodes activated by seed set $S$ is submodular (diminishing returns). Adding a node to a larger set provides less marginal gain than adding it to a smaller set.

---

## 7. The Independent Cascade Model

The **Independent Cascade Model (ICM)** provides an alternative to threshold-based activation. In the ICM, each edge represents a single chance for influence, and activation attempts occur probabilistically.

### 7.1 Formal Definition

Consider a directed graph $G = (V, E)$ where each edge $(i, j)$ has an associated **propagation probability** $p_{ij} \in [0, 1]$.

**Dynamics:**
1. At time $t = 0$, a seed set $S_0$ is activated.
2. When node $i$ first becomes active at time $t$, it has a single chance to activate each inactive neighbor $j$: with probability $p_{ij}$, node $j$ becomes active at time $t + 1$.
3. Once node $i$ has attempted to activate all its neighbors, it remains active but makes no further activation attempts.
4. The process continues until no new activations occur.

### 7.2 States and Timing

**Node states:**
- **Inactive:** Has not been activated; can still be influenced.
- **Active:** Has been activated; will attempt to influence neighbors once.
- **Exhausted:** Has completed activation attempts; remains active but influences no one new.

**Temporal dynamics:** The ICM unfolds in generations:
- Generation 0: Seed set $S_0$
- Generation 1: Nodes activated by direct influence from $S_0$
- Generation 2: Nodes activated by influence from Generation 1
- And so on...

### 7.3 Expected Spread Calculation

Computing the expected spread $\sigma(S) = \mathbb{E}[|\text{nodes activated by } S|]$ is #P-hard in general (Chen et al., 2010). However, several approximation approaches exist:

**Monte Carlo simulation:** Run many simulations and average the results. Standard error decreases as $1/\sqrt{N}$ with $N$ simulations.

**Live-edge graph representation:** An equivalent formulation represents the ICM as a random subgraph process:
- For each edge $(i, j)$, independently include it in a "live-edge graph" with probability $p_{ij}$.
- The nodes activated from seed $S$ are exactly those reachable from $S$ in the live-edge graph.
- Expected spread is the expected size of the reachable set over random live-edge graphs.

**Recursive formulation:** Let $\sigma_S(i)$ be the probability that node $i$ is activated from seed $S$. Then:

$$\sigma_S(i) = 1 - \prod_{j \in N_i} (1 - \sigma_S(j) \cdot p_{ji})$$

This system of equations can be solved iteratively (though it has no closed-form solution for general graphs).

### 7.4 Relationship to Percolation Theory

The live-edge graph formulation connects the ICM to **bond percolation** in statistical physics. In bond percolation, each edge of a graph is retained with probability $p$ (independently) and removed otherwise. The ICM with uniform $p_{ij} = p$ is equivalent to percolation from a given source set.

This connection enables the application of percolation theory results:
- **Percolation threshold:** For infinite graphs, there exists a critical $p_c$ below which only finite clusters form and above which an infinite cluster exists.
- **Critical behavior:** Near $p_c$, the distribution of cluster sizes follows power laws.
- **Universality:** Certain properties are independent of microscopic details and depend only on dimension and network type.

### 7.5 ICM vs. LTM

| Feature | Independent Cascade | Linear Threshold |
|---------|-------------------|------------------|
| Activation trigger | Single successful trial | Cumulative influence |
| Multiple exposures | Redundant (first success matters) | Reinforcing (all contribute) |
| Edge semantics | Probability of success | Influence weight |
| Temporal pattern | Generational waves | Threshold crossing |
| Best for | Information diffusion, viral spread | Adoption decisions, behavior change |

Both models are simplifications of real influence processes. The ICM captures simple contagion where exposure probability matters; the LTM captures complex contagion where cumulative influence matters. Real-world influence often involves elements of both.

---

## 8. Generalized Threshold Models

Real social influence involves heterogeneity, noise, and adaptation that basic threshold models don't capture. This section reviews extensions that add realism.

### 8.1 Noisy Thresholds

In the **noisy threshold model**, node $i$ activates when:

$$\sum_{j \in N_i^{active}} w_{ji} + \epsilon_i \geq \theta_i$$

where $\epsilon_i$ is random noise (often Gaussian or uniform). This captures:
- Individual variation in decision-making
- Unobserved factors affecting adoption
- Stochastic elements in influence transmission

**Effect of noise:** Noise smooths the sharp threshold, making adoption probability a sigmoid function of cumulative influence rather than a step function. This connects threshold models to logistic regression and neural networks with sigmoid activations.

### 8.2 Heterogeneous Influence Weights

The basic LTM assumes static influence weights $w_{ji}$. Extensions allow weights to depend on:
- **Node attributes:** Influence from similar nodes is stronger (homophily)
- **Tie strength:** Close friends have higher influence weights
- **Temporal dynamics:** Influence decays over time
- **Content:** Different weights for different types of influence

Formally, $w_{ji}(t) = f(\text{attributes}_j, \text{tie_strength}_{ji}, t, \text{content})$ for some function $f$.

### 8.3 Adaptive Thresholds

In **adaptive threshold models**, thresholds change based on experience:
- **Successive adoption:** Once a node activates, its threshold for future behaviors may decrease (susceptibility increases).
- **Failed activation:** If influence attempts repeatedly fail, thresholds may increase (skepticism grows).
- **Social proof accumulation:** Observing others adopt successfully may lower thresholds.

**Formulation:** $\theta_i(t+1) = g(\theta_i(t), \text{outcomes}_t)$ where $g$ represents the adaptation rule.

### 8.4 Continuous State Models

Rather than binary active/inactive states, continuous models represent influence as gradual attitude change:

$$x_i(t+1) = (1 - \alpha_i) x_i(t) + \alpha_i \sum_{j \in N_i} w_{ji} x_j(t)$$

where $x_i(t) \in [0, 1]$ represents the strength of adoption/opinion, and $\alpha_i$ is the susceptibility of node $i$. This is essentially the **DeGroot model** of opinion dynamics (discussed in detail in the Opinion Dynamics document), which we mention here because it represents a continuous generalization of threshold models.

---

## 9. Influence Maximization

A practical problem arising in viral marketing, public health, and social movements is: **Given a network and an influence model, which k nodes should we initially activate to maximize the expected cascade size?** This is the **influence maximization problem**.

### 9.1 Problem Statement

**Given:**
- Graph $G = (V, E)$
- Influence model (LTM or ICM with known parameters)
- Budget $k$ (number of seed nodes)

**Find:**
- Seed set $S^* \subseteq V$ with $|S^*| = k$ that maximizes expected spread:

$$S^* = \arg\max_{S: |S| = k} \sigma(S)$$

where $\sigma(S)$ is the expected number of nodes activated from seed $S$.

### 9.2 Computational Complexity

**Theorem (Kempe, Kleinberg & Tardos, 2003):** Influence maximization is NP-hard for both the Linear Threshold Model and the Independent Cascade Model.

**Proof sketch:** The problem can encode the NP-complete Set Cover problem. Given a set cover instance, construct a network where selecting seed nodes corresponds to selecting sets, and coverage corresponds to activation spread. Optimal seed selection solves Set Cover.

Despite NP-hardness, Kempe et al. (2003) showed that approximation is possible using properties of the objective function.

### 9.3 Submodularity and the Greedy Algorithm

A set function $f: 2^V \rightarrow \mathbb{R}$ is **submodular** if for all $A \subseteq B \subseteq V$ and $v \notin B$:

$$f(A \cup \{v\}) - f(A) \geq f(B \cup \{v\}) - f(B)$$

This captures **diminishing returns:** adding an element to a smaller set provides at least as much marginal gain as adding it to a larger set.

**Theorem:** For both LTM and ICM, the influence function $\sigma(S)$ is monotone (adding nodes never decreases spread) and submodular.

**Implication:** The **greedy algorithm**—iteratively adding the node with maximum marginal gain—provides a $(1 - 1/e)$ approximation guarantee (~63% of optimal).

### 9.4 The Greedy Algorithm

```
Algorithm: Greedy Influence Maximization
Input: Graph G, influence model M, budget k
Output: Seed set S

1. S ← ∅
2. For i = 1 to k:
   a. For each node v ∉ S:
      i. Compute marginal gain: Δ(v) = σ(S ∪ {v}) - σ(S)
   b. v* ← argmax_v Δ(v)
   c. S ← S ∪ {v*}
3. Return S
```

**Complexity:** Each marginal gain computation requires estimating $\sigma(\cdot)$, which is expensive. For $k$ iterations and $|V|$ nodes per iteration, with $R$ Monte Carlo simulations per estimate, complexity is $O(k \cdot |V| \cdot R \cdot |E|)$.

### 9.5 Accelerations and Improvements

Several approaches reduce computational cost:

**CELF (Cost-Effective Lazy Forward) algorithm (Leskovec et al., 2007):** Exploits submodularity to avoid recomputing marginal gains for all nodes. If a node's marginal gain was small in a previous iteration, and the seed set has only grown, its current marginal gain cannot be larger. This pruning provides 700× speedup in practice.

**SIMPATH (Goyal et al., 2011):** For the LTM, computes influence spread by enumerating short paths rather than Monte Carlo simulation.

**Influence maximization under IC (Borgs et al., 2014):** Randomized algorithms achieve near-linear time complexity with $(1 - 1/e - \epsilon)$ approximation.

**Node embeddings:** Learn low-dimensional representations where influence is approximated by distance in embedding space, enabling fast approximate maximization.

### 9.6 Extensions

**Adaptive influence maximization:** Seed nodes are selected sequentially, with later selections made after observing the outcomes of earlier activations (Golovin & Krause, 2011).

**Robust influence maximization:** Seed selection that performs well under uncertainty about network structure or influence parameters (Chen et al., 2016).

**Fairness constraints:** Ensure selected seeds represent different demographic groups or that influence spreads equitably (Ali et al., 2019; Tsang et al., 2019).

---

## 10. Competitive Influence

In many scenarios, multiple innovations, behaviors, or ideas compete for adoption within the same network. **Competitive influence** models extend single-behavior frameworks to capture these dynamics.

### 10.1 Competitive Threshold Models

In the **competitive linear threshold model**, multiple competing behaviors $c \in \{1, 2, ..., m\}$ vie for adoption. Each node $i$ has:
- Thresholds $\theta_i^c$ for each competing behavior
- Weights $w_{ji}^c$ representing influence of neighbor $j$'s adoption of behavior $c$

Node $i$ adopts behavior $c$ when:

$$\sum_{j \in N_i^c} w_{ji}^c \geq \theta_i^c$$

and $c$ is the first behavior to exceed its threshold (or the one with maximum margin).

**Key difference from single-behavior models:** Adoption is typically exclusive—once a node adopts behavior $c$, it becomes unavailable for competing behaviors. This creates a "race" dynamics where earlier activation matters.

### 10.2 Competitive Independent Cascade

In the **competitive independent cascade model**, competing influences propagate simultaneously. When node $i$ attempts to influence neighbor $j$:
- With probability $p_{ij}^1$, behavior 1 spreads
- With probability $p_{ij}^2$, behavior 2 spreads
- With probability $p_{ij}^{both}$, both spread (if co-adoption is allowed)
- With probability $1 - p_{ij}^1 - p_{ij}^2 - p_{ij}^{both}$, neither spreads

If co-adoption is not allowed and multiple behaviors attempt activation simultaneously, tie-breaking rules determine the outcome (e.g., first arrival wins, stronger influence wins).

### 10.3 Game-Theoretic Aspects

Competitive influence can be modeled as a game between competing parties (e.g., firms marketing competing products, political campaigns):

**Players:** Two or more influencers with budgets $k_1, k_2, ...$

**Strategies:** Choice of seed sets $S_1, S_2, ...$

**Payoffs:** Expected number of nodes adopting their behavior

**Game types:**
- **Simultaneous move:** Players choose seeds simultaneously (Nash equilibrium analysis)
- **Sequential move:** First mover advantage in seed selection (Stackelberg game)
- **Adaptive:** Players observe competitor activations and respond

**Key results:**
- Competitive influence games are not submodular in general (Bharathi et al., 2007)
- Pure Nash equilibria may not exist
- Price of anarchy (efficiency loss from competition) can be bounded in some settings

### 10.4 Applications

**Viral marketing:** Competing firms launching similar products; optimal seed selection depends on competitor strategies.

**Public health:** Promoting health behaviors (vaccination, exercise) against competing unhealthy alternatives.

**Political campaigns:** Competing candidates seeking to maximize their influence on voter opinions.

---

## 11. Temporal Aspects of Influence

Real-world influence unfolds over time, with important implications for when interventions should occur and how quickly cascades develop.

### 11.1 Time-Decay of Influence

The impact of an exposure typically decays over time:

$$w_{ji}(t) = w_{ji}^0 \cdot e^{-\lambda (t - t_j)}$$

where $t_j$ is when node $j$ became active, and $\lambda$ is the decay rate.

**Implications:**
- Early exposures matter more than late ones
- Influence windows: there's a limited time during which a node can influence its neighbors
- Cascades have natural time horizons

### 11.2 Memory Effects

Nodes may exhibit **memory** of past exposures:
- **Cumulative memory:** All past exposures count (standard threshold model)
- **Finite memory:** Only exposures within the last $\tau$ time steps count
- **Weighted memory:** Recent exposures weighted more heavily

Memory effects can change cascade dynamics qualitatively. With short memory, cascades may die out before reaching equilibrium; with long memory, cascades may persist and revive.

### 11.3 Temporal Networks

In **temporal networks**, edges exist only at specific times. The constraint that influence must flow along **time-respecting paths** (where edge times are increasing) fundamentally changes reachability.

**Time-respecting path:** A path $v_1 \rightarrow v_2 \rightarrow ... \rightarrow v_k$ where the time of edge $(v_i, v_{i+1})$ is less than the time of edge $(v_{i+1}, v_{i+2})$.

**Implications for influence:**
- Network connectivity overestimates influence spread in temporal networks
- Timing of seed selection matters—seeds placed too early or too late may miss critical transmission windows
- Optimal influence strategies require coordinating seed timing with network dynamics

### 11.4 When Timing Matters

Temporal considerations are particularly important when:
- **External events** create windows of opportunity or vulnerability
- **Saturation effects** limit how long influence remains effective
- **Competitive dynamics** create races where early activation determines adoption
- **Resource constraints** require sequential rather than simultaneous seeding

---

## 12. Network Structure Effects

The structure of the underlying network profoundly affects influence dynamics. Understanding which network structures facilitate or inhibit influence is crucial for intervention design.

### 12.1 Network Properties and Influence Spread

| Network Property | Effect on Simple Contagion | Effect on Complex Contagion |
|-----------------|---------------------------|----------------------------|
| High degree variance | Facilitates (hubs spread) | Mixed (hubs help spread, but may lack reinforcement) |
| High clustering | Inhibits | Facilitates |
| Short average path | Facilitates | Mixed |
| Community structure | Can contain cascades | Can enable local adoption |
| Core-periphery | Core seeds most effective | Depends on periphery clustering |

### 12.2 The Role of Communities

**Community structure** (dense internal connectivity, sparse external connectivity) affects influence in several ways:

**Cascade containment:** Strong community boundaries can prevent cascades from spreading globally. A cascade that starts in one community may saturate it without crossing to others.

**Community bridges:** Nodes that connect different communities (high betweenness centrality) are critical for cross-community influence. Targeting these nodes can enable global cascades.

**Local vs. global influence:** Within-community influence is easier due to higher density and homophily. Cross-community influence requires either bridge nodes or coordinated multi-community seeding.

### 12.3 Bridges and Brokers

**Structural holes** (Burt, 1992)—gaps between non-redundant contacts—create opportunities for brokerage. Nodes spanning structural holes:
- Access diverse information from disconnected groups
- Control information flow between groups
- Are well-positioned to start cascades that bridge communities

However, for **complex contagion**, pure bridges may be less effective because they lack the local reinforcement that clustered networks provide. The optimal influencers for complex contagion combine brokerage position with local clustering.

### 12.4 Which Structures Maximize/Minimize Influence?

**Maximizing influence:**
- **Scale-free networks** with high-degree hubs facilitate simple contagion
- **Clustered networks** with community structure facilitate complex contagion
- **Small-world networks** balance local reinforcement with global reach

**Minimizing influence (containment):**
- **Low-degree, regular networks** lack super-spreaders
- **Fragmented networks** with strong community boundaries
- **Temporal networks** with limited time-respecting paths

---

## 13. Empirical Studies

Social influence has been studied across diverse empirical settings, from large-scale observational studies to controlled experiments.

### 13.1 Christakis and Fowler's Contagion Studies

**Nicholas Christakis** and **James Fowler** (2007, 2008) analyzed the Framingham Heart Study social network, claiming that obesity, smoking, happiness, and other traits spread through social networks up to three degrees of separation (friend of a friend of a friend).

**Key findings:**
- Obesity: A person's likelihood of becoming obese increased by 57% if they had a friend who became obese
- Smoking cessation: Smoking behavior was similarly contagious
- Happiness: Happy clusters in the network were significantly larger than expected by chance
- Network effects extended to 3 degrees of separation

**Methods:** Longitudinal network analysis tracking individuals and their social ties over 32 years, using regression models with network-lagged dependent variables.

### 13.2 Critiques and Debates

The Christakis-Fowler findings generated significant methodological debate:

**Shalizi and Thomas (2011):** Argued that the statistical methods could not distinguish genuine influence from **latent homophily**—unobserved shared characteristics that cause both friendship formation and similar behavior. Friends might both become obese not because one influences the other, but because they share an environment (e.g., same gym, same food options) or unmeasured traits (e.g., genetics, socioeconomic status).

**Lyons (2011):** Demonstrated that the 3-degrees-of-separation finding could arise from model misspecification and argued that the statistical evidence for network contagion was weaker than claimed.

**Manski's reflection problem (1993):** Showed that distinguishing endogenous effects (peer influence), exogenous effects (peer characteristics), and correlated effects (shared environment) is fundamentally difficult with observational data alone.

**Response:** Christakis and Fowler defended their methods, noting that they used longitudinal data to establish temporal ordering (friends' obesity preceded ego's obesity) and that alternative explanations couldn't fully account for the observed patterns.

### 13.3 The Causal Identification Problem

The debate highlights the **causal identification problem** in network influence studies:

**Confounding sources of network correlation:**
1. **Influence:** A's behavior causes B's behavior
2. **Homophily:** Similar individuals select each other as friends
3. **Shared environment:** A and B are exposed to common external factors
4. **Network endogeneity:** The network itself changes based on behavior

**Solutions:**
- **Randomized experiments:** Randomly assign influence exposure (e.g., Bond et al., 2012 Facebook experiment)
- **Instrumental variables:** Find exogenous variation in network position or exposure
- **Statistical controls:** Include fixed effects for shared environment, use detailed longitudinal data
- **Structural models:** Specify explicit models of selection and influence, estimate jointly

### 13.4 Experimental Evidence

**Bond et al. (2012):** Conducted a randomized experiment on 61 million Facebook users, showing that messages about voting behavior increased real-world voter turnout. Demonstrated causal social influence on political participation at massive scale.

**Centola (2010):** Experimental comparison of clustered vs. random networks for health behavior adoption, demonstrating the superiority of clustered networks for complex contagion.

**Aral & Walker (2012):** Used randomized product recommendations on a social network to distinguish influence from homophily, showing both mechanisms operate but influence effects are smaller than often assumed.

---

## 14. Applications

Social influence models have been applied across diverse domains to understand and design diffusion processes.

### 14.1 Viral Marketing

**Objective:** Maximize product adoption through word-of-mouth and social sharing.

**Approaches:**
- **Influence maximization:** Identify optimal seed users for product seeding
- **Referral programs:** Design incentive structures that leverage social influence
- **Viral content design:** Engineer content characteristics that promote sharing

**Key insights:**
- The "influentials hypothesis" (targeting high-degree nodes) works for simple contagion but may fail for complex contagion
- Seeding strategies should match the product's complexity and risk profile
- Timing and context (what else is happening in the network) matter

### 14.2 Political Mobilization

**Objective:** Increase voter turnout, campaign volunteering, or political engagement.

**Approaches:**
- **Social pressure:** Informing voters that their participation is publicly observed (Gerber et al., 2008)
- **Network targeting:** Recruiting campaign volunteers by leveraging their social ties
- **Echo chamber dynamics:** Understanding how network structure affects political polarization

**Key insights:**
- Close friends are more influential than acquaintances for political behavior
- Social pressure can increase turnout but raises ethical concerns
- Network interventions can be more cost-effective than mass media

### 14.3 Public Health Interventions

**Objective:** Promote healthy behaviors (vaccination, exercise, safe sex) and discourage unhealthy ones.

**Approaches:**
- **Peer education:** Train influential community members to promote health behaviors
- **Social norms marketing:** Correct misperceptions about peer behavior
- **Network-based targeting:** Identify optimal nodes for intervention

**Key insights:**
- Health behaviors often exhibit complex contagion (require social reinforcement)
- Local network structure (clustering) may be more important than global centrality
- Cultural adaptation of interventions requires understanding local influence patterns

### 14.4 Technology Adoption

**Objective:** Accelerate adoption of new technologies, platforms, or standards.

**Approaches:**
- **Two-sided market seeding:** Target both producers and consumers
- **Cross-platform influence:** Leverage existing networks to bootstrap new platforms
- **Threshold targeting:** Identify communities near critical mass thresholds

**Key insights:**
- Network effects create adoption thresholds—technologies fail unless they reach critical mass
- Early adopters are often structurally peripheral (less constrained by existing ties)
- Compatibility with existing networks affects adoption speed

---

## 15. How Lutufi Implements Social Influence

Lutufi provides a unified probabilistic framework for social influence modeling, integrating network structure with Bayesian inference to enable uncertainty-aware influence analysis.

### 15.1 Probabilistic Threshold Models

Lutufi represents threshold models as **probabilistic graphical models** where:
- Node activation states are random variables
- Thresholds are probability distributions rather than fixed values
- Influence weights have associated uncertainty

**Representation:**
```
For each node i:
  - Activation state A_i ∈ {0, 1} (inactive/active)
  - Threshold θ_i ~ Distribution(μ_i, σ_i)
  - Influence weights w_{ji} ~ Distribution(parameters)

Activation probability:
  P(A_i = 1 | A_{N_i}) = P(Σ_{j ∈ N_i} w_{ji} · A_j ≥ θ_i)
```

**Advantages:**
- Captures uncertainty in thresholds and influence strengths
- Enables Bayesian updating as activation data is observed
- Supports heterogeneous populations naturally

### 15.2 Uncertainty in Influence Parameters

Lutufi treats influence parameters (thresholds, weights, propagation probabilities) as uncertain quantities to be inferred from data:

**Prior specification:** Based on domain knowledge or historical data
**Likelihood:** From observed activation cascades
**Posterior:** Updated beliefs about parameters after observing data

This enables:
- **Learning from partial observations:** Infer influence parameters even when not all activations are observed
- **Robust predictions:** Account for parameter uncertainty when predicting future cascades
- **Model comparison:** Evaluate which influence model (LTM, ICM, etc.) best fits observed data

### 15.3 Intervention Optimization

Lutufi supports **decision-making under uncertainty** for influence maximization:

**Expected utility of interventions:**
```
EU(S) = E[σ(S; θ, w) | data] - c(S)
```

where $S$ is the seed set, $\sigma$ is the spread function, $\theta$ and $w$ are uncertain parameters, and $c(S)$ is the cost of seeding.

**Optimization:**
- Expected value of sample information (EVSI) to determine when to gather more data
- Robust optimization for worst-case scenarios
- Multi-objective optimization (spread, fairness, cost)

### 15.4 Dynamic Influence Tracking

For temporal networks, Lutufi enables:
- **Online inference:** Update beliefs about influence parameters as new activation data arrives
- **Change-point detection:** Identify when influence dynamics shift
- **Predictive monitoring:** Forecast cascade evolution with uncertainty bounds

### 15.5 Integration with Causal Inference

Lutufi's causal inference capabilities enable:
- **Estimating causal effects** of network position on influence (controlling for homophily)
- **Counterfactual analysis:** What would have happened with different seed sets?
- **Intervention evaluation:** Did a public health campaign actually increase adoption?

---

## 16. Key References

1. **Granovetter, M. (1978).** "Threshold Models of Collective Behavior." *American Journal of Sociology*, 83(6), 1420–1443. — The foundational threshold model paper.

2. **Kempe, D., Kleinberg, J. & Tardos, É. (2003).** "Maximizing the Spread of Influence through a Social Network." *Proceedings of KDD*, 137–146. — Formalized influence maximization and proved approximation guarantees.

3. **Centola, D. & Macy, M. (2007).** "Complex Contagions and the Weakness of Long Ties." *American Journal of Sociology*, 113(3), 702–734. — Distinguished simple from complex contagion and showed network structure effects.

4. **Watts, D. J. (2002).** "A Simple Model of Global Cascades on Random Networks." *PNAS*, 99(9), 5766–5771. — Cascade dynamics and cascade windows in random networks.

5. **Christakis, N. A. & Fowler, J. H. (2007).** "The Spread of Obesity in a Large Social Network over 32 Years." *New England Journal of Medicine*, 357(4), 370–379. — Large-scale empirical study of health behavior contagion.

6. **Banerjee, A. V. (1992).** "A Simple Model of Herd Behavior." *Quarterly Journal of Economics*, 107(3), 797–817. — Information cascades and herding.

7. **French, J. R. P. (1956).** "A Formal Theory of Social Power." *Psychological Review*, 63(3), 181–194. — Early formalization of social influence using graph theory.

8. **Katz, E. & Lazarsfeld, P. F. (1955).** *Personal Influence: The Part Played by People in the Flow of Mass Communications*. Free Press. — Two-step flow of communication.

9. **Bikhchandani, S., Hirshleifer, D. & Welch, I. (1992).** "A Theory of Fads, Fashion, Custom, and Cultural Change as Informational Cascades." *Journal of Political Economy*, 100(5), 992–1026. — Information cascade theory.

10. **Burt, R. S. (1992).** *Structural Holes: The Social Structure of Competition*. Harvard University Press. — Structural holes and brokerage advantage.

11. **Centola, D. (2010).** "The Spread of Behavior in an Online Social Network Experiment." *Science*, 329(5996), 1194–1197. — Experimental evidence for complex contagion.

12. **Leskovec, J., Krause, A., Guestrin, C., Faloutsos, C., VanBriesen, J. & Glance, N. (2007).** "Cost-effective Outbreak Detection in Networks." *Proceedings of KDD*, 420–429. — CELF algorithm for efficient influence maximization.

13. **Shalizi, C. R. & Thomas, A. C. (2011).** "Homophily and Contagion Are Generically Confounded in Observational Social Network Studies." *Sociological Methods & Research*, 40(2), 211–239. — Critique of causal inference in network studies.

14. **Bharathi, S., Kempe, D. & Salek, M. (2007).** "Competitive Influence Maximization in Social Networks." *Internet and Network Economics*, 306–311. — Competitive influence models.

15. **Manski, C. F. (1993).** "Identification of Endogenous Social Effects: The Reflection Problem." *Review of Economic Studies*, 60(3), 531–542. — Reflection problem in social influence estimation.

16. **Bond, R. M., Fariss, C. J., Jones, J. J., Kramer, A. D., Marlow, C., Settle, J. E. & Fowler, J. H. (2012).** "A 61-Million-Person Experiment in Social Influence and Political Mobilization." *Nature*, 489(7415), 295–298. — Large-scale experimental evidence for social influence on voting.

17. **Aral, S. & Walker, D. (2012).** "Identifying Influential and Susceptible Members of Social Networks." *Science*, 337(6092), 337–341. — Distinguishing influence from homophily empirically.

18. **Goyal, S., Heidari, H. & Kearns, M. (2019).** "Competing Contagions in Networks." *Games and Economic Behavior*, — Game-theoretic analysis of competitive diffusion.

---

*"Influence is not about being loud; it's about being at the right place in the network at the right time." — Adapted from Mark Granovetter*
