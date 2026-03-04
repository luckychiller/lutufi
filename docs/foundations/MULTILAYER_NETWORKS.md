# Multilayer and Multiplex Networks

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Introduction](#introduction)
2. [Why Multilayer Networks](#why-multilayer-networks)
3. [Formal Definitions](#formal-definitions)
4. [Types of Multilayer Networks](#types-of-multilayer-networks)
5. [Adjacency and Representations](#adjacency-and-representations)
6. [Paths in Multilayer Networks](#paths-in-multilayer-networks)
7. [Centrality in Multilayer Networks](#centrality-in-multilayer-networks)
8. [Clustering and Communities](#clustering-and-communities)
9. [Structural Measures](#structural-measures)
10. [Coupling and Layer Interdependence](#coupling-and-layer-interdependence)
11. [Diffusion and Dynamics on Multilayer Networks](#diffusion-and-dynamics-on-multilayer-networks)
12. [Co-evolution and Interdependence](#co-evolution-and-interdependence)
13. [Dimensionality Reduction](#dimensionality-reduction)
14. [Statistical Models](#statistical-models)
15. [Applications](#applications)
16. [How Lutufi Handles Multilayer Networks](#how-lutufi-handles-multilayer-networks)
17. [Key References](#key-references)

---

## Introduction

The study of complex networks has traditionally focused on single-layer representations where nodes represent entities and edges represent relationships of a single type. However, real-world systems are rarely this simple. Social networks involve multiple types of relationships—friendship, professional collaboration, family ties, geographical proximity—each operating simultaneously. Transportation networks comprise different modes of transport. Financial networks span multiple markets and instruments. Biological networks involve multiple molecular interaction types.

Multilayer network theory provides a comprehensive mathematical framework for analyzing systems with multiple types of relationships, multiple modalities of interaction, or multiple scales of organization. This document presents the theoretical foundations of multilayer networks, their mathematical representations, analytical methods, and applications to social and economic systems that Lutufi is designed to model.

---

## Why Multilayer Networks

### The Limitations of Single-Layer Analysis

Consider a corporate board interlock network where nodes represent companies and edges represent shared directors. Traditional single-layer analysis might identify central companies based on their number of board interlocks. However, this analysis misses critical dimensions:

- **Temporal evolution:** Board memberships change over time
- **Relationship strength:** Some shared directors are more influential than others
- **Multiple relationship types:** Companies are also connected through supply chains, strategic alliances, and competitive relationships
- **Institutional context:** Companies operate in different industries and regulatory environments

A multilayer approach captures these dimensions simultaneously, revealing patterns invisible to single-layer analysis.

### Emergent Properties in Multilayer Systems

Multilayer networks exhibit properties that emerge from layer interactions and cannot be understood by analyzing layers independently:

**Inter-layer correlations:** Relationships in one layer may strongly correlate with relationships in another. In social networks, friendship ties often predict professional collaboration.

**Layer-specific dynamics:** Information may spread faster on some layers than others. In transportation networks, air travel enables rapid disease spread across continents, while local transit drives local epidemics.

**Coupled resilience:** Failures in one layer can cascade to others. The 2008 financial crisis demonstrated how distress in mortgage markets propagated through interconnected banking, insurance, and securities layers.

**Topological patterns:** Community structures may align across layers (multislice communities) or be layer-specific. Understanding when communities persist across layers reveals fundamental organizational principles.

### The Aspect Concept

Kivelä et al. (2014) formalized the concept of "aspects" to describe different ways networks can be multilayered:

- **Type aspect:** Different types of relationships (friendship, collaboration)
- **Time aspect:** Network evolution over time
- **Category aspect:** Different categories of entities or contexts
- **Scale aspect:** Different resolutions or granularities

A network can be multilayered along multiple aspects simultaneously, creating rich, high-dimensional representations of complex systems.

---

## Formal Definitions

### The General Multilayer Network

Following Kivelä et al. (2014), we define a **multilayer network** as:

$$M = (V_M, E_M, V, L)$$

where:

- $V$ is the set of entities (physical nodes)
- $L = \{L_1, L_2, \ldots, L_d\}$ is a sequence of sets called **aspects**, where each $L_i$ represents a different dimension of layering
- $V_M \subseteq V \times L_1 \times L_2 \times \cdots \times L_d$ is the set of **state nodes** (node-layer tuples)
- $E_M \subseteq V_M \times V_M$ is the set of edges

An element $(v, \alpha_1, \ldots, \alpha_d) \in V_M$ represents entity $v$ in layer configuration $(\alpha_1, \ldots, \alpha_d)$.

### Key Components

**Nodes and Node-Layers:**
- A **physical node** $v \in V$ represents an entity
- A **state node** (or node-layer) $\tilde{v} = (v, \alpha) \in V_M$ represents that entity in a specific context
- The **layer** of a state node is its coordinate in aspect space

**Intra-layer and Inter-layer Edges:**
- An **intra-layer edge** connects state nodes in the same layer: $((v, \alpha), (w, \alpha)) \in E_M$
- An **inter-layer edge** connects state nodes in different layers: $((v, \alpha), (w, \beta)) \in E_M$ where $\alpha \neq \beta$

**The Coupling Edges:**
Inter-layer edges connecting the same physical node across layers are called **coupling edges** or **identity edges**:
$$E_{coupling} = \{((v, \alpha), (v, \beta)) \in E_M : \alpha \neq \beta\}$$

### Node-Aligned vs. Node-Disjoint Networks

**Node-Aligned Networks:**
All physical nodes exist in all layers:
$$V_M = V \times L_1 \times \cdots \times L_d$$

This is the most common case in social networks, where every person has the potential for multiple relationship types. The number of state nodes is $|V| \times |L_1| \times \cdots \times |L_d|$.

**Node-Disjoint Networks:**
Different layers contain different sets of physical nodes:
$$V_M \subset V \times L_1 \times \cdots \times L_d$$

This occurs in transportation networks where different modes (air, rail, road) serve different sets of locations, or in biological networks where different molecular species participate in different interaction types.

**Partially-Aligned Networks:**
Some nodes exist in multiple layers, others in only one:
$$V_M = \bigcup_{\alpha \in \mathcal{L}} V^{(\alpha)} \times \{\alpha\}$$

where $V^{(\alpha)}$ is the set of nodes present in layer $\alpha$.

### The Supra-Graph Representation

Any multilayer network can be represented as a **supra-graph** (or overlay graph) $G_M = (V_M, E_M)$, which is simply the graph formed by treating all state nodes as vertices. This representation loses some multilayer structure but enables application of standard graph algorithms.

The supra-graph has:
- $|V_M|$ vertices
- $|E_M|$ edges
- Potentially very different properties from any single layer

---

## Types of Multilayer Networks

### Multiplex Networks

A **multiplex network** is a multilayer network where:
1. All layers share the same set of physical nodes (node-aligned)
2. Edges are only intra-layer (no inter-layer edges between different nodes)
3. The only inter-layer edges are coupling edges connecting a node to itself across layers

Formally, for a multiplex network with $L$ layers:
- $V_M = V \times \{1, 2, \ldots, L\}$
- $E_M = \bigcup_{l=1}^L E^{(l)} \cup E_{coupling}$

where $E^{(l)}$ are edges in layer $l$.

**Example:** A social network with friendship, professional, and family layers. Each person exists in all layers, relationships are within-layer, and coupling edges allow considering a person's full profile.

### Interconnected Multilayer Networks

An **interconnected multilayer network** (or interdependent network) allows:
1. Different nodes in different layers (possibly disjoint)
2. Inter-layer edges connecting different physical nodes across layers

Formally:
- $V^{(l)}$ may differ across layers $l$
- $E_M$ contains edges $((v, \alpha), (w, \beta))$ where $v \neq w$ and $\alpha \neq \beta$

**Example:** A transportation network where airports (air layer) connect to train stations (rail layer) through inter-modal edges.

### Multilevel Networks

**Multilevel networks** have hierarchical layer structure where layers are organized into a hierarchy or ontology:
- Higher layers aggregate lower layers
- Nodes at different levels represent entities at different scales
- Edges may connect across the hierarchy

**Example:** An organizational network with individual, team, department, and company levels.

### Multidimensional Networks

A **multidimensional network** uses multiple aspects simultaneously:
$$M = (V_M, E_M, V, L_1, L_2, \ldots, L_d)$$

For example, a temporal multiplex network has both time and relationship type aspects:
$$V_M = V \times T \times R$$

where $T$ is the set of time slices and $R$ is the set of relationship types.

### Edge-Colored Multigraphs

An **edge-colored multigraph** is equivalent to a multiplex network where edges are colored by their layer:
$$G = (V, E, c)$$

where $c: E \rightarrow \{1, 2, \ldots, L\}$ assigns each edge to a color (layer).

This representation emphasizes that the same node pair can have edges of multiple colors (multiple relationship types).

---

## Adjacency and Representations

### Multilayer Adjacency Tensor

The most general representation uses a **4th-order adjacency tensor** $A_{vw\alpha\beta}$ where:

$$A_{vw\alpha\beta} = \begin{cases} 1 & \text{if } ((v, \alpha), (w, \beta)) \in E_M \\ 0 & \text{otherwise} \end{cases}$$

For undirected networks, $A_{vw\alpha\beta} = A_{wv\beta\alpha}$.

For a multiplex network with $L$ layers, this is a block tensor where:
- Diagonal blocks $A_{\cdot\cdot ll}$ represent intra-layer adjacency for layer $l$
- Off-diagonal blocks $A_{\cdot\cdot l_1 l_2}$ represent inter-layer adjacency between layers $l_1$ and $l_2$

### Supra-Adjacency Matrix

The **supra-adjacency matrix** $\mathcal{A}$ is an $|V_M| \times |V_M|$ matrix representing the supra-graph:

$$\mathcal{A} = \begin{pmatrix} A^{(1)} & D^{(12)} & \cdots & D^{(1L)} \\ D^{(21)} & A^{(2)} & \cdots & D^{(2L)} \\ \vdots & \vdots & \ddots & \vdots \\ D^{(L1)} & D^{(L2)} & \cdots & A^{(L)} \end{pmatrix}$$

where:
- $A^{(l)}$ is the $|V| \times |V|$ adjacency matrix of layer $l$
- $D^{(ll')}$ contains inter-layer edges between layers $l$ and $l'$

For node-aligned multiplex networks with diagonal coupling:
$$D^{(ll')} = \omega_{ll'} I$$

where $\omega_{ll'}$ is the coupling strength between layers $l$ and $l'$, and $I$ is the identity matrix.

### Tensor Decomposition

The multilayer adjacency tensor can be decomposed using tensor factorization methods:

**PARAFAC/CANDECOMP Decomposition:**
$$A \approx \sum_{r=1}^{R} \lambda_r \, u_r^{(1)} \circ u_r^{(2)} \circ u_r^{(3)} \circ u_r^{(4)}$$

where $\circ$ denotes outer product and $R$ is the rank.

**Tucker Decomposition:**
$$A = G \times_1 U^{(1)} \times_2 U^{(2)} \times_3 U^{(3)} \times_4 U^{(4)}$$

where $G$ is the core tensor and $\times_n$ denotes mode-$n$ multiplication.

These decompositions reveal latent structures across layers.

### Layer Projection

**Flattening to a single layer** can be done through various projections:

**OR (Union) Projection:**
$$A^{OR}_{vw} = \max_{\alpha,\beta} A_{vw\alpha\beta}$$

An edge exists if it exists in any layer.

**AND (Intersection) Projection:**
$$A^{AND}_{vw} = \min_{\alpha,\beta} A_{vw\alpha\beta}$$

An edge exists only if it exists in all layers.

**Weighted Aggregation:**
$$A^{agg}_{vw} = \sum_{\alpha} w_{\alpha} A^{(\alpha)}_{vw}$$

where $w_{\alpha}$ are layer weights.

**Warning:** These projections lose information about layer-specific structure and inter-layer coupling.

---

## Paths in Multilayer Networks

### Multilayer Paths

A **multilayer path** of length $k$ from $(v_0, \alpha_0)$ to $(v_k, \alpha_k)$ is a sequence:

$$(v_0, \alpha_0), (v_1, \alpha_1), \ldots, (v_k, \alpha_k)$$

where $((v_i, \alpha_i), (v_{i+1}, \alpha_{i+1})) \in E_M$ for all $i$.

Unlike single-layer paths, multilayer paths can:
- Traverse edges within the same layer
- Switch layers via coupling edges
- Use inter-layer edges between different nodes

### Allowed Layer Transitions

The structure of allowed layer transitions defines what paths are possible:

**Categorical Coupling:** Transitions allowed only between specific layer pairs (e.g., adjacent time slices)

**Diagonal Coupling:** Transitions allowed only for same node across layers

**Off-Diagonal Coupling:** Transitions between different nodes across layers (e.g., airport to train station)

**Temporal Constraint:** In temporal networks, transitions only forward in time (causal paths)

### Shortest Paths with Layer Changes

The **shortest multilayer path** problem finds the minimum-weight path considering both intra-layer and inter-layer edges:

$$d_M((v, \alpha), (w, \beta)) = \min_{P \in \mathcal{P}_{(v,\alpha),(w,\beta)}} \sum_{e \in P} w(e)$$

where $\mathcal{P}_{(v,\alpha),(w,\beta)}$ is the set of all multilayer paths and $w(e)$ is edge weight.

**Algorithm:** Standard shortest path algorithms (Dijkstra, Bellman-Ford) work on the supra-graph.

**Complexity:** $O(|E_M| + |V_M| \log |V_M|)$ with appropriate data structures.

### Path Types and Constraints

**Intra-layer paths:** Paths confined to a single layer, representing relationships within one context.

**Inter-layer paths:** Paths that traverse multiple layers, representing cross-contextual connections.

**Causal paths:** In temporal networks, paths that respect time ordering (no backward time transitions).

**Layer-restricted paths:** Paths constrained to specific subsets of layers (e.g., only professional and acquaintance layers, excluding family).

---

## Centrality in Multilayer Networks

### Generalizing Single-Layer Centrality

Centrality measures in multilayer networks must account for:
1. Connectivity within each layer
2. Ability to traverse across layers
3. Layer-specific importance or weighting

### Multilayer Degree Centrality

**Layer-Specific Degree:**
$$k_i^{(\alpha)} = \sum_j A_{ij}^{(\alpha)}$$

**Aggregate Degree:**
$$K_i = \sum_{\alpha} k_i^{(\alpha)}$$

**Weighted Aggregate Degree:**
$$K_i^w = \sum_{\alpha} w_{\alpha} k_i^{(\alpha)}$$

**Versatility (De Domenico et al., 2013):** The number of layers in which a node has non-zero degree:
$$V_i = |\{\alpha : k_i^{(\alpha)} > 0\}|$$

### Multilayer Eigenvector Centrality

**Definition:** The multilayer eigenvector centrality $\mathbf{x}$ satisfies:

$$\lambda x_{i\alpha} = \sum_{j,\beta} \mathcal{A}_{(i,\alpha),(j,\beta)} x_{j\beta}$$

or in matrix form:
$$\lambda \mathbf{x} = \mathcal{A} \mathbf{x}$$

where $\mathcal{A}$ is the supra-adjacency matrix.

**Interpretation:** Node-layer importance depends on the importance of neighbors in the full multilayer structure.

**Node Eigenvector Centrality:**
$$x_i = \sum_{\alpha} x_{i\alpha}$$

aggregates importance across layers.

### Multilayer PageRank

**Random Walk on Multilayer Networks:**
With probability $p$, follow an edge (intra or inter-layer). With probability $1-p$, restart at a random node-layer.

The **Multilayer PageRank** $\pi$ satisfies:
$$\pi_{i\alpha} = p \sum_{j,\beta} \frac{\mathcal{A}_{(j,\beta),(i,\alpha)}}{k_j^{out}} \pi_{j\beta} + \frac{1-p}{|V_M|}$$

**Layer-Specific PageRank:**
Can define PageRank constrained to individual layers or specific layer subsets.

### Multilayer Betweenness Centrality

**Definition:** The fraction of shortest multilayer paths passing through a node-layer:

$$C_B((v, \alpha)) = \sum_{(s,\sigma) \neq (v,\alpha) \neq (t,\tau)} \frac{\sigma_{(s,\sigma),(t,\tau)}((v,\alpha))}{\sigma_{(s,\sigma),(t,\tau)}}$$

where $\sigma_{(s,\sigma),(t,\tau)}$ is the number of shortest paths and $\sigma_{(s,\sigma),(t,\tau)}((v,\alpha))$ is the number passing through $(v,\alpha)$.

**Bridge Nodes:** Nodes with high betweenness due to inter-layer connections act as bridges between network contexts.

### Versatility and Cross-Layer Centrality

**Versatility (De Domenico et al., 2015):** Measures a node's ability to participate in network functions across layers:

$$V_i = \sum_{\alpha} \left(\frac{x_{i\alpha}}{x_i^{max}}\right)^q$$

where $x_{i\alpha}$ is centrality in layer $\alpha$ and $q$ is a tuning parameter.

Nodes with high versatility are important across multiple contexts, making them critical connectors or bottlenecks.

---

## Clustering and Communities

### Multilayer Modularity

**Definition:** Extending Newman's modularity to multilayer networks:

$$Q_M = \frac{1}{2\mu} \sum_{ij\alpha\beta} \left[ A_{ij\alpha\beta} - \gamma_{\alpha\beta} \frac{k_{i\alpha} k_{j\beta}}{2\mu} \right] \delta(c_{i\alpha}, c_{j\beta})$$

where:
- $c_{i\alpha}$ is the community assignment of node $i$ in layer $\alpha$
- $\gamma_{\alpha\beta}$ is the resolution parameter for layer pair $(\alpha, \beta)$
- $\mu = \frac{1}{2}\sum_{ij\alpha\beta} A_{ij\alpha\beta}$ is the total edge weight

**Interpretation:** Rewards dense intra-community connections and sparse inter-community connections, considering all layer pairs.

**Resolution Parameters:**
- High $\gamma_{\alpha\alpha}$: many small communities within layer $\alpha$
- High $\gamma_{\alpha\beta}$ for $\alpha \neq \beta$: encourages nodes to be in different communities across layers

### Layer-Aware Community Detection

**Approaches:**

1. **Independent:** Run community detection on each layer separately
   - Pros: Simple, layer-specific patterns
   - Cons: Misses cross-layer communities

2. **Aggregate:** Project to single layer, then detect communities
   - Pros: Simple, unified partition
   - Cons: Loses layer-specific information

3. **Multilayer optimization:** Maximize $Q_M$ directly
   - Pros: Unified framework, cross-layer information
   - Cons: Computationally demanding, parameter tuning

4. **Temporal smoothing:** For temporal networks, encourage community stability
   $$Q_{temp} = Q_M + \omega \sum_{t} \text{similarity}(C_t, C_{t+1})$$

### Tensor-Based Methods

**Non-negative Matrix Factorization (NMF):**
Decompose supra-adjacency into community assignments:
$$\mathcal{A} \approx H H^T$$

where $H_{(i,\alpha),c}$ is the strength of membership of node-layer $(i,\alpha)$ in community $c$.

**Tensor Decomposition:**
Decompose the 4th-order adjacency tensor to reveal community structure across dimensions.

### Detecting Communities That Persist Across Layers

**Persistent Communities:** Groups of nodes that form communities together across multiple layers.

**Detection Methods:**

1. **Community matching:** Find communities in each layer, then match across layers
2. **Joint optimization:** Optimize modularity with constraints encouraging persistence
3. **Core-periphery detection:** Identify core nodes present in all layer communities

**Significance Testing:** Compare observed persistence to null models where layer communities are independent.

---

## Structural Measures

### Multilayer Clustering Coefficient

**Generalized Definition:** Measures the density of triangles in the multilayer structure.

**3-Cycle Types:**
1. **Intra-layer triangle:** All three edges in the same layer
2. **Intra-layer 2-path with inter-layer closing:** Two edges in layer $\alpha$, closing edge in layer $\beta$
3. **Three-layer cycle:** Edges in three different layers

**Multilayer Clustering Coefficient (Cozzo et al., 2015):**
$$C_M(v) = \frac{\sum_{\alpha\beta\gamma} \sum_{j,k} A_{vj\alpha\beta} A_{jk\beta\gamma} A_{kv\gamma\alpha}}{\sum_{\alpha\beta} \sum_{j} A_{vj\alpha\beta} (k_j^{out} - 1)}$$

**Interpretation:** Generalizes the standard clustering coefficient to account for paths and closures across all layer combinations.

### Degree Correlations Across Layers

**Inter-layer Degree Correlation:**
$$\rho_{kl} = \frac{\text{Cov}(k^{(k)}, k^{(l)})}{\sigma_{k^{(k)}} \sigma_{k^{(l)}}}$$

measures whether high-degree nodes in layer $k$ also have high degree in layer $l$.

**Multiplexity:** The fraction of node pairs connected in multiple layers:
$$m = \frac{\sum_{i<j} \mathbb{I}(\sum_{\alpha} A_{ij}^{(\alpha)} > 1)}{\sum_{i<j} \mathbb{I}(\sum_{\alpha} A_{ij}^{(\alpha)} > 0)}$$

**Edge Overlap:**
$$O = \frac{\sum_{i<j, \alpha} A_{ij}^{(\alpha)} A_{ij}^{(\beta)}}{\sum_{i<j} \min(k_i^{tot}, k_j^{tot})}$$

### Layer-Specific and Aggregate Measures

**Layer Entropy:** Measures the diversity of a node's layer participation:
$$H_i = -\sum_{\alpha} p_{i\alpha} \log p_{i\alpha}$$

where $p_{i\alpha} = k_i^{(\alpha)} / K_i$.

**Node Participation Coefficient:**
$$P_i = \frac{L}{L-1} \left[1 - \sum_{\alpha} \left(\frac{k_i^{(\alpha)}}{K_i}\right)^2\right]$$

ranges from 0 (all connections in one layer) to 1 (uniformly distributed across layers).

---

## Coupling and Layer Interdependence

### Inter-Layer Coupling Strength

**Coupling Parameter $\omega$:** Controls the relative weight of inter-layer edges in the supra-adjacency:

$$\mathcal{A} = \begin{pmatrix} A^{(1)} & \omega I & \cdots \\ \omega I & A^{(2)} & \cdots \\ \vdots & \vdots & \ddots \end{pmatrix}$$

**Regimes:**
- $\omega \rightarrow 0$: Layers are independent
- $\omega \rightarrow \infty$: Layers are identical (all nodes equivalent across layers)
- Intermediate $\omega$: Balance between intra and inter-layer structure

### Layer Similarity

**Structural Similarity:**
$$s_{\alpha\beta} = \frac{\langle A^{(\alpha)}, A^{(\beta)} \rangle}{||A^{(\alpha)}||_F ||A^{(\beta)}||_F}$$

measures correlation between adjacency matrices.

**Similarity Metrics:**
- **Jaccard similarity:** $|E^{(\alpha)} \cap E^{(\beta)}| / |E^{(\alpha)} \cup E^{(\beta)}|$
- **Spectral similarity:** Compare eigenvalue distributions
- **Community similarity:** Compare community assignments (NMI, ARI)

### Layer Reducibility

**Definition:** A multilayer network is **reducible** if some layers can be merged without significant information loss.

**Detection:** Use cluster analysis on the layer similarity matrix to identify redundant layers.

**Implications:**
- Reduces dimensionality
- Reveals fundamental relationship types
- Indicates which relationship dimensions are independent

---

## Diffusion and Dynamics on Multilayer Networks

### Random Walks on Multilayer Networks

**Definition:** A walker at node-layer $(v, \alpha)$ transitions to neighbor $(w, \beta)$ with probability:

$$P((v, \alpha) \rightarrow (w, \beta)) = \frac{A_{vw\alpha\beta}}{k_{v\alpha}^{out}}$$

where $k_{v\alpha}^{out} = \sum_{w,\beta} A_{vw\alpha\beta}$ is the total out-degree.

**Transition Matrix:**
$$T = D^{-1} \mathcal{A}$$

where $D$ is the diagonal matrix of out-degrees.

**Stationary Distribution:**
$$\pi = \pi T$$

**Layer-Changing Walks:** The probability of changing layers depends on coupling strength and layer-specific degree distributions.

### Diffusion Processes

**Continuous-Time Random Walk:**
$$\frac{d\mathbf{p}}{dt} = -L \mathbf{p}$$

where $L = D - \mathcal{A}$ is the supra-Laplacian.

**Diffusion Time:** The time for information to spread across the network depends on:
- Intra-layer connectivity
- Inter-layer coupling
- Network size and structure

**Eigenvalue Analysis:** The spectral gap (difference between first and second eigenvalues of $L$) determines diffusion rate.

### Epidemic Models on Multilayer Networks

**SIS Model Extension:**
Each node-layer $(i, \alpha)$ has state $X_{i\alpha} \in \{S, I\}$.

**Infection Rate:**
$$\lambda_{i\alpha} = \beta \sum_{j,\beta} A_{ij\alpha\beta} \mathbb{I}(X_{j\beta} = I)$$

Infection can spread within layers and across layers.

**Epidemic Threshold:**
The epidemic threshold depends on the largest eigenvalue of the supra-adjacency:
$$\beta_c = \frac{\mu}{\lambda_1(\mathcal{A})}$$

**Layer-Specific Dynamics:**
- Different layers may have different transmission rates $\beta_{\alpha}$
- Some layers may be "super-spreaders"
- Inter-layer coupling can enable global epidemics even if no layer alone supports one

### Optimal Diffusion Strategies

**Problem:** Given limited resources (e.g., seed nodes, edges to add), maximize diffusion efficiency.

**Strategies:**
1. **Intra-layer seeding:** Seed nodes in high-centrality positions within individual layers
2. **Inter-layer bridges:** Seed nodes that bridge disconnected communities across layers
3. **Layer coupling:** Strengthen coupling between layers with complementary structures

**Mathematical Optimization:**
$$\max_{S \subset V_M, |S| = k} \sigma(S)$$

where $\sigma(S)$ is the expected diffusion coverage from seed set $S$.

---

## Co-evolution and Interdependence

### How Layers Influence Each Other

**Adaptive Coupling:** Inter-layer edge weights change based on node states:
$$\frac{d\omega_{ij}^{\alpha\beta}}{dt} = f(X_i^{(\alpha)}, X_j^{(\beta)})$$

**Example:** In social networks, professional ties strengthen when friends collaborate successfully.

**Structural Co-evolution:**
Nodes create, maintain, or dissolve edges in one layer based on connections in other layers.

**Preferential Attachment Across Layers:**
New edges in layer $\alpha$ preferentially connect to nodes that are central in layer $\beta$.

### Adaptive Layers

**Definition:** Network layers that change their structure in response to dynamics on other layers.

**Examples:**
- **Financial networks:** Banks form new lending relationships (layer $\alpha$) based on payment system exposures (layer $\beta$)
- **Social networks:** People form new friendships based on professional collaborations
- **Ecological networks:** Species develop new interaction types based on existing relationships

**Modeling Framework:**
$$\frac{dA^{(\alpha)}}{dt} = F(A^{(1)}, A^{(2)}, \ldots, A^{(L)}, \mathbf{X})$$

where $\mathbf{X}$ represents node attributes or states.

### Feedback Between Network Structure and Dynamics

**Positive Feedback:** Success in one layer strengthens connections in another, which enhances performance:
$$\text{Performance}_i^{(\alpha)} \uparrow \Rightarrow A_{ij}^{(\beta)} \uparrow \Rightarrow \text{Performance}_i^{(\beta)} \uparrow \Rightarrow A_{ij}^{(\alpha)} \uparrow$$

**Negative Feedback:** Success in one layer reduces need for connections in another:
$$\text{Performance}_i^{(\alpha)} \uparrow \Rightarrow A_{ij}^{(\beta)} \downarrow$$

**Tipping Points:** Feedback can lead to abrupt transitions in network structure.

---

## Dimensionality Reduction

### Flattening Approaches

**Simple Projection:**
$$A^{flat} = \sum_{\alpha} w_{\alpha} A^{(\alpha)}$$

**Limitations:**
- Loses layer-specific information
- Cannot represent inter-layer edges
- Assumes layer commensurability

**When Flattening is Appropriate:**
- All layers measure similar phenomena
- Goal is overall connectivity, not layer-specific patterns
- Inter-layer structure is less important than aggregate connectivity

### Principled Dimensionality Reduction

**Low-Rank Approximation:**
Approximate the multilayer adjacency tensor with lower-rank structure:
$$A \approx \sum_{r=1}^{R} \lambda_r \mathbf{u}_r \circ \mathbf{v}_r \circ \mathbf{w}_r \circ \mathbf{z}_r$$

where $R \ll \min(N^2, L^2)$.

**Spectral Methods:**
Use eigenvectors of the supra-adjacency to project to lower-dimensional space.

**Manifold Learning:**
Apply techniques like Isomap, LLE, or t-SNE to the supra-graph to reveal low-dimensional structure.

### Tensor Factorization for Compression

**Tucker Decomposition:**
$$A = G \times_1 U \times_2 V \times_3 W \times_4 Z$$

Compression ratio: $\frac{N^2 L^2}{R_1 R_2 R_3 R_4 + NR_1 + NR_2 + LR_3 + LR_4}$

**Applications:**
- Efficient storage of large multilayer networks
- Denoising by keeping only significant components
- Revealing latent community structure

---

## Statistical Models

### Multilayer ERGMs

**Exponential Random Graph Models** extended to multilayer networks:

$$P(Y = y) = \frac{\exp\{\theta^T g(y)\}}{Z(\theta)}$$

where $g(y)$ includes statistics that capture:
- Intra-layer structure (edges, triangles, stars)
- Inter-layer structure (cross-layer edges, multiplex triangles)
- Layer correlations (degree correlations, community alignment)

**Key Statistics:**
- **Cross-layer edges:** Count of edges between layers
- **Multiplex triangles:** Triangles spanning multiple layers
- **Layer alignment:** Correlation of degrees across layers
- **Community persistence:** Stability of community assignments

### Multilayer Stochastic Block Models

**Definition:** Nodes have latent community memberships that determine connection probabilities within and across layers.

**Generative Model:**
$$A_{ij}^{(\alpha)} \sim \text{Bernoulli}(P_{c_i c_j}^{(\alpha)})$$

where $c_i$ is node $i$'s community and $P^{(\alpha)}$ is layer-specific probability matrix.

**Extensions:**
- **Shared communities:** Same community structure across all layers
- **Layer-specific communities:** Different community assignments per layer
- **Mixed memberships:** Nodes belong to multiple communities

**Inference:** EM algorithm, variational inference, or MCMC to estimate memberships and probabilities.

### Generative Models

**Multiplex Configuration Model:**
Generate random multilayer networks preserving:
- Degree sequences in each layer
- Inter-layer degree correlations

**Algorithm:**
1. Assign stubs to nodes in each layer according to desired degree sequences
2. Randomly match stubs within layers
3. Add inter-layer edges according to correlation structure

**Growing Multilayer Networks:**
Preferential attachment extended to multiple layers:
- New nodes connect in all layers with probability proportional to existing degree
- Connection probability in layer $\alpha$ depends on degree in layers $\beta \neq \alpha$

---

## Applications

### Social Networks: Multiplex Ties

**Facebook Network Study:**
- Layer 1: Friendship ties
- Layer 2: Family relationships
- Layer 3: Professional connections
- Layer 4: Geographical co-location

**Findings:**
- Strong inter-layer correlations: Friends often share professional ties
- Multiplex triangles common: If A is friends with B and family with C, likely B and C have some connection
- Community structure differs across layers: Professional communities are larger and sparser

**Implications for Diffusion:**
Information spreads differently depending on which layers it traverses. Gossip spreads fast on friendship layers; professional opportunities spread through work layers.

### Transportation Networks

**Multi-Modal Transport:**
- Air layer: Flight connections
- Rail layer: Train routes
- Road layer: Highway network
- Transit layer: Local public transport

**Inter-Layer Connections:**
- Airports connect to train stations (inter-modal edges)
- Stations serve as hubs across multiple modes

**Optimization Applications:**
- Shortest path = fastest route considering mode changes
- Resilience analysis = impact of mode failure on overall connectivity
- Capacity planning = identifying bottlenecks in multi-modal flows

### Brain Networks

**Multi-Modal Neuroimaging:**
- Structural connectivity (diffusion MRI)
- Functional connectivity (fMRI correlations)
- Effective connectivity (causal influence)

**Analysis Goals:**
- Understand how structure constrains function
- Identify brain regions that play different roles across modalities
- Study disease effects on multi-modal connectivity

**Key Finding:** Brain regions with high betweenness in structural networks often have high eigenvector centrality in functional networks, indicating structure-function coupling.

### Financial Networks

**Multi-Market Networks:**
- Layer 1: Interbank lending
- Layer 2: Derivatives exposures
- Layer 3: Cross-holdings
- Layer 4: Payment system flows

**Systemic Risk Analysis:**
- Distress propagation across markets
- Identification of systemically important institutions
- Stress testing multi-market scenarios

**2008 Crisis Insight:**
The crisis propagated from mortgage markets through derivatives to interbank lending to payment systems—demonstrating critical importance of multilayer analysis.

---

## How Lutufi Handles Multilayer Networks

### Factor Graph Representation Across Layers

Lutufi represents multilayer networks using **factor graphs** that capture both intra-layer and inter-layer dependencies:

```
Layer 1 Factor Graph:    Layer 2 Factor Graph:
  f_A(X_1, X_2)            f_D(X_1, X_3)
  f_B(X_2, X_3)            f_E(X_2, X_4)
  f_C(X_3, X_4)            f_F(X_3, X_4)

Inter-Layer Coupling:
  f_coupling(X_1^(1), X_1^(2))
  f_coupling(X_2^(1), X_2^(2))
  ...
```

**Unified Representation:** All layers and couplings exist in a single factor graph, enabling seamless inference across the full multilayer structure.

### Inference Across Coupled Networks

**Belief Propagation:** Lutufi extends belief propagation to the supra-factor-graph:

$$m_{i\alpha \rightarrow j\beta}(x_{j\beta}) = \sum_{x_{i\alpha}} \psi_{i\alpha}(x_{i\alpha}) \psi_{ij\alpha\beta}(x_{i\alpha}, x_{j\beta}) \prod_{(k,\gamma) \in \mathcal{N}(i,\alpha) \setminus (j,\beta)} m_{k\gamma \rightarrow i\alpha}(x_{i\alpha})$$

**Computational Efficiency:**
- Leverages sparsity in both intra and inter-layer connections
- Supports parallel message passing within layers
- Adaptive damping for convergence in strongly coupled systems

**Uncertainty Quantification:**
- Provides marginal distributions for all node states across all layers
- Propagates uncertainty from observed layers to unobserved layers
- Quantifies confidence in cross-layer inferences

### Unified Modeling

**Hybrid Models:** Lutufi enables hybrid representations combining:
- Bayesian networks within layers (probabilistic relationships)
- ERGM priors for network structure
- Multilayer coupling factors

**Example:** A financial network where:
- Layer 1: Bayesian network of bank balance sheets
- Layer 2: ERGM for interbank lending patterns
- Coupling: Factors linking bank solvency to lending behavior

**Joint Learning:**
Lutufi learns parameters across all layers simultaneously:
$$\max_{\theta} P(D | \theta) = \prod_{\alpha} P(D^{(\alpha)} | \theta^{(\alpha)}) \prod_{\alpha,\beta} P(D^{(\alpha\beta)} | \theta^{(\alpha\beta)})$$

where $D^{(\alpha)}$ is data from layer $\alpha$ and $D^{(\alpha\beta)}$ is inter-layer data.

---

## Key References

1. **Kivelä, M., Arenas, A., Barthelemy, M., Gleeson, J. P., Moreno, Y., & Porter, M. A.** (2014). Multilayer networks. *Journal of Complex Networks*, 2(3), 203-271.

2. **Boccaletti, S., Bianconi, G., Criado, R., del Genio, C. I., Gómez-Gardeñes, J., Romance, M., ... & Zanin, M.** (2014). The structure and dynamics of multilayer networks. *Physics Reports*, 544(1), 1-122.

3. **De Domenico, M., Solé-Ribalta, A., Cozzo, E., Kivelä, M., Moreno, Y., Porter, M. A., ... & Arenas, A.** (2013). Mathematical formulation of multilayer networks. *Physical Review X*, 3(4), 041022.

4. **De Domenico, M., Solé-Ribalta, A., Omodei, E., Gómez, S., & Arenas, A.** (2015). Ranking in interconnected multilayer networks reveals versatile nodes. *Nature Communications*, 6, 6868.

5. **Cozzo, E., Banos, R. A., Meloni, S., & Moreno, Y.** (2013). Contact-based social contagion in multiplex networks. *Physical Review E*, 88(5), 050801.

6. **Battiston, F., Nicosia, V., & Latora, V.** (2014). Structural measures for multiplex networks. *Physical Review E*, 89(3), 032804.

7. **Gómez, S., Díaz-Guilera, A., Gómez-Gardeñes, J., Pérez-Vicente, C. J., Moreno, Y., & Arenas, A.** (2013). Diffusion dynamics on multiplex networks. *Physical Review Letters*, 110(2), 028701.

8. **Cellai, D., López, E., Zhou, J., Gleeson, J. P., & Bianconi, G.** (2013). Percolation in multiplex networks with overlap. *Physical Review E*, 88(5), 052811.

9. **Radicchi, F., & Arenas, A.** (2013). Abrupt transition in the structural formation of interconnected networks. *Nature Physics*, 9(11), 717-720.

10. **Buldyrev, S. V., Parshani, R., Paul, G., Stanley, H. E., & Havlin, S.** (2010). Catastrophic cascade of failures in interdependent networks. *Nature*, 464(7291), 1025-1028.

11. **Saumell-Mendiola, A., Serrano, M. A., & Boguñá, M.** (2012). Epidemic spreading on interconnected networks. *Physical Review E*, 86(2), 026106.

12. **Dickison, M., Havlin, S., & Stanley, H. E.** (2012). Epidemics on interconnected networks. *Physical Review E*, 85(6), 066109.

13. **Mucha, P. J., Richardson, T., Macon, K., Porter, M. A., & Onnela, J. P.** (2010). Community structure in time-dependent, multiscale, and multiplex networks. *Science*, 328(5980), 876-878.

14. **Cozzo, E., Kivelä, M., De Domenico, M., Solé, A., Arenas, A., Gómez, S., ... & Moreno, Y.** (2015). Structure of triadic relations in multiplex networks. *New Journal of Physics*, 17(7), 073029.

---

*This document is part of the Lutufi documentation. For questions or contributions, please refer to the project's contribution guidelines.*
