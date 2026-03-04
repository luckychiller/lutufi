# Stochastic Processes Relevant to Lutufi

**Document Version 1.0**  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Introduction](#introduction)
2. [Random Walks](#random-walks)
3. [Markov Chains](#markov-chains)
4. [Continuous-Time Markov Chains (CTMC)](#continuous-time-markov-chains-ctmc)
5. [Stochastic Differential Equations (SDE)](#stochastic-differential-equations-sde)
6. [Diffusion Processes on Networks](#diffusion-processes-on-networks)
7. [Epidemic Models as Stochastic Processes](#epidemic-models-as-stochastic-processes)
8. [Branching Processes](#branching-processes)
9. [Point Processes on Networks](#point-processes-on-networks)
10. [Stochastic Block Models](#stochastic-block-models)
11. [Markov Random Fields](#markov-random-fields)
12. [Applications to Social/Economic Networks](#applications-to-socialeconomic-networks)
13. [How Lutufi Uses Stochastic Processes](#how-lutufi-uses-stochastic-processes)
14. [Key References](#key-references)

---

## Introduction

Stochastic processes form the mathematical backbone of temporal dynamics in network science. Unlike static network representations that capture structure at a single moment, stochastic processes enable us to model how networks evolve, how information or disease spreads through them, and how individual behaviors change over time. For Lutufi—a library unifying Bayesian networks with social and economic network analysis—stochastic processes are not merely theoretical constructs but essential computational primitives.

A **stochastic process** is formally defined as a collection of random variables $\{X(t) : t \in \mathcal{T}\}$ indexed by a parameter $t$ (typically time), taking values in a **state space** $\mathcal{S}$. When $\mathcal{T}$ is discrete (e.g., $\mathbb{N}$ or $\mathbb{Z}$), we have a discrete-time stochastic process; when $\mathcal{T}$ is continuous (e.g., $\mathbb{R}^+$), we have a continuous-time process. The state space $\mathcal{S}$ may be discrete (finite or countably infinite) or continuous.

The importance of stochastic processes for Lutufi manifests in several critical ways:

1. **Dynamic Bayesian Networks (DBNs):** These extend static BNs by modeling temporal dependencies through Markov chains, allowing us to reason about network evolution and perform filtering, smoothing, and prediction.

2. **Network Formation Models:** Stochastic processes generate network structures—preferential attachment, small-world models, and exponential random graph models all rely on stochastic mechanisms.

3. **Diffusion and Cascade Models:** The spread of information, behaviors, diseases, or innovations through networks is inherently stochastic, requiring process-based models.

4. **Inference Over Time:** When network data arrives sequentially, stochastic processes provide the machinery for online inference and sequential estimation.

5. **Simulation and Synthetic Data:** Stochastic processes enable realistic network simulations for testing algorithms, validating models, and studying network properties under controlled conditions.

This document provides a comprehensive treatment of stochastic processes most relevant to network science and Lutufi's implementation. We proceed from fundamental concepts (random walks, Markov chains) to advanced topics (stochastic differential equations, point processes), emphasizing both theoretical foundations and practical computational considerations.

---

## Random Walks

Random walks represent the simplest yet most fundamental stochastic process on graphs. A random walk models a particle (or agent) that moves from node to node, choosing each step randomly according to some probability distribution over neighbors.

### Simple Random Walks on Graphs

Consider an undirected graph $G = (V, E)$ with $n = |V|$ nodes and adjacency matrix $A \in \{0, 1\}^{n \times n}$, where $A_{ij} = 1$ if and only if $(i, j) \in E$. Let $d_i = \sum_j A_{ij}$ denote the degree of node $i$, and $D = \text{diag}(d_1, \ldots, d_n)$ be the degree matrix.

A **simple random walk** on $G$ is a discrete-time stochastic process $\{X_t\}_{t \geq 0}$ where:
- $X_t \in V$ represents the position at time $t$
- The transition probability from node $i$ to node $j$ is:

$$P(X_{t+1} = j \mid X_t = i) = \frac{A_{ij}}{d_i} = T_{ij}$$

The matrix $T = D^{-1}A$ is called the **transition matrix** (or right-stochastic matrix), satisfying $\sum_j T_{ij} = 1$ for all $i$.

For directed graphs, we use out-degrees: $d_i^{\text{out}} = \sum_j A_{ij}$, and the transition matrix becomes $T_{ij} = A_{ij} / d_i^{\text{out}}$.

**Example:** Consider a line graph with 5 nodes: $1 - 2 - 3 - 4 - 5$. From interior nodes, the walk moves left or right with probability $1/2$ each. From the endpoints, it moves inward with probability 1. The transition matrix is:

$$T = \begin{pmatrix}
0 & 1 & 0 & 0 & 0 \\
1/2 & 0 & 1/2 & 0 & 0 \\
0 & 1/2 & 0 & 1/2 & 0 \\
0 & 0 & 1/2 & 0 & 1/2 \\
0 & 0 & 0 & 1 & 0
\end{pmatrix}$$

### Transition Matrices and Multi-Step Probabilities

The $t$-step transition probability $P(X_t = j \mid X_0 = i)$ is given by the $(i,j)$ entry of $T^t$, denoted $T^t_{ij}$. This follows from the Chapman-Kolmogorov equations:

$$T^{s+t}_{ij} = \sum_k T^s_{ik} T^t_{kj}$$

The **spectral decomposition** of $T$ provides insight into long-term behavior. For an undirected connected graph, $T$ has eigenvalues $1 = \lambda_1 > \lambda_2 \geq \cdots \geq \lambda_n \geq -1$. The eigenvalue gap $(1 - \lambda_2)$ determines the rate of convergence to stationarity.

### Stationary Distribution

A probability distribution $\pi$ over nodes is **stationary** (or invariant) if:

$$\pi^T T = \pi^T \quad \text{or equivalently} \quad \sum_i \pi_i T_{ij} = \pi_j$$

For a simple random walk on an undirected connected graph, the unique stationary distribution is:

$$\pi_i = \frac{d_i}{\sum_j d_j} = \frac{d_i}{2|E|}$$

This reflects that high-degree nodes are visited more frequently proportionally to their degree.

**Proof:** We verify $\pi^T T = \pi^T$:

$$\sum_i \pi_i T_{ij} = \sum_i \frac{d_i}{2|E|} \cdot \frac{A_{ij}}{d_i} = \frac{1}{2|E|} \sum_i A_{ij} = \frac{d_j}{2|E|} = \pi_j$$

### Hitting Times and Cover Times

The **hitting time** (or first passage time) from $i$ to $j$ is:

$$H_{ij} = \mathbb{E}[\min\{t \geq 0 : X_t = j\} \mid X_0 = i]$$

The **commute time** between $i$ and $j$ is $C_{ij} = H_{ij} + H_{ji}$. Remarkably, for undirected graphs, commute time has an elegant electrical network interpretation:

$$C_{ij} = 2|E| \cdot R_{ij}$$

where $R_{ij}$ is the effective resistance between $i$ and $j$ when each edge is a unit resistor.

The **cover time** from $i$ is the expected time to visit all nodes:

$$\text{Cover}(i) = \mathbb{E}[\min\{t : \cup_{s=0}^t \{X_s\} = V\} \mid X_0 = i]$$

For any connected graph, the cover time satisfies:

$$(1 - o(1)) n \ln n \leq \text{Cover} \leq (1 + o(1)) \frac{4}{27} n^3$$

The lower bound is achieved by the complete graph, the upper bound by the lollipop graph.

### Mixing Times

The **mixing time** measures how quickly the random walk approaches its stationary distribution. Define the **total variation distance** between the $t$-step distribution and stationarity:

$$\Delta(t) = \max_i \|T^t_{i\cdot} - \pi\|_{TV} = \max_i \frac{1}{2} \sum_j |T^t_{ij} - \pi_j|$$

The **mixing time** is:

$$\tau(\epsilon) = \min\{t : \Delta(t) \leq \epsilon\}$$

For a lazy random walk (staying put with probability 1/2), the mixing time is bounded by:

$$\tau(\epsilon) \leq \frac{1}{1 - \lambda_2} \log\left(\frac{1}{\epsilon \sqrt{\pi_{\min}}}\right)$$

where $\lambda_2$ is the second-largest eigenvalue and $\pi_{\min} = \min_i \pi_i$.

### Connection to PageRank

PageRank, the algorithm that powered Google's search engine, is intimately connected to random walks. The **PageRank** vector $\mathbf{r}$ satisfies:

$$\mathbf{r} = \alpha T^T \mathbf{r} + (1 - \alpha) \mathbf{v}$$

where $\alpha \in (0, 1)$ is the damping factor (typically 0.85) and $\mathbf{v}$ is a personalization (teleportation) distribution.

This equation describes a **random walk with restart**: with probability $\alpha$, follow a random walk step; with probability $1 - \alpha$, jump to a random node according to $\mathbf{v}$. PageRank solves the problem of periodicity and slow mixing in regular random walks while providing a centrality measure that accounts for the global graph structure.

**Algorithm:** PageRank can be computed via power iteration:

```
Initialize: r^(0) = v (uniform or personalized)
Repeat until convergence:
    r^(k+1) = α T^T r^(k) + (1 - α) v
Return: r = r^(k+1)
```

The convergence rate depends on $\alpha$; smaller $\alpha$ gives faster convergence but less faithful to graph structure.

---

## Markov Chains

Markov chains generalize random walks to arbitrary discrete state spaces with memoryless (Markov) dynamics. They are foundational to dynamic Bayesian networks, MCMC methods, and countless models in network science.

### Formal Definition

A **discrete-time Markov chain (DTMC)** on a finite or countable state space $\mathcal{S}$ is a stochastic process $\{X_t\}_{t \geq 0}$ satisfying the **Markov property**:

$$P(X_{t+1} = j \mid X_t = i, X_{t-1} = i_{t-1}, \ldots, X_0 = i_0) = P(X_{t+1} = j \mid X_t = i) = P_{ij}$$

The matrix $P = [P_{ij}]$ is the **transition matrix**, with $P_{ij} \geq 0$ and $\sum_j P_{ij} = 1$.

The Markov property states that the future depends on the past only through the present—the process is **memoryless**. This assumption, while seemingly restrictive, captures many real-world phenomena and enables tractable analysis.

### Transition Matrices and State Evolution

Given initial distribution $\mu^{(0)}$ where $\mu^{(0)}_i = P(X_0 = i)$, the distribution at time $t$ is:

$$\mu^{(t)} = \mu^{(0)} P^t$$

The $(i,j)$ entry of $P^t$ gives the $t$-step transition probability $P(X_t = j \mid X_0 = i)$.

**Example: Two-State Markov Chain**

Consider a network where nodes are either "active" or "inactive." The transition matrix:

$$P = \begin{pmatrix}
0.9 & 0.1 \\
0.3 & 0.7
\end{pmatrix}$$

indicates that active nodes remain active with probability 0.9, inactive nodes become active with probability 0.3. Starting from $\mu^{(0)} = (1, 0)$ (all active), we compute:
- $\mu^{(1)} = (0.9, 0.1)$
- $\mu^{(2)} = (0.84, 0.16)$
- As $t \to \infty$: $\mu^{(t)} \to (0.75, 0.25)$

### Classification of States

States in a Markov chain are classified based on their long-term behavior:

**Reachability:** State $j$ is **reachable** from $i$ (denoted $i \to j$) if there exists $t \geq 0$ such that $P^t_{ij} > 0$.

**Communication:** States $i$ and $j$ **communicate** ($i \leftrightarrow j$) if $i \to j$ and $j \to i$. Communication is an equivalence relation, partitioning the state space into **communicating classes**.

**Irreducibility:** A chain is **irreducible** if all states communicate (single communicating class).

**Recurrence vs. Transience:**
- State $i$ is **recurrent** if the chain returns to $i$ with probability 1: $P(T_i < \infty \mid X_0 = i) = 1$, where $T_i = \min\{t > 0 : X_t = i\}$
- State $i$ is **transient** if there's positive probability of never returning: $P(T_i = \infty \mid X_0 = i) > 0$

Recurrence implies infinite expected number of visits; transience implies finite expected visits.

**Periodicity:** The **period** of state $i$ is $d(i) = \gcd\{t > 0 : P^t_{ii} > 0\}$. If $d(i) = 1$, the state is **aperiodic**; otherwise, it's **periodic** with period $d(i)$. All states in a communicating class share the same period.

### Stationarity and Detailed Balance

A distribution $\pi$ is **stationary** if $\pi^T P = \pi^T$. For finite chains, at least one stationary distribution always exists. For irreducible chains, the stationary distribution is unique.

A stronger condition is **detailed balance** (or reversibility):

$$\pi_i P_{ij} = \pi_j P_{ji} \quad \text{for all } i, j$$

Chains satisfying detailed balance are called **reversible**. Summing detailed balance over $j$ yields stationarity:

$$\sum_j \pi_i P_{ij} = \sum_j \pi_j P_{ji} = \pi_i \sum_j P_{ji} = \pi_i$$

**Metropolis-Hastings chains** (used in MCMC) are constructed to satisfy detailed balance with respect to a target distribution, ensuring convergence to that distribution.

### Ergodicity and Convergence Theorems

A Markov chain is **ergodic** (or regular) if it is irreducible and aperiodic. For finite ergodic chains, the following fundamental convergence theorem holds:

**Theorem (Fundamental Theorem of Markov Chains):** For an ergodic Markov chain with transition matrix $P$, there exists a unique stationary distribution $\pi$ such that for all initial distributions $\mu$:

$$\lim_{t \to \infty} \mu P^t = \pi$$

Equivalently, $\lim_{t \to \infty} P^t_{ij} = \pi_j$ for all $i, j$.

The convergence is geometric: $\|P^t_{i\cdot} - \pi\|_{TV} \leq C \cdot \rho^t$ for some $\rho < 1$.

**Ergodic Theorem:** For an ergodic chain and any function $f: \mathcal{S} \to \mathbb{R}$:

$$\frac{1}{T} \sum_{t=0}^{T-1} f(X_t) \xrightarrow{a.s.} \mathbb{E}_{\pi}[f(X)] = \sum_i \pi_i f(i)$$

This justifies Monte Carlo estimation using Markov chain samples.

### Absorption and Absorbing Chains

A state $i$ is **absorbing** if $P_{ii} = 1$ (once entered, never left). A chain is **absorbing** if:
1. There is at least one absorbing state
2. From every state, some absorbing state is reachable

For absorbing chains, we define:
- **Canonical form:** Reorder states so $P = \begin{pmatrix} Q & R \\ 0 & I \end{pmatrix}$ where $Q$ governs transitions among transient states, $R$ governs transitions from transient to absorbing states
- **Fundamental matrix:** $N = (I - Q)^{-1} = I + Q + Q^2 + \cdots$
- **Expected steps until absorption:** If starting in transient state $i$, $\mathbb{E}[\text{absorption}] = (N \mathbf{1})_i$
- **Absorption probabilities:** The probability of absorption in absorbing state $j$ when starting from transient state $i$ is $(NR)_{ij}$

Absorbing chains model processes like epidemic extinction, rumor spreading completion, or consensus formation.

---

## Continuous-Time Markov Chains (CTMC)

Continuous-time Markov chains extend discrete-time chains to continuous time, essential for modeling real-world processes where events occur at varying rates.

### Definition and Generator Matrices

A **continuous-time Markov chain** $\{X(t) : t \geq 0\}$ satisfies:

$$P(X(t+s) = j \mid X(s) = i, \{X(u) : u < s\}) = P(X(t+s) = j \mid X(s) = i) = P_{ij}(t)$$

The process is **time-homogeneous** if $P_{ij}(t)$ does not depend on $s$.

Instead of a transition matrix, CTMCs are characterized by a **generator matrix** (or rate matrix) $Q$, where:
- $Q_{ij} \geq 0$ for $i \neq j$ (transition rate from $i$ to $j$)
- $Q_{ii} = -\sum_{j \neq i} Q_{ij}$ (negative total exit rate)
- $\sum_j Q_{ij} = 0$ for all $i$

The off-diagonal entries $Q_{ij}$ represent the instantaneous rate of transitioning from $i$ to $j$.

### Exponential Holding Times

In a CTMC, the time spent in state $i$ before transitioning (the **holding time**) follows an exponential distribution with rate $q_i = -Q_{ii} = \sum_{j \neq i} Q_{ij}$:

$$P(\text{hold time} > t \mid X(0) = i) = e^{-q_i t}$$

When a transition occurs, the probability of moving to state $j \neq i$ is $Q_{ij}/q_i$.

This structure—exponential holding times followed by discrete jumps—characterizes CTMCs and enables efficient simulation.

### Kolmogorov Equations

The transition probabilities $P_{ij}(t) = P(X(t) = j \mid X(0) = i)$ satisfy differential equations:

**Forward Equation (Kolmogorov Forward / Fokker-Planck):**

$$\frac{d}{dt} P(t) = P(t) Q$$

**Backward Equation:**

$$\frac{d}{dt} P(t) = Q P(t)$$

with initial condition $P(0) = I$.

The solution is the **matrix exponential**:

$$P(t) = e^{Qt} = \sum_{k=0}^{\infty} \frac{(Qt)^k}{k!}$$

### Stationary Distribution

A distribution $\pi$ is stationary for a CTMC if $\pi^T Q = 0^T$, or equivalently:

$$\sum_{i \neq j} \pi_i Q_{ij} = \pi_j \sum_{i \neq j} Q_{ji}$$

This **global balance** states that the total flow into state $j$ equals the total flow out. Under irreducibility and positive recurrence, the stationary distribution is unique and satisfies $\lim_{t \to \infty} P(X(t) = j) = \pi_j$.

**Detailed balance** for CTMCs:

$$\pi_i Q_{ij} = \pi_j Q_{ji}$$

### Gillespie Algorithm

The **Gillespie algorithm** (or stochastic simulation algorithm) efficiently simulates CTMCs by exploiting the exponential holding time property:

```
Algorithm: Gillespie Simulation
Input: Initial state X(0), generator Q, maximum time T_max
Output: Trajectory {(t_k, X(t_k))}

1. Initialize: t = 0, X = X(0)
2. While t < T_max:
   a. Compute total rate: q_X = -Q_{XX}
   b. Sample holding time: Δt ~ Exponential(q_X)
   c. Sample next state: Choose j ≠ X with probability Q_{Xj}/q_X
   d. Update: t = t + Δt, X = j
   e. Record: Append (t, X) to trajectory
3. Return trajectory
```

**Complexity:** Each step requires $O(deg(X))$ operations to sample the next state. For $M$ transitions, total complexity is $O(M \cdot d_{\text{avg}})$.

**Optimization (Gibson-Bruck):** For large state spaces, use a priority queue or binary tree to achieve $O(\log n)$ per step.

---

## Stochastic Differential Equations (SDE)

Stochastic differential equations extend ordinary differential equations by incorporating random noise, providing continuous models for network dynamics subject to uncertainty.

### Brownian Motion (Wiener Process)

**Standard Brownian motion** $\{W(t) : t \geq 0\}$ is a continuous-time stochastic process with:
1. $W(0) = 0$
2. Independent increments: $W(t) - W(s) \perp W(u) - W(v)$ for non-overlapping intervals
3. Gaussian increments: $W(t) - W(s) \sim \mathcal{N}(0, t-s)$
4. Continuous paths (with probability 1)

Brownian motion is nowhere differentiable but has quadratic variation $[W, W](t) = t$.

### Itô Processes and Itô Calculus

An **Itô process** (or diffusion process) is defined by the SDE:

$$dX(t) = \mu(X(t), t) dt + \sigma(X(t), t) dW(t)$$

where:
- $\mu(X(t), t)$ is the **drift** (deterministic tendency)
- $\sigma(X(t), t)$ is the **diffusion coefficient** (volatility)
- $dW(t)$ represents Brownian motion increments

In integral form:

$$X(t) = X(0) + \int_0^t \mu(X(s), s) ds + \int_0^t \sigma(X(s), s) dW(s)$$

The last term is an **Itô integral**, defined as a limit of Riemann sums where the integrand is evaluated at the left endpoint of each interval.

**Itô's Lemma:** For a function $f(X(t), t)$ where $X(t)$ follows the SDE above:

$$df = \left(\frac{\partial f}{\partial t} + \mu \frac{\partial f}{\partial x} + \frac{1}{2}\sigma^2 \frac{\partial^2 f}{\partial x^2}\right) dt + \sigma \frac{\partial f}{\partial x} dW$$

The extra term $\frac{1}{2}\sigma^2 \frac{\partial^2 f}{\partial x^2}$ arises from the non-zero quadratic variation of Brownian motion ($dW^2 = dt$).

### Drift and Diffusion on Networks

Network dynamics can be modeled as SDEs on the node space. Consider opinion dynamics where each node $i$ has opinion $x_i(t) \in \mathbb{R}$:

$$dx_i = \sum_j A_{ij} (x_j - x_i) dt + \sigma dW_i$$

The drift term pulls opinions toward neighbors (social influence), while diffusion adds random fluctuations (individual variation).

In matrix form for the full network:

$$d\mathbf{x} = -L \mathbf{x} dt + \sigma d\mathbf{W}$$

where $L = D - A$ is the graph Laplacian. This is a multivariate Ornstein-Uhlenbeck process with analytical solutions.

### Connection to Network Dynamics

SDEs connect to network science through:

1. **Mean-field approximations:** Stochastic epidemic models reduce to SDEs as population size grows
2. **Opinion dynamics:** The Deffuant model and Hegselmann-Krause model have SDE limits
3. **Price processes:** Financial networks generate correlated asset prices via coupled SDEs
4. **Fluctuation analysis:** SDEs quantify fluctuations around deterministic network flows

---

## Diffusion Processes on Networks

Diffusion—where quantities spread from high to low concentration—is fundamental to understanding information flow, heat transfer, and random walks on networks.

### The Graph Laplacian

For an undirected graph with adjacency matrix $A$ and degree matrix $D$, the (unnormalized) **graph Laplacian** is:

$$L = D - A$$

**Properties:**
- $L$ is symmetric and positive semi-definite
- $L \mathbf{1} = 0$ (constant vector is eigenvector with eigenvalue 0)
- Multiplicity of 0 eigenvalue equals number of connected components
- Eigenvalues: $0 = \lambda_1 \leq \lambda_2 \leq \cdots \leq \lambda_n$
- $\lambda_2$ (algebraic connectivity) measures graph connectivity

The normalized Laplacians are:
- **Random walk normalized:** $L_{rw} = D^{-1}L = I - D^{-1}A = I - T$
- **Symmetric normalized:** $L_{sym} = D^{-1/2}LD^{-1/2} = I - D^{-1/2}AD^{-1/2}$

### Heat Kernel on Graphs

The **heat equation** on a graph describes continuous-time diffusion:

$$\frac{d\mathbf{u}}{dt} = -L\mathbf{u}$$

where $u_i(t)$ is the "heat" (or information, concentration) at node $i$ at time $t$.

The solution with initial condition $\mathbf{u}(0) = \mathbf{u}_0$ is:

$$\mathbf{u}(t) = e^{-Lt} \mathbf{u}_0 = H_t \mathbf{u}_0$$

where $H_t = e^{-Lt}$ is the **heat kernel** (matrix exponential of $-Lt$).

The heat kernel entries $[H_t]_{ij}$ represent the amount of heat transferred from node $j$ to node $i$ in time $t$. This provides a continuous-time generalization of random walk transition probabilities.

### Spectral Properties and Diffusion Modes

Spectrally decomposing $L = V \Lambda V^T$ where $\Lambda = \text{diag}(\lambda_1, \ldots, \lambda_n)$:

$$H_t = V e^{-\Lambda t} V^T = \sum_{k=1}^n e^{-\lambda_k t} \mathbf{v}_k \mathbf{v}_k^T$$

The eigenvectors $\mathbf{v}_k$ (Fiedler vectors) represent **diffusion modes**:
- $\mathbf{v}_1 = \frac{1}{\sqrt{n}}\mathbf{1}$ (uniform distribution, steady state)
- $\mathbf{v}_2$ (Fiedler vector) indicates the dominant bipartition; sign structure gives graph clustering
- Higher modes capture finer structure

Diffusion rapidly suppresses high-frequency modes (large $\lambda_k$), effectively smoothing the initial distribution.

### Diffusion Distance

The **diffusion distance** between nodes $i$ and $j$ at time $t$ measures how similarly they receive diffusing quantities:

$$D_t(i, j) = \|\mathbf{h}_i(t) - \mathbf{h}_j(t)\|_{L^2}$$

where $\mathbf{h}_i(t)$ is the $i$-th row of $H_t$ (the heat profile when starting from node $i$).

Equivalently, using the spectral decomposition:

$$D_t^2(i, j) = \sum_{k=1}^n e^{-2\lambda_k t} (v_{ki} - v_{kj})^2$$

Diffusion distance accounts for all paths between nodes, weighted by their length, and captures graph geometry at multiple scales (controlled by $t$).

---

## Epidemic Models as Stochastic Processes

Epidemic models on networks are inherently stochastic. While deterministic compartmental models (ODEs) provide mean-field approximations, stochastic models capture the discrete nature of individuals and transmission events.

### SIR Model on Networks

The **SIR model** divides the population into:
- $S$: Susceptible (can contract disease)
- $I$: Infected (can transmit disease)
- $R$: Recovered/Removed (immune or dead)

**Continuous-Time Markov Chain Formulation:**

For a network with adjacency matrix $A$, let $X_i(t) \in \{S, I, R\}$ be the state of node $i$ at time $t$.

**Transition rates:**
- Infection: $S_i \to I_i$ at rate $\beta \cdot \sum_j A_{ij} \mathbb{I}[X_j = I]$
  (susceptible node gets infected by infected neighbors at rate $\beta$ per contact)
- Recovery: $I_i \to R_i$ at rate $\gamma$

The total rate of all possible transitions determines the holding time via the Gillespie algorithm.

**State space:** $\{S, I, R\}^n$ (exponential in network size; exact simulation is infeasible for large networks).

### SEIR Extension

The **SEIR model** adds an **Exposed** (latent) compartment:
- $S \to E$ at rate $\beta$ (per infected contact)
- $E \to I$ at rate $\sigma$ (latent period ends)
- $I \to R$ at rate $\gamma$

This captures diseases with incubation periods where individuals are infected but not yet infectious.

### Gillespie Algorithms for Epidemics

Efficient simulation uses the Gillespie algorithm with two event types:

```
Algorithm: Network SIR Simulation (Gillespie)
Input: Graph G=(V,E), patient zero i_0, rates β, γ
Output: Infection times for each node

1. Initialize: All nodes S, set X_{i_0} = I at t=0
2. Maintain: Priority queue of next events
3. While infected nodes exist:
   a. For each infected node i:
      - Recovery event at t + Exp(γ)
      - For each susceptible neighbor j:
        Infection event at t + Exp(β) (per edge)
   b. Pop earliest event from queue
   c. Execute: Update states, schedule new events
4. Return infection/recovery times
```

**Next-Reaction Method:** Use a binary heap to achieve $O(\log n)$ per event, total $O(M \log n)$ for $M$ events.

### Moment Closure Approximations

Exact stochastic models are analytically intractable for large networks. **Moment closure** approximates dynamics by tracking only low-order moments (means, pairwise correlations).

Let $[S_i] = P(X_i = S)$, $[I_i] = P(X_i = I)$, and $[S_i I_j] = P(X_i = S, X_j = I)$ for edge $(i,j)$.

**Pair approximation:**

$$\frac{d[I_i]}{dt} = \beta \sum_j A_{ij} [S_i I_j] - \gamma [I_i]$$

The equation for $[S_i I_j]$ depends on triples $[S_i I_j S_k]$, leading to an infinite hierarchy. Moment closure approximates higher-order moments in terms of lower-order ones, e.g.:

$$[S_i I_j S_k] \approx \frac{[S_i I_j][I_j S_k]}{[I_j]}$$

This assumes conditional independence given the middle node.

### Basic Reproduction Number $R_0$

The **basic reproduction number** $R_0$ is the expected number of secondary infections caused by a single infected individual in a fully susceptible population. For network SIR:

$$R_0 = \frac{\beta}{\gamma} \cdot \frac{\langle k^2 \rangle - \langle k \rangle}{\langle k \rangle}$$

where $\langle k \rangle$ and $\langle k^2 \rangle$ are the mean and mean-squared degree.

**Epidemic threshold:** If $R_0 < 1$, the disease dies out almost surely; if $R_0 > 1$, there is positive probability of a major outbreak.

For scale-free networks with degree distribution $P(k) \sim k^{-\alpha}$ and $\alpha \leq 3$, $\langle k^2 \rangle$ diverges, implying $R_0 \to \infty$ and no epidemic threshold—any positive infection rate leads to potential epidemics.

---

## Branching Processes

Branching processes model population growth and extinction, with direct applications to cascade sizes in networks, rumor spreading, and viral marketing.

### Galton-Watson Branching Process

The **Galton-Watson process** is the simplest branching process:
- Start with a single individual (the progenitor)
- Each individual produces $k$ offspring with probability $p_k$, independently
- Offspring become parents in the next generation
- Process continues indefinitely or until extinction

Let $Z_n$ be the population in generation $n$, with $Z_0 = 1$. The generating function for offspring distribution is:

$$f(s) = \sum_{k=0}^{\infty} p_k s^k$$

The generating function for $Z_n$ is the $n$-fold composition: $f_n(s) = f(f(\cdots f(s)\cdots))$.

### Extinction Probabilities

The **extinction probability** $q = P(\text{extinction})$ satisfies:

$$q = f(q)$$

**Theorem:**
- If $\mu = f'(1) \leq 1$ (subcritical or critical), then $q = 1$ (certain extinction)
- If $\mu > 1$ (supercritical), then $q < 1$ is the unique solution in $[0, 1)$ to $q = f(q)$

The expected population grows as $\mathbb{E}[Z_n] = \mu^n$.

### Relation to Cascade Size in Networks

In network cascades (information spread, adoption, infection), the cascade size distribution approximates a branching process when:
- The network is large and locally tree-like (few short cycles)
- Activation decisions are independent across edges

**Approximation:** Starting from a random node, the number of new activations at each wave follows approximately the **excess degree distribution**:

$$q_k = \frac{(k+1)p_{k+1}}{\langle k \rangle}$$

where $p_k$ is the degree distribution. This accounts for the friendship paradox: reached nodes have higher expected degree than random nodes.

The cascade size distribution can be computed via generating functions:

$$H(s) = s \cdot G_0(H_1(s))$$

where $G_0$ generates the degree distribution and $H_1$ satisfies $H_1(s) = s \cdot G_1(H_1(s))$ with $G_1$ generating the excess degree.

For global cascades (percolation), the condition is $\mathbb{E}[\text{excess degree}] \cdot T > 1$ where $T$ is the transmission probability.

---

## Point Processes on Networks

Point processes model discrete events in continuous time, ideal for social interactions, message passing, or financial transactions on networks.

### Poisson Processes

A **Poisson process** on a network generates events at each node (or edge) with exponentially distributed inter-event times:
- Events at node $i$ occur at rate $\lambda_i$
- Inter-event times $\sim \text{Exp}(\lambda_i)$
- Events are independent across nodes

The number of events in interval $[0, t]$ follows a Poisson distribution: $N(t) \sim \text{Poisson}(\lambda t)$.

### Hawkes Processes

**Hawkes processes** model **mutually-exciting** events—where one event increases the probability of future events. Essential for modeling social contagion, retweet cascades, or market shocks.

The conditional intensity at node $i$ at time $t$:

$$\lambda_i(t) = \mu_i + \sum_{j} \sum_{t_k^{(j)} < t} \phi_{ij}(t - t_k^{(j)})$$

where:
- $\mu_i \geq 0$ is the baseline intensity (exogenous events)
- $t_k^{(j)}$ are event times at node $j$
- $\phi_{ij}(\tau) \geq 0$ is the **triggering kernel** (influence of $j$ on $i$ after delay $\tau$)

Common kernel choices:
- Exponential: $\phi_{ij}(\tau) = \alpha_{ij} e^{-\beta \tau}$
- Power-law: $\phi_{ij}(\tau) = \alpha_{ij} (\tau + c)^{-(1+\theta)}$

**Branching ratio:** $\rho = \sum_{i,j} \int_0^{\infty} \phi_{ij}(\tau) d\tau / \sum_i \mu_i$. If $\rho < 1$, the process is subcritical (finite cascades); if $\rho > 1$, it may explode.

### Time Rescaling

The **time rescaling theorem** enables goodness-of-fit testing and simulation:

Given a point process with conditional intensity $\lambda(t)$, define the rescaled time:

$$\Lambda(t) = \int_0^t \lambda(s) ds$$

Then the rescaled event times $\{\Lambda(t_k)\}$ form a homogeneous Poisson process with rate 1.

This transforms general point processes to simple Poisson processes, facilitating hypothesis testing and confidence interval construction.

---

## Stochastic Block Models

Stochastic Block Models (SBMs) generate random graphs with community structure, serving as both generative models and inference targets for community detection.

### SBM as a Generative Process

The **Stochastic Block Model** with $K$ communities:
- Each node $i$ has community assignment $z_i \in \{1, \ldots, K\}$ (latent)
- Edge $(i, j)$ exists with probability $P_{z_i z_j}$ depending only on community memberships

**Matrix formulation:** Let $Z \in \{0, 1\}^{n \times K}$ be the assignment matrix ($Z_{ik} = 1$ iff $z_i = k$). The expected adjacency is:

$$\mathbb{E}[A] = Z P Z^T$$

**Variants:**
- **Binary SBM:** $A_{ij} \sim \text{Bernoulli}(P_{z_i z_j})$
- **Degree-corrected SBM (DCSBM):** $A_{ij} \sim \text{Bernoulli}(\theta_i \theta_j P_{z_i z_j})$ with degree parameters $\theta_i$

### SBM as a Stochastic Process

The SBM can be viewed as a two-stage stochastic process:
1. **Latent stage:** Sample community assignments $z_i \sim \text{Categorical}(\pi)$ for each node
2. **Observation stage:** Sample edges $A_{ij} \sim \text{Bernoulli}(P_{z_i z_j})$ conditionally independently

This generative perspective enables Bayesian inference over community structure.

### Relation to Community Detection

Community detection aims to recover $Z$ from observed $A$. The **likelihood** is:

$$P(A \mid Z, P) = \prod_{i<j} P_{z_i z_j}^{A_{ij}} (1 - P_{z_i z_j})^{1 - A_{ij}}$$

**Inference methods:**
- **Maximum likelihood:** $\hat{Z}, \hat{P} = \arg\max_{Z, P} P(A \mid Z, P)$ (computationally intractable)
- **Variational EM:** Approximate posterior $q(Z) \approx P(Z \mid A)$
- **MCMC:** Sample from $P(Z, P \mid A)$ using Gibbs or Metropolis-Hastings

**Statistical limits:** The **detectability threshold** states that communities are statistically recoverable if and only if:

$$(a - b)^2 > K(a + (K-1)b)$$

for a symmetric SBM with $P_{kk} = a/n$ and $P_{k\ell} = b/n$ for $k \neq \ell$.

---

## Markov Random Fields

Markov Random Fields (MRFs) generalize Markov chains to arbitrary graph structures, providing the probabilistic foundation for undirected graphical models.

### MRF Definition

An MRF over graph $G = (V, E)$ is a joint distribution $P(X_1, \ldots, X_n)$ satisfying:

$$P(X_i \mid X_{V \setminus \{i\}}) = P(X_i \mid X_{\mathcal{N}(i)})$$

where $\mathcal{N}(i)$ are the neighbors of $i$ in $G$. Each variable is conditionally independent of all others given its neighbors—the **local Markov property**.

### Gibbs Sampling as a Markov Chain

**Gibbs sampling** generates samples from an MRF by iteratively sampling each variable from its conditional:

```
Algorithm: Gibbs Sampling for MRF
Input: Graph G, conditional distributions P(X_i | X_{N(i)})
Output: Samples approximately from joint distribution

1. Initialize: x^(0) arbitrarily
2. For t = 1 to T:
   a. For each node i in V (or random order):
      Sample: x_i^(t) ~ P(X_i | x_{N(i)}^(t-1))
   b. Update: x^(t) = (x_1^(t), ..., x_n^(t))
3. Return samples {x^(B), ..., x^(T)} (after burn-in B)
```

**Properties:**
- Gibbs sampling defines a Markov chain on the state space $\mathcal{X}^n$
- The chain is reversible with respect to the target distribution $P(X)$
- Under mild conditions, the chain converges to $P(X)$

### Mixing Properties

The **mixing time** of Gibbs sampling depends on graph structure:
- Trees: Rapid mixing (polynomial in $n$)
- General graphs: Can be exponentially slow (torpid mixing)
- High-temperature regimes: Faster mixing
- Low-temperature (near-deterministic): Slower mixing, potential multimodality

**Dobrushin condition:** If the total influence of neighbors is bounded:

$$\max_i \sum_{j \in \mathcal{N}(i)} C_{ij} < 1$$

where $C_{ij}$ measures coupling strength, then Gibbs sampling mixes rapidly.

---

## Applications to Social/Economic Networks

Stochastic processes provide powerful models for social and economic phenomena on networks.

### Opinion Dynamics as Stochastic Processes

Opinion formation models can be formulated as stochastic processes:

**Voter Model:** Each node holds opinion $\pm 1$. At rate 1, a random node adopts a random neighbor's opinion. This is a continuous-time Markov chain with absorbing states at consensus.

**Mean exit time** from discord to consensus scales as $O(n^2)$ on a line, $O(n \log n)$ on expanders, $O(n)$ on complete graphs.

**Deffuant Model (Bounded Confidence):** Nodes interact if opinion difference is below threshold $\epsilon$. With noise, this becomes an SDE:

$$dx_i = \sum_j A_{ij} (x_j - x_i) \mathbb{I}[|x_j - x_i| < \epsilon] dt + \sigma dW_i$$

### Market Volatility as Network-Driven Diffusion

Financial networks generate correlated volatility through coupled SDEs:

$$dS_i = \mu_i S_i dt + \sigma_i S_i \left(\sqrt{1 - \rho} dW_i + \sqrt{\rho} dW_{\text{market}}\right)$$

The correlation structure $\rho$ emerges from the network of cross-holdings, common exposures, and information channels.

Systemic risk—the risk of cascade failures—can be modeled as a branching process where bank defaults trigger counterparty defaults.

### Rumor Spreading

Rumor spreading models combine branching processes with network structure:
- **Ignorant-Spreader-Stifler (ISS):** Nodes transition through states as they learn, spread, and lose interest in a rumor
- **Push-pull protocols:** Nodes actively query neighbors for information

The completion time (time until all nodes know the rumor) on expander graphs is $O(\log n)$ with high probability.

---

## How Lutufi Uses Stochastic Processes

Lutufi leverages stochastic processes across multiple computational modules:

### Temporal Network Models

**Dynamic Bayesian Networks (DBNs):** Lutufi implements DBNs as two-time-slice BNs (2TBNs) where the transition model is a Markov chain:

$$P(X^{(t)} \mid X^{(t-1)}) = \prod_i P(X_i^{(t)} \mid \text{Pa}(X_i^{(t)}))$$

The parents $\text{Pa}(X_i^{(t)})$ can include variables from both the current and previous time slices, enabling temporal dependencies of arbitrary order.

**Learning:** Parameter learning in DBNs uses the EM algorithm with forward-backward smoothing (analogous to HMMs) or particle filtering for non-linear/non-Gaussian cases.

### Simulation Engines

Lutufi's simulation module provides efficient implementations of:
- **Gillespie algorithm:** For exact CTMC simulation on networks
- **SDE integrators:** Euler-Maruyama, Milstein methods for continuous dynamics
- **Branching process simulators:** For cascade size distributions
- **Hawkes process engines:** With exponential and power-law kernels

All simulators support network-structured state spaces and parallel execution.

### Inference Over Time-Series

For temporal network data, Lutufi implements:
- **Filtering:** Particle filters and Kalman filters for online state estimation
- **Smoothing:** Forward-backward algorithms for retrospective inference
- **Prediction:** Rolling-horizon forecasts using learned transition models
- **Structure learning:** Scoring temporal networks using BIC, AIC, or Bayesian scores

### Monte Carlo Methods

Lutufi's inference engine relies heavily on Monte Carlo methods:
- **MCMC:** Gibbs sampling, Metropolis-Hastings, and Hamiltonian Monte Carlo for posterior inference
- **Importance sampling:** For rare event estimation (cascades, defaults)
- **Sequential Monte Carlo:** For online inference in dynamic networks

All methods are implemented with numerical stability in log-space and support for sparse network structures.

---

## Key References

1. Norris, J. R. (1997). *Markov Chains*. Cambridge University Press. The definitive reference for discrete and continuous-time Markov chains, covering ergodic theory, mixing times, and applications.

2. Allen, L. J. S. (2003). *An Introduction to Stochastic Processes with Applications to Biology*. Pearson. Excellent treatment of branching processes, epidemics, and biological applications.

3. Gardiner, C. W. (2009). *Stochastic Methods: A Handbook for the Natural and Social Sciences* (4th ed.). Springer. Comprehensive coverage of SDEs, master equations, and Fokker-Planck equations.

4. Van Mieghem, P. (2010). *Graph Spectra for Complex Networks*. Cambridge University Press. Deep treatment of graph Laplacians, spectral properties, and their role in diffusion and random walks.

5. Van Mieghem, P. (2014). *Performance Analysis of Complex Networks and Systems*. Cambridge University Press. Extensive coverage of epidemic processes on networks, including exact and approximate methods.

6. Daley, D. J., & Vere-Jones, D. (2003). *An Introduction to the Theory of Point Processes* (2nd ed., Vols. I-II). Springer. The definitive reference for point processes, including Hawkes processes.

7. Laub, P. J., Taimre, T., & Pollett, P. K. (2015). Hawkes Processes. *arXiv:1507.02822*. Accessible introduction to Hawkes processes with computational considerations.

8. Newman, M. E. J. (2018). *Networks* (2nd ed.). Oxford University Press. Broad coverage of network science including random walks, epidemics, and generative models.

9. Pastor-Satorras, R., Castellano, C., Van Mieghem, P., & Vespignani, A. (2015). Epidemic processes in complex networks. *Reviews of Modern Physics*, 87(3), 925. Comprehensive review of epidemic modeling on networks.

10. Lambiotte, R., Masuda, N., & Salnikov, V. (2016). Relating modularity maximization and stochastic block models in multilayer networks. *arXiv:1606.01556*. Connects SBMs to community detection and spectral methods.

11. Kolaczyk, E. D. (2009). *Statistical Analysis of Network Data: Methods and Models*. Springer. Statistical foundations for network analysis including missing data and sampling.

12. Murphy, K. P. (2012). *Machine Learning: A Probabilistic Perspective*. MIT Press. Chapter on DBNs and state-space models with practical algorithms.

13. Robert, C. P., & Casella, G. (2004). *Monte Carlo Statistical Methods* (2nd ed.). Springer. Comprehensive reference for MCMC methods including convergence diagnostics.

14. Hoffmann, M. D., & Gelman, A. (2014). The No-U-Turn Sampler: Adaptively Setting Path Lengths in Hamiltonian Monte Carlo. *Journal of Machine Learning Research*, 15, 1593-1623. State-of-the-art MCMC method implemented in Lutufi.

---

*This document is part of the Lutufi Mathematical Foundations series. For related topics, see `BAYESIAN_NETWORKS.md`, `CONDITIONAL_INDEPENDENCE.md`, and `MATHEMATICS.md`.*
