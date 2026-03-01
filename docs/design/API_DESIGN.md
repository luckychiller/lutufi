# API Design Principles: Lutufi

## User-Centricity
Lutufi's API is designed around the **questions researchers actually ask**, moving away from abstract graph operations toward domain-relevant queries.

## Core Conventions
- **Explicit over Implicit:** Functions should clearly state their intent (e.g., `infer_probability()` rather than `calculate()`).
- **Chainable Operations:** Building a model should feel like a fluent, logical progression.
- **Interpretation-First:** Results should be returned with semantic context, not just raw numerical arrays.

## High-Level API Examples (Conceptual)

### 1. Model Creation
```python
import lutufi as lf

# Initialize a network from an existing structural graph
model = lf.Model(graph=my_networkx_graph)

# Define conditional dependencies
model.add_dependency(child="Node_A", parents=["Node_B", "Node_C"])
```

### 2. Probabilistic Querying
```python
# Instead of low-level graph traversal, ask research-oriented questions
result = model.query(
    target="Node_X", 
    evidence={"Node_Y": 1, "Node_Z": 0},
    method="belief_propagation"
)
```

### 3. Influence Analysis
```python
# Identify critical nodes for information spread
hubs = model.analyze_influence(metric="propagation_probability")
```

## Informative Error Handling
Lutufi errors are designed to be pedagogical. Instead of a generic `ValueError`, the library should provide actionable feedback:
- *"This network contains cycles which violate the assumption of your chosen inference method. Consider switching to `method='loopy_bp'`."*
- *"Node_A has inconsistent probability tables (do not sum to 1.0). Use `normalize=True` to resolve."*

## Serialization & Reproducibility
The API includes first-class support for saving and loading full models:
```python
# Save the entire state: structure, probabilities, and metadata
model.save("research_model_v1.lutufi")

# Load exactly for reproduction
reproduced_model = lf.load_model("research_model_v1.lutufi")
```

## Metadata & Attribution
Every model object in Lutufi carries a metadata dictionary for tracking provenance, authorship, and ethical considerations, ensuring that "how" and "why" are saved alongside the "what."
