# Network Science Foundations: Lutufi

## Structural Properties
Lutufi incorporates the core tenets of **Social Network Analysis (SNA)** and **Economic Network Theory** to model how structural position influences probabilistic outcomes.

### 1. Centrality & Influence
- **Degree Centrality:** Number of direct ties a node has. In Lutufi, high degree often correlates with high influence over the network's belief state.
- **Betweenness Centrality:** Measures a node's role as a bridge between separate clusters. These nodes are critical for "bottleneck" propagation scenarios.

### 2. Network Topology Models
- **Scale-Free Networks (Barabási–Albert):** Characterized by power-law degree distributions. These networks are robust to random failure but highly vulnerable to targeted "hub" disruption.
- **Small-World Networks (Watts–Strogatz):** High clustering and short path lengths, ideal for modeling the "Six Degrees of Separation" phenomenon in information spread.

## Economic Network Theory
Lutufi treats economic entities as nodes in a dynamic, state-based system.

### Systemic Risk & Contagion
- **Interbank Lending:** Nodes represent banks, and edges represent exposures. Lutufi models the probability of a "default cascade" using stochastic processes over these exposures.
- **Supply Chain Resilience:** Modeling how the failure of a single supplier propagates probabilistic "risk of delay" through an entire manufacturing network.

## Strategic Interaction
Social and economic actors are not passive nodes. They make decisions based on:
- **Homophily:** The tendency to adopt beliefs or behaviors of similar nodes.
- **Threshold Models:** A node adopts a state only after a certain percentage of its neighbors have done so. Lutufi implements these as **Conditional Probability Tables** influenced by neighbor states.

## Multi-Layer (Multiplex) Networks
Lutufi allows for nodes to be connected via multiple edge types simultaneously. A belief might propagate through a *social* network while risk propagates through a *financial* network between the same set of actors. Lutufi's architecture treats these as coupled layers in a unified Bayesian framework.
