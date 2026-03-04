# Temporal and Dynamic Networks

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Abstract

This document presents a comprehensive treatment of temporal and dynamic networks, essential for Lutufi's Dynamic Bayesian Network (DBN) capabilities. Real social and economic networks are inherently temporal — relationships form, evolve, and dissolve over time. Static analysis captures only snapshots, losing crucial information about causal sequences, propagation dynamics, and evolutionary patterns. We cover representations, path concepts, centrality measures, models, and statistical methods for temporal networks, with explicit connections to Lutufi's DBN architecture.

---

## Table of Contents

1. [Why Networks are Temporal](#1-why-networks-are-temporal)
2. [Representations of Temporal Networks](#2-representations-of-temporal-networks)
3. [Temporal Paths](#3-temporal-paths)
4. [Centrality in Temporal Networks](#4-centrality-in-temporal-networks)
5. [Temporal Clustering and Communities](#5-temporal-clustering-and-communities)
6. [Burstiness and Temporal Patterns](#6-burstiness-and-temporal-patterns)
7. [Temporal Correlations](#7-temporal-correlations)
8. [Temporal Network Models](#8-temporal-network-models)
9. [Temporal Motifs](#9-temporal-motifs)
10. [Spreader Dynamics on Temporal Networks](#10-spreader-dynamics-on-temporal-networks)
11. [Network Evolution](#11-network-evolution)
12. [Data Collection and Representation](#12-data-collection-and-representation)
13. [Statistical Analysis of Temporal Networks](#13-statistical-analysis-of-temporal-networks)
14. [Applications to Social/Economic Networks](#14-applications-to-socialeconomic-networks)
15. [How Lutufi Handles Temporal Networks](#15-how-lutufi-handles-temporal-networks)
16. [Key References](#16-key-references)

---

## 1. Why Networks are Temporal

### 1.1 The Dynamic Nature of Real Systems

Real-world networks are not static; they evolve continuously:

**Social Networks:**
- Friendships form, strengthen, weaken, and dissolve
- Professional relationships develop over careers
- Communication patterns vary by time of day, week, season

**Economic Networks:**
- Trade relationships change with market conditions
- Credit relationships have durations and renewals
- Supply chains adapt to disruptions and opportunities

**Information Networks:**
- Hyperlinks are created and removed
- Citations accumulate over time
- Content spreads in temporal bursts

### 1.2 Loss of Information in Static Analysis

Aggregating temporal data into static networks loses critical information:

| Information Lost | Consequence |
|-----------------|-------------|
| Edge ordering | Cannot determine causal sequences |
| Edge duration | Cannot distinguish transient from persistent ties |
| Inter-event times | Cannot capture burstiness or periodicity |
| Concurrent edges | Cannot identify simultaneous interactions |
| Network evolution | Cannot model growth, adaptation, aging |

### 1.3 Temporal Analysis Questions

Temporal network analysis addresses questions impossible to answer from static snapshots:
- Which sequence of interactions leads to contagion?
- When is information most likely to reach a target?
- How do communities emerge, merge, split, and dissolve?
- What temporal patterns precede network changes?
- How does network structure co-evolve with node attributes?

---

## 2. Representations of Temporal Networks

### 2.1 Time-Ordered Graph (Events)

**Definition 2.1 (Time-Ordered Graph).** A *time-ordered graph* represents temporal networks as a set of timestamped events:
$$\mathcal{G}_T = (V, E_T)$$

where $E_T = \{(u, v, t, \Delta t) : \text{edge from } u \text{ to } v \text{ at time } t \text{ with duration } \Delta t\}$

For instantaneous contacts, $\Delta t = 0$ and we write $(u, v, t)$.

### 2.2 Contact Sequences

**Definition 2.2 (Contact Sequence).** A *contact sequence* is an ordered list of interactions:
$$C = ((u_1, v_1, t_1), (u_2, v_2, t_2), \ldots, (u_m, v_m, t_m))$$

where $t_1 \leq t_2 \leq \cdots \leq t_m$.

Contact sequences are the rawest representation, preserving all temporal information without aggregation.

### 2.3 Time-Aggregated Snapshots

**Definition 2.3 (Time-Aggregated Snapshot).** Divide the observation period $[0, T]$ into intervals $[t_0, t_1), [t_1, t_2), \ldots, [t_{k-1}, t_k)$. The *snapshot* at interval $i$:
$$G_i = (V, E_i) \text{ where } E_i = \{(u,v) : \exists (u,v,t) \in E_T \text{ with } t \in [t_{i-1}, t_i)\}$$

**Aggregation Trade-offs:**
- Fine-grained: Captures dynamics but may be noisy
- Coarse-grained: Smoother but loses temporal detail
- The optimal aggregation level depends on the research question

### 2.4 Time-Varying Graphs

**Definition 2.4 (Time-Varying Graph).** A *time-varying graph* explicitly models the graph as a function of time:
$$G(t) = (V, E(t))$$

where $E(t) \subseteq V \times V$ is the edge set at time $t$. The graph topology changes as edges appear and disappear.

### 2.5 Link Stream Model

**Definition 2.5 (Link Stream).** A *link stream* is a quadruple $L = (T, V, E, \mathcal{T})$ where:
- $T \subseteq \mathbb{R}$ is the time interval
- $V$ is the set of vertices
- $E \subseteq V \times V$ is the set of possible edges
- $\mathcal{T}: E \rightarrow \mathcal{P}(T)$ maps each edge to the set of times it is active

The link stream elegantly handles edges that are active over intervals rather than instantaneous.

**Comparison of Representations:**

| Representation | Best For | Limitations |
|---------------|----------|-------------|
| Contact sequence | Detailed interaction data | Computationally expensive |
| Snapshots | Discrete-time analysis | Loses intra-interval timing |
| Time-varying graph | Continuous dynamics | May be continuous-time intractable |
| Link stream | Interval-activity edges | Assumes edge persistence |

---

## 3. Temporal Paths

### 3.1 Time-Respecting Paths

**Definition 3.1 (Time-Respecting Path).** A *time-respecting path* (or *journey*) from $u$ to $v$ is a sequence of edges:
$$P = ((u_0, u_1, t_1), (u_1, u_2, t_2), \ldots, (u_{k-1}, u_k, t_k))$$

where $u_0 = u$, $u_k = v$, and $t_1 < t_2 < \cdots < t_k$.

Unlike static paths, temporal paths must respect the arrow of time — we cannot traverse edges in reverse chronological order.

### 3.2 Types of Temporal Paths

**Definition 3.2 (Fastest Path).** The *fastest path* from $u$ to $v$ minimizes traversal time:
$$\arg\min_P (t_k - t_1)$$

This is crucial for understanding information propagation speed.

**Definition 3.3 (Foremost Path).** The *foremost path* from $u$ to $v$ starting at time $t_0$ minimizes arrival time:
$$\arg\min_P t_k \text{ subject to } t_1 \geq t_0$$

**Definition 3.4 (Shortest Temporal Path).** The *shortest temporal path* minimizes the number of hops:
$$\arg\min_P k$$

These definitions may yield different paths. For example, a direct but late edge may be shortest but not fastest.

### 3.3 Reachability in Temporal Networks

**Definition 3.5 (Temporal Reachability).** Vertex $v$ is *temporally reachable* from $u$ starting at time $t_0$ if there exists a time-respecting path from $u$ to $v$ with all edges after $t_0$.

**Theorem 3.1 (Reachability is Not Symmetric).** Temporal reachability is inherently asymmetric: $u$ can reach $v$ but not vice versa, even in undirected temporal networks.

**Algorithm 3.1 (Temporal Reachability via Modified BFS).**

```
procedure TemporalReachability(G_T, source, start_time):
    earliest_arrival ← array of size n, initialized to ∞
    earliest_arrival[source] ← start_time
    queue ← priority queue ordered by arrival time
    enqueue(queue, (start_time, source))
    
    while queue not empty:
        (current_time, u) ← dequeue(queue)
        for each edge (u, v, t) where t > current_time:
            if t < earliest_arrival[v]:
                earliest_arrival[v] ← t
                enqueue(queue, (t, v))
    
    return earliest_arrival
```

**Complexity:** $O(m \log n)$ using a priority queue.

### 3.4 Temporal Distance Metrics

**Definition 3.6 (Temporal Distance).** Multiple temporal distance metrics exist:
- **Earliest arrival time:** $\min_P t_k$ for paths starting after $t_0$
- **Minimum hop count:** $\min_P |P|$
- **Minimum traversal time:** $\min_P (t_k - t_1)$
- **Minimum waiting time:** $\min_P \sum \text{waiting time at intermediate nodes}$

These metrics form a metric space but do not satisfy all properties simultaneously.

---

## 4. Centrality in Temporal Networks

### 4.1 Temporal Betweenness

**Definition 4.1 (Temporal Betweenness).** *Temporal betweenness* measures the fraction of temporal shortest paths passing through a vertex:
$$C_B^{\text{temp}}(v) = \sum_{s \neq v \neq t} \frac{\sigma_{st}^{\text{temp}}(v)}{\sigma_{st}^{\text{temp}}}$$

where $\sigma_{st}^{\text{temp}}$ counts temporal shortest paths.

**Interpretation:** Nodes with high temporal betweenness control the timing of information flow, not just its topology.

**Algorithm 4.1 (Computing Temporal Betweenness).**

```
procedure TemporalBetweenness(G_T):
    CB ← array of size n, initialized to 0
    
    for each source s and start time t_0:
        // Compute all temporal shortest paths
        (paths, counts) ← TemporalShortestPaths(G_T, s, t_0)
        
        // Accumulate dependencies
        for each target t:
            for each vertex v on paths from s to t:
                if v ≠ s and v ≠ t:
                    CB[v] += counts[s→t via v] / counts[s→t]
    
    return CB
```

**Complexity:** $O(n \cdot m \cdot T)$ where $T$ is the number of time steps.

### 4.2 Temporal Closeness

**Definition 4.2 (Temporal Closeness).** *Temporal closeness* is the reciprocal of average temporal distance:
$$C_C^{\text{temp}}(v) = \frac{n - 1}{\sum_{u \neq v} d^{\text{temp}}(v, u)}$$

where $d^{\text{temp}}$ can be earliest arrival, minimum hops, or other temporal distance.

### 4.3 Time-Dependent PageRank

**Definition 4.3 (Temporal PageRank).** *Temporal PageRank* extends PageRank to respect temporal order:

$$PR^{\text{temp}}(v, t) = \alpha \sum_{u: (u,v,t') \in E_T, t' < t} \frac{PR^{\text{temp}}(u, t')}{\deg^{\text{out}}(u, t')} + (1 - \alpha) \frac{1}{n}$$

This can be computed forward in time, incorporating only past influence.

### 4.4 Dynamic Communicability

**Definition 4.4 (Dynamic Communicability).** *Dynamic communicability* counts weighted temporal walks:

$$Q_{ij} = \sum_{k=1}^{\infty} \sum_{\text{temp walks } P: i \to j} \prod_{e \in P} \alpha w(e)$$

where walks must respect temporal ordering, and $w(e)$ are edge weights.

This generalizes Katz centrality to temporal networks.

---

## 5. Temporal Clustering and Communities

### 5.1 Tracking Communities Over Time

**Definition 5.1 (Community Evolution).** Communities evolve through events:
- **Birth:** New community appears
- **Death:** Community disappears
- **Growth:** Community gains members
- **Contraction:** Community loses members
- **Merge:** Two communities combine
- **Split:** One community divides
- **Persist:** Community continues unchanged

**Algorithm 5.1 (Community Tracking).**

```
procedure TrackCommunities(communities_over_time):
    events ← empty list
    
    for t from 1 to T-1:
        C_t ← communities at time t
        C_{t+1} ← communities at time t+1
        
        // Compute Jaccard similarity between consecutive communities
        similarity_matrix ← compute_jaccard(C_t, C_{t+1})
        
        // Match communities
        matches ← bipartite_matching(similarity_matrix, threshold=0.5)
        
        // Identify events
        for c in C_t:
            if c unmatched: events.add((t, "death", c))
        for c in C_{t+1}:
            if c unmatched: events.add((t+1, "birth", c))
        for (c1, c2) in matches:
            if |c1| < |c2|: events.add((t+1, "growth", c2))
            elif |c1| > |c2|: events.add((t+1, "contraction", c2))
            else: events.add((t+1, "persist", c2))
    
    return events
```

### 5.2 Temporal Modularity

**Definition 5.2 (Temporal Modularity).** *Temporal modularity* extends modularity to incorporate time:

$$Q^{\text{temp}} = \frac{1}{T} \sum_{t=1}^T Q(G_t, C_t) - \lambda \sum_{t=1}^{T-1} \text{cost}(C_t, C_{t+1})$$

where $Q(G_t, C_t)$ is static modularity at time $t$, and the cost term penalizes community changes for smoothness.

### 5.3 Online Community Detection

**Definition 5.3 (Online Detection).** *Online* (incremental) community detection updates communities as edges arrive, without reprocessing the entire history.

**Algorithm 5.2 (Incremental Louvain).**

```
procedure IncrementalLouvain(G_t, C_{t-1}, new_edges):
    // Start from previous communities
    C_t ← C_{t-1}
    
    // Add new edges
    for (u, v) in new_edges:
        // Local search for community improvements
        delta_modularity ← compute_modularity_change(u, v, C_t)
        if delta_modularity > 0:
            update_communities(C_t, u, v)
    
    // Optional: periodic global optimization
    if should_optimize():
        C_t ← Louvain(G_t)
    
    return C_t
```

### 5.4 Persistence and Stability

**Definition 5.4 (Community Persistence).** The *persistence* of a community is the fraction of time it exists:
$$\text{Persistence}(c) = \frac{|\{t : c \in C_t\}|}{T}$$

**Definition 5.5 (Community Stability).** *Stability* measures how much community membership changes:
$$\text{Stability}(c) = 1 - \frac{1}{|T_c| - 1} \sum_{t \in T_c} \frac{|c_t \Delta c_{t+1}|}{|c_t \cup c_{t+1}|}$$

where $\Delta$ is symmetric difference.

---

## 6. Burstiness and Temporal Patterns

### 6.1 Bursty Activity Patterns

**Definition 6.1 (Burstiness).** *Burstiness* refers to the clustering of events in time — periods of high activity separated by periods of quiescence.

**Definition 6.2 (Burstiness Parameter).** Following Goh and Barabási, the burstiness parameter for a sequence of inter-event times $\{\tau_i\}$:
$$B = \frac{\sigma_\tau - \mu_\tau}{\sigma_\tau + \mu_\tau}$$

where $\mu_\tau$ and $\sigma_\tau$ are mean and standard deviation. $B \in [-1, 1]$ with $B = 0$ for Poisson, $B > 0$ for bursty, $B < 0$ for regular.

### 6.2 Inter-Event Time Distributions

**Definition 6.3 (Inter-Event Time).** The *inter-event time* $\tau$ is the time between consecutive interactions involving a given node or edge.

Common distributions in real networks:
- **Power law:** $P(\tau) \sim \tau^{-\alpha}$ (bursty, scale-free waiting times)
- **Log-normal:** Heavy-tailed but with finite moments
- **Weibull:** Flexible shape parameter captures various patterns
- **Exponential:** Memoryless (Poisson) — rare in real data

### 6.3 Circadian Rhythms and Periodicities

Real networks often exhibit periodic patterns:
- **Daily cycles:** Communication peaks during waking hours
- **Weekly cycles:** Reduced activity on weekends
- **Seasonal patterns:** Academic networks follow semester schedules
- **Event-driven:** Spikes during significant events

**Definition 6.4 (Circadian Pattern).** The *circadian pattern* for node $v$:
$$\rho_v(h) = \frac{\text{activity in hour } h}{\text{average hourly activity}}$$

where $h \in [0, 24)$.

### 6.4 Burstiness Paradox

**Definition 6.5 (Burstiness Paradox).** Bursty interaction patterns paradoxically slow down spreading processes despite higher peak activity.

**Explanation:** Bursty patterns create periods of network isolation (no active edges) that trap diffusion processes. The long waiting times between bursts delay propagation more than the benefits of clustered activity.

**Theorem 6.1 (Effect of Burstiness on Diffusion).** For a SI (Susceptible-Infected) process, the expected time to full infection increases with burstiness, even holding the total number of contacts constant.

---

## 7. Temporal Correlations

### 7.1 Non-Markovian Properties

**Definition 7.1 (Markovian Process).** A temporal network is *Markovian* if the probability of future edges depends only on the current state, not the history.

**Theorem 7.1 (Real Networks are Non-Markovian).** Empirical temporal networks exhibit strong non-Markovian properties — the probability of future edges depends on the distant past, not just recent activity.

### 7.2 Link Persistence (Reinforcement)

**Definition 7.2 (Link Persistence).** *Link persistence* (or *reinforcement*) is the tendency for edges that have occurred recently to occur again:
$$P((u,v) \text{ at } t+1 | (u,v) \text{ at } t) > P((u,v) \text{ at } t+1)$$

**Definition 7.3 (Reciprocity Dynamics).** *Reciprocity* in temporal networks — the probability that a reply follows a message:
$$P(v \to u \text{ at } t+\Delta t | u \to v \text{ at } t)$$

Real communication networks show strong reciprocity with characteristic time scales.

### 7.3 Memory Effects

**Definition 7.4 (Network Memory).** *Memory* in temporal networks refers to correlations between current and past network states extending beyond immediate recency.

**Types of Memory:**
- **Edge memory:** Repeated contacts between same nodes
- **Triadic memory:** Closure of wedges over time
- **Community memory:** Persistence of group structures

**Algorithm 7.1 (Testing for Memory Effects).**

```
procedure TestMemory(G_T, lag):
    // Compute empirical transition probabilities
    P_empirical ← compute_edge_transitions(G_T, lag)
    
    // Generate null model (time-shuffled)
    null_distributions ← empty list
    for i from 1 to iterations:
        G_null ← shuffle_times(G_T)
        P_null ← compute_edge_transitions(G_null, lag)
        append null_distributions with P_null
    
    // Compare empirical to null
    z_scores ← (P_empirical - mean(null)) / std(null)
    
    return z_scores
```

---

## 8. Temporal Network Models

### 8.1 Dynamic ERGMs (TERGMs)

**Definition 8.1 (Temporal ERGM).** A *Temporal Exponential Random Graph Model* (TERGM) specifies the joint distribution of networks across time:

$$P(G_1, G_2, \ldots, G_T) = \frac{1}{Z(\theta)} \exp\left(\sum_t \sum_i \theta_i s_i(G_t) + \sum_t \sum_j \eta_j r_j(G_t, G_{t-1})\right)$$

where $s_i$ are static statistics and $r_j$ are temporal dependence statistics.

**Temporal Statistics:**
- Edge stability: $\sum_{(u,v)} A_{uv}^{(t)} A_{uv}^{(t-1)}$
- Edge formation: $\sum_{(u,v)} (1 - A_{uv}^{(t-1)}) A_{uv}^{(t)}$
- Edge dissolution: $\sum_{(u,v)} A_{uv}^{(t-1)} (1 - A_{uv}^{(t)})$

### 8.2 Stochastic Actor-Oriented Models (SAOMs)

**Definition 8.2 (SAOM).** *Stochastic Actor-Oriented Models* (SAOMs), implemented in SIENA, model network dynamics as a continuous-time Markov process where actors control their outgoing ties.

**Rate Function:** The rate at which actor $i$ makes changes:
$$\lambda_i(\mathbf{x}, t) = \rho_i \exp\left(\sum_k \alpha_k s_{ik}(\mathbf{x})\right)$$

**Objective Function:** When making a change, actor $i$ chooses the new network state $\mathbf{x}'$ maximizing:
$$f_i(\mathbf{x}, \mathbf{x}') = \sum_k \beta_k s_{ik}(\mathbf{x}, \mathbf{x}')$$

### 8.3 Temporal SBM

**Definition 8.3 (Temporal SBM).** The *Temporal Stochastic Block Model* assumes block memberships may change over time:

$$P(A_{ij}^{(t)} = 1 | z_i^{(t)}, z_j^{(t)}) = B_{z_i^{(t)}, z_j^{(t)}}$$

**Block Transition:** Block memberships evolve according to a Markov chain:
$$P(z_i^{(t)} = b | z_i^{(t-1)} = a) = \Pi_{ab}$$

### 8.4 Continuous-Time Markov Chains for Edge Dynamics

**Definition 8.4 (CTMC Edge Model).** Model each edge as a continuous-time Markov chain with two states (active/inactive) and transition rates:
- $\lambda_{ij}$: rate of edge formation (inactive → active)
- $\mu_{ij}$: rate of edge dissolution (active → inactive)

**Equilibrium Properties:**
- Expected edge duration: $1/\mu_{ij}$
- Steady-state edge probability: $\lambda_{ij} / (\lambda_{ij} + \mu_{ij})$

---

## 9. Temporal Motifs

### 9.1 Time-Ordered Subgraph Patterns

**Definition 9.1 (Temporal Motif).** A *temporal motif* is an isomorphism class of time-ordered subgraphs with specified temporal constraints.

**Definition 9.2 (Δt-Motif).** A *$\Delta t$-motif* is a temporal motif where all edges occur within a time window of duration $\Delta t$.

### 9.2 Motif Detection Algorithms

**Algorithm 9.1 (Counting Temporal Motifs).**

```
procedure CountTemporalMotifs(G_T, motif_template, delta_t):
    count ← 0
    
    for each central edge (u, v, t) in G_T:
        // Find edges in temporal neighborhood
        neighborhood ← edges in [t, t + delta_t] involving u or v
        
        // Count motif instances centered at (u, v, t)
        for each combination of edges forming motif_template:
            if is_temporal_match(combination, motif_template):
                count ← count + 1
    
    // Adjust for overcounting
    return count / symmetry_factor(motif_template)
```

### 9.3 Significance Testing Against Null Models

To determine if a temporal motif is significant, compare to null models:

**Null Model 1 (Time-Shuffled):** Shuffle timestamps while preserving graph structure.
**Null Model 2 (Edge-Shuffled):** Shuffle edges while preserving timestamps.
**Null Model 3 (Configuration):** Preserve degree sequence and timestamp distribution.

**Z-Score:**
$$Z(M) = \frac{N_M^{\text{obs}} - \mathbb{E}[N_M^{\text{null}}]}{\sigma_M^{\text{null}}}$$

---

## 10. Spreader Dynamics on Temporal Networks

### 10.1 Epidemic Models on Time-Varying Networks

Standard epidemic models (SI, SIR, SIS) require adaptation for temporal networks:

**Definition 10.1 (Temporal SI Model).** In the Susceptible-Infected (SI) model on temporal networks:
- Infection can only traverse edges when they are active
- Infected nodes remain infected forever
- The temporal constraint: infection spreads only along time-respecting paths

**Definition 10.2 (Temporal SIR Model).** In the Susceptible-Infected-Recovered (SIR) model:
- Nodes recover after infection period $\tau_I$
- Recovery creates temporary immunity (or permanent in immune SIR)
- The interplay of recovery time and edge timing determines outbreak size

### 10.2 Importance of Temporal Structure

**Theorem 10.1 (Temporal Structure Matters).** The final outbreak size in temporal networks can differ dramatically from static approximations, even when the aggregate network is identical.

**Mechanisms:**
- **Burstiness slows spread:** Long inter-event times create waiting periods
- **Edge timing determines paths:** Early edges seed large outbreaks
- **Causality constraints:** Some static paths are temporally invalid

### 10.3 SI/SIR on Temporal Networks

**Algorithm 10.1 (Temporal SIR Simulation).**

```
procedure TemporalSIR(G_T, patient_zero, beta, gamma, T_max):
    state ← array of size n, initialized to S (susceptible)
    infection_time ← array of size n, initialized to ∞
    recovery_time ← array of size n, initialized to ∞
    
    state[patient_zero] ← I (infected)
    infection_time[patient_zero] ← 0
    recovery_time[patient_zero] ← gamma
    
    queue ← priority queue of events
    enqueue(queue, (infection_time[patient_zero], patient_zero, "infect"))
    enqueue(queue, (recovery_time[patient_zero], patient_zero, "recover"))
    
    while queue not empty and current_time < T_max:
        (t, v, event) ← dequeue(queue)
        
        if event = "infect":
            // Try to infect neighbors via temporal edges
            for each (v, u, t_edge) in G_T where t_edge > t:
                if state[u] = S and random() < beta:
                    state[u] ← I
                    infection_time[u] ← t_edge
                    recovery_time[u] ← t_edge + gamma
                    enqueue(queue, (infection_time[u], u, "infect"))
                    enqueue(queue, (recovery_time[u], u, "recover"))
        
        else if event = "recover":
            state[v] ← R
    
    return state, infection_time, recovery_time
```

---

## 11. Network Evolution

### 11.1 Preferential Attachment Over Time

**Definition 11.1 (Temporal Preferential Attachment).** In temporal networks, preferential attachment is:
$$P(\text{new edge to } v \text{ at time } t) \propto \deg(v, t)^{\alpha}$$

**Theorem 11.1 (Degree Dynamics).** Under linear preferential attachment ($\alpha = 1$), the degree distribution follows a power law with $\gamma = 3$, matching the static BA model.

However, temporal analysis reveals:
- **Aging effects:** Older nodes may lose attractiveness
- **Fitness:** Nodes have intrinsic quality affecting attachment
- **Local attachment:** Preferential attachment to neighbors of neighbors

### 11.2 Link Prediction

**Definition 11.2 (Link Prediction).** *Link prediction* forecasts which edges will form in the future based on current and past network structure.

**Temporal Features:**
- Common neighbors across time
- Temporal path counts
- Edge persistence patterns
- Burstiness of past interactions

**Algorithm 11.1 (Temporal Link Prediction).**

```
procedure PredictLinks(G_T, current_time, k):
    scores ← empty dictionary
    
    for each non-edge (u, v) in G_current:
        // Temporal common neighbors
        temporal_CN ← |{w : time_respecting_path(u, w, v)}|
        
        // Temporal Jaccard
        temporal_Jaccard ← temporal_CN / |temporal_union(N(u), N(v))|
        
        // Edge persistence
        persistence ← correlation(past_edges(u,v), past_edges(u,v) shifted)
        
        // Recency-weighted features
        recency ← sum over past edges of decay_function(time_since_edge)
        
        scores[(u, v)] ← combine_features(temporal_CN, temporal_Jaccard, 
                                          persistence, recency)
    
    return top_k(scores, k)
```

### 11.3 Predicting Future Network Structure

Beyond individual links, predicting global properties:
- **Community evolution:** Which communities will merge or split?
- **Centrality shifts:** Which nodes will become important?
- **Network growth:** How will network size and density change?

---

## 12. Data Collection and Representation

### 12.1 Timestamped Interaction Data

**Data Requirements:**
- **Entity identifiers:** Unique IDs for nodes
- **Timestamps:** High-precision timestamps for each interaction
- **Event types:** Classification of interaction types
- **Attributes:** Node and edge attributes
- **Duration:** For non-instantaneous interactions

**Common Formats:**
- Edge list with timestamps: $(u, v, t, \text{weight}, \text{type})$
- Adjacency list with time intervals
- Event logs with full metadata

### 12.2 Handling Irregular Sampling

Real data often has irregular temporal sampling:
- Missing observations
- Varying observation rates
- Bursty data collection

**Imputation Strategies:**
- Assume persistence (edges remain until observed otherwise)
- Interpolate between observations
- Use generative models to fill gaps

### 12.3 Continuous vs Discrete Time

**Continuous Time:**
- Pros: Exact event ordering, natural for point processes
- Cons: Computational complexity, requires event-based simulation

**Discrete Time:**
- Pros: Simpler algorithms, aligns with many data sources
- Cons: Loses intra-interval ordering, requires binning decisions

**Recommendation:** Use continuous time for fine-grained analysis, discrete time for aggregate statistics.

---

## 13. Statistical Analysis of Temporal Networks

### 13.1 Event History Analysis

**Definition 13.1 (Event History).** *Event history analysis* models the timing and occurrence of events using survival analysis techniques.

**Hazard Function:** The instantaneous rate of edge formation:
$$h_{ij}(t) = \lim_{\Delta t \to 0} \frac{P(t \leq T_{ij} < t + \Delta t | T_{ij} \geq t)}{\Delta t}$$

### 13.2 Survival Models for Edge Duration

**Definition 13.2 (Edge Duration).** Model the time an edge remains active using survival functions:
$$S_{ij}(t) = P(\text{edge } (i,j) \text{ survives past time } t)$$

**Common Models:**
- **Exponential:** $S(t) = \exp(-\lambda t)$ — memoryless
- **Weibull:** $S(t) = \exp(-(\lambda t)^k)$ — flexible shape
- **Log-normal:** $S(t) = 1 - \Phi((\log t - \mu)/\sigma)$ — heavy-tailed

### 13.3 Cox Proportional Hazards for Edges

**Definition 13.3 (Cox Model).** The *Cox proportional hazards model* for edge $(i,j)$:

$$h_{ij}(t) = h_0(t) \exp\left(\sum_k \beta_k X_{ij}^{(k)}(t)\right)$$

where $h_0(t)$ is the baseline hazard and $X_{ij}^{(k)}(t)$ are time-varying covariates.

**Covariate Examples:**
- Time since last interaction
- Current network distance
- Shared neighbors
- Node attributes

---

## 14. Applications to Social/Economic Networks

### 14.1 Contact Patterns

**Human Contact Networks:**
- **Proximity data:** Bluetooth or RFID captures face-to-face contacts
- **Temporal structure:** Contacts cluster during work hours, weekdays
- **Implications for disease:** Timing of contacts determines outbreak potential

**Communication Networks:**
- **Email:** Bursty patterns, response time distributions
- **Phone calls:** Reciprocity, circadian rhythms
- **Social media:** Event-driven spikes, viral cascades

### 14.2 Financial Transaction Timing

**Interbank Networks:**
- Intraday patterns: Settlement peaks at day end
- Seasonal effects: Quarter-end, year-end patterns
- Crisis periods: Increased volatility, changed network structure

**Payment Networks:**
- Transaction sequences reveal money laundering
- Timing patterns identify market manipulation
- Network dynamics predict liquidity crises

### 14.3 Information Propagation Timing

**Information Cascades:**
- Adoption curves and tipping points
- The role of "influentials" and timing
- Competition between information pieces

**Viral Content:**
- Temporal signatures of viral vs. non-viral content
- Optimal timing for maximum reach
- Platform-specific temporal patterns

---

## 15. How Lutufi Handles Temporal Networks

### 15.1 Dynamic Bayesian Network Architecture

Lutufi's core innovation is unifying temporal network analysis with Dynamic Bayesian Networks (DBNs):

**Time-Slice Model:**
```
procedure TimeSliceDBN(nodes, edges_T, time_steps):
    for t from 1 to T:
        // Intra-slice edges (within time t)
        for each edge (u, v) in edges[t]:
            add_intra_slice_dependency(u_t, v_t, strength)
        
        // Inter-slice edges (from t-1 to t)
        for each node u:
            add_temporal_dependency(u_{t-1}, u_t, persistence)
            
        // Cross-node temporal dependencies
        for each temporal_edge (u_{t-1}, v_t):
            add_cross_temporal_dependency(u_{t-1}, v_t)
    
    return factor_graph
```

### 15.2 Time-Slice Models

**Definition 15.1 (Time-Slice DBN).** A *time-slice DBN* unrolls the temporal network into a large static Bayesian network with:
- One copy of each variable for each time step
- Intra-slice edges from the instantaneous network
- Inter-slice edges encoding temporal persistence and influence

**Complexity Management:**
- **Rolling window:** Only include recent history
- **Approximate inference:** Variational methods for large networks
- **Structure learning:** Discover temporal dependencies from data

### 15.3 Inference Over Time

**Filtering:** $P(X_t | Y_{1:t})$ — belief at current time given all observations
**Prediction:** $P(X_{t+k} | Y_{1:t})$ — forecast future states
**Smoothing:** $P(X_t | Y_{1:T})$ — refined beliefs using future information
**Most Probable Explanation:** $\arg\max_{X_{1:T}} P(X_{1:T} | Y_{1:T})$ — best trajectory

**Algorithm 15.1 (Temporal Belief Propagation).**

```
procedure TemporalBeliefPropagation(DBN, observations, operation):
    if operation = "filtering":
        beliefs ← forward_pass(DBN, observations)
    else if operation = "smoothing":
        forward ← forward_pass(DBN, observations)
        backward ← backward_pass(DBN, observations)
        beliefs ← combine(forward, backward)
    else if operation = "prediction":
        beliefs ← forward_pass(DBN, observations, horizon)
    
    return beliefs

procedure forward_pass(DBN, observations):
    messages ← initialize_uniform()
    for t from 1 to T:
        // Incorporate observations
        messages[t] ← messages[t] * likelihood(observations[t])
        
        // Propagate to next time slice
        if t < T:
            messages[t+1] ← propagate(messages[t], DBN.inter_slice[t])
    
    return messages
```

### 15.4 Integration with Temporal Graph Analysis

Lutufi integrates temporal graph metrics with probabilistic inference:

**Temporal Centrality → Prior Importance:**
- Nodes with high temporal betweenness receive higher prior probabilities for influence

**Temporal Communities → Factor Grouping:**
- Communities at each time step guide factor graph clustering
- Persistent communities indicate stable probabilistic dependencies

**Burstiness → Observation Reliability:**
- Bursty periods may indicate unreliable observations
- Smooth periods suggest stable system state

**Reachability → Influence Bounds:**
- Temporal reachability constrains possible influence propagation
- Impossible time-respecting paths have zero influence probability

---

## 16. Key References

1. **Holme, P., & Saramäki, J.** (2012). Temporal networks. *Physics Reports*, 519(3), 97-125. Comprehensive review of temporal network theory and methods.

2. **Masuda, N., & Lambiotte, R.** (2016). *A Guide to Temporal Networks*. World Scientific. Accessible introduction to temporal network analysis with applications.

3. **Kempe, D., Kleinberg, J., & Kumar, A.** (2002). Connectivity and inference problems for temporal networks. *Journal of Computer and System Sciences*, 64(4), 820-842. Foundational work on temporal paths and connectivity.

4. **Kivelä, M., Pan, R. K., Kaski, K., Kertész, J., Karsai, M., & Saramäki, J.** (2012). Multiscale analysis of spreading in a large communication network. *Journal of Statistical Mechanics: Theory and Experiment*, 2012(03), P03005. Empirical analysis of temporal patterns in communication.

5. **Moody, J.** (2002). The importance of relationship timing for diffusion. *Social Forces*, 81(1), 25-56. Demonstrates importance of temporal ordering in social networks.

6. **Pan, R. K., & Saramäki, J.** (2011). Path lengths, correlations, and centrality in temporal networks. *Physical Review E*, 84(1), 016105. Temporal centrality measures and their properties.

7. **Tang, J., Scellato, S., Musolesi, M., Mascolo, C., & Latora, V.** (2010). Small-world behavior in time-varying graphs. *Physical Review E*, 81(5), 055101(R). Analysis of small-world properties in temporal networks.

8. **Caceres, R. S., Berger-Wolf, T. Y., & Grossman, R.** (2011). Temporal scale of processes in dynamic networks. In *2011 IEEE 11th International Conference on Data Mining Workshops* (pp. 925-932). IEEE. Time scales and processes in dynamic networks.

9. **Goh, K. I., & Barabási, A. L.** (2008). Burstiness and memory in complex systems. *EPL (Europhysics Letters)*, 81(4), 48002. Characterization of burstiness in temporal patterns.

10. **Barabási, A. L.** (2005). The origin of bursts and heavy tails in human dynamics. *Nature*, 435(7039), 207-211. Origin of bursty behavior in human activity.

11. **Vazquez, A., Rácz, B., Lukács, A., & Barabási, A. L.** (2007). Impact of non-Poissonian activity patterns on spreading processes. *Physical Review Letters*, 98(15), 158702. Effect of burstiness on spreading.

12. **Stehlé, J., Voirin, N., Barrat, A., Cattuto, C., Isella, L., Pinton, J. F., ... & Van den Broeck, W.** (2011). High-resolution measurements of face-to-face contact patterns in a primary school. *PloS One*, 6(8), e23176. High-resolution empirical temporal network data.

13. **Karsai, M., Kivelä, M., Pan, R. K., Kaski, K., Kertész, J., Barabási, A. L., & Saramäki, J.** (2011). Small but slow world: How network topology and burstiness slow down spreading. *Physical Review E*, 83(2), 025102(R). Combined effect of structure and burstiness.

14. **Scholtes, I., Wider, N., Pfitzner, R., Garas, A., Tessone, C. J., & Schweitzer, F.** (2014). Causality-driven slow-down and speed-up of diffusion in non-Markovian temporal networks. *Nature Communications*, 5, 5024. Non-Markovian effects on diffusion.

15. **Murphy, K. P.** (2002). *Dynamic Bayesian Networks: Representation, Inference and Learning*. PhD thesis, UC Berkeley. Comprehensive treatment of DBNs with applications.

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | March 2026 | Initial comprehensive document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's governance guidelines.*
