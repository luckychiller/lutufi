# Epidemiological Models

---

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Basic Compartmental Models](#2-basic-compartmental-models)
3. [Network Epidemiology](#3-network-epidemiology)
4. [SIR on Networks](#4-sir-on-networks)
5. [SIS on Networks](#5-sis-on-networks)
6. [Temporal Networks and Epidemics](#6-temporal-networks-and-epidemics)
7. [Adaptive Epidemics](#7-adaptive-epidemics)
8. [Structured Populations](#8-structured-populations)
9. [Stochastic Epidemic Models](#9-stochastic-epidemic-models)
10. [Control and Intervention](#10-control-and-intervention)
11. [Inference from Epidemic Data](#11-inference-from-epidemic-data)
12. [Epidemics as Analogies](#12-epidemics-as-analogies)
13. [How Lutufi Handles Epidemic Models](#13-how-lutufi-handles-epidemic-models)
14. [Key References](#14-key-references)

---

## 1. Introduction

Epidemiological models provide the mathematical foundation for understanding how infectious diseases spread through populations. Developed initially to study human diseases, these models have proven remarkably general, with applications to computer viruses, information diffusion, rumor spreading, financial contagion, and social influence. The formal analogies between disease spread and other propagation phenomena make epidemiological models a cornerstone of network diffusion theory.

### 1.1 Historical Development

**The Kermack-McKendrick Era (1927):** The field began with **William Ogilvy Kermack** and **Anderson Gray McKendrick's** seminal paper introducing the SIR (Susceptible-Infected-Recovered) model. Working at the Royal College of Physicians in Edinburgh, they derived the fundamental threshold condition for epidemic outbreaks and the final size equation relating initial susceptibility to total attack rate.

**The Deterministic Era (1930s–1970s):** Following Kermack-McKendrick, epidemiological modeling focused on differential equation models assuming homogeneous mixing (well-mixed populations). Key developments included:
- Endemic disease models with vital dynamics
- Age-structured models
- Multiple strain models

**The Stochastic Turn (1970s–1990s):** Recognition that chance effects matter, especially early in outbreaks when infected individuals are few. Stochastic models became essential for understanding extinction probabilities and critical community sizes.

**The Network Revolution (1990s–present):** **Roy Anderson** and **Robert May's** (1991) foundational work on infectious diseases of humans established the importance of contact structure. **Romualdo Pastor-Satorras** and **Alessandro Vespignani's** (2001) discovery that epidemic thresholds vanish in scale-free networks launched the modern field of network epidemiology.

### 1.2 Compartmental Modeling

The fundamental approach of mathematical epidemiology is **compartmental modeling:** the population is divided into discrete categories (compartments) based on disease status, and flows between compartments are described mathematically.

Common compartments include:
- **S (Susceptible):** Can be infected
- **E (Exposed):** Infected but not yet infectious
- **I (Infectious):** Can transmit the disease
- **R (Recovered/Removed):** Immune, isolated, or deceased
- **C (Carrier):** Asymptomatic but infectious

### 1.3 The Basic Reproduction Number

The **basic reproduction number** $R_0$ (pronounced "R-naught") is the most important quantity in epidemiology:

$$R_0 = \text{Expected number of secondary infections from one infected individual in a fully susceptible population}$$

**Interpretation:**
- If $R_0 > 1$: Each infected person infects more than one other; epidemic grows exponentially initially
- If $R_0 < 1$: Each infected person infects less than one other; outbreak dies out
- If $R_0 = 1$: Disease is at critical threshold

$R_0$ combines biological factors (infectiousness, duration) with social factors (contact rates). For many diseases, $R_0$ has been estimated:
- Measles: 12–18
- Influenza: 2–3
- COVID-19: 2–5 (wild-type)
- Ebola: 1.5–2.5

---

## 2. Basic Compartmental Models

### 2.1 The SI Model

The **SI (Susceptible-Infected)** model is the simplest epidemic model. Once infected, individuals remain infected indefinitely (no recovery).

**Assumptions:**
- Population divided into susceptibles $S(t)$ and infected $I(t)$
- Homogeneous mixing: every individual contacts every other at equal rate
- Infection occurs upon contact between susceptible and infected

**Dynamics:**

$$\frac{dS}{dt} = -\beta S I$$
$$\frac{dI}{dt} = \beta S I$$

where $\beta$ is the transmission rate (product of contact rate and infection probability per contact).

**Key properties:**
- Total population $N = S + I$ is constant
- Eventually everyone becomes infected ($S \rightarrow 0$, $I \rightarrow N$)
- No epidemic threshold; any introduction leads to eventual full infection

**Applications:** Diseases without recovery (some chronic infections), irreversible adoption processes, permanent attitude change.

### 2.2 The SIS Model

The **SIS (Susceptible-Infected-Susceptible)** model adds recovery without immunity—individuals return to the susceptible state after infection.

**Dynamics:**

$$\frac{dS}{dt} = -\beta S I + \gamma I$$
$$\frac{dI}{dt} = \beta S I - \gamma I$$

where $\gamma$ is the recovery rate (average infectious period is $1/\gamma$).

**Basic reproduction number:**

$$R_0 = \frac{\beta N}{\gamma}$$

**Behavior:**
- If $R_0 < 1$: Infection dies out ($I \rightarrow 0$)
- If $R_0 > 1$: Endemic equilibrium reached:

$$I^* = N\left(1 - \frac{1}{R_0}\right) = N\left(1 - \frac{\gamma}{\beta N}\right)$$

**Key insight:** The SIS model exhibits a **transcritical bifurcation** at $R_0 = 1$. Below threshold, only disease-free equilibrium exists. Above threshold, a stable endemic equilibrium emerges.

**Applications:** Common cold, bacterial infections, computer viruses, rumor spreading (where hearing a rumor doesn't prevent hearing it again).

### 2.3 The SIR Model

The **SIR (Susceptible-Infected-Recovered)** model assumes recovery confers permanent immunity.

**Dynamics:**

$$\frac{dS}{dt} = -\beta S I$$
$$\frac{dI}{dt} = \beta S I - \gamma I$$
$$\frac{dR}{dt} = \gamma I$$

**Basic reproduction number:**

$$R_0 = \frac{\beta N}{\gamma}$$

**Epidemic curve:** The number of infectious individuals $I(t)$ typically follows a characteristic trajectory:
1. Initial exponential growth (when $S \approx N$)
2. Peak when $S$ drops to $N/R_0$
3. Decline as susceptibles are depleted

**Final size equation:** The fraction eventually infected $z = R(\infty)/N$ satisfies:

$$z = 1 - e^{-R_0 z}$$

This transcendental equation relates $R_0$ to final epidemic size. For $R_0 = 2$, about 80% are eventually infected; for $R_0 = 3$, about 94%.

**Herd immunity threshold:** The critical vaccination coverage to prevent epidemics:

$$p_c = 1 - \frac{1}{R_0}$$

For measles ($R_0 \approx 15$), $p_c \approx 93\%$ vaccination coverage needed.

### 2.4 The SEIR Model

The **SEIR (Susceptible-Exposed-Infectious-Recovered)** model adds an **exposed/latent period** between infection and infectiousness.

**Dynamics:**

$$\frac{dS}{dt} = -\beta S I$$
$$\frac{dE}{dt} = \beta S I - \sigma E$$
$$\frac{dI}{dt} = \sigma E - \gamma I$$
$$\frac{dR}{dt} = \gamma I$$

where $\sigma$ is the rate of progression from exposed to infectious (average latent period $1/\sigma$).

**Key effects of latent period:**
- Delayed epidemic peak
- Longer overall epidemic duration
- No change to $R_0$ or final size (latent period doesn't affect total infections)

**Applications:** Diseases with significant incubation periods (measles, chickenpox, COVID-19).

### 2.5 Extensions

**SIRS Model:** Recovered individuals lose immunity and return to susceptible:

$$\frac{dR}{dt} = \gamma I - \omega R$$

where $\omega$ is the rate of immunity loss. Alternates between epidemic and endemic phases.

**Carrier States:** Some recovered individuals remain infectious (carriers):

$$\frac{dC}{dt} = \rho \gamma I$$

where $\rho$ is the fraction becoming carriers.

**Vital Dynamics:** Births and deaths:

$$\frac{dS}{dt} = \mu N - \beta S I - \mu S$$

where $\mu$ is the birth/death rate. Allows disease persistence even when $R_0$ varies.

---

## 3. Network Epidemiology

The homogeneous mixing assumption of classical models is often unrealistic. **Network epidemiology** explicitly models contact structure, representing individuals as nodes and potential transmission paths as edges.

### 3.1 Mean-Field Approximations

**Heterogeneous Mean-Field (HMF):** Tracks $[I_k]$, the fraction of infected individuals among degree-$k$ nodes:

$$\frac{d[I_k]}{dt} = -\gamma [I_k] + \beta k [S_k] \sum_{k'} \frac{k' P(k')}{\langle k \rangle} [I_{k'}]$$

where $P(k)$ is the degree distribution and the sum represents the probability that a neighbor is infected.

**Key result:** For configuration model networks, the epidemic threshold is:

$$\lambda_c = \frac{\langle k \rangle}{\langle k^2 \rangle}$$

where $\lambda = \beta/\gamma$ is the effective transmission rate.

**Implication:** Networks with high degree variance (heavy tails) have lower epidemic thresholds. Scale-free networks with $\langle k^2 \rangle \rightarrow \infty$ have **no epidemic threshold**—any positive transmission rate leads to epidemics (Pastor-Satorras & Vespignani, 2001).

### 3.2 Quenched Mean-Field

**Quenched Mean-Field (QMF)** uses the actual adjacency matrix $A_{ij}$ rather than degree statistics:

$$\frac{dI_i}{dt} = -\gamma I_i + \beta (1 - I_i) \sum_j A_{ij} I_j$$

**Advantage:** Captures specific network structure (not just degree distribution).

**Disadvantage:** Computationally expensive for large networks; ignores dynamical correlations.

### 3.3 Pair Approximations

Mean-field approximations ignore correlations between neighbors. **Pair approximations** track pairs of connected nodes:

$$\frac{d[SI]}{dt} = -\gamma [SI] + \beta ([SSI] - [ISI]) - \beta [SI]$$

where $[XY]$ is the density of $X$-$Y$ edges and $[XYZ]$ is connected triples.

**Closure:** Approximate triples in terms of pairs (e.g., $[SSI] \approx \frac{[SS][SI]}{[S]}$).

**Advantage:** Captures local clustering effects.

### 3.4 Percolation Theory Connection

Epidemic outbreaks are intimately connected to **percolation** in statistical physics:

**Bond percolation:** Each edge is retained with probability $p$ (independently). The SIR model with uniform transmission probability $T$ is equivalent to bond percolation with $p = T$.

**Giant component:** The epidemic outbreak corresponds to the giant connected component in the percolated network. If a giant component exists, epidemics can occur.

**Critical threshold:** The percolation threshold equals the epidemic threshold.

### 3.5 Message Passing Approaches

**Message passing** (or belief propagation) provides accurate approximations for SIR dynamics on tree-like networks:

Each node $i$ passes messages to neighbors $j$ representing the probability that $i$ would infect $j$ if $j$ were susceptible. These satisfy recursive equations that can be solved iteratively.

**Advantage:** Highly accurate for locally tree-like networks.

**Limitation:** Assumes no loops (fails for highly clustered networks).

---

## 4. SIR on Networks

### 4.1 Bond Percolation Mapping

The SIR model on networks maps exactly to **bond percolation:**

1. For each edge $(i, j)$, decide independently with probability $T = \frac{\beta}{\beta + \gamma}$ whether transmission would occur if $i$ became infected while $j$ was susceptible.
2. Keep only the "transmission" edges—this forms the **percolated network**.
3. The nodes infected in an outbreak starting from $i$ are exactly those reachable from $i$ in the percolated network.

**Transmission probability:**

$$T = 1 - e^{-\beta / \gamma} \approx \frac{\beta}{\gamma} \quad \text{(for small } \beta/\gamma)$$

### 4.2 Epidemic Threshold on Networks

For the SIR model on a configuration model network with degree distribution $P(k)$, the epidemic threshold is:

$$T_c = \frac{\langle k \rangle}{\langle k^2 \rangle - \langle k \rangle}$$

**Scale-free networks:** If $P(k) \sim k^{-\gamma}$ with $\gamma \leq 3$, then $\langle k^2 \rangle$ diverges, and $T_c \rightarrow 0$. Any positive transmission probability leads to epidemics.

**Implication:** Targeted interventions on high-degree nodes are crucial for controlling epidemics in scale-free networks.

### 4.3 Giant Component of Infected

The **final size** of an SIR epidemic (fraction eventually infected) equals the fraction of nodes in the giant component of the percolated network.

For configuration model networks, this can be computed using generating functions. Let $G_0(z) = \sum_k P(k) z^k$ be the degree distribution generating function.

The fraction eventually infected starting from a random node is:

$$z = 1 - G_0(u)$$

where $u$ satisfies:

$$u = 1 - T + T G_1(u)$$

and $G_1(z) = G_0'(z)/G_0'(1)$ is the excess degree distribution.

### 4.4 Final Outbreak Size Calculation

**Numerical algorithm:**

```
1. Compute T = β / (β + γ)
2. Solve u = 1 - T + T·G₁(u) for u ∈ [0,1]
3. Final size z = 1 - G₀(u)
```

**Example:** For Erdős-Rényi networks with mean degree $\langle k \rangle = c$:
- $G_0(z) = G_1(z) = e^{c(z-1)}$
- Threshold at $T_c = 1/c$
- Above threshold, final size satisfies: $z = 1 - e^{-c T z}$

---

## 5. SIS on Networks

### 5.1 Quenched Mean-Field for SIS

The SIS model on networks is more complex than SIR because reinfection is possible. The QMF approximation gives:

$$\frac{d\rho_i}{dt} = -\gamma \rho_i + \beta (1 - \rho_i) \sum_j A_{ij} \rho_j$$

where $\rho_i$ is the probability that node $i$ is infected.

**Epidemic threshold:** The threshold occurs when the largest eigenvalue of the adjacency matrix exceeds $\gamma/\beta$:

$$\lambda_c = \frac{1}{\lambda_1(A)}$$

where $\lambda_1(A)$ is the largest eigenvalue of $A$.

### 5.2 Endemic Threshold

For $R_0 > 1$ (above threshold), the SIS model reaches a **quasi-stationary state** with non-zero infection prevalence.

**Prevalence:** The fraction of infected nodes at endemic equilibrium:

$$\rho^* \approx \frac{\beta \lambda_1(A) - \gamma}{\beta \lambda_1(A)} = 1 - \frac{1}{R_0^{\text{eff}}}$$

where $R_0^{\text{eff}} = \beta \lambda_1(A) / \gamma$ is the effective reproduction number on the network.

### 5.3 Metastability

The SIS model on finite networks has a unique absorbing state (all susceptible). However, for large networks above threshold, the system exhibits **metastability:**
- Long periods near endemic equilibrium
- Rare fluctuations to extinction
- Extinction time grows exponentially with network size

**Implication:** The endemic state is effectively stable for large populations, even though ultimate extinction is guaranteed.

---

## 6. Temporal Networks and Epidemics

Real contact networks change over time. **Temporal networks** capture this dynamics, with important implications for disease spread.

### 6.1 Importance of Contact Sequence

In temporal networks, the **order** of contacts matters:
- A contact from an infected to a susceptible can only cause transmission if the infected is already infected
- Static network analysis overestimates reachability by ignoring time ordering

**Example:** If $A$ infects $B$ at time 2, and $B$ contacts $C$ at time 1, transmission to $C$ is impossible (reverse order in time).

### 6.2 Causality Constraints

Epidemic spread must follow **causal paths:** sequences of contacts where each contact occurs after the previous one.

**Time-respecting path:** A path $v_1 \rightarrow v_2 \rightarrow ... \rightarrow v_k$ where the time of contact $(v_i, v_{i+1})$ is less than the time of contact $(v_{i+1}, v_{i+2})$.

**Reachability:** The set of nodes reachable from an index case is those connected by time-respecting paths.

### 6.3 Reachability in Temporal Networks

The **outbreak size** in temporal networks is bounded by reachability:

$$\text{Expected outbreak size} \leq \text{Expected number of nodes reachable via time-respecting paths}$$

**Calculation:** Requires enumeration of all time-respecting paths, computationally expensive for large networks.

### 6.4 Time-Respecting Paths

**Algorithm for counting time-respecting paths:**

```
For each time step t in chronological order:
  For each active edge (u, v) at time t:
    Add new paths: paths_to(v) += paths_to(u)
```

**Bursty dynamics:** Real contacts are often bursty (clustered in time). Burstiness slows epidemic spread compared to Poisson contact processes because contacts within bursts are redundant.

---

## 7. Adaptive Epidemics

Human behavior responds to disease risk. **Adaptive epidemic models** incorporate behavioral feedback, where contact patterns change based on disease prevalence.

### 7.1 Behavioral Response to Epidemic

**Risk perception:** Individuals reduce contacts when disease prevalence is high:

$$c(t) = c_0 \cdot f(I(t))$$

where $c(t)$ is contact rate and $f$ is a decreasing function of prevalence.

**Effects:**
- Reduced peak prevalence
- Delayed epidemic peak
- Potential for sustained oscillations (prevalence → behavioral response → reduced prevalence → reduced response → increased prevalence...)

### 7.2 Co-evolution of Disease and Awareness

Disease spread co-evolves with awareness/information spread:

- **Ubiquitous awareness:** Information spreads faster than disease; everyone knows about outbreak quickly
- **Local awareness:** Information spreads through same network as disease; some individuals unaware

**Funk et al. (2009) model:**
- Disease dynamics: SIR-type
- Awareness dynamics: Information spread (potentially through different network)
- Coupling: Aware individuals reduce transmission probability

### 7.3 Adaptive Rewiring

**Social distancing:** Individuals drop connections to infected neighbors and form new connections to non-infected individuals.

**Gross et al. (2006) model:**
- With rate $\omega$, susceptibles rewire away from infected neighbors
- Rewiring changes network structure, which affects epidemic dynamics

**Effects:**
- Reduced epidemic size
- Increased network clustering
- Potential for multiple epidemic waves as network restructures

---

## 8. Structured Populations

### 8.1 Age-Structured Models

Different age groups have different contact patterns and different disease severity.

**Who-Acquired-Infection-From-Whom (WAIFW) matrix:** $W_{ij}$ is the rate at which individuals in group $j$ infect individuals in group $i$.

**Next Generation Matrix:** $K_{ij} = W_{ij} \cdot D_j$ where $D_j$ is infectious duration in group $j$.

$R_0$ is the largest eigenvalue of $K$.

**Implications:** Vaccination strategies should target groups with highest contributions to transmission (not necessarily those with highest risk).

### 8.2 Spatial Models

Disease spread in physical space can be modeled using:

**Reaction-diffusion equations:**

$$\frac{\partial I}{\partial t} = D \nabla^2 I + \beta S I - \gamma I$$

where $D$ is diffusion coefficient (random movement).

**Metapopulation models:** Population divided into patches (cities) with migration between them.

**Spatial networks:** Connections depend on geographic distance (e.g., power-law decay with distance).

### 8.3 Metapopulation Models

**Structure:** Population divided into subpopulations (patches) with internal dynamics and coupling through migration.

**Colizza et al. (2006) model:**
- Air travel network connects subpopulations
- Within each city: SIR dynamics
- Between cities: infection transfer proportional to travel rates

**Global invasion threshold:** Even if $R_0 > 1$ locally, global spread requires sufficient coupling between patches.

---

## 9. Stochastic Epidemic Models

Real epidemics involve chance. **Stochastic models** capture this randomness, essential for understanding early outbreak dynamics and extinction.

### 9.1 Continuous-Time Markov Chains

The stochastic SIR model is a **continuous-time Markov chain** with transition rates:

- Infection: $(S, I, R) \rightarrow (S-1, I+1, R)$ at rate $\beta S I / N$
- Recovery: $(S, I, R) \rightarrow (S, I-1, R+1)$ at rate $\gamma I$

**Master equation:**

$$\frac{dP(S, I, R)}{dt} = \beta \frac{(S+1)(I-1)}{N} P(S+1, I-1, R) + \gamma (I+1) P(S, I+1, R-1) - \left(\beta \frac{SI}{N} + \gamma I\right) P(S, I, R)$$

### 9.2 Gillespie Algorithm

The **Gillespie algorithm** simulates continuous-time stochastic processes exactly:

```
1. Initialize state (S, I, R)
2. While I > 0:
   a. Calculate total rate: r_total = β·S·I/N + γ·I
   b. Sample time to next event: Δt ~ Exponential(r_total)
   c. Choose event type: infection with prob (β·S·I/N)/r_total, else recovery
   d. Update state accordingly
   e. Update time: t ← t + Δt
```

**Advantage:** Exact simulation of stochastic process (no time discretization error).

### 9.3 Moment Closure Approximations

Stochastic models are analytically intractable due to moment hierarchy: equations for means depend on second moments, which depend on third moments, etc.

**Moment closure:** Approximate higher moments in terms of lower moments. Common approximations:
- **Mean-field:** Ignore all fluctuations
- **Pair approximation:** Include means and variances/covariances
- **Gaussian closure:** Assume distribution is approximately normal

### 9.4 Branching Process Approximation

Early in an outbreak, when few are infected, the epidemic resembles a **branching process:**
- Each infected individual produces a random number of secondary infections
- Secondary infections are independent (susceptible depletion negligible)

**Extinction probability:** If $R_0 < 1$, extinction is certain. If $R_0 > 1$, extinction probability $q$ satisfies:

$$q = G(q)$$

where $G$ is the probability generating function of offspring distribution.

**Critical community size:** The population size below which stochastic extinction is likely even for $R_0 > 1$.

---

## 10. Control and Intervention

Mathematical epidemiology informs public health interventions by identifying optimal strategies for reducing transmission.

### 10.1 Vaccination Strategies

**Random vaccination:** Randomly select fraction $p$ of population to vaccinate. Effective reproduction number:

$$R_{\text{eff}} = (1 - p) R_0$$

**Critical vaccination coverage:**

$$p_c = 1 - \frac{1}{R_0}$$

**Targeted vaccination:** Vaccinate high-degree nodes (hubs). For scale-free networks:
- Random vaccination requires vaccinating nearly everyone
- Targeted vaccination can be effective with small coverage

**Acquaintance vaccination:** Vaccinate random neighbors of random individuals (targets high-degree nodes without needing to know the network).

### 10.2 Contact Tracing

**Mechanism:** Identify contacts of infected individuals and quarantine/test them.

**Effectiveness:** Depends on:
- Fraction of contacts traced
- Speed of tracing relative to disease generation time
- Compliance with quarantine

**Network effects:** Contact tracing is most effective when network has high clustering (many contacts of contacts know each other, making tracing efficient).

### 10.3 Quarantine and Isolation

**Isolation:** Separate infected individuals from susceptible population.

**Quarantine:** Restrict movement of potentially exposed individuals.

**Modeling:** Reduce effective contact rates:

$$\beta_{\text{eff}} = \beta \cdot (1 - f_{\text{isolated}}) \cdot (1 - f_{\text{quarantined}})$$

**Timing:** Early intervention (when cases are few) is exponentially more effective than late intervention.

### 10.4 Social Distancing Measures

**Generalized interventions:** Reduce overall contact rates (school closures, gathering limits, lockdowns).

**Model:** Multiply transmission rate by factor $(1 - \epsilon)$ where $\epsilon$ is intervention effectiveness.

**Trade-offs:** Economic costs vs. health benefits; intervention fatigue over time.

### 10.5 Optimal Resource Allocation

Given limited resources (vaccines, tests, quarantine capacity), how should they be allocated?

**Optimization problem:**

$$\min_{\text{allocation}} \text{Expected infections}$$

subject to resource constraints.

**Network-based solutions:**
- Target high-degree nodes
- Target nodes bridging communities
- Target nodes with high centrality in contact network

---

## 11. Inference from Epidemic Data

Estimating epidemic parameters from observed data is crucial for real-time response.

### 11.1 Estimating $R_0$ from Early Growth

During early exponential growth, $I(t) \approx I_0 e^{rt}$ where $r$ is the **growth rate**.

**Relationship to $R_0$:**

For SIR: $R_0 = 1 + \frac{r}{\gamma}$

For SEIR: $R_0 \approx \left(1 + \frac{r}{\sigma}\right)\left(1 + \frac{r}{\gamma}\right)$

**Method:** Fit exponential to early case data; estimate $r$; combine with estimate of generation time to infer $R_0$.

### 11.2 Back-Calculation Methods

Given observed incidence data, infer the time series of infections:

$$E[\text{cases at } t] = \sum_{s < t} E[\text{infections at } s] \cdot p(\text{report delay } = t - s)$$

**Deconvolution:** Solve for infection curve given case curve and delay distribution.

**Applications:** Estimate true infection numbers accounting for reporting delays; reconstruct epidemic trajectory.

### 11.3 Phylogenetic Inference

Pathogen genetic sequences contain information about transmission history.

**Molecular clock:** Mutations accumulate at roughly constant rate. Differences between sequences indicate time since common ancestor.

**Phylogenetic tree:** Represents evolutionary relationships. Branching times inform about transmission events.

**Phylodynamic methods:** Jointly model epidemiological dynamics and viral evolution to infer:
- Effective reproduction number over time
- Migration rates between locations
- Super-spreading events

### 11.4 Digital Epidemiology

Digital data sources enable real-time epidemic monitoring:

**Sources:**
- Internet search trends (Google Flu Trends)
- Social media (Twitter symptom reports)
- Mobile phone data (mobility patterns)
- Wearable devices (heart rate anomalies)

**Methods:**
- Correlation with traditional surveillance
- Machine learning prediction
- Nowcasting (estimating current state given reporting delays)

**Challenges:**
- Data representativeness
- Algorithm changes affecting signals
- Privacy concerns

---

## 12. Epidemics as Analogies

Epidemic models have proven powerful analogies for non-biological propagation phenomena.

### 12.1 Information Diffusion

Information spreads through social networks similarly to diseases:
- **Simple contagion:** Single exposure sufficient (like SI/SIR)
- **Complex contagion:** Multiple exposures needed (modifications to standard models)
- **Immunity:** Information "saturates"—once known, can't be "unlearned"

**Key difference:** Information can be shared with multiple simultaneously (broadcast), unlike diseases which require individual contact events.

### 12.2 Rumor Spreading

The **DK (Daley-Kendall) model** and **MK (Maki-Thompson) model** adapt epidemic models to rumors:

**States:**
- **Ignorant:** Haven't heard rumor
- **Spreader:** Know and actively spread rumor
- **Stifler:** Know rumor but don't spread it

**Dynamics:** Spreaders become stiflers upon contact with other spreaders or stiflers (analogous to recovery).

**Key insight:** Rumors have a natural "death" mechanism—when spreaders meet, they may stop spreading—leading to different dynamics than diseases.

### 12.3 Innovation Adoption

**Bass diffusion model:**

$$\frac{f(t)}{1 - F(t)} = p + q F(t)$$

where $f(t)$ is adoption rate, $F(t)$ is cumulative adoption, $p$ is innovation coefficient (external influence), and $q$ is imitation coefficient (internal influence).

**Connection to epidemics:**
- External influence ($p$): Like spontaneous infection from source outside population
- Internal influence ($q$): Like transmission through contact (social contagion)

### 12.4 Financial Contagion

Financial crises spread between institutions through:
- **Direct exposure:** One institution's default causes losses for creditors (network transmission)
- **Information contagion:** News about one institution affects confidence in similar institutions
- **Fire sales:** Asset sales by distressed institutions depress prices for others

**Eisenberg-Noe model:** Network of interbank liabilities; clearing payments given initial shocks.

**Acemoglu et al. (2015):** Phase transitions in financial contagion—small shocks absorbed, large shocks cascade.

---

## 13. How Lutufi Handles Epidemic Models

Lutufi integrates epidemic modeling with probabilistic network analysis, enabling uncertainty-aware disease dynamics and intervention optimization.

### 13.1 Probabilistic Compartmental Models on Networks

Lutufi represents epidemic states as **probability distributions** over compartment memberships:

```
For each node i at time t:
  - P(S_i^{(t)} = 1): Probability susceptible
  - P(I_i^{(t)} = 1): Probability infected
  - P(R_i^{(t)} = 1): Probability recovered
```

**Dynamics:** Probabilities evolve according to Bayesian update rules incorporating:
- Transmission probabilities along edges
- Recovery rates
- Observations (test results, symptom reports)

### 13.2 Inference of Transmission Rates from Data

Given partial observations (some individuals tested, many unknown), Lutufi infers:

**Posterior over transmission rates:**

$$P(\beta, \gamma | \text{observations}) \propto P(\text{observations} | \beta, \gamma) \cdot P(\beta, \gamma)$$

**Methods:**
- Markov Chain Monte Carlo (MCMC) sampling
- Variational inference for large networks
- Sequential Monte Carlo for real-time updating

**Applications:** Real-time $R_0$ estimation during outbreaks; detecting superspreading events.

### 13.3 Intervention Optimization with Uncertainty

Lutufi optimizes interventions accounting for uncertainty:

**Expected utility of intervention strategy $S$:**

$$EU(S) = \mathbb{E}_{P(\theta)}[\text{Outcome}(S, \theta)] - \text{Cost}(S)$$

where $\theta$ represents uncertain parameters (transmission rates, network structure).

**Optimization:**
- Vaccination targeting under uncertainty about contact patterns
- Quarantine strategies robust to unknown transmission rates
- Adaptive strategies that update as epidemic unfolds

**Robust optimization:** Minimize worst-case outcomes:

$$\min_S \max_{\theta \in \Theta} \text{Outcome}(S, \theta)$$

where $\Theta$ is uncertainty set for parameters.

### 13.4 Integration with Social Influence Models

Lutufi uniquely connects epidemiological models with social influence:

- **Behavior-disease coupling:** Models where disease prevalence affects contact rates through influence processes
- **Intervention compliance:** Models social influence on whether individuals follow public health guidance
- **Information spread:** Couples disease dynamics with spread of awareness/information about disease

---

## 14. Key References

1. **Kermack, W. O. & McKendrick, A. G. (1927).** "A Contribution to the Mathematical Theory of Epidemics." *Proceedings of the Royal Society A*, 115(772), 700–721. — Foundational paper on compartmental models.

2. **Anderson, R. M. & May, R. M. (1991).** *Infectious Diseases of Humans: Dynamics and Control*. Oxford University Press. — The definitive reference on mathematical epidemiology.

3. **Pastor-Satorras, R. & Vespignani, A. (2001).** "Epidemic Spreading in Scale-Free Networks." *Physical Review Letters*, 86(14), 3200–3203. — Discovered vanishing epidemic threshold in scale-free networks.

4. **Kiss, I. Z., Miller, J. C. & Simon, P. L. (2017).** *Mathematics of Epidemics on Networks: From Exact to Approximate Models*. Springer. — Comprehensive modern treatment of network epidemiology.

5. **Newman, M. E. J. (2002).** "Spread of Epidemic Disease on Networks." *Physical Review E*, 66(1), 016128. — Percolation theory approach to epidemic modeling.

6. **Colizza, V., Barrat, A., Barthélemy, M. & Vespignani, A. (2006).** "The Role of the Airline Transportation Network in the Prediction and Predictability of Global Epidemics." *PNAS*, 103(7), 2015–2020. — Metapopulation epidemic modeling.

7. **Keeling, M. J. & Eames, K. T. (2005).** "Networks and Epidemic Models." *Journal of the Royal Society Interface*, 2(4), 295–307. — Review of network epidemic models.

8. **Funk, S., Salathé, M. & Jansen, V. A. (2010).** "Modelling the Influence of Human Behaviour on the Spread of Infectious Diseases: A Review." *Journal of the Royal Society Interface*, 7(50), 1247–1256. — Review of adaptive epidemic models.

9. **Gross, T., D'Lima, C. J. & Blasius, B. (2006).** "Epidemic Dynamics on an Adaptive Network." *Physical Review Letters*, 96(20), 208701. — Adaptive rewiring in epidemic models.

10. **Bansal, S., Read, J., Pourbohloul, B. & Meyers, L. A. (2010).** "The Dynamic Nature of Contact Networks in Infectious Disease Epidemiology." *Journal of Biological Dynamics*, 4(5), 478–489. — Temporal networks in epidemiology.

11. **Heesterbeek, J. A. P. et al. (2015).** "Modeling Infectious Disease Dynamics in the Complex Landscape of Global Health." *Science*, 347(6227), aaa4339. — Modern perspective on epidemic modeling challenges.

12. **Brockmann, D. & Helbing, D. (2013).** "The Hidden Geometry of Complex, Network-Driven Contagion Phenomena." *Science*, 342(6164), 1337–1342. — Effective distance approach to epidemic spread.

13. **Acemoglu, D., Ozdaglar, A. & Tahbaz-Salehi, A. (2015).** "Systemic Risk and Stability in Financial Networks." *American Economic Review*, 105(2), 564–608. — Financial contagion as epidemic process.

14. **Daley, D. J. & Kendall, D. G. (1964).** "Epidemics and Rumours." *Nature*, 204(4963), 1118. — Original rumor spreading model.

15. **Maki, D. P. & Thompson, M. (1973).** *Mathematical Models and Applications: With Emphasis on the Social, Life, and Management Sciences*. Prentice-Hall. — Alternative rumor model.

16. **Bass, F. M. (1969).** "A New Product Growth for Model Consumer Durables." *Management Science*, 15(5), 215–227. — Bass diffusion model for innovation adoption.

17. **Eubank, S. et al. (2004).** "Modelling Disease Outbreaks in Realistic Social Networks." *Nature*, 429(6988), 180–184. — Large-scale epidemic simulation on realistic networks.

18. **Lloyd, A. L. & May, R. M. (2001).** "Epidemiology: How Viruses Spread Among Computers and People." *Science*, 292(5520), 1316–1317. — Connection between computer and biological viruses.

---

*"The germ is nothing; the terrain is everything." — Claude Bernard (epidemiology teaches us that pathogens alone don't determine outbreaks—the network structure of hosts matters equally.)*
