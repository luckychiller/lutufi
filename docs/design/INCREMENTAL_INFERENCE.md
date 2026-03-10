# Incremental Inference Architecture

For dynamic networks or cases with streaming evidence, recomputing inference from scratch is prohibitively expensive. Lutufi implements an **Incremental Inference Architecture** to handle these scenarios.

## Core Principles

### Junction Tree Caching
The primary inference mechanism for exact results is based on a **Junction Tree**.
1. **Compilation**: The graph is moralized and triangulated once.
2. **Persistence**: The Junction Tree structure is cached.
3. **Local Updates**: When evidence changes on a node $X$, only the cliques containing $X$ and their neighbors in the Junction Tree need to be updated.

### Message Passing Optimization
In belief propagation:
- **Upward Pass**: Propagate evidence from leaves to a root.
- **Downward Pass**: Propagate marginals from the root to the leaves.

For incremental updates:
- We track which messages are "dirty" (need recalculation).
- If evidence is updated in clique $C_i$, we only recompute messages on paths starting from $C_i$.

## API Support for Dynamic Evidence

```python
engine = IncrementalInference(model)
# Initial inference
marginals = engine.query(["A", "B"])

# Update evidence for A only
engine.update_evidence({"A": 1})
# This call is O(TreeWidth) rather than O(GraphSize)
new_marginals = engine.query(["B"])
```

## Streaming/Dynamic Bayesian Networks (DBNs)
For DBNs, we maintain a "sliding window" of the Junction Tree.
- Old time slices are marginalized out and removed.
- New time slices are expanded and linked to the previous state.
- Transition matrices are pre-computed and cached.

This allows Lutufi to support real-time monitoring applications (like the Alarm network) with constant-time updates relative to the window size.
