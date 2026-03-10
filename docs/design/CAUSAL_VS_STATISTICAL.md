# Causal vs Statistical Graphs

In Lutufi, we maintain a clear distinction between **Statistical Graphs** (undirected or directed) and **Causal Graphs** (Directed Acyclic Graphs with specific causal semantics).

## Statistical Graphs
A statistical graph encodes conditional independence (CI) relations.
- **Undirected Graphs (MRFs)**: Represent symmetric relations. CI is determined by simple graph separation.
- **Directed Graphs (BNs)**: Represent factorizations of joint probability distributions. CI is determined by d-separation.

## Causal Graphs
A causal graph encodes the mechanism of data generation. In addition to CI relations, it defines the effect of **interventions** (using the `do()` operator).

## API Enforcement

### Type System
Lutufi uses distinct classes for these models:
- `BayesianNetwork`: For statistical modeling.
- `CausalModel`: Inherits from `BayesianNetwork` but adds intervention capabilities and requires a DAG.

### Validation
The API enforces structural constraints:
- `CausalModel` creation will fail if the graph contains cycles.
- Interventions are only permitted on `CausalModel` instances.

### Explicit Semantic Layer
Nodes in a `CausalModel` can be tagged with exogenous variables (errors/shocks), enforcing the structural equation model (SEM) perspective.

```python
# Example of intervention
model = CausalModel(structure="A -> B")
result = model.do("A", value=1).query("B")
```
This query computes $P(B | do(A=1))$, which is distinct from $P(B | A=1)$ if there are unobserved confounders (though in this simple case they might match if no confounders exist).
