# What Social Networks Are

---

**Document Version:** 1.0
**Status:** Working Draft — Research Phase
**Author:** Wasswa Lutufi Sebbanja
**Last Updated:** March 2026
**License:** Apache 2.0

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Historical Development](#2-historical-development)
3. [Formal Definitions](#3-formal-definitions)
4. [Key Metrics and Measures](#4-key-metrics-and-measures)
5. [Structural Concepts](#5-structural-concepts)
6. [Tie Properties](#6-tie-properties)
7. [Homophily and Social Selection](#7-homophily-and-social-selection)
8. [Social Influence and Diffusion](#8-social-influence-and-diffusion)
9. [Network Dynamics](#9-network-dynamics)
10. [Data Collection Methods](#10-data-collection-methods)
11. [Challenges in Social Network Analysis](#11-challenges-in-social-network-analysis)
12. [Key Researchers and Schools](#12-key-researchers-and-schools)
13. [How Social Networks Connect to Lutufi](#13-how-social-networks-connect-to-lutufi)
14. [Key References](#14-key-references)

---

## 1. Introduction

A social network is a set of social actors — people, organizations, nations, or other social entities — connected by social relationships — friendships, kinship, professional collaboration, communication, economic transactions, or any form of social interaction. Social network analysis (SNA) is the study of these structures: their properties, the positions of actors within them, the consequences of network structure for individual and collective outcomes, and the processes by which networks form, evolve, and dissolve.

The fundamental insight of social network analysis is that the structure of relationships matters. An individual's behavior, opportunities, information, and influence are shaped not only by their own attributes (age, income, education) but by the pattern of relationships in which they are embedded. A well-connected person in a sparse network occupies a fundamentally different social position than an equally well-connected person in a dense network. A person who bridges two otherwise disconnected communities has access to different information and opportunities than a person embedded entirely within a single cluster. These structural insights cannot be captured by studying individuals in isolation — they require the relational perspective that social network analysis provides.

Social network analysis draws on sociology, anthropology, psychology, mathematics (graph theory), computer science, and physics. It has been applied to understand phenomena ranging from the diffusion of innovations, the spread of diseases, the formation of political alliances, the structure of criminal organizations, the dynamics of online communities, to the architecture of organizational collaboration. Its methods have become indispensable across the social sciences and increasingly in the natural sciences, public health, and computational social science.

This document provides a comprehensive treatment of social networks — their history, formal definitions, key metrics, structural concepts, dynamic processes, data collection methods, analytical challenges, and foundational researchers. It is written to serve as both a reference and a conceptual foundation for understanding how Lutufi integrates social network analysis with probabilistic reasoning.

---

## 2. Historical Development

### 2.1 The Origins: Sociometry and Sociograms (1930s)

The formal study of social networks begins with **Jacob Moreno** (1889–1974), a Romanian-American psychiatrist and social scientist. In 1934, Moreno published *Who Shall Survive?*, a groundbreaking work that introduced **sociometry** — the quantitative study of social relationships — and the **sociogram** — a graphical representation of social relationships where individuals are depicted as points and relationships as lines connecting them.

Moreno developed sociometry while studying the social dynamics of residents at the Hudson School for Girls in New York. He asked each resident to name whom they would prefer as a neighbor or companion, then mapped these choices as a directed graph. The resulting sociograms revealed patterns invisible to casual observation: isolated individuals, popular "stars," mutual choices, cliques, and chains of indirect connections. Moreno's student and collaborator, **Helen Hall Jennings**, refined these methods and contributed to their theoretical development.

Moreno's innovation was both conceptual and methodological. Conceptually, he shifted the unit of analysis from the individual to the relationship. Methodologically, he provided a visual and mathematical tool for representing and analyzing social structures. Though his work was initially embedded in group therapy and social psychology, it laid the foundation for the entire field of social network analysis.

### 2.2 The Anthropological Tradition (1940s–1960s)

In the 1940s and 1950s, social anthropologists at the **Manchester school** — notably **John Barnes**, **Elizabeth Bott**, and **J. Clyde Mitchell** — adopted network concepts to study communities in Africa, Norway, and Britain. Barnes (1954) studied a Norwegian fishing village and introduced the concept of a "social network" as an analytical tool, noting that the village's social structure could not be adequately described by formal groups alone — the informal web of personal ties was equally important.

Mitchell (1969) formalized many of the concepts used to describe network properties — density, reachability, range — and wrote a highly influential synthesis, *Social Networks in Urban Situations*, that established the theoretical vocabulary for network analysis in anthropology.

Elizabeth Bott's *Family and Social Network* (1957) demonstrated that the structure of a married couple's social network (whether their friends all knew each other or not) explained differences in conjugal role segregation. This was an early and powerful demonstration that network structure has causal consequences for behavior.

### 2.3 The Harvard Structural Analysis Program (1960s–1970s)

**Harrison White** and his students at Harvard — including **Mark Granovetter**, **Barry Wellman**, **Ronald Burt**, and **Scott Feld** — transformed social network analysis from a descriptive tool into a rigorous analytical framework. White brought algebraic approaches to social structure, developing formal theories of structural equivalence (positions in a network defined by identical patterns of relationships) and blockmodeling (partitioning a network into structurally equivalent positions).

**Mark Granovetter's** 1973 paper, "The Strength of Weak Ties," became one of the most cited papers in the social sciences. Granovetter argued that weak ties — acquaintances rather than close friends — are paradoxically more valuable for accessing novel information (such as job opportunities) because they bridge otherwise disconnected social circles. Strong ties tend to connect people who know the same things; weak ties connect people to different information worlds.

This period also saw the development of formal mathematical tools for network analysis. In particular, the application of graph theory and matrix algebra to social network data became standard practice.

### 2.4 The Small World Experiments and Beyond (1960s–1990s)

**Stanley Milgram's** small world experiments (1967) demonstrated that any two people in the United States were connected by a surprisingly short chain of acquaintances — on average about six intermediaries, giving rise to the popular concept of "six degrees of separation." Milgram's experimental design — asking participants to forward a letter to a target person through personal acquaintances — was simple but profound, revealing that large social networks have short average path lengths.

Milgram's findings were given mathematical formalization by **Duncan Watts** and **Steven Strogatz** (1998), who proposed the "small-world network" model. They showed that adding just a few random "shortcut" edges to a regular lattice graph dramatically reduces the average path length while maintaining high local clustering — a combination they argued was characteristic of many real-world social networks.

### 2.5 The Scale-Free Network Revolution (1999–2000s)

**Albert-László Barabási** and **Réka Albert** (1999) discovered that many real-world networks — including the World Wide Web, citation networks, and some social networks — exhibit **scale-free** degree distributions: the probability that a node has degree k follows a power law P(k) ~ k^(-γ), with most nodes having few connections and a few "hubs" having many. They proposed the **preferential attachment** mechanism as an explanation: new nodes joining the network are more likely to connect to nodes that already have many connections ("the rich get richer").

The Barabási-Albert model sparked an explosion of research in network science, drawing physicists, computer scientists, and mathematicians into what had been primarily a sociological enterprise. This interdisciplinary convergence created the modern field of **network science** — a broader discipline that studies networks in all domains (biological, technological, social, economic) using common mathematical and computational tools.

### 2.6 The Computational and Digital Turn (2000s–Present)

The rise of digital communication and social media platforms (Facebook, Twitter/X, LinkedIn, WeChat) has transformed social network analysis in several ways:

- **Scale.** Networks of millions or billions of nodes are now observable, far exceeding what was possible with survey-based methods.
- **Granularity.** Digital trace data provides timestamped, content-rich records of interactions — who communicated with whom, when, about what.
- **Dynamics.** Time-stamped data enables the study of network evolution in real-time, rather than through periodic snapshots.
- **Computational methods.** The scale of digital network data has driven the development of scalable algorithms for community detection, influence estimation, link prediction, and network embedding.

Simultaneously, **Nicholas Christakis** and **James Fowler** (2007, 2009) brought social network analysis to public attention with studies claiming that obesity, smoking, happiness, and other traits spread through social networks like contagions. Their work in the Framingham Heart Study social network reignited debates about social influence versus homophily and about the methodological challenges of causal inference from observational network data.

---

## 3. Formal Definitions

### 3.1 Graph Representations

Formally, a social network is represented as a graph **G = (V, E)** where:

- **V = {v₁, v₂, …, vₙ}** is the set of **nodes** (also called vertices, actors, or agents), representing social entities.
- **E ⊆ V × V** is the set of **edges** (also called ties, links, or arcs), representing relationships between entities.

### 3.2 Directed and Undirected Networks

- **Undirected network.** An edge {u, v} represents a symmetric relationship — if u is connected to v, then v is connected to u. Examples: co-membership in an organization, co-authorship, mutual friendship (on some platforms).
- **Directed network.** An edge (u, v) represents an asymmetric relationship — u is related to v, but not necessarily vice versa. Examples: following on Twitter, sending an email, citing a paper, hierarchical authority.

### 3.3 Weighted Networks

In a **weighted network**, each edge carries a numerical weight w(u, v) representing the strength, frequency, capacity, or importance of the relationship. Examples: frequency of communication, volume of trade, emotional closeness.

### 3.4 Bipartite (Two-Mode) Networks

A **bipartite network** has two disjoint sets of nodes (e.g., people and organizations, authors and papers), with edges only between nodes of different types. Bipartite networks are natural for affiliation data (people connected to events, organizations, or groups they belong to). One-mode projections can be derived: e.g., a co-authorship network (two authors connected if they share a paper) is a projection of the bipartite author-paper network.

### 3.5 Multiplex Networks

A **multiplex network** has multiple types of edges between the same set of nodes. For example, two people might be connected by a friendship tie, a professional collaboration tie, and an email communication tie. Each type of tie forms a separate **layer** of the multiplex network. Formally: G = (V, E₁, E₂, …, Eₖ) where each Eᵢ is a set of edges of type i.

### 3.6 Temporal (Dynamic) Networks

A **temporal network** records when edges exist. Formally: G(t) = (V(t), E(t)), where both the node set and edge set may change over time. Alternatively, each edge may carry a timestamp or a set of time intervals during which it is active. Temporal networks enable the study of network evolution, cascade dynamics, and temporal reachability.

### 3.7 Matrix Representations

**Adjacency matrix.** An n × n matrix **A** where Aᵢⱼ = 1 if there is an edge from vᵢ to vⱼ, and Aᵢⱼ = 0 otherwise. For weighted networks, Aᵢⱼ = w(vᵢ, vⱼ). For undirected networks, A is symmetric: Aᵢⱼ = Aⱼᵢ.

**Edge list.** A list of pairs (or triples, for weighted networks): [(v₁, v₂, w₁₂), (v₃, v₄, w₃₄), …]. Compact for sparse networks.

**Incidence matrix.** An n × m matrix **B** where n is the number of nodes and m is the number of edges: Bᵢⱼ = 1 if node vᵢ is incident to edge eⱼ. Used less commonly than adjacency matrices but useful for certain algebraic analyses.

---

## 4. Key Metrics and Measures

### 4.1 Degree and Degree Distribution

The **degree** of a node is the number of edges incident to it.

- In undirected networks: deg(v) = |{u ∈ V : {u, v} ∈ E}|
- In directed networks: **in-degree** deg⁻(v) = |{u : (u, v) ∈ E}| and **out-degree** deg⁺(v) = |{u : (v, u) ∈ E}|

The **degree distribution** P(k) gives the fraction of nodes with degree k. Many real-world social networks exhibit right-skewed degree distributions, often approximated by power laws (P(k) ~ k^(-γ)) or log-normal distributions.

**Mean degree:** ⟨k⟩ = (1/n) Σᵢ deg(vᵢ) = 2|E|/n for undirected networks.

### 4.2 Centrality Measures

Centrality measures quantify the importance or prominence of a node within the network. Different centrality measures capture different notions of importance:

**Degree centrality.** C_D(v) = deg(v) / (n - 1). The simplest measure: important nodes are those with many connections. High degree centrality indicates local popularity or activity.

**Betweenness centrality.** C_B(v) = Σ_{s≠v≠t} σ_st(v) / σ_st, where σ_st is the total number of shortest paths from s to t, and σ_st(v) is the number of those paths passing through v. Introduced by **Linton Freeman** (1977). High betweenness centrality identifies **brokers** — nodes that control information flow between different parts of the network. Removing a high-betweenness node can disconnect the network.

**Closeness centrality.** C_C(v) = (n - 1) / Σ_{u≠v} d(v, u), where d(v, u) is the shortest path length between v and u. Nodes with high closeness centrality can reach all other nodes quickly. Closeness is problematic in disconnected networks (paths to unreachable nodes have infinite length), leading to variants like harmonic centrality: C_H(v) = (1/(n-1)) Σ_{u≠v} 1/d(v,u).

**Eigenvector centrality.** The centrality of a node is proportional to the sum of the centralities of its neighbors. Formally, the eigenvector centrality vector **x** satisfies A**x** = λ**x**, where A is the adjacency matrix and λ is the largest eigenvalue. Introduced by **Phillip Bonacich** (1972, 1987). High eigenvector centrality means being connected to other important nodes, not just many nodes.

**PageRank.** Developed by **Larry Page** and **Sergey Brin** (1998) for ranking web pages. PageRank is a variant of eigenvector centrality for directed networks that incorporates a damping factor: with probability d (typically 0.85), follow a random outgoing link; with probability 1-d, jump to a random node. PageRank can be interpreted as the stationary distribution of a random walk on the directed network.

**Katz centrality.** C_Katz(v) = Σ_{k=1}^∞ Σ_{u} α^k (A^k)_{vu}, where α is an attenuation factor (0 < α < 1/λ₁) and A^k counts walks of length k. Katz centrality counts all paths (not just shortest paths) from every node to v, with longer paths attenuated by α^k.

### 4.3 Clustering and Transitivity

**Clustering coefficient** (local). The fraction of a node's neighbors that are connected to each other:

C(v) = 2 · |{(u, w) : u, w ∈ N(v), {u, w} ∈ E}| / (deg(v) · (deg(v) - 1))

where N(v) is v's neighbor set. High clustering indicates that a node's friends tend to also be friends with each other — the network is locally cohesive.

**Transitivity** (global clustering coefficient). The fraction of all possible triangles that are actually present:

T = 3 × (number of triangles) / (number of connected triples)

The factor of 3 accounts for the fact that each triangle contains three connected triples.

Real-world social networks typically show much higher clustering than random graphs of the same size and density, reflecting the sociological tendency toward triadic closure ("a friend of my friend is likely to become my friend").

### 4.4 Path Length and Diameter

**Shortest path length** (geodesic distance) d(u, v) is the minimum number of edges on any path from u to v.

**Average path length:** L = (1 / n(n-1)) Σ_{u≠v} d(u, v). In many real-world social networks, L is small relative to the network size — the "small world" property.

**Diameter:** D = max_{u,v} d(u, v). The longest shortest path in the network.

### 4.5 Density

**Density** is the fraction of possible edges that are present:

ρ = |E| / (n(n-1)/2) for undirected networks
ρ = |E| / (n(n-1)) for directed networks

Social networks are typically **sparse** — density decreases as the network grows, because the number of edges grows slower than n². Dunbar's number (approximately 150) suggests a cognitive limit on the number of stable social relationships a person can maintain, contributing to sparsity in large networks.

### 4.6 Assortativity

**Assortativity** (or degree correlation) measures whether high-degree nodes tend to connect to other high-degree nodes (assortative mixing) or to low-degree nodes (disassortative mixing).

The **assortativity coefficient** r (Newman, 2002) is the Pearson correlation of degrees at the two ends of an edge:

r = [Σᵢ jᵢkᵢ - (Σᵢ (jᵢ + kᵢ)/2)² / M] / [Σᵢ (jᵢ² + kᵢ²)/2 - (Σᵢ (jᵢ + kᵢ)/2)² / M]

where jᵢ, kᵢ are the degrees of the two endpoints of edge i, and M is the number of edges.

Social networks tend to be **assortative** (r > 0) — popular people associate with popular people. This contrasts with many technological and biological networks, which tend to be disassortative.

---

## 5. Structural Concepts

### 5.1 Communities and Modules

A **community** (also called a module, cluster, or group) is a subset of nodes that are more densely connected to each other than to nodes outside the subset. Community detection is one of the most active areas in network analysis.

**Modularity** Q, introduced by Newman and Girvan (2004), measures the quality of a partition into communities:

Q = (1/2M) Σᵢⱼ [Aᵢⱼ - kᵢkⱼ/(2M)] δ(cᵢ, cⱼ)

where M is the total number of edges, kᵢ is the degree of node i, cᵢ is the community assignment of node i, and δ is the Kronecker delta. Q compares the actual edge density within communities to the expected density under a random null model.

**Community detection algorithms** include:
- **Girvan-Newman** (2002): iteratively removes edges with the highest betweenness centrality, revealing community structure.
- **Louvain algorithm** (Blondel et al., 2008): a fast, greedy modularity optimization method. The most widely used algorithm for large networks.
- **Label propagation** (Raghavan et al., 2007): each node adopts the label most common among its neighbors. Simple and fast.
- **Spectral methods**: use the eigenvalues and eigenvectors of the Laplacian matrix L = D - A (where D is the diagonal degree matrix) to identify communities. The Fiedler vector (second-smallest eigenvector of L) provides a bipartition.
- **Stochastic block models** (Holland, Laskey & Leinhardt, 1983): generative probabilistic models that explicitly model group membership and between-group connection probabilities. Provide a principled statistical framework for community detection and model selection.

### 5.2 Core-Periphery Structure

Many networks exhibit a **core-periphery** structure: a densely connected core of nodes and a sparsely connected periphery. Core nodes are connected to both core and periphery; periphery nodes are connected mainly to core nodes, not to each other. This structure was formalized by **Borgatti and Everett** (2000).

Core-periphery structure is particularly important in economic networks (e.g., the interbank lending network has a core of large, interconnected banks and a periphery of smaller banks) and in organizational networks (core members vs. peripheral members).

### 5.3 Bridges and Brokers

A **bridge** is an edge whose removal would disconnect the network (or increase the number of connected components). In practice, true bridges are rare in social networks; more useful is the concept of a **local bridge** — an edge between two nodes whose neighborhoods do not overlap, meaning it provides the shortest path (of length 2) between two otherwise more distant regions of the network.

**Structural holes** — a concept introduced by **Ronald Burt** (1992) — are gaps between non-redundant contacts. An actor who spans structural holes — who is connected to people who are not connected to each other — has access to diverse information and occupies a powerful brokerage position. Burt developed the **constraint** measure to quantify the extent to which an actor's contacts are redundant (connected to each other), with low constraint indicating many structural holes.

Burt's structural holes theory is closely related to Granovetter's weak ties theory: weak ties are more likely to bridge structural holes, providing non-redundant information.

### 5.4 Structural Equivalence and Role Analysis

Two nodes are **structurally equivalent** if they have identical relationships to every other node in the network. Formally, nodes u and v are structurally equivalent if N(u) \ {v} = N(v) \ {u} — they have the same neighbors (excluding each other).

Structural equivalence is typically too strict for practical analysis (exact equivalence is rare). Relaxations include:
- **Regular equivalence** (White & Reitz, 1983): nodes are regularly equivalent if they have equivalent neighbors, allowing recursion.
- **Automorphic equivalence**: nodes are equivalent if there is a graph automorphism mapping one to the other.

**Blockmodeling** (White, Boorman & Breiger, 1976) partitions the network into positions (blocks of structurally equivalent or regularly equivalent nodes) and describes the relationships between positions. The result is a reduced, interpretable image of the network's social structure.

### 5.5 Cliques, K-Cores, and K-Plexes

**Clique.** A maximal complete subgraph — a set of nodes where every pair is connected. In social networks, cliques represent tightly knit groups. Finding all maximal cliques is computationally expensive (the Bron-Kerbosch algorithm is the standard method).

**K-core.** The maximal subgraph in which every node has at least k connections within the subgraph. K-core decomposition provides a hierarchical view of network cohesion — higher k-cores represent more cohesive regions.

**K-plex.** A subgraph in which every node is connected to all but at most k other members. This is a relaxation of the clique concept that allows some missing ties.

---

## 6. Tie Properties

### 6.1 Strong and Weak Ties

**Tie strength** is a multi-dimensional concept encompassing the amount of time spent together, emotional intensity, intimacy (mutual confiding), and reciprocal services. This conceptualization comes from **Granovetter** (1973), who distinguished between strong ties (close relationships with high emotional investment) and weak ties (acquaintances with lower emotional investment).

### 6.2 The Strength of Weak Ties

Granovetter's most influential argument is that **weak ties are disproportionately important for transmitting novel information**. The reasoning is structural:

1. Strong ties tend to form clusters — if A is strongly tied to both B and C, then B and C are likely also strongly tied to each other (creating a triangle). This means strong ties connect people within the same information environment.
2. Weak ties are more likely to be **bridges** — connecting people in different clusters who have access to different information.
3. Therefore, novel information (e.g., about job opportunities, innovations, or news) is more likely to travel through weak ties, which bridge different social worlds.

Granovetter found empirical support: people who found jobs through personal contacts were more likely to have heard about the job from an acquaintance (weak tie) than a close friend (strong tie).

The theory has been extensively tested and refined. It applies most clearly to the diffusion of information and opportunities. For complex tasks requiring trust and cooperation, strong ties may be more important (Hansen, 1999; Uzzi, 1997).

### 6.3 Multiplexity

**Multiplexity** refers to the number of different types of relationships between two actors. Multiplex ties (e.g., being both friends and colleagues) are generally stronger, more durable, and more influential than uniplex ties. Multiplexity is a key concept for understanding the layered nature of real social relationships.

### 6.4 Reciprocity

**Reciprocity** is the tendency for relationships to be mutual — if A is connected to B, then B is connected to A. In directed networks, the reciprocity coefficient measures the fraction of mutual dyads:

r = (number of mutual dyads) / (number of dyads with at least one tie)

Reciprocity is generally high in social networks (most friendships, for instance, are mutual) but varies by relationship type (e.g., Twitter following has lower reciprocity).

---

## 7. Homophily and Social Selection

### 7.1 The Homophily Principle

**Homophily** — literally "love of the same" — is the tendency for people to form ties with others who are similar to them. The aphorism "birds of a feather flock together" captures this phenomenon, which is one of the most robust findings in the social sciences.

**McPherson, Smith-Lovin, and Cook** (2001) provided a comprehensive review, documenting homophily along dimensions including race, ethnicity, age, religion, education, occupation, gender, attitudes, and behavior. The strength and pattern of homophily varies across dimensions and contexts, but the general tendency is pervasive.

### 7.2 Status Homophily vs. Value Homophily

**Lazarsfeld and Merton** (1954) distinguished between:
- **Status homophily**: similarity on ascribed (race, sex, age) or achieved (education, occupation, income) status characteristics.
- **Value homophily**: similarity on attitudes, beliefs, and values.

Both forms drive tie formation, but status homophily is generally stronger and more visible.

### 7.3 Selection vs. Influence

A critical methodological challenge is distinguishing **selection** (people choose friends who are similar to them) from **influence** (people become similar to their friends). Both processes produce the same observed pattern (similarity between connected individuals), but they have different causal mechanisms and different policy implications.

For example, if smokers are friends with other smokers, is this because:
- **(a) Selection:** Smokers preferentially befriend other smokers.
- **(b) Influence:** Being friends with smokers makes one more likely to start or continue smoking.
- **(c) Confounding:** A shared environment (e.g., living in the same dormitory) causes both the friendship and the smoking.

Disentangling these mechanisms requires longitudinal data and sophisticated statistical methods (e.g., SIENA models by Snijders et al., 2010, or Manski's reflection problem framework). This selection-vs-influence distinction is a central concern for Lutufi's causal inference features.

---

## 8. Social Influence and Diffusion

### 8.1 Types of Diffusion Processes

Social networks are conduits for the spread of information, behaviors, innovations, diseases, and cultural practices. Several canonical models describe diffusion processes:

**Simple contagion.** Each exposure to an "infected" neighbor independently creates a probability of adoption/infection. This is the mechanism in standard epidemiological models (SIR, SIS). A single contact with one infected neighbor may be sufficient for transmission. Simple contagion favors networks with short path lengths and high connectivity.

**Complex contagion** (Centola & Mace, 2007). Adoption requires exposure to multiple independent sources — reinforcement from several neighbors rather than a single contact. Complex contagion is more relevant for behaviors that are risky, costly, or socially contested (e.g., adopting a new technology, joining a social movement, changing political opinions). Complex contagion favors clustered networks with redundant ties, because an individual needs multiple neighbors to have adopted before they adopt themselves.

**Threshold models** (Granovetter, 1978). Each individual has a **threshold** — the fraction (or number) of their neighbors who must have adopted before they will adopt. Thresholds may vary across individuals. Granovetter showed that even small changes in the distribution of thresholds can produce dramatically different cascade outcomes — from near-zero adoption to near-complete adoption.

**Linear threshold and independent cascade models** (Kempe, Kleinberg & Tardos, 2003). Formalized influence maximization: given a network and a diffusion model, find the k "seed" nodes whose initial adoption will maximize the expected number of eventual adopters. This problem is NP-hard in general but can be approximated using submodularity.

### 8.2 Information Diffusion

Information spreads through social networks via direct communication, observation, and intermediated sharing. Key phenomena include:

- **Viral spreading.** Content (news, memes, rumors) that spreads rapidly through network sharing.
- **Echo chambers and filter bubbles.** Homophily and algorithmic filtering can create network structures where an individual is exposed primarily to information that confirms their existing beliefs.
- **Misinformation and disinformation.** False information can spread through social networks, sometimes faster than corrections. Understanding the network structure of misinformation propagation is critical for designing effective countermeasures.

### 8.3 Innovation Diffusion

**Everett Rogers'** *Diffusion of Innovations* (1962, 5th ed. 2003) provided the classic framework for understanding how new ideas, technologies, and practices spread through social systems. Rogers identified categories of adopters (innovators, early adopters, early majority, late majority, laggards), the S-shaped adoption curve, and the role of opinion leaders and change agents. Network position — particularly centrality and connectivity — strongly predicts who adopts early and who serves as a bridge for diffusion.

### 8.4 Social Influence on Behavior

The Framingham Heart Study social network analyses by **Christakis and Fowler** (2007, 2008) claimed to demonstrate that obesity, smoking, and happiness spread through social networks. Their provocative finding — that a friend of a friend of a friend's behavior could influence one's own — generated both excitement and methodological critique. **Shalizi and Thomas** (2011) and **Lyons** (2011) pointed out that the statistical methods used could not distinguish genuine influence from latent homophily (unobserved shared characteristics that drive both tie formation and behavior). This debate underscores the importance of careful causal reasoning in network analysis — precisely the capability that Lutufi aims to provide.

---

## 9. Network Dynamics

### 9.1 Network Formation

How do social networks form? Several mechanisms have been identified:

**Propinquity (proximity).** People form ties with those who are physically or socially nearby. Proximity creates opportunities for interaction, which is a necessary precondition for tie formation. The classic studies by **Festinger, Schachter, and Back** (1950) showed that friendship formation in a housing project was strongly predicted by physical distance.

**Triadic closure.** If A knows B and A knows C, there is an elevated probability that B and C will form a tie. This is the mechanism behind the high clustering observed in social networks. Triadic closure is driven by increased opportunity (B and C meet through A), social pressure (A introduces B and C), and trust transitivity (if A trusts both B and C, B and C have reason to trust each other). Rapoport (1953) first identified triadic closure as a key process; Granovetter (1973) connected it to his weak ties theory.

**Preferential attachment.** New nodes arriving to the network preferentially connect to nodes that already have many connections. In social networks, this manifests as the "rich get richer" or "Matthew effect" — popular individuals attract more connections because they are visible, accessible, and perceived as valuable contacts.

**Homophily.** As discussed in Section 7, similarity drives tie formation.

**Strategic tie formation.** In some contexts (e.g., professional networking, alliance formation, organizational partnerships), tie formation is deliberately strategic — actors choose ties that maximize their access to resources, information, or influence. This perspective connects to the game-theoretic approach to network formation (Jackson & Wolinsky, 1996).

### 9.2 Network Evolution Models

**Erdős–Rényi (ER) random graph model** (1959, 1960). Each possible edge exists independently with probability p. The ER model produces networks with Poisson degree distributions, low clustering, and short path lengths. It serves as the simplest null model — a baseline against which to compare real network properties.

**Barabási-Albert (BA) preferential attachment model** (1999). Nodes are added one at a time, each connecting to m existing nodes with probability proportional to their current degree. The BA model produces scale-free networks with power-law degree distributions. It captures the "rich get richer" dynamic observed in many real-world networks.

**Watts-Strogatz (WS) small-world model** (1998). Start with a regular lattice (e.g., a ring where each node is connected to its k nearest neighbors). With probability p, rewire each edge to a randomly chosen node. Small p produces high clustering and short path lengths — the small-world property. The WS model captures the co-existence of local cohesion and global connectivity.

**Stochastic actor-oriented models (SAOMs)** — **Tom Snijders** and colleagues (Snijders, 2001; Snijders, van de Bunt & Steglich, 2010). SAOMs model network evolution as a continuous-time Markov process where actors make myopic decisions about tie formation and dissolution. These models can simultaneously estimate the effects of network structure (e.g., triadic closure), actor attributes (e.g., homophily), and the co-evolution of networks and behavior (e.g., selection vs. influence). The **SIENA** (Simulation Investigation for Empirical Network Analysis) software implements SAOMs.

**Exponential random graph models (ERGMs)** — also called p* models (Wasserman & Pattison, 1996; Robins et al., 2007). ERGMs are statistical models for network structure that express the probability of an observed network as an exponential function of network statistics (e.g., number of edges, triangles, stars). ERGMs enable hypothesis testing about which structural features are significantly over- or under-represented in an observed network compared to chance.

### 9.3 Network Dissolution

Networks dissolve when ties are broken permanently or when nodes exit. Tie dissolution is driven by:
- **Life changes**: geographic moves, job changes, retirement, death
- **Conflict**: interpersonal disputes, broken trust
- **Opportunity cost**: maintaining ties requires effort; as new ties form, old ties may be dropped
- **Structural decay**: without reinforcement through interaction, ties weaken over time

The dissolution side of network dynamics has received less attention than formation, but it is critical for understanding network resilience and stability.

---

## 10. Data Collection Methods

### 10.1 Survey Methods

**Name generators** ask respondents to list people with whom they have a specific type of relationship ("Please name up to five people you consider your closest friends"). Name generators provide detailed relational data but are limited by the number of alters that can be named and by the respondent's recall and willingness to disclose.

**Name interpreters** (also called name attribute questions) ask follow-up questions about each named alter — their demographics, the nature of the relationship, and whether the named alters know each other. This provides rich data about tie properties and local network structure.

**Position generators** (Lin & Dumin, 1986) ask respondents whether they know someone in each of a set of occupational positions. This provides a measure of social capital — access to diverse resources through network ties.

**Roster methods** present respondents with a complete list of potential alters (e.g., all members of a class, workplace, or organization) and ask them to indicate their relationships with each. Roster methods provide complete network data within a bounded population but are impractical for large or open populations.

### 10.2 Digital Trace Data

Digital platforms generate vast quantities of relational data:
- **Social media**: friendship/following links, retweets, mentions, comments, likes
- **Communication records**: email headers (sender/recipient), phone call records (metadata, not content), messaging app contacts
- **Collaboration platforms**: co-editing documents, shared calendar events, team memberships
- **Transaction data**: financial transfers, co-purchases, co-attendance

Digital trace data offers unprecedented scale and temporal resolution but raises issues of representativeness (not everyone uses every platform), behavioral interpretation (does a "like" constitute a meaningful social tie?), and privacy.

### 10.3 Observational Methods

In some settings, networks are constructed from observed behavior:
- **Ethnographic observation**: researchers observe and record interactions in natural settings (workplaces, schools, community gatherings).
- **Behavioral indicators**: physical proximity tracked by sensors (sociometric badges, Bluetooth, WiFi), shared activities, co-presence.

### 10.4 Snowball Sampling and Respondent-Driven Sampling

For hidden or hard-to-reach populations (e.g., drug users, sex workers, undocumented immigrants), probability sampling is often impossible. **Snowball sampling** starts with initial "seed" respondents and asks them to recruit their social contacts, who in turn recruit their contacts.

**Respondent-driven sampling (RDS)** — developed by **Douglas Heckathorn** (1997, 2002) — is a variant of snowball sampling with mathematical adjustments to produce unbiased population estimates. RDS uses a dual-incentive system: respondents are rewarded for being interviewed and for recruiting others. Statistical weights are computed based on each respondent's degree (network size) to correct for the non-random sampling pattern.

---

## 11. Challenges in Social Network Analysis

### 11.1 Boundary Specification

Every network analysis requires defining the boundary of the network — who is "in" and who is "out." This is straightforward for bounded populations (a classroom, a company) but problematic for open social systems. The choice of boundary affects all network measures — a network that appears disconnected at one boundary may be connected if the boundary is expanded.

**Laumann, Marsden, and Prensky** (1983) distinguished between **realist** boundaries (using actors' own definitions of group membership) and **nominalist** boundaries (defined by the researcher based on analytic criteria). Neither approach is universally correct; the choice depends on the research question.

### 11.2 Missing Data

Missing data is endemic in social network analysis and particularly damaging because the unit of analysis is relational — a missing node removes all its incident edges, and a missing edge removes a relationship between two nodes.

**Kossinets** (2006) systematically studied the effects of missing data on network statistics and found that even small amounts of missing data can substantially bias centrality, clustering, and community detection results. The bias pattern depends on whether data is missing at random or systematically.

**Types of missing data in networks:**
- **Missing nodes**: entire actors are absent from the data (e.g., non-respondents in a survey).
- **Missing edges**: some relationships are not reported (e.g., respondents forget or choose not to disclose certain contacts).
- **Missing attributes**: actor characteristics are incomplete.
- **Boundary-induced missingness**: ties that cross the network boundary are unobserved.

The Rubin taxonomy (MCAR, MAR, MNAR) applies to network data but with additional complexity because missingness can depend on network structure (e.g., peripheral nodes are more likely to be missing).

### 11.3 Ethical Concerns and Privacy

Social network data raises significant ethical concerns:

- **Informed consent.** In a network study, data about person A inherently involves information about A's contacts. If A consents but their contacts do not, is the use of their relational data ethical?
- **Identification risk.** Network data is inherently identifiable — even with names removed, the structural position of a node can be unique, enabling re-identification. Backstrom, Dwork, and Kleinberg (2007) demonstrated that attackers can re-identify individuals in anonymized social networks using structural information.
- **Downstream harms.** Network analysis can be used for surveillance, targeting, and manipulation. Social network analysis has been used in counter-terrorism (mapping terrorist networks), law enforcement (identifying criminal networks), and marketing (identifying influential individuals for targeting). These applications raise questions about the balance between analytical utility and potential for harm.
- **Platform-specific concerns.** Data from social media platforms is typically governed by terms of service that may restrict research use. The Cambridge Analytica scandal (2018) highlighted the risks of inadequate data governance.

### 11.4 Online vs. Offline Networks

Digital social networks (Facebook friends, Twitter followers) and offline social networks (face-to-face friends, family) overlap but are not identical. Key differences include:
- **Tie strength**: many online ties are weak or inactive.
- **Context**: online interactions lack physical co-presence cues.
- **Selection effects**: who uses a platform and how they use it is non-random.
- **Platform affordances**: the features of the platform (directed vs. undirected ties, public vs. private interactions, algorithmic curation) shape the network structure.

Research increasingly studies the interplay between online and offline networks, recognizing that they are complementary rather than substitutable.

### 11.5 Causality and Endogeneity

Establishing causal claims from network data is exceptionally challenging due to:

- **Endogeneity**: network ties are not randomly assigned. People choose their friends, and the same factors that drive tie formation may also drive the outcomes being studied.
- **Homophily/confounder confusion**: as discussed in Section 7.3, observed similarity between connected individuals can be due to selection, influence, or shared environment.
- **Simultaneity**: network structure affects individual outcomes, but individual outcomes also affect network structure (feedback loops).
- **Manski's reflection problem** (1993): in a model where an individual's outcome depends on the average outcome of their group (network neighbors), it is generally impossible to separately identify endogenous effects (influence from peers' outcomes), exogenous effects (influence from peers' characteristics), and correlated effects (shared environment).

Addressing these challenges requires either experimental designs (randomized interventions on networks), instrumental variable approaches, or structural models with strong identifying assumptions.

---

## 12. Key Researchers and Schools

### 12.1 Foundational Figures

**Jacob Moreno (1889–1974).** Founder of sociometry. Introduced sociograms and the quantitative study of social relationships. *Who Shall Survive?* (1934).

**Linton Freeman (1927–2018).** Formalized betweenness centrality (1977) and wrote the definitive history of social network analysis (*The Development of Social Network Analysis*, 2004). Founded the journal *Social Networks*.

**Harrison White (1930–2024).** Harvard sociologist who brought mathematical rigor to social structure. Developed blockmodeling and algebraic approaches. Trained a generation of network analysts including Granovetter, Wellman, and Burt.

**Mark Granovetter (1943–).** "The Strength of Weak Ties" (1973). Demonstrated that network structure shapes economic opportunities. *Getting a Job* (1974/1995) showed how job-seekers find employment through weak ties.

### 12.2 Structural Analysis School

**Ronald Burt (1949–).** Structural holes theory (1992). Developed the constraint measure and demonstrated the advantages of brokerage positions in organizations. *Structural Holes: The Social Structure of Competition* (1992), *Brokerage and Closure* (2005).

**Stanley Wasserman** and **Katherine Faust.** Authors of *Social Network Analysis: Methods and Applications* (1994), the standard methods textbook for the field. Wasserman also contributed to the development of p*/ERGM models.

**Peter Marsden.** Made major contributions to network measurement methodology, including the study of discussion networks and the General Social Survey network module.

### 12.3 Network Science and Physics

**Albert-László Barabási (1967–).** Discovered scale-free networks with Réka Albert (1999). Established the network science paradigm bridging physics and social science. *Network Science* (2016) textbook.

**Duncan Watts (1971–).** Co-developed the small-world model with Steven Strogatz (1998). *Six Degrees: The Science of a Connected Age* (2003). Now at University of Pennsylvania, studying collective intelligence and experimental social science.

**Mark Newman.** Physicist who made major contributions to community detection (modularity), network metrics (assortativity), and network theory. *Networks: An Introduction* (2010, 2nd ed. 2018).

### 12.4 Statistical Network Modeling

**Tom Snijders.** Developed stochastic actor-oriented models (SAOMs) for network dynamics and the SIENA software. A leader in longitudinal network analysis and multilevel network analysis.

**Pip Pattison** and **Garry Robins.** Developed the ERGM (p*) framework for statistical modeling of network structure. Made ERGMs practical by developing improved estimation methods (MCMC-MLE) and goodness-of-fit procedures.

### 12.5 Social Influence and Health

**Nicholas Christakis (1962–)** and **James Fowler.** Claimed that health behaviors and emotions spread through social networks (the "three degrees of influence" hypothesis). *Connected: The Surprising Power of Our Social Networks* (2009). Their work, while controversial, brought social network analysis to a wide audience and spurred methodological advances in network causal inference.

### 12.6 Computational Social Science

**David Lazer.** A leader in computational social science and network analysis of political systems. Co-authored "Computational Social Science" (2009, *Science*) and "The Parable of Google Flu" (2014).

**Sinan Aral.** Studies social influence and diffusion in large-scale digital networks. Conducted large-scale randomized experiments to distinguish influence from homophily. *The Hype Machine* (2020).

---

## 13. How Social Networks Connect to Lutufi

### 13.1 The Pain Points Lutufi Addresses

Social network analysts face several practical challenges that Lutufi is designed to solve:

**The two-tool problem.** Currently, analyzing a social network probabilistically requires using a graph analysis library (NetworkX, igraph) for structural metrics and a separate probabilistic modeling library (pgmpy, PyMC, Stan) for inference. The translation between these tools is manual, error-prone, and lossy — network structure must be re-encoded as a probabilistic model, with no guarantee that the translation preserves important properties.

**Uncertainty quantification.** Standard network metrics (centrality, community membership) are computed as point estimates, ignoring the uncertainty inherent in the data. If the network data has missing edges, measurement error, or boundary effects, the point estimates may be misleading. Lutufi computes network metrics as probability distributions, quantifying uncertainty and enabling robust decision-making.

**Causal reasoning.** Social scientists are increasingly interested in causal questions — Does network position cause outcomes, or is it merely correlated? Does influence propagation cause similar behavior among friends, or is it selection? — but standard network analysis tools do not support causal reasoning. Lutufi's do-calculus implementation enables interventional and counterfactual queries on social networks.

**Missing data.** Social network data is almost always incomplete. Standard tools either discard incomplete observations or require the user to impute missing data as a preprocessing step. Lutufi handles missing data within the probabilistic inference framework, using EM and multiple imputation methods that account for the network structure of missingness.

**Temporal dynamics.** Social networks evolve over time, but most tools provide only static snapshots. Lutufi's dynamic Bayesian network support enables modeling network evolution, temporal influence propagation, and change-point detection.

### 13.2 What Lutufi Enables for Social Network Research

With Lutufi, social network researchers can:

- **Model influence propagation probabilistically.** Specify a dynamic Bayesian network where each actor's opinion at time t depends on their opinion at time t-1 and on the opinions of their network neighbors. Run inference to compute the posterior probability of specific influence cascades.

- **Assess the impact of network interventions.** Using do-calculus: "What is the posterior probability of a cascade if we immunize (or remove, or amplify) specific nodes?" This supports intervention targeting in public health, counter-terrorism, and marketing.

- **Quantify network metric uncertainty.** Instead of computing a single betweenness centrality value, compute the posterior distribution of betweenness centrality given uncertainty about which edges exist.

- **Detect covert network structure.** Given partial observations (some known ties, some known non-ties, many unknowns), use Bayesian inference to estimate the probability that each unobserved edge exists. This is directly relevant to intelligence analysis and law enforcement.

- **Disentangle selection from influence.** By specifying causal models that distinguish selection effects from influence effects, and using Lutufi's causal inference machinery to estimate causal effects from longitudinal data.

- **Model multiplex networks.** Treat different relationship types as different random variables, with dependencies between layers specified by the Bayesian network structure.

### 13.3 Technical Integration

Lutufi integrates social network concepts in several concrete ways:

- **Network-native data model.** Nodes carry both structural attributes (degree, centrality, community membership) and probabilistic attributes (random variable states, CPDs). Edges carry both structural properties (weight, type, timestamp) and probabilistic properties (dependency strength, causal direction).

- **Network metric computation.** Standard SNA metrics are computed within the probabilistic framework, enabling uncertainty-aware versions of all standard measures.

- **Community detection as latent variable inference.** Community membership is treated as a latent variable, inferred jointly with other unknowns. This provides a probabilistic assignment of nodes to communities (soft clustering) with uncertainty quantification, rather than a hard partition.

- **Network visualization.** Lutufi's visualization layer can display both structural properties (layout algorithms, community coloring) and probabilistic properties (node color intensity for posterior probability, edge width for conditional dependency strength).

---

## 14. Key References

1. **Wasserman, S. & Faust, K.** (1994). *Social Network Analysis: Methods and Applications*. Cambridge University Press. — The standard methods textbook for social network analysis.

2. **Newman, M. E. J.** (2018). *Networks* (2nd ed.). Oxford University Press. — The most comprehensive modern textbook on network science, covering both social and non-social networks.

3. **Granovetter, M. S.** (1973). "The Strength of Weak Ties." *American Journal of Sociology*, 78(6), 1360–1380. — One of the most cited papers in the social sciences, establishing the paradoxical importance of weak ties.

4. **Barabási, A.-L. & Albert, R.** (1999). "Emergence of Scaling in Random Networks." *Science*, 286(5439), 509–512. — Discovered scale-free networks and the preferential attachment mechanism.

5. **Watts, D. J. & Strogatz, S. H.** (1998). "Collective Dynamics of 'Small-World' Networks." *Nature*, 393(6684), 440–442. — Introduced the small-world model.

6. **Burt, R. S.** (1992). *Structural Holes: The Social Structure of Competition*. Harvard University Press. — Introduced the structural holes theory and the concept of brokerage advantage.

7. **Freeman, L. C.** (1977). "A Set of Measures of Centrality Based on Betweenness." *Sociometry*, 40(1), 35–41. — Formalized betweenness centrality.

8. **McPherson, M., Smith-Lovin, L. & Cook, J. M.** (2001). "Birds of a Feather: Homophily in Social Networks." *Annual Review of Sociology*, 27, 415–444. — The comprehensive review of homophily across all dimensions.

9. **Snijders, T. A. B., van de Bunt, G. G. & Steglich, C. E. G.** (2010). "Introduction to Stochastic Actor-Oriented Models for Network Dynamics." *Social Networks*, 32(1), 44–60. — Key reference for longitudinal network analysis and the SIENA approach.

10. **Christakis, N. A. & Fowler, J. H.** (2007). "The Spread of Obesity in a Large Social Network over 32 Years." *New England Journal of Medicine*, 357(4), 370–379. — Influential (and contested) study of health behavior diffusion in social networks.

11. **Moreno, J. L.** (1934). *Who Shall Survive? A New Approach to the Problem of Human Interrelations*. Beacon House. — The founding text of sociometry.

12. **Jackson, M. O.** (2008). *Social and Economic Networks*. Princeton University Press. — Bridges social network analysis and economic network theory.

13. **Kempe, D., Kleinberg, J. & Tardos, É.** (2003). "Maximizing the Spread of Influence through a Social Network." *KDD '03*. — Formalized influence maximization as an optimization problem.

14. **Kossinets, G.** (2006). "Effects of Missing Data in Social Networks." *Social Networks*, 28(3), 247–268. — Systematic study of how missing data affects network statistics.

15. **Robins, G., Pattison, P., Kalish, Y. & Lusher, D.** (2007). "An Introduction to Exponential Random Graph (p*) Models for Social Networks." *Social Networks*, 29(2), 173–191. — Key reference for ERGM methodology.

---

*"Trust in Allah, then tie your camel." — Prophet Muhammad (ﷺ)*
