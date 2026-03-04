# Lutufi Data Flow Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Overview](#overview)
3. [Input Stage](#input-stage)
4. [Model Construction Stage](#model-construction-stage)
5. [Preprocessing Stage](#preprocessing-stage)
6. [Inference Stage](#inference-stage)
7. [Learning Stage](#learning-stage)
8. [Query Stage](#query-stage)
9. [Output Stage](#output-stage)
10. [Streaming Data Flow](#streaming-data-flow)
11. [Data Transformation Pipeline](#data-transformation-pipeline)
12. [Caching and Memoization](#caching-and-memoization)
13. [Lazy Evaluation](#lazy-evaluation)
14. [Error Handling in Data Flow](#error-handling-in-data-flow)
15. [Performance Monitoring](#performance-monitoring)
16. [Data Flow Diagrams](#data-flow-diagrams)
17. [Security Considerations](#security-considerations)
18. [Key References](#key-references)

---

## Executive Summary

This document describes how data flows through the Lutufi library—from initial input through various processing stages to final output. Understanding these data flows is essential for optimizing performance, debugging issues, and extending the library.

The data flow is organized around a pipeline architecture where data progresses through distinct stages: input parsing, model construction, preprocessing, computation (inference or learning), and output formatting. Each stage has defined inputs, outputs, and error handling strategies.

---

## Overview

### What Data Flow Means in Lutufi

Data flow in Lutufi encompasses:

1. **Model Data Flow**: How model structures and parameters are loaded, transformed, and stored
2. **Evidence Data Flow**: How observations propagate through inference algorithms
3. **Message Data Flow**: How beliefs are passed between variables and factors during inference
4. **Learning Data Flow**: How training data drives parameter and structure updates
5. **Query Data Flow**: How user queries are processed into results

### Stages of Data Processing

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          LUTUFI DATA FLOW PIPELINE                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐ │
│  │    INPUT     │──►│ CONSTRUCTION │──►│PREPROCESSING │──►│  COMPUTATION │ │
│  │              │   │              │   │              │   │              │ │
│  │ • File Parse │   │ • Validation │   │ • Moralize   │   │ • Inference  │ │
│  │ • Format Conv│   │ • Canonical  │   │ • Triangulate│   │ • Learning   │ │
│  │ • Validation │   │ • Transform  │   │ • Algorithm  │   │ • Query Exec │ │
│  │              │   │              │   │   Selection  │   │              │ │
│  └──────────────┘   └──────────────┘   └──────────────┘   └──────────────┘ │
│                                                                   │         │
│                                                                   ▼         │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐ │
│  │   EXPORT     │◄──│  SERIALIZE   │◄──│    FORMAT    │◄──│    OUTPUT    │ │
│  │              │   │              │   │              │   │              │ │
│  │ • File Write │   │ • Binary     │   │ • Results    │   │ • Results    │ │
│  │ • Format Conv│   │ • JSON       │   │ • Visualize  │   │ • Errors     │ │
│  │ • NetworkX   │   │ • Protobuf   │   │ • Transform  │   │ • Metadata   │ │
│  └──────────────┘   └──────────────┘   └──────────────┘   └──────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Input Stage

### Supported Input Formats

Lutufi supports multiple input formats for models, data, and evidence.

**Graph Formats:**
- **GraphML**: XML-based graph format with attributes
- **GEXF**: Gephi exchange format
- **Pajek .net**: Simple edge list format
- **Adjacency List**: Text-based adjacency representation
- **Edge List**: Simple source-target pairs

**Probabilistic Model Formats:**
- **BIF**: Bayesian Interchange Format
- **XMLBIF**: XML version of BIF
- **UAI**: Uncertainty in AI competition format
- **JSON**: Custom JSON format with schema

**Data Formats:**
- **CSV/TSV**: Tabular data with headers
- **HDF5**: Hierarchical data format for large datasets
- **Parquet**: Columnar storage format
- **NumPy**: .npy and .npz array files

### Validation and Parsing

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         INPUT VALIDATION PIPELINE                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Raw Input                                                                 │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────┐                                                        │
│  │  Format Detect  │  ──► Determine parser based on extension/content      │
│  └─────────────────┘                                                        │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────┐                                                        │
│  │  Syntax Parse   │  ──► Parse raw bytes into AST/data structure          │
│  └─────────────────┘                                                        │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────┐                                                        │
│  │ Schema Validate │  ──► Check against format schema                      │
│  └─────────────────┘                                                        │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────┐                                                        │
│  │ Semantic Check  │  ──► Validate graph properties, probability sums      │
│  └─────────────────┘                                                        │
│      │                                                                      │
│      ▼                                                                      │
│  Parsed Model                                                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Validation Stages:**

1. **Format Detection**
   ```python
   def detect_format(source: Union[str, bytes, Path]) -> FormatType:
       """Detect format from file extension or content sniffing."""
       if isinstance(source, Path):
           ext = source.suffix.lower()
           format_map = {
               '.bif': FormatType.BIF,
               '.xml': FormatType.XMLBIF,
               '.json': FormatType.JSON,
               '.uai': FormatType.UAI,
           }
           if ext in format_map:
               return format_map[ext]
       
       # Content-based detection
       header = source[:256] if isinstance(source, bytes) else source[:256].encode()
       if b'<?xml' in header:
           if b'GRAPHML' in header:
               return FormatType.GRAPHML
           return FormatType.XMLBIF
       # ... more detection logic
   ```

2. **Syntax Parsing**
   Each format has a dedicated parser:
   ```rust
   trait ModelParser {
       fn parse(&self, source: &[u8]) -> Result<ParsedModel, ParseError>;
       fn can_parse(&self, source: &[u8]) -> bool;
   }
   
   struct BifParser;
   impl ModelParser for BifParser {
       fn parse(&self, source: &[u8]) -> Result<ParsedModel, ParseError> {
           // Parse BIF grammar
           let tokens = tokenize(source)?;
           let ast = parse_bif_ast(&tokens)?;
           Ok(convert_ast_to_model(ast)?)
       }
   }
   ```

3. **Schema Validation**
   ```python
   def validate_schema(model_dict: dict, schema: dict) -> ValidationResult:
       """Validate parsed model against JSON schema."""
       errors = []
       
       # Check required fields
       for field in schema.get('required', []):
           if field not in model_dict:
               errors.append(f"Missing required field: {field}")
       
       # Check field types
       for field, field_spec in schema.get('properties', {}).items():
           if field in model_dict:
               if not validate_type(model_dict[field], field_spec['type']):
                   errors.append(f"Field {field} has wrong type")
       
       return ValidationResult(valid=len(errors) == 0, errors=errors)
   ```

4. **Semantic Validation**
   ```rust
   fn validate_semantics(model: &ParsedModel) -> Result<(), SemanticError> {
       // Check for cycles in Bayesian network
       if has_cycle(&model.structure) {
           return Err(SemanticError::CycleDetected);
       }
       
       // Validate CPD normalization
       for (node, cpd) in &model.cpds {
           if !cpd.is_normalized() {
               return Err(SemanticError::NonNormalizedCPD(node.clone()));
           }
       }
       
       // Check variable references
       for edge in &model.edges {
           if !model.variables.contains(&edge.parent) {
               return Err(SemanticError::UndefinedVariable(edge.parent.clone()));
           }
       }
       
       Ok(())
   }
   ```

---

## Model Construction Stage

### Building the Internal Representation

The construction stage transforms parsed models into Lutufi's internal factor graph representation.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      MODEL CONSTRUCTION PIPELINE                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Parsed Model                                                               │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 1. CREATE VARIABLES                                                 │   │
│  │    • Allocate Variable structs                                       │   │
│  │    • Assign VariableIds                                              │   │
│  │    • Store domains                                                   │   │
│  │    • Build name-to-id index                                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 2. CREATE FACTORS                                                   │   │
│  │    • Convert CPDs to factor representation                           │   │
│  │    • Handle different CPD types (tabular, Gaussian, etc.)            │   │
│  │    • Build factor index                                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 3. BUILD EDGES                                                      │   │
│  │    • Connect variables to factors                                    │   │
│  │    • Compute adjacency structures                                    │   │
│  │    • Optimize storage format                                         │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 4. COMPUTE DERIVED STRUCTURES                                       │   │
│  │    • Variable adjacency lists                                        │   │
│  │    • Factor adjacency lists                                          │   │
│  │    • CSR/CSC representations                                         │   │
│  │    • Graph statistics (treewidth estimate, etc.)                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  Factor Graph                                                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Conversion to Canonical Form

```rust
fn build_factor_graph(parsed: ParsedModel) -> Result<FactorGraph, ConstructionError> {
    let mut builder = FactorGraphBuilder::new();
    
    // Step 1: Create variables
    for (name, domain) in parsed.variables {
        let var_id = builder.add_variable(Variable {
            id: 0, // Will be assigned
            name: name.clone(),
            domain: convert_domain(domain),
            ..Default::default()
        });
        builder.name_index.insert(name, var_id);
    }
    
    // Step 2: Create factors from CPDs
    for (child, cpd) in parsed.cpds {
        let child_id = builder.name_index[&child];
        let parent_ids: Vec<_> = cpd.parents.iter()
            .map(|p| builder.name_index[p])
            .collect();
        
        let factor = match cpd.type_ {
            CpdType::Tabular { values } => {
                Arc::new(TabularFactor::new(
                    std::iter::once(child_id).chain(parent_ids).collect(),
                    values,
                ))
            }
            CpdType::Gaussian { mean, variance } => {
                Arc::new(GaussianCPD::new(child_id, parent_ids, mean, variance).to_factor())
            }
            // ... other CPD types
        };
        
        builder.add_factor(factor);
    }
    
    // Step 3: Build and validate
    let graph = builder.build()?;
    validate_graph(&graph)?;
    
    Ok(graph)
}
```

### Semantic Validation

```rust
fn validate_graph(graph: &FactorGraph) -> Result<(), ValidationError> {
    // Check factor scopes are valid
    for factor in &graph.factors {
        for &var in factor.scope() {
            if var as usize >= graph.variables.len() {
                return Err(ValidationError::InvalidVariableId(var));
            }
        }
    }
    
    // Check edges match factor scopes
    for (factor_id, factor) in graph.factors.iter().enumerate() {
        let incident_edges = &graph.factor_adjacency[factor_id];
        if incident_edges.len() != factor.scope().len() {
            return Err(ValidationError::EdgeScopeMismatch { ... });
        }
    }
    
    // Check graph is connected (warning if not)
    if !is_connected(graph) {
        warn!("Factor graph has multiple connected components");
    }
    
    Ok(())
}
```

---

## Preprocessing Stage

### Moralization (for Undirected Inference)

Moralization converts a directed graph to an undirected one by "marrying" parents.

```
Before Moralization:          After Moralization:
                                
    A → B                       A — B
    ↓   ↓                       │ X │
    C ← D                       C — D
                                  │
                                  A (A and D are married)
```

```rust
fn moralize(graph: &FactorGraph) -> MoralGraph {
    let mut moral_edges: HashSet<(VariableId, VariableId)> = HashSet::new();
    
    // Add edges from factor scopes (undirected version)
    for factor in &graph.factors {
        let scope = factor.scope();
        for i in 0..scope.len() {
            for j in (i + 1)..scope.len() {
                moral_edges.insert(order_pair(scope[i], scope[j]));
            }
        }
    }
    
    // "Marry" parents of each variable
    for factor in &graph.factors {
        if factor.scope().len() > 1 {
            // Parents are all variables except the first (child)
            let parents = &factor.scope()[1..];
            for i in 0..parents.len() {
                for j in (i + 1)..parents.len() {
                    moral_edges.insert(order_pair(parents[i], parents[j]));
                }
            }
        }
    }
    
    MoralGraph::from_edges(moral_edges)
}
```

### Triangulation for Junction Tree

Triangulation makes a graph chordal by adding fill edges.

```rust
fn triangulate(graph: &MoralGraph, 
               elimination_order: &[VariableId]) -> TriangulatedGraph {
    let mut result = graph.clone();
    let mut fill_edges = Vec::new();
    
    // Eliminate variables in order
    for &var in elimination_order {
        let neighbors: Vec<_> = result.neighbors(var).collect();
        
        // Add edges between all pairs of neighbors (fill edges)
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                if !result.has_edge(neighbors[i], neighbors[j]) {
                    result.add_edge(neighbors[i], neighbors[j]);
                    fill_edges.push((neighbors[i], neighbors[j]));
                }
            }
        }
        
        // Remove eliminated variable
        result.remove_node(var);
    }
    
    TriangulatedGraph {
        graph: result,
        fill_edges,
        elimination_order: elimination_order.to_vec(),
    }
}
```

### Algorithm Selection

The system automatically selects appropriate inference algorithms based on model characteristics.

```rust
fn select_inference_algorithm(graph: &FactorGraph, 
                               query: &Query,
                               constraints: &InferenceConstraints) -> AlgorithmChoice {
    // Compute treewidth
    let treewidth = estimate_treewidth(graph);
    
    // Check if exact inference is feasible
    if treewidth <= constraints.max_exact_treewidth {
        // Use junction tree for exact inference
        return AlgorithmChoice::JunctionTree {
            elimination_heuristic: MinFill,
        };
    }
    
    // Check query type
    match query.query_type {
        QueryType::Marginal => {
            if graph.is_tree() {
                AlgorithmChoice::BeliefPropagation {
                    max_iterations: 1, // Exact on trees
                }
            } else {
                AlgorithmChoice::BeliefPropagation {
                    max_iterations: constraints.max_iterations,
                }
            }
        }
        QueryType::MAP => {
            if treewidth <= 20 {
                AlgorithmChoice::VariableElimination
            } else {
                AlgorithmChoice::LocalSearchMAP
            }
        }
        // ... other query types
    }
}
```

---

## Inference Stage

### Data Flow During Inference

#### Variable Elimination

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     VARIABLE ELIMINATION DATA FLOW                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Initial Factors: f₁(A,B), f₂(B,C), f₃(C,D)                                 │
│                                                                             │
│  Eliminate B:                                                               │
│    ┌────────────────────────────────────────────────────────────────────┐  │
│    │ Gather factors containing B: f₁(A,B), f₂(B,C)                       │  │
│    │ Multiply: f₁(A,B) × f₂(B,C) = f₄(A,B,C)                             │  │
│    │ Sum out B: Σ_B f₄(A,B,C) = f₅(A,C)                                  │  │
│    │ Store f₅(A,C), remove f₁, f₂                                        │  │
│    └────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  Remaining factors: f₅(A,C), f₃(C,D)                                        │
│                                                                             │
│  Eliminate C:                                                               │
│    ┌────────────────────────────────────────────────────────────────────┐  │
│    │ Gather factors containing C: f₅(A,C), f₃(C,D)                       │  │
│    │ Multiply: f₅(A,C) × f₃(C,D) = f₆(A,C,D)                             │  │
│    │ Sum out C: Σ_C f₆(A,C,D) = f₇(A,D)                                  │  │
│    │ Store f₇(A,D), remove f₅, f₃                                        │  │
│    └────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  Result: f₇(A,D) = P(A,D)                                                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

```rust
fn variable_elimination(
    factors: Vec<Arc<dyn Factor>>,
    elimination_order: &[VariableId],
    query_vars: &[VariableId]
) -> Arc<dyn Factor> {
    let mut active_factors = factors;
    
    for &var in elimination_order {
        // Partition factors
        let (containing_var, not_containing): (Vec<_>, Vec<_>) = active_factors
            .into_iter()
            .partition(|f| f.scope().contains(&var));
        
        if containing_var.is_empty() {
            continue;
        }
        
        // Multiply all factors containing var
        let product = containing_var.into_iter()
            .reduce(|a, b| a.multiply(&*b))
            .unwrap();
        
        // Marginalize out var
        let marginalized = product.marginalize(&[var]);
        
        // Update active factors
        active_factors = not_containing;
        if marginalized.scope().len() > 0 {
            active_factors.push(marginalized);
        }
    }
    
    // Multiply remaining factors
    active_factors.into_iter()
        .reduce(|a, b| a.multiply(&*b))
        .unwrap_or_else(|| Arc::new(EmptyFactor))
}
```

#### Belief Propagation (Message Passing)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     BELIEF PROPAGATION DATA FLOW                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Factor Graph:         f₁ ─ A ─ f₂ ─ B ─ f₃ ─ C                             │
│                                                                             │
│  Message Passing Schedule:                                                  │
│                                                                             │
│  Iteration 1:                                                               │
│    ┌────────────────────────────────────────────────────────────────────┐  │
│    │ Leaf to root:                                                      │  │
│    │   μ_{A→f₂}(A) = f₁(A)                                              │  │
│    │   μ_{C→f₃}(C) = 1.0 (no factor beyond C)                          │  │
│    │                                                                    │  │
│    │ Compute at f₂:                                                     │  │
│    │   μ_{f₂→B}(B) = Σ_A f₂(A,B) × μ_{A→f₂}(A)                         │  │
│    │                                                                    │  │
│    │ Compute at f₃:                                                     │  │
│    │   μ_{f₃→B}(B) = Σ_C f₃(B,C) × μ_{C→f₃}(C)                         │  │
│    └────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  Iteration 2:                                                               │
│    ┌────────────────────────────────────────────────────────────────────┐  │
│    │ Root to leaves:                                                    │  │
│    │   μ_{B→f₂}(B) = μ_{f₃→B}(B)                                        │  │
│    │   μ_{B→f₃}(B) = μ_{f₂→B}(B)                                        │  │
│    │                                                                    │  │
│    │ Compute beliefs:                                                   │  │
│    │   b(A) ∝ f₁(A) × μ_{f₂→A}(A)                                       │  │
│    │   b(B) ∝ μ_{f₂→B}(B) × μ_{f₃→B}(B)                                 │  │
│    │   b(C) ∝ f₃(C) × μ_{f₃→C}(C)                                       │  │
│    └────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  Convergence Check: max |b_new - b_old| < ε                                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

```rust
fn belief_propagation(graph: &FactorGraph, max_iter: usize, tol: f64) -> Beliefs {
    let mut messages = initialize_messages(graph);
    let mut beliefs = Beliefs::new();
    
    for iter in 0..max_iter {
        let mut max_change = 0.0;
        
        // Variable to factor messages
        for edge in graph.edges() {
            let msg = compute_var_to_factor_msg(graph, edge, &messages);
            max_change = max_change.max(message_change(&msg, &messages[edge]));
            messages[edge].var_to_factor = msg;
        }
        
        // Factor to variable messages
        for edge in graph.edges() {
            let msg = compute_factor_to_var_msg(graph, edge, &messages);
            max_change = max_change.max(message_change(&msg, &messages[edge]));
            messages[edge].factor_to_var = msg;
        }
        
        if max_change < tol {
            info!("Belief propagation converged after {} iterations", iter + 1);
            break;
        }
    }
    
    // Compute final beliefs
    compute_beliefs(graph, &messages)
}

fn compute_var_to_factor_msg(
    graph: &FactorGraph,
    edge: &Edge,
    messages: &MessageStore
) -> Message {
    let var = edge.variable;
    let factor = edge.factor;
    
    // Product of incoming messages from other factors
    let mut belief = Belief::ones(graph.variable(var).domain.cardinality());
    
    for other_edge in graph.incident_edges(var) {
        if other_edge.factor != factor {
            belief = belief.multiply(&messages[other_edge].factor_to_var);
        }
    }
    
    // Incorporate evidence if present
    if let Some(evidence) = graph.variable(var).evidence {
        belief = belief.apply_evidence(evidence);
    }
    
    Message::from_belief(belief)
}

fn compute_factor_to_var_msg(
    graph: &FactorGraph,
    edge: &Edge,
    messages: &MessageStore
) -> Message {
    let factor = &graph.factors[edge.factor];
    let var = edge.variable;
    
    // Start with factor
    let mut factor_belief = FactorBelief::from_factor(factor);
    
    // Multiply incoming messages from other variables
    for other_edge in graph.factor_edges(edge.factor) {
        if other_edge.variable != var {
            factor_belief = factor_belief.multiply_message(
                &messages[other_edge].var_to_factor
            );
        }
    }
    
    // Marginalize to target variable
    Message::from_belief(factor_belief.marginalize(var))
}
```

### Convergence Monitoring

```rust
struct ConvergenceMonitor {
    history: Vec<f64>,
    tolerance: f64,
    window_size: usize,
}

impl ConvergenceMonitor {
    fn check(&mut self, change: f64) -> ConvergenceStatus {
        self.history.push(change);
        
        if change < self.tolerance {
            return ConvergenceStatus::Converged;
        }
        
        // Check for oscillation
        if self.history.len() >= self.window_size {
            let recent = &self.history[self.history.len() - self.window_size..];
            let mean_change = recent.iter().sum::<f64>() / recent.len() as f64;
            let variance = recent.iter()
                .map(|x| (x - mean_change).powi(2))
                .sum::<f64>() / recent.len() as f64;
            
            if variance < self.tolerance {
                return ConvergenceStatus::Oscillating;
            }
        }
        
        ConvergenceStatus::Continue
    }
}
```

---

## Learning Stage

### Parameter Learning Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    PARAMETER LEARNING DATA FLOW                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Training Data                                                              │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 1. SUFFICIENT STATISTICS                                            │   │
│  │    For each CPD:                                                     │   │
│  │    • Count co-occurrences of variable and parent values              │   │
│  │    • Handle missing data (EM or complete case)                       │   │
│  │    • Apply weights if weighted learning                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 2. PARAMETER ESTIMATION                                             │   │
│  │    MLE:  θ_ijk = N_ijk / N_ij                                        │   │
│  │    Bayesian: θ_ijk = (N_ijk + α_ijk) / (N_ij + α_ij)                 │   │
│  │    where N_ijk = count(X_i=k, Parents_i=j)                           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 3. REGULARIZATION (optional)                                        │   │
│  │    • Laplace smoothing                                               │   │
│  │    • Dirichlet prior                                                 │   │
│  │    • Structural penalties                                            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  Updated CPDs                                                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

```rust
fn learn_parameters_mle(
    model: &mut BayesianNetwork,
    data: &DataFrame,
    smoothing: Option<f64>
) -> Result<(), LearningError> {
    let alpha = smoothing.unwrap_or(0.0);
    
    for node in model.nodes() {
        let parents = model.get_parents(node);
        let cpd = model.get_cpd_mut(node);
        
        // Compute sufficient statistics
        let stats = compute_sufficient_statistics(data, node, &parents);
        
        // Estimate parameters
        for parent_config in 0..stats.n_parent_configs {
            let total_count: f64 = stats.counts[parent_config].iter().sum::<f64>() + 
                                   alpha * stats.n_variable_states as f64;
            
            for state in 0..stats.n_variable_states {
                let count = stats.counts[parent_config][state] + alpha;
                cpd.set_probability(state, parent_config, count / total_count);
            }
        }
    }
    
    Ok(())
}

struct SufficientStatistics {
    n_variable_states: usize,
    n_parent_configs: usize,
    /// counts[parent_config][variable_state]
    counts: Vec<Vec<f64>>,
}

fn compute_sufficient_statistics(
    data: &DataFrame,
    variable: &str,
    parents: &[String]
) -> SufficientStatistics {
    let n_states = data[variable].n_unique();
    let n_parent_configs = if parents.is_empty() {
        1
    } else {
        parents.iter().map(|p| data[p].n_unique()).product()
    };
    
    let mut counts = vec![vec![0.0; n_states]; n_parent_configs];
    
    for row in data.rows() {
        let parent_config = encode_parent_config(row, parents);
        let state = row[variable].as_index();
        counts[parent_config][state] += 1.0;
    }
    
    SufficientStatistics {
        n_variable_states: n_states,
        n_parent_configs,
        counts,
    }
}
```

### Structure Learning Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    STRUCTURE LEARNING DATA FLOW                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Training Data                                                              │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ SCORE-BASED METHODS (e.g., Hill Climbing)                           │   │
│  │                                                                     │   │
│  │  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐        │   │
│  │  │ Initial Graph│────►│ Compute Score│────►│ Find Best    │        │   │
│  │  │ (empty/chain)│     │ (BIC/BDeu)   │     │ Neighbor     │        │   │
│  │  └──────────────┘     └──────────────┘     └──────────────┘        │   │
│  │         ▲                                            │              │   │
│  │         │                                            ▼              │   │
│  │         │                              ┌────────────────────────┐   │   │
│  │         └──────────────────────────────│ Apply Best Operation   │   │   │
│  │            (if score improves)         │ (add/remove/flip edge) │   │   │
│  │                                        └────────────────────────┘   │   │
│  │                                                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ CONSTRAINT-BASED METHODS (e.g., PC Algorithm)                       │   │
│  │                                                                     │   │
│  │  1. Independence Testing:                                          │   │
│  │     For each pair (X,Y):                                           │   │
│  │       Test X ⊥ Y | Z for all conditioning sets Z                   │   │
│  │                                                                     │   │
│  │  2. Skeleton Construction:                                         │   │
│  │     Remove edge X-Y if X ⊥ Y | Z for some Z                        │   │
│  │                                                                     │   │
│  │  3. Orientation:                                                   │   │
│  │     Apply orientation rules (Meek's rules)                         │   │
│  │     to determine edge directions                                   │   │
│  │                                                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

```rust
fn hill_climbing_structure_learning(
    data: &DataFrame,
    scoring_method: ScoreType,
    max_iterations: usize
) -> BayesianNetwork {
    let mut current_model = BayesianNetwork::new(data.columns());
    let mut current_score = score_model(&current_model, data, scoring_method);
    
    for iter in 0..max_iterations {
        let mut best_neighbor = None;
        let mut best_neighbor_score = current_score;
        
        // Generate all possible edge modifications
        let candidates = generate_candidate_operations(&current_model);
        
        for op in candidates {
            let mut neighbor = current_model.clone();
            op.apply(&mut neighbor);
            
            if !neighbor.is_valid() {
                continue;
            }
            
            let score = score_model(&neighbor, data, scoring_method);
            
            if score > best_neighbor_score {
                best_neighbor_score = score;
                best_neighbor = Some((op, neighbor));
            }
        }
        
        // Check for convergence
        if best_neighbor.is_none() || best_neighbor_score <= current_score {
            break;
        }
        
        // Move to best neighbor
        current_model = best_neighbor.unwrap().1;
        current_score = best_neighbor_score;
    }
    
    current_model
}

fn score_model(model: &BayesianNetwork, data: &DataFrame, method: ScoreType) -> f64 {
    match method {
        ScoreType::BIC => score_bic(model, data),
        ScoreType::BDeu { equivalent_sample_size } => {
            score_bdeu(model, data, equivalent_sample_size)
        }
        ScoreType::AIC => score_aic(model, data),
    }
}

fn score_bic(model: &BayesianNetwork, data: &DataFrame) -> f64 {
    let log_likelihood = compute_log_likelihood(model, data);
    let n_samples = data.n_rows() as f64;
    let n_params = model.count_parameters() as f64;
    
    log_likelihood - 0.5 * n_params * n_samples.ln()
}
```

---

## Query Stage

### Processing User Queries

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        QUERY PROCESSING FLOW                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  User Query: P(Disease | Symptom=fever, Test=positive)                      │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 1. QUERY PARSING                                                    │   │
│  │    • Parse variable names                                           │   │
│  │    • Validate against model                                         │   │
│  │    • Parse evidence values                                          │   │
│  │    • Convert to internal representation                             │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 2. QUERY OPTIMIZATION                                               │   │
│  │    • Identify irrelevant variables                                  │   │
│  │    • Determine optimal elimination order                            │   │
│  │    • Select inference algorithm                                     │   │
│  │    • Check cache for similar queries                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 3. EVIDENCE INCORPORATION                                           │   │
│  │    • Add evidence factors to graph                                  │   │
│  │    • Propagate evidence (barren node removal)                       │   │
│  │    • Reduce graph size                                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 4. INFERENCE EXECUTION                                              │   │
│  │    • Run selected algorithm                                         │   │
│  │    • Monitor convergence                                            │   │
│  │    • Handle numerical issues                                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ 5. RESULT AGGREGATION                                               │   │
│  │    • Collect marginal distributions                                 │   │
│  │    • Format for output                                              │   │
│  │    • Compute confidence metrics                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│      │                                                                      │
│      ▼                                                                      │
│  QueryResult                                                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Query Plan Optimization

```rust
struct QueryPlan {
    target_variables: Vec<VariableId>,
    evidence: EvidenceSet,
    elimination_order: Vec<VariableId>,
    algorithm: AlgorithmChoice,
    irrelevant_variables: Vec<VariableId>,
}

fn optimize_query(graph: &FactorGraph, query: &Query) -> QueryPlan {
    // Find relevant variables
    let relevant = find_relevant_variables(graph, &query.variables, &query.evidence);
    let irrelevant: Vec<_> = graph.variables()
        .filter(|v| !relevant.contains(v))
        .collect();
    
    // Create reduced graph
    let reduced = graph.remove_variables(&irrelevant);
    
    // Compute optimal elimination order
    let elimination_order = compute_elimination_order(&reduced, &query.variables);
    
    // Select algorithm
    let algorithm = select_algorithm(&reduced, &query);
    
    QueryPlan {
        target_variables: query.variables.clone(),
        evidence: query.evidence.clone(),
        elimination_order,
        algorithm,
        irrelevant_variables: irrelevant,
    }
}

fn find_relevant_variables(
    graph: &FactorGraph,
    targets: &[VariableId],
    evidence: &EvidenceSet
) -> HashSet<VariableId> {
    // Start with targets and evidence variables
    let mut relevant: HashSet<_> = targets.iter().cloned().collect();
    for ev in &evidence.evidence {
        relevant.insert(ev.variable());
    }
    
    // Add ancestors (for Bayesian networks)
    let mut to_process: Vec<_> = relevant.iter().cloned().collect();
    while let Some(var) = to_process.pop() {
        for parent in graph.parents(var) {
            if relevant.insert(parent) {
                to_process.push(parent);
            }
        }
    }
    
    relevant
}

fn compute_elimination_order(
    graph: &FactorGraph,
    targets: &[VariableId]
) -> Vec<VariableId> {
    // Min-fill heuristic
    let mut order = Vec::new();
    let mut remaining: HashSet<_> = graph.variables().collect();
    
    // Never eliminate target variables
    for target in targets {
        remaining.remove(target);
    }
    
    while !remaining.is_empty() {
        // Find variable that adds minimum fill edges when eliminated
        let (&best_var, _) = remaining.iter()
            .map(|&v| (v, count_fill_edges(graph, v, &remaining)))
            .min_by_key(|&(_, count)| count)
            .unwrap();
        
        order.push(best_var);
        remaining.remove(&best_var);
    }
    
    order
}
```

### Result Aggregation

```rust
fn aggregate_results(
    beliefs: HashMap<VariableId, Belief>,
    query_vars: &[VariableId],
    joint: Option<JointBelief>
) -> QueryResult {
    let mut marginals = HashMap::new();
    
    for &var in query_vars {
        if let Some(belief) = beliefs.get(&var) {
            marginals.insert(var, belief.clone());
        }
    }
    
    QueryResult {
        variables: query_vars.to_vec(),
        marginals,
        joint,
        metadata: QueryMetadata {
            computation_time: None, // Will be filled
            algorithm_used: None,
            convergence_status: None,
        },
    }
}
```

---

## Output Stage

### Result Formatting

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      OUTPUT FORMATTING PIPELINE                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  QueryResult                                                                │
│      │                                                                      │
│      ├──────────────────────────────────────────────────────────────┐       │
│      ▼                                                              ▼       │
│  ┌──────────────┐                                             ┌──────────┐ │
│  │  DataFrame   │                                             │  Dict    │ │
│  │  Format      │                                             │  Format  │ │
│  │              │                                             │          │ │
│  │  • Columnar  │                                             │  • JSON  │ │
│  │  • Indexed   │                                             │    compat│ │
│  │  • Typed     │                                             │  • Nested│ │ │
│  └──────────────┘                                             │    struct│ │
│      │                                                        └──────────┘ │
│      ▼                                                                      │
│  ┌──────────────┐                                                           │
│  │  Export to   │                                                           │
│  │  External    │                                                           │
│  │  Formats     │                                                           │
│  │              │                                                           │
│  │  • CSV       │                                                           │
│  │  • JSON      │                                                           │
│  │  • HDF5      │                                                           │
│  │  • NumPy     │                                                           │
│  └──────────────┘                                                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

```rust
impl QueryResult {
    fn to_dataframe(&self) -> DataFrame {
        let mut columns = Vec::new();
        let mut data = Vec::new();
        
        for (var_id, belief) in &self.marginals {
            match belief {
                Belief::Discrete { probabilities, labels } => {
                    for (i, &prob) in probabilities.iter().enumerate() {
                        let state = labels.as_ref()
                            .map(|l| l[i].clone())
                            .unwrap_or_else(|| i.to_string());
                        
                        columns.push(format!("{}_{}", var_id, state));
                        data.push(prob);
                    }
                }
                // ... other belief types
            }
        }
        
        DataFrame::from_columns(columns, vec![data])
    }
    
    fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        
        for (var_id, belief) in &self.marginals {
            let belief_json = match belief {
                Belief::Discrete { probabilities, labels } => {
                    let states = labels.as_ref()
                        .map(|l| l.clone())
                        .unwrap_or_else(|| 
                            (0..probabilities.len()).map(|i| i.to_string()).collect()
                        );
                    
                    json!({
                        "type": "discrete",
                        "states": states,
                        "probabilities": probabilities,
                    })
                }
                Belief::Gaussian { mean, variance } => {
                    json!({
                        "type": "gaussian",
                        "mean": mean,
                        "variance": variance,
                    })
                }
                // ... other types
            };
            
            result.insert(var_id.to_string(), belief_json);
        }
        
        result
    }
}
```

### Serialization

```rust
fn serialize_result(result: &QueryResult, format: OutputFormat) -> Vec<u8> {
    match format {
        OutputFormat::Binary => serialize_binary(result),
        OutputFormat::Json => serialize_json(result),
        OutputFormat::MessagePack => serialize_msgpack(result),
    }
}

fn serialize_binary(result: &QueryResult) -> Vec<u8> {
    let mut buffer = Vec::new();
    
    // Header
    buffer.extend_from_slice(b"LTR\0"); // Magic
    buffer.extend_from_slice(&1u16.to_le_bytes()); // Version
    
    // Number of variables
    buffer.extend_from_slice(&(result.variables.len() as u32).to_le_bytes());
    
    // Each variable's belief
    for (var_id, belief) in &result.marginals {
        buffer.extend_from_slice(&var_id.0.to_le_bytes());
        
        match belief {
            Belief::Discrete { probabilities, .. } => {
                buffer.push(0x01); // Type tag
                buffer.extend_from_slice(&(probabilities.len() as u32).to_le_bytes());
                for &p in probabilities {
                    buffer.extend_from_slice(&p.to_le_bytes());
                }
            }
            // ... other types
        }
    }
    
    buffer
}
```

### Visualization Preparation

```rust
fn prepare_for_visualization(result: &QueryResult) -> VisualizationData {
    let mut node_data = Vec::new();
    let mut edge_data = Vec::new();
    
    for (var_id, belief) in &result.marginals {
        let (color, size) = match belief {
            Belief::Discrete { probabilities, .. } => {
                // Entropy-based coloring
                let entropy = compute_entropy(probabilities);
                let color = entropy_to_color(entropy);
                let size = 1.0 + probabilities.iter().copied().fold(0.0, f64::max);
                (color, size)
            }
            _ => ("gray".to_string(), 1.0),
        };
        
        node_data.push(NodeVisualization {
            id: var_id.to_string(),
            color,
            size,
            label: format!("{}\n{}", var_id, belief_to_string(belief)),
        });
    }
    
    VisualizationData { nodes: node_data, edges: edge_data }
}
```

---

## Streaming Data Flow

### Handling Continuous Data Streams

```rust
struct StreamingInference {
    /// Rolling window of recent observations
    window: RollingWindow,
    
    /// Current belief state
    current_beliefs: Beliefs,
    
    /// Update interval
    update_interval: Duration,
    
    /// Last update time
    last_update: Instant,
}

impl StreamingInference {
    fn process_observation(&mut self, obs: Observation) -> Option<Beliefs> {
        // Add to window
        self.window.add(obs);
        
        // Check if we should update
        if self.last_update.elapsed() >= self.update_interval {
            self.update_beliefs()
        } else {
            None
        }
    }
    
    fn update_beliefs(&mut self) -> Beliefs {
        // Create factor graph from window
        let graph = self.window.to_factor_graph();
        
        // Run inference
        self.current_beliefs = belief_propagation(&graph, 100, 1e-6);
        self.last_update = Instant::now();
        
        self.current_beliefs.clone()
    }
}
```

### Online Inference

```rust
struct OnlineInference {
    base_model: FactorGraph,
    incoming_messages: mpsc::Receiver<Message>,
    outgoing_beliefs: mpsc::Sender<Beliefs>,
}

impl OnlineInference {
    fn run(mut self) {
        let mut current_graph = self.base_model.clone();
        
        while let Ok(msg) = self.incoming_messages.recv() {
            // Incorporate new message
            current_graph.update_message(msg);
            
            // Run incremental inference
            let beliefs = incremental_bp(&current_graph, max_iter=5);
            
            // Send updated beliefs
            self.outgoing_beliefs.send(beliefs).ok();
        }
    }
}

fn incremental_bp(graph: &FactorGraph, max_iter: usize) -> Beliefs {
    // Only propagate messages in affected region
    let affected_region = find_affected_region(graph);
    
    // Run BP only on affected region
    let partial_beliefs = belief_propagation_subset(graph, &affected_region, max_iter);
    
    // Merge with previous beliefs
    merge_beliefs(&previous_beliefs, &partial_beliefs)
}
```

### Incremental Updates

```rust
struct IncrementalUpdater {
    model: FactorGraph,
    cache: InferenceCache,
}

impl IncrementalUpdater {
    fn add_evidence(&mut self, evidence: Evidence) -> Beliefs {
        // Check if evidence affects cached results
        if self.cache.is_still_valid(&evidence) {
            // Only update affected beliefs
            let affected = self.find_affected_variables(&evidence);
            let new_beliefs = self.update_beliefs_incremental(&affected, &evidence);
            self.cache.update(&affected, &new_beliefs);
            new_beliefs
        } else {
            // Full recomputation needed
            self.model.apply_evidence(evidence);
            let beliefs = self.run_full_inference();
            self.cache.clear_and_set(beliefs.clone());
            beliefs
        }
    }
}
```

---

## Data Transformation Pipeline

### The Pipeline Pattern

```rust
trait PipelineStage<Input, Output> {
    fn process(&self, input: Input) -> Result<Output, PipelineError>;
}

struct Pipeline<I, O> {
    stages: Vec<Box<dyn PipelineStage<I, O>>>,
}

impl<I, O> Pipeline<I, O> {
    fn add_stage<S: PipelineStage<I, O> + 'static>(mut self, stage: S) -> Self {
        self.stages.push(Box::new(stage));
        self
    }
    
    fn execute(&self, mut data: I) -> Result<O, PipelineError> {
        for stage in &self.stages {
            data = stage.process(data)?;
        }
        Ok(data)
    }
}
```

### Multi-Stage Processing

```
Raw Data ──► Clean ──► Transform ──► Feature Eng ──► Model Input
                │           │              │
                ▼           ▼              ▼
           Handle     Normalize       Create factor
           Missing    Encode cat       representations
           Values     variables
```

```rust
struct DataCleaningStage;
impl PipelineStage<DataFrame, DataFrame> for DataCleaningStage {
    fn process(&self, df: DataFrame) -> Result<DataFrame, PipelineError> {
        // Remove duplicates
        let df = df.drop_duplicates()?;
        
        // Handle missing values
        let df = df.fill_missing(FillStrategy::Forward)?;
        
        // Remove outliers
        let df = df.filter(|row| !is_outlier(row))?;
        
        Ok(df)
    }
}

struct EncodingStage;
impl PipelineStage<DataFrame, DataFrame> for EncodingStage {
    fn process(&self, df: DataFrame) -> Result<DataFrame, PipelineError> {
        // One-hot encode categoricals
        let df = one_hot_encode(df, &categorical_columns(&df))?;
        
        // Normalize continuous variables
        let df = normalize(df, &continuous_columns(&df))?;
        
        Ok(df)
    }
}

// Build and run pipeline
let pipeline = Pipeline::new()
    .add_stage(DataCleaningStage)
    .add_stage(EncodingStage)
    .add_stage(ModelInputStage);

let model_input = pipeline.execute(raw_data)?;
```

### Intermediate Representations

```rust
/// Intermediate representation between stages
enum IntermediateRepr {
    /// Raw parsed data
    Parsed(ParsedModel),
    
    /// Validated model
    Validated(ValidatedModel),
    
    /// Factor graph
    FactorGraph(FactorGraph),
    
    /// Preprocessed for specific algorithm
    Preprocessed(PreprocessedModel),
    
    /// Inference results
    Results(QueryResult),
}

struct PipelineContext {
    /// Current stage
    stage: usize,
    
    /// Intermediate representations by stage
    intermediates: Vec<(String, IntermediateRepr)>,
    
    /// Timing information
    timings: Vec<(String, Duration)>,
}
```

---

## Caching and Memoization

### What Gets Cached

```rust
struct InferenceCache {
    /// Cached triangulations
    triangulations: LruCache<GraphSignature, TriangulatedGraph>,
    
    /// Cached elimination orders
    elimination_orders: LruCache<GraphSignature, Vec<VariableId>>,
    
    /// Cached messages from belief propagation
    messages: LruCache<EvidenceSignature, MessageStore>,
    
    /// Cached scores for structure learning
    local_scores: LruCache<FamilySignature, f64>,
    
    /// Cached inference results
    query_results: LruCache<QuerySignature, QueryResult>,
}
```

### Cache Invalidation Strategies

```rust
struct CacheManager {
    cache: InferenceCache,
    version: u64,
}

impl CacheManager {
    fn get_or_compute<F>(
        &mut self,
        key: &CacheKey,
        compute: F
    ) -> CachedValue
    where F: FnOnce() -> CachedValue
    {
        // Check if cache entry is valid
        if let Some(entry) = self.cache.get(key) {
            if entry.version == self.version {
                return entry.value.clone();
            }
        }
        
        // Compute and cache
        let value = compute();
        self.cache.insert(key.clone(), CacheEntry {
            value: value.clone(),
            version: self.version,
        });
        
        value
    }
    
    fn invalidate_on_model_change(&mut self) {
        self.version += 1;
        // Lazy invalidation: entries will be replaced on next access
    }
    
    fn invalidate_on_evidence_change(&mut self, changed_vars: &[VariableId]) {
        // Remove affected query results
        self.cache.query_results.retain(|key, _| {
            !key.affected_by(changed_vars)
        });
    }
}
```

### Memoization for Expensive Operations

```rust
use std::sync::Mutex;
use lru::LruCache;

struct MemoizedScorer {
    cache: Mutex<LruCache<FamilySignature, f64>>,
    data: Arc<DataFrame>,
}

impl MemoizedScorer {
    fn local_score(&self, node: VariableId, parents: &[VariableId]) -> f64 {
        let key = FamilySignature { node, parents: parents.to_vec() };
        
        // Try cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(&score) = cache.get(&key) {
                return score;
            }
        }
        
        // Compute score
        let score = compute_bic_score(&self.data, node, parents);
        
        // Store in cache
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(key, score);
        }
        
        score
    }
}
```

---

## Lazy Evaluation

### Deferred Computation

```rust
/// Lazy factor multiplication
trait LazyFactor: Factor {
    fn is_evaluated(&self) -> bool;
    fn evaluate_now(&mut self);
}

struct LazyProduct {
    factors: Vec<Arc<dyn Factor>>,
    cached_result: Option<Arc<dyn Factor>>,
}

impl Factor for LazyProduct {
    fn evaluate(&self, assignment: &Assignment) -> f64 {
        if let Some(ref cached) = self.cached_result {
            return cached.evaluate(assignment);
        }
        
        // Compute on demand
        self.factors.iter()
            .map(|f| f.evaluate(assignment))
            .product()
    }
    
    fn scope(&self) -> &[VariableId] {
        // Union of all factor scopes
        &self.computed_scope
    }
}
```

### Promise-Based Results

```rust
struct Promise<T> {
    state: Arc<Mutex<PromiseState<T>>>,
}

enum PromiseState<T> {
    Pending(Box<dyn FnOnce() -> T + Send>),
    Computing,
    Resolved(T),
    Failed(Box<dyn Error>),
}

impl<T: Clone> Promise<T> {
    fn get(&self) -> Result<T, Box<dyn Error>> {
        let mut state = self.state.lock().unwrap();
        
        match &*state {
            PromiseState::Resolved(val) => return Ok(val.clone()),
            PromiseState::Failed(err) => return Err(err.clone()),
            PromiseState::Pending(_) => {
                // Start computation
                if let PromiseState::Pending(f) = 
                    std::mem::replace(&mut *state, PromiseState::Computing) {
                    drop(state); // Release lock during computation
                    
                    let result = f();
                    
                    let mut state = self.state.lock().unwrap();
                    *state = PromiseState::Resolved(result);
                }
            }
            PromiseState::Computing => {
                // Wait for computation to complete
                drop(state);
                while let PromiseState::Computing = *self.state.lock().unwrap() {
                    thread::yield_now();
                }
            }
        }
        
        self.get() // Retry
    }
}
```

### Optimization Opportunities

```rust
/// Query optimizer that exploits lazy evaluation
struct QueryOptimizer;

impl QueryOptimizer {
    fn optimize(&self, query: Query, graph: &FactorGraph) -> OptimizedQuery {
        // 1. Push evidence down
        let query = self.push_evidence(query);
        
        // 2. Eliminate irrelevant variables
        let query = self.prune_irrelevant(query, graph);
        
        // 3. Reorder operations for cache efficiency
        let query = self.reorder_operations(query);
        
        // 4. Identify shared subexpressions
        let query = self.identify_common_subexpressions(query);
        
        OptimizedQuery(query)
    }
    
    fn push_evidence(&self, mut query: Query) -> Query {
        // Apply evidence as early as possible
        query.apply_early_projection();
        query
    }
}
```

---

## Error Handling in Data Flow

### Validation at Each Stage

```rust
enum PipelineStage {
    Input,
    Construction,
    Preprocessing,
    Inference,
    Learning,
    Output,
}

struct StageValidator;

impl StageValidator {
    fn validate(stage: PipelineStage, data: &IntermediateRepr) -> ValidationResult {
        match stage {
            PipelineStage::Input => Self::validate_input(data),
            PipelineStage::Construction => Self::validate_construction(data),
            PipelineStage::Preprocessing => Self::validate_preprocessing(data),
            PipelineStage::Inference => Self::validate_inference(data),
            _ => ValidationResult::Valid,
        }
    }
    
    fn validate_input(data: &IntermediateRepr) -> ValidationResult {
        if let IntermediateRepr::Parsed(parsed) = data {
            // Check required fields
            if parsed.variables.is_empty() {
                return ValidationResult::Invalid(
                    "Model must have at least one variable".to_string()
                );
            }
            
            // Check for duplicate names
            let names: HashSet<_> = parsed.variables.iter().map(|v| &v.name).collect();
            if names.len() != parsed.variables.len() {
                return ValidationResult::Invalid(
                    "Duplicate variable names found".to_string()
                );
            }
        }
        
        ValidationResult::Valid
    }
    
    fn validate_construction(data: &IntermediateRepr) -> ValidationResult {
        if let IntermediateRepr::FactorGraph(graph) = data {
            // Check graph consistency
            if let Err(e) = graph.validate() {
                return ValidationResult::Invalid(e.to_string());
            }
        }
        
        ValidationResult::Valid
    }
}
```

### Graceful Failure

```rust
enum ComputationResult<T> {
    Success(T),
    Partial(T, Vec<String>), // Result with warnings
    Failure(Box<dyn Error>),
}

fn run_inference_with_fallback(
    graph: &FactorGraph,
    query: &Query
) -> ComputationResult<QueryResult> {
    // Try exact inference first
    match try_exact_inference(graph, query) {
        Ok(result) => ComputationResult::Success(result),
        Err(InferenceError::Intractable { .. }) => {
            warn!("Exact inference intractable, falling back to approximate");
            
            // Try approximate inference
            match try_approximate_inference(graph, query) {
                Ok(result) => {
                    ComputationResult::Partial(
                        result,
                        vec!["Used approximate inference".to_string()]
                    )
                }
                Err(e) => ComputationResult::Failure(Box::new(e)),
            }
        }
        Err(e) => ComputationResult::Failure(Box::new(e)),
    }
}
```

### Partial Result Handling

```rust
struct PartialQueryResult {
    /// Computed marginals
    completed: HashMap<VariableId, Belief>,
    
    /// Variables that couldn't be computed
    failed: Vec<(VariableId, String)>,
    
    /// Approximate results
    approximate: HashMap<VariableId, Belief>,
}

impl PartialQueryResult {
    fn merge(self, other: PartialQueryResult) -> Self {
        Self {
            completed: merge_maps(self.completed, other.completed),
            failed: merge_vecs(self.failed, other.failed),
            approximate: merge_maps(self.approximate, other.approximate),
        }
    }
    
    fn to_query_result(self) -> Result<QueryResult, PartialResultError> {
        if !self.failed.is_empty() {
            Err(PartialResultError {
                partial_result: self.completed,
                failures: self.failed,
            })
        } else {
            Ok(QueryResult {
                marginals: self.completed,
                ..Default::default()
            })
        }
    }
}
```

---

## Performance Monitoring

### Profiling Hooks

```rust
struct PerformanceMonitor {
    /// Active timers
    timers: HashMap<String, Instant>,
    
    /// Completed measurements
    measurements: Vec<Measurement>,
    
    /// Memory tracking
    memory_snapshots: Vec<MemorySnapshot>,
}

struct Measurement {
    stage: String,
    duration: Duration,
    memory_delta: i64,
}

impl PerformanceMonitor {
    fn start(&mut self, stage: &str) {
        self.timers.insert(stage.to_string(), Instant::now());
    }
    
    fn end(&mut self, stage: &str) {
        if let Some(start) = self.timers.remove(stage) {
            let duration = start.elapsed();
            let memory = get_memory_usage();
            
            self.measurements.push(Measurement {
                stage: stage.to_string(),
                duration,
                memory_delta: memory as i64 - self.last_memory as i64,
            });
        }
    }
    
    fn report(&self) -> PerformanceReport {
        let mut stage_totals: HashMap<String, (Duration, i64)> = HashMap::new();
        
        for m in &self.measurements {
            let entry = stage_totals.entry(m.stage.clone()).or_insert(
                (Duration::ZERO, 0)
            );
            entry.0 += m.duration;
            entry.1 += m.memory_delta;
        }
        
        PerformanceReport { stages: stage_totals }
    }
}
```

### Timing Instrumentation

```rust
#[macro_export]
macro_rules! timed {
    ($name:expr, $block:expr) => {{
        let _timer = lutufi::timing::start_timer($name);
        $block
    }};
}

// Usage
let result = timed!("inference", {
    engine.query(&variables, &evidence)
});
```

### Memory Tracking

```rust
struct MemoryTracker;

impl MemoryTracker {
    fn snapshot() -> MemorySnapshot {
        MemorySnapshot {
            heap_allocated: get_heap_size(),
            factor_storage: get_factor_storage_size(),
            message_storage: get_message_storage_size(),
            cache_size: get_cache_size(),
            timestamp: Instant::now(),
        }
    }
    
    fn check_leaks(before: &MemorySnapshot, after: &MemorySnapshot) -> Option<MemoryLeak> {
        let delta = after.heap_allocated - before.heap_allocated;
        if delta > 100_000_000 { // 100MB threshold
            Some(MemoryLeak {
                size: delta,
                suspected_locations: analyze_allocation_stack(),
            })
        } else {
            None
        }
    }
}
```

---

## Data Flow Diagrams

### Variable Elimination Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    VARIABLE ELIMINATION FLOW DIAGRAM                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│    Input: Factors {f₁, f₂, ..., fₙ}, Elimination Order [X₁, X₂, ..., Xₖ]    │
│                              │                                              │
│                              ▼                                              │
│    ┌────────────────────────────────────────────────────────────────┐      │
│    │ Initialize: active_factors = {f₁, f₂, ..., fₙ}                  │      │
│    └────────────────────────────────────────────────────────────────┘      │
│                              │                                              │
│                              ▼                                              │
│         ┌───────────────────────────────────────┐                          │
│    ┌────│    For each variable Xᵢ in order:     │                          │
│    │    └───────────────────────────────────────┘                          │
│    │                     │                                                   │
│    │                     ▼                                                   │
│    │    ┌────────────────────────────────────────────────────────────────┐  │
│    │    │ 1. Gather: factors_i = {f ∈ active_factors : Xᵢ ∈ scope(f)}   │  │
│    │    └────────────────────────────────────────────────────────────────┘  │
│    │                     │                                                   │
│    │                     ▼                                                   │
│    │    ┌────────────────────────────────────────────────────────────────┐  │
│    │    │ 2. Multiply: product = ∏_{f ∈ factors_i} f                     │  │
│    │    └────────────────────────────────────────────────────────────────┘  │
│    │                     │                                                   │
│    │                     ▼                                                   │
│    │    ┌────────────────────────────────────────────────────────────────┐  │
│    │    │ 3. Marginalize: new_factor = Σ_{Xᵢ} product                    │  │
│    │    └────────────────────────────────────────────────────────────────┘  │
│    │                     │                                                   │
│    │                     ▼                                                   │
│    │    ┌────────────────────────────────────────────────────────────────┐  │
│    │    │ 4. Update: active_factors = (active_factors - factors_i) ∪    │  │
│    │    │              {new_factor}                                       │  │
│    │    └────────────────────────────────────────────────────────────────┘  │
│    │                     │                                                   │
│    └─────────────────────┘ (next variable)                                  │
│                              │                                              │
│                              ▼                                              │
│    ┌────────────────────────────────────────────────────────────────┐      │
│    │ Output: Result = ∏_{f ∈ active_factors} f                       │      │
│    └────────────────────────────────────────────────────────────────┘      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Belief Propagation Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     BELIEF PROPAGATION FLOW DIAGRAM                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│    Input: Factor Graph G = (V, F, E)                                        │
│                              │                                              │
│                              ▼                                              │
│    ┌────────────────────────────────────────────────────────────────┐      │
│    │ Initialize all messages to uniform (or prior)                   │      │
│    └────────────────────────────────────────────────────────────────┘      │
│                              │                                              │
│                              ▼                                              │
│         ┌───────────────────────────────────────────┐                       │
│    ┌────│        For iteration t = 1 to T:          │                       │
│    │    └───────────────────────────────────────────┘                       │
│    │                     │                                                   │
│    │    ┌────────────────┴────────────────┐                                 │
│    │    ▼                                 ▼                                 │
│    │ ┌──────────────┐               ┌──────────────┐                        │
│    │ │ Variable →   │               │ Factor →     │                        │
│    │ │ Factor Msgs  │               │ Variable Msgs│                        │
│    │ └──────────────┘               └──────────────┘                        │
│    │    │                                 │                                  │
│    │    ▼                                 ▼                                  │
│    │ ┌─────────────────────────────────────────┐                             │
│    │ │ For each edge e = (v, f):             │                             │
│    │ │   μᵥ→f(Xᵥ) = ∏_{f' ∈ N(v)\{f}} μ_f'→ᵥ │                             │
│    │ └─────────────────────────────────────────┘                             │
│    │    │                                 │                                  │
│    │    ▼                                 ▼                                  │
│    │ ┌─────────────────────────────────────────┐                             │
│    │ │ For each edge e = (f, v):             │                             │
│    │ │   μ_f→ᵥ(Xᵥ) = Σ_{X_f\{v}} f(X_f) ×   │                             │
│    │ │              ∏_{v' ∈ N(f)\{v}} μᵥ'→f  │                             │
│    │ └─────────────────────────────────────────┘                             │
│    │                     │                                                   │
│    │    ┌────────────────────────────────────────────────────────────────┐  │
│    │    │ Compute change = max |μ⁽ᵗ⁾ - μ⁽ᵗ⁻¹⁾|                          │  │
│    │    └────────────────────────────────────────────────────────────────┘  │
│    │                     │                                                   │
│    │    ┌────────────────────────────────────────────────────────────────┐  │
│    │    │ If change < ε: converged, break                                │  │
│    │    └────────────────────────────────────────────────────────────────┘  │
│    │                     │                                                   │
│    └─────────────────────┘ (next iteration)                                 │
│                              │                                              │
│                              ▼                                              │
│    ┌────────────────────────────────────────────────────────────────┐      │
│    │ Compute beliefs: b(Xᵥ) ∝ ∏_{f ∈ N(v)} μ_f→ᵥ(Xᵥ)                 │      │
│    └────────────────────────────────────────────────────────────────┘      │
│                              │                                              │
│                              ▼                                              │
│    Output: Beliefs {b(Xᵥ) : v ∈ V}                                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Learning Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      LEARNING DATA FLOW DIAGRAM                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│    Input: Training Data D = {(x₁, y₁), (x₂, y₂), ..., (xₙ, yₙ)}             │
│                              │                                              │
│                              ▼                                              │
│    ┌────────────────────────────────────────────────────────────────┐      │
│    │ Preprocess: Handle missing, encode categoricals, normalize      │      │
│    └────────────────────────────────────────────────────────────────┘      │
│                              │                                              │
│              ┌───────────────┴───────────────┐                              │
│              ▼                               ▼                              │
│    ┌──────────────────┐          ┌──────────────────┐                       │
│    │ Structure Learn  │          │ Parameter Learn  │                       │
│    └──────────────────┘          └──────────────────┘                       │
│              │                               │                              │
│              ▼                               ▼                              │
│    ┌──────────────────┐          ┌──────────────────┐                       │
│    │ Score-Based:     │          │ Count Statistics │                       │
│    │ • Search space   │          │ • Sufficient     │                       │
│    │ • Score models   │          │   stats          │                       │
│    │ • Optimize       │          │ • Handle missing │                       │
│    └──────────────────┘          └──────────────────┘                       │
│              │                               │                              │
│              ▼                               ▼                              │
│    ┌──────────────────┐          ┌──────────────────┐                       │
│    │ Constraint-Based:│          │ Estimate Params  │                       │
│    │ • Independence   │          │ • MLE / Bayesian │                       │
│    │   tests          │          │ • Regularization │                       │
│    │ • Orient edges   │          └──────────────────┘                       │
│    └──────────────────┘                      │                              │
│              │                               │                              │
│              └───────────────┬───────────────┘                              │
│                              ▼                                              │
│    ┌────────────────────────────────────────────────────────────────┐      │
│    │ Validate: Cross-validate, check overfitting, test on holdout    │      │
│    └────────────────────────────────────────────────────────────────┘      │
│                              │                                              │
│                              ▼                                              │
│    Output: Learned Model                                                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Security Considerations

### Input Validation

```rust
struct InputValidator;

impl InputValidator {
    fn validate_file_size(path: &Path, max_size: usize) -> Result<(), SecurityError> {
        let metadata = std::fs::metadata(path)?;
        if metadata.len() > max_size as u64 {
            return Err(SecurityError::FileTooLarge {
                path: path.to_path_buf(),
                size: metadata.len(),
                max: max_size,
            });
        }
        Ok(())
    }
    
    fn validate_recursion_depth<T>(data: &T, max_depth: usize) -> Result<(), SecurityError> {
        let depth = compute_nesting_depth(data);
        if depth > max_depth {
            return Err(SecurityError::ExcessiveNesting {
                depth,
                max: max_depth,
            });
        }
        Ok(())
    }
    
    fn validate_variable_count(n: usize, max: usize) -> Result<(), SecurityError> {
        if n > max {
            return Err(SecurityError::TooManyVariables { count: n, max });
        }
        Ok(())
    }
}
```

### Safe Deserialization

```rust
/// Safe deserialization with resource limits
struct SafeDeserializer {
    max_variables: usize,
    max_factors: usize,
    max_table_size: usize,
    max_recursion: usize,
}

impl SafeDeserializer {
    fn deserialize(&self, bytes: &[u8]) -> Result<FactorGraph, DeserializationError> {
        // Check header
        let header = self.parse_header(bytes)?;
        
        // Validate counts
        if header.n_variables > self.max_variables {
            return Err(DeserializationError::TooManyVariables);
        }
        if header.n_factors > self.max_factors {
            return Err(DeserializationError::TooManyFactors);
        }
        
        // Deserialize with limits
        let mut parser = Parser::with_limits(
            &bytes[HEADER_SIZE..],
            ParseLimits {
                max_depth: self.max_recursion,
                max_string_len: 1024,
            }
        );
        
        parser.parse_factor_graph()
    }
}
```

### DoS Prevention

```rust
struct ResourceLimiter {
    max_memory_mb: usize,
    max_compute_time_secs: u64,
    max_iterations: usize,
}

impl ResourceLimiter {
    fn check_inference_limits(&self, graph: &FactorGraph) -> Result<(), LimitError> {
        // Estimate memory usage
        let estimated_memory = self.estimate_memory_usage(graph);
        if estimated_memory > self.max_memory_mb * 1024 * 1024 {
            return Err(LimitError::WouldExceedMemory {
                estimated: estimated_memory,
                limit: self.max_memory_mb * 1024 * 1024,
            });
        }
        
        // Check treewidth for exact inference
        let treewidth = estimate_treewidth(graph);
        if treewidth > 50 {
            return Err(LimitError::IntractableTreewidth { treewidth });
        }
        
        Ok(())
    }
    
    fn wrap_inference<F, T>(&self, f: F) -> Result<T, LimitError>
    where F: FnOnce() -> T
    {
        let start = Instant::now();
        
        let result = std::panic::catch_unwind(|| {
            // Set up timeout
            let timeout = Duration::from_secs(self.max_compute_time_secs);
            
            crossbeam::scope(|s| {
                let handle = s.spawn(|_| f());
                
                match handle.join_timeout(timeout) {
                    Ok(result) => Ok(result),
                    Err(_) => Err(LimitError::Timeout),
                }
            }).unwrap()
        });
        
        match result {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(LimitError::Panic),
        }
    }
}
```

---

## Key References

### Data Flow Architectures

1. **Hohpe, G., & Woolf, B. (2003).** *Enterprise Integration Patterns: Designing, Building, and Deploying Messaging Solutions.* Addison-Wesley.
   - Pipeline and message routing patterns

2. **Fowler, M. (2002).** *Patterns of Enterprise Application Architecture.* Addison-Wesley.
   - Data flow and transformation patterns

### Pipeline Patterns

3. **Kleppmann, M. (2017).** *Designing Data-Intensive Applications.* O'Reilly Media.
   - Stream processing and data pipelines

4. **Akidau, T., et al. (2015).** "The Dataflow Model: A Practical Approach to Balancing Correctness, Latency, and Cost in Massive-Scale, Unbounded, Out-of-Order Data Processing." *VLDB 2015.*
   - Unified batch and streaming processing

### Stream Processing

5. **Zaharia, M., et al. (2013).** "Discretized Streams: Fault-Tolerant Streaming Computation at Scale." *SOSP 2013.*
   - Spark Streaming architecture

6. **Carbone, P., et al. (2015).** "Apache Flink: Stream and Batch Processing in a Single Engine." *IEEE Data Engineering Bulletin.*
   - Modern stream processing

### Caching Strategies

7. **Fedotova, N., & Valtchev, P. (2010)." "Measuring the Performance of Cache Strategies." *International Journal on Advances in Software.*
   - Cache replacement policies

8. **Nishtala, R., et al. (2013).** "Scaling Memcache at Facebook." *NSDI 2013.*
   - Large-scale caching systems

### Error Handling

9. **Marasco, J. (2011).** "The Defensive Programming Paradox." *IEEE Software.*
   - Robust error handling strategies

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete data flow document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
