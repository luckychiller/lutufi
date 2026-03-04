# Lutufi Visualization Design Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Introduction](#introduction)
2. [Visualization Philosophy](#visualization-philosophy)
3. [2D Network Visualization](#2d-network-visualization)
4. [3D Network Visualization](#3d-network-visualization)
5. [Probabilistic and Uncertainty Visualization](#probabilistic-and-uncertainty-visualization)
6. [Temporal and Dynamic Visualization](#temporal-and-dynamic-visualization)
7. [Causal Visualization](#causal-visualization)
8. [Interactive Features](#interactive-features)
9. [Export and Publishing](#export-and-publishing)
10. [Technology Stack](#technology-stack)
11. [Integration with Analysis Workflow](#integration-with-analysis-workflow)
12. [Large Network Strategies](#large-network-strategies)
13. [Special Visualizations](#special-visualizations)
14. [Design System and Style Guide](#design-system-and-style-guide)
15. [Accessibility](#accessibility)
16. [Implementation Roadmap](#implementation-roadmap)
17. [Key References](#key-references)

---

## Introduction

### Why Visualization Matters for Probabilistic Network Analysis

Visualization serves as the primary interface between complex probabilistic models and human understanding. In the context of Lutufi, where Bayesian networks intersect with social and economic network analysis, effective visualization is not merely a presentation layer—it is a fundamental tool for model validation, hypothesis generation, and communication of results.

Network structures encode intricate dependency relationships that are difficult to comprehend from tabular data alone. A Bayesian network with 50 nodes may contain hundreds of conditional dependencies, each representing a quantitative relationship between variables. Without visualization, identifying patterns such as v-structures, colliders, or causal chains becomes an exercise in combinatorial enumeration. Visualization externalizes these structures, allowing analysts to leverage human visual cognition for pattern recognition.

Probabilistic inference produces rich, multidimensional outputs. Marginal distributions, conditional dependencies, and uncertainty quantification must be conveyed in ways that support decision-making. Research in judgment and decision-making demonstrates that the format of probability presentation significantly affects interpretation accuracy. Well-designed visualizations can reduce cognitive biases in probabilistic reasoning and improve calibration between stated confidence and actual accuracy.

### The Cognitive Role of Visualization in Understanding Uncertainty

Human cognition exhibits systematic biases when processing probabilistic information. The representativeness heuristic leads to neglect of base rates; the conjunction fallacy causes overestimation of joint probabilities; and overconfidence bias distorts subjective uncertainty estimates. Visualization design must account for these cognitive limitations and leverage perceptual capabilities that transcend them.

Visual encoding of uncertainty through opacity, saturation, or error bars provides direct perceptual access to confidence levels. Research by Hullman, Kay, and others demonstrates that frequency-based visualizations (such as icon arrays) improve Bayesian reasoning compared to probability statements alone. Lutufi's visualization system incorporates these findings by providing multiple representation formats suited to different reasoning tasks.

The cognitive fit theory suggests that performance improves when the representation matches the task requirements. For structure understanding, node-link diagrams excel. For probability comparison, position encoding on a common scale outperforms angle or area encodings. For temporal evolution, animation or small multiples facilitate change detection. Lutufi's visualization API provides task-appropriate representations as defaults while allowing user customization.

### How Visualization Differs for Probabilistic vs Deterministic Networks

Deterministic network visualization (as in traditional graph theory or social network analysis) represents relationships as binary present/absent connections. Edge existence is certain; the visualization problem concerns layout, aesthetics, and readability. Probabilistic network visualization must encode additional dimensions: relationship strength, directionality of influence, conditional dependencies that vary with evidence, and uncertainty about structure itself.

In deterministic networks, node color typically encodes a single categorical attribute (e.g., community membership). In probabilistic networks, nodes carry entire probability distributions that must be visualized—potentially as bar charts, pie segments, or density plots embedded within the node glyph. Edge visualization similarly expands from single lines to encodings that represent conditional probability tables, influence magnitudes, or posterior probabilities of edge existence in structure learning contexts.

The dynamic nature of probabilistic inference creates additional visualization requirements. When evidence propagates through a network, marginal distributions update throughout the graph. Effective visualization must support comparison between prior and posterior beliefs, identification of evidence propagation paths, and understanding of how local observations affect global beliefs.

### Integration with Analysis Workflow

Visualization in Lutufi is designed as an integral component of the analysis workflow, not an afterthought for final presentation. Throughout the model development process—from exploratory data analysis through structure learning, parameter estimation, inference, and validation—appropriate visualizations guide decision-making and reveal model behavior.

**Exploratory Phase:** Initial data visualization reveals variable distributions, correlations, and potential structural relationships. Structure learning visualization shows the search process, score landscapes, and algorithm convergence. These visualizations inform decisions about algorithm selection, constraint specification, and model complexity.

**Model Construction Phase:** As the network structure is built or learned, real-time visualization provides immediate feedback on graph properties. Visual validation identifies cycles (invalid for Bayesian networks), disconnected components, and unexpected structural patterns. CPD visualization enables verification that probability tables match domain knowledge.

**Inference Phase:** Query results are inherently visual. Marginal distributions, conditional dependencies, and evidence propagation benefit from graphical representation. Interactive visualization allows analysts to explore "what-if" scenarios by adjusting evidence and observing belief updates.

**Validation Phase:** Model comparison, sensitivity analysis, and predictive performance all rely on visualization. Residual analysis, calibration plots, and structure comparison views support rigorous model criticism.

---

## Visualization Philosophy

### Visual Encoding of Uncertainty

Uncertainty visualization represents a core challenge in probabilistic network analysis. Lutufi adopts a multi-channel approach to uncertainty encoding, allowing redundant representation through multiple visual channels to support viewers with different perceptual capabilities and task requirements.

**Opacity Encoding:** Certainty maps to fully opaque visual elements; uncertainty introduces transparency. A node with a sharply peaked marginal distribution appears opaque, while a node with high entropy appears increasingly transparent. This encoding naturally suggests the solidity of knowledge—uncertain elements fade into the background. Implementation uses alpha blending with perceptually uniform scaling to maintain discriminability across the uncertainty range.

**Color Saturation:** Within a given hue, saturation represents uncertainty. High certainty uses fully saturated colors; uncertainty desaturates toward gray. This encoding works synergistically with opacity—together they create strong perceptual cues for confidence levels. The saturation channel is particularly effective for categorical variables where hue already encodes state.

**Edge Thickness:** For probabilistic edges, thickness encodes relationship strength or posterior probability of edge existence. Strong dependencies appear as thick, prominent edges; weak or uncertain relationships appear thin. This encoding aligns with Gestalt principles of proximity and connectedness—thicker edges appear more significant.

**Glyph Complexity:** Nodes incorporate embedded glyphs representing their probability distributions. Discrete variables display as miniature bar charts or pie charts within the node boundary. Continuous variables show density plots or box plots. The glyph size scales with node size while maintaining readability constraints.

**Error Bars and Confidence Regions:** For inferred quantities, error bars, confidence bands, and shaded regions represent uncertainty ranges. In causal inference visualizations, treatment effect estimates include confidence intervals. In structure learning, edge existence probabilities may be represented through graduated shading or uncertainty bands around edges.

### Layered Visualization Approach

Lutufi organizes network visualization into conceptual layers that can be independently controlled, enabling users to manage visual complexity while accessing rich information:

**Structure Layer:** The foundational layer showing nodes and edges as geometric elements. This layer answers "what is connected to what" and provides the spatial framework for all other information. Structure visualization includes layout algorithms, node positioning, and basic edge routing. Users can toggle structure visibility, adjust layout parameters, or freeze layouts after initial computation.

**Probability Layer:** Superimposed on the structure layer, this layer encodes probabilistic beliefs. Node colors show marginal distributions; edge colors encode conditional dependencies; embedded glyphs display full distributions. The probability layer updates dynamically as evidence changes or inference proceeds. Transparency in this layer enables focus on high-confidence regions.

**Dynamics Layer:** For temporal networks and simulation outputs, the dynamics layer shows evolution over time. This includes node state trajectories, flow visualization along edges, and animation of belief propagation. The dynamics layer can be rendered as animation, small multiples, or trajectories overlaid on the structure.

**Uncertainty Layer:** Explicit representation of confidence, variance, and model uncertainty. This includes error bars on estimates, confidence regions around predictions, and visual indicators of structural uncertainty. The uncertainty layer is essential for honest communication of model limitations.

**Annotation Layer:** User annotations, automated labels, tooltips, and contextual information. This layer provides semantic context without cluttering the core visualization. Annotations include node labels, edge labels, evidence indicators, and query results.

Layer composition follows painter's algorithm with configurable blending modes. Users control layer visibility, opacity, and rendering order through the visualization API or interactive controls.

### Interactive vs Static Visualization Tradeoffs

Lutufi supports both interactive and static visualization modes, recognizing that different contexts demand different approaches:

**Interactive Visualization Advantages:**
- Exploration of large state spaces through pan, zoom, and filtering
- Drill-down from overview to detail on demand
- Dynamic query construction through selection and brushing
- Real-time feedback during model development
- Personalized views for different analysis questions

**Interactive Visualization Costs:**
- Requires runtime environment (Jupyter, web browser, application)
- Higher computational overhead for interactivity
- Potential for user disorientation in complex navigation spaces
- Accessibility challenges for certain user populations

**Static Visualization Advantages:**
- Archival stability and reproducibility
- Publication compatibility (papers, reports, presentations)
- Performance efficiency for large-scale batch generation
- Universal accessibility (no technology requirements)
- Compositional control for narrative presentation

**Static Visualization Limitations:**
- Fixed viewpoint and level of detail
- Cannot encode arbitrary large datasets
- Limited comparison capability without small multiples
- No dynamic exploration of alternative scenarios

Lutufi's design provides seamless transitions between modes. Interactive visualizations can be captured as publication-ready static images. Static specifications can be "activated" into interactive versions. The underlying visualization grammar is consistent across modes.

### Accessibility Considerations

Accessibility in visualization extends beyond compliance to encompass inclusive design that enables diverse users to extract insight from probabilistic network representations.

**Color Vision Deficiency:** Approximately 8% of males and 0.5% of females have some form of color vision deficiency. Lutufi's default color palettes are designed to be perceptually distinguishable under protanopia, deuteranopia, and tritanopia simulations. Redundant encoding—using shape, pattern, or texture alongside color—ensures that critical information is not color-dependent alone.

**Screen Reader Compatibility:** For visually impaired users accessing visualizations through screen readers, Lutufi provides:
- Structured data tables underlying all visualizations
- Alt text generation describing network topology and key findings
- Sonification options mapping network properties to auditory cues
- Keyboard navigation of interactive visualizations

**Motor Accessibility:** Interactive visualizations support keyboard-only operation with visible focus indicators. Click targets meet minimum size requirements. Drag-and-drop operations have keyboard-accessible alternatives.

**Cognitive Accessibility:** Visualizations avoid unnecessary complexity. Progressive disclosure reveals detail on demand. Consistent visual encoding across the library reduces learning burden. Plain language descriptions accompany technical visualizations.

### Scientific Accuracy vs Aesthetic Appeal

The tension between scientific accuracy and aesthetic appeal is resolved through prioritization: scientific accuracy is mandatory; aesthetic appeal supports engagement and communication but never compromises correctness.

**Accuracy Requirements:**
- Probability representations must sum to unity
- Conditional dependencies must accurately reflect model structure
- Uncertainty ranges must correspond to valid confidence levels
- Layouts must preserve meaningful structural properties (hierarchy, clusters)

**Aesthetic Enhancements:**
- Subtle shadows and depth cues improve depth perception without distorting data
- Smooth animations facilitate tracking but can be disabled
- Refined typography improves readability
- Thoughtful color harmonies reduce visual fatigue

**Anti-Patterns Avoided:**
- 3D perspective distortions that distort quantitative comparisons
- Excessive chart junk that obscures data
- Gradient backgrounds that interfere with color encoding
- Decorative elements that imply false precision

---

## 2D Network Visualization

### Layout Algorithms

The arrangement of nodes in 2D space fundamentally affects the interpretability of network visualizations. Lutufi provides multiple layout algorithms selected based on network properties and analysis goals.

#### Force-Directed Layouts

Force-directed layouts model the network as a physical system where nodes repel each other while edges act as springs attracting connected nodes. The system evolves toward an energy minimum, producing layouts that reflect network topology through spatial proximity.

**Fruchterman-Reingold Algorithm:** This classical approach uses repulsive forces between all node pairs and attractive forces along edges. The temperature parameter controls the maximum displacement per iteration, typically decreasing over time for convergence. Complexity is O(n²) per iteration, making it suitable for networks up to approximately 1,000 nodes. The algorithm produces layouts that emphasize clusters and naturally separate disconnected components.

**ForceAtlas2:** An enhanced force-directed algorithm optimized for visualization quality. Features include:
- Linear attraction (prevents central nodes from clustering too tightly)
- Gravity (prevents disconnected components from drifting apart)
- Scaling (adapts to network size)
- Preventing overlap (optional node repulsion)
- Lin-log mode (emphasizes communities)

ForceAtlas2 excels at revealing community structure and is the default for exploratory visualization of social networks.

**Implementation Considerations:** Force-directed layouts are stochastic—different random seeds produce different layouts. For reproducibility, Lutufi exposes the random state parameter. Multiple layout runs can be compared using quality metrics to select the best result. Layout animation during optimization can help users understand the convergence process.

#### Hierarchical Layouts

Hierarchical layouts position nodes according to a partial order, typically reflecting causal or temporal precedence. These layouts are essential for Bayesian networks where the directed acyclic graph structure implies a generative order.

**Sugiyama Framework:** The standard approach for layered graph drawing operates in four phases:
1. **Cycle removal:** Reverse edges to create a DAG (preserving logical structure through notation)
2. **Layer assignment:** Assign nodes to horizontal layers respecting edge direction
3. **Crossing reduction:** Order nodes within layers to minimize edge crossings
4. **Coordinate assignment:** Determine exact positions to minimize edge lengths and bends

The Sugiyama layout clearly shows the flow of influence from root causes to downstream effects, making it ideal for causal Bayesian networks.

**Reingold-Tilford Tree Layout:** Optimized for tree structures but applicable to DAGs through expansion. The algorithm recursively assigns positions to subtrees, ensuring that parent nodes are centered over their children. The result is compact, symmetrical layouts that reveal hierarchical patterns. Extensions support variable node sizes and edge orientations.

**Constraint Integration:** Hierarchical layouts can incorporate additional constraints:
- Fixed node positions (for maintaining user adjustments)
- Minimum layer separation (for readability)
- Alignment constraints (grouping related nodes vertically)
- Rank constraints (preserving known orderings)

#### Circular and Radial Layouts

Circular arrangements position nodes on a circumference with edges drawn as chords or arcs. These layouts emphasize cyclical relationships and provide uniform treatment of all nodes.

**Simple Circle Layout:** Nodes are placed at equal angular intervals around a circle. Edge length encodes relationship strength in the original graph—shorter chords indicate stronger connections. This layout is particularly effective for dense networks where the circular arrangement prevents the "hairball" effect of force-directed layouts.

**Radial Layout:** Root nodes are placed at the center with other nodes arranged in concentric circles according to graph distance (shortest path length) from the root. This layout emphasizes reachability and path structure. Multiple roots can be supported through multi-focal radial layouts.

**Arc Diagram:** Nodes are positioned linearly along one axis with edges drawn as semicircular arcs above or below. This compact representation suits networks with clear node ordering (e.g., temporal or genomic sequences).

#### Constraint-Based Layouts

User knowledge often includes spatial constraints that should be respected in visualization:

**Fixed Position Constraints:** Domain knowledge may dictate that certain nodes appear at specific locations (e.g., geographic coordinates for spatial networks, anatomical positions for biological networks). Lutufi supports fixed position constraints that pin nodes while allowing the layout algorithm to position remaining nodes optimally.

**Relative Position Constraints:** Constraints that nodes A and B should appear in a particular relative position (above, left of, etc.) without specifying exact coordinates. These guide the layout while maintaining flexibility.

**Cluster Constraints:** Requirements that certain nodes appear in proximity, implemented through additional attractive forces or cluster-specific layout stages.

#### Layout for Temporal Networks

Temporal networks evolve over time, requiring layouts that balance readability at each timestep with temporal stability.

**Static Small Multiples:** Each time slice rendered with the same layout algorithm, potentially with different results per slice. Comparison between slices requires mental alignment; this approach works best when network structure is stable.

**Anchored Layouts:** Compute a reference layout (e.g., aggregate network or first time slice) and use it as initial positions for subsequent slices. Force-directed optimization adjusts positions while the initialization maintains stability.

**Dynamic Layouts:** Layout algorithms that explicitly optimize for temporal stability. The objective function includes terms penalizing node movement between consecutive time slices, weighted by a stability parameter that trades off against layout quality.

#### Layout Quality Metrics and Adaptive Selection

Different networks benefit from different layout algorithms. Lutufi provides quality metrics for layout evaluation and adaptive selection:

**Stress:** Measures the difference between graph-theoretic distances (shortest path lengths) and Euclidean distances in the layout. Lower stress indicates that spatial proximity accurately reflects network proximity.

**Crossing Number:** Counts edge crossings. Fewer crossings improve readability, though some crossings are unavoidable in non-planar graphs.

**Angular Resolution:** Measures the minimum angle between edges incident to a node. Higher angular resolution improves edge distinguishability.

**Aspect Ratio:** The ratio of layout width to height. Extreme aspect ratios waste space or require excessive scrolling.

**Adaptive Selection:** Based on network properties (size, density, clustering coefficient, presence of hierarchy), Lutufi can recommend appropriate layout algorithms. Users retain full control but benefit from sensible defaults.

### Node Visualization

Nodes represent random variables in probabilistic networks. Effective node visualization must encode variable identity, type, current beliefs, and uncertainty.

#### Node Size Encoding

Node area (not diameter) encodes quantitative attributes. Linear area scaling ensures accurate magnitude comparison.

**Centrality Measures:** Node size can represent betweenness centrality (influence over information flow), degree centrality (number of connections), or eigenvector centrality (influence based on connection to influential nodes). In probabilistic networks, size may represent expected influence on query variables given evidence.

**Marginal Probability:** For binary variables, node size encodes the probability of the "positive" state. Multi-state variables use size to encode the maximum probability (certainty of the most likely state) or entropy (uncertainty).

**Constraints:** Minimum and maximum size limits prevent illegibility and dominance. Size legends explain the encoding.

#### Node Color Encoding

Color represents categorical or quantitative node attributes. Lutufi uses perceptually uniform color spaces (CIELAB, HCL) to ensure that equal data differences appear as equal color differences.

**Discrete States:** Categorical variables use distinct hues. The number of discriminable hues limits the palette—beyond 8-12 categories, additional encoding channels (shape, pattern) are necessary.

**Continuous Values:** Quantitative variables use sequential color scales. Sequential scales vary in lightness while maintaining constant hue, supporting accurate magnitude estimation. Diverging scales (two hues meeting at a neutral midpoint) suit signed values or deviations from a reference.

**Uncertainty:** Uncertainty can modulate color saturation or opacity. Alternatively, a separate uncertainty color scale (e.g., blue for certain, red for uncertain) provides explicit encoding.

#### Node Shape Encoding

Shape distinguishes node types or categories. The limited discriminability of shapes (typically 6-8 readily distinguished) constrains this encoding to high-level categorization.

**Standard Shapes:**
- Circles: Default, general-purpose nodes
- Squares: Observed/evidence nodes
- Diamonds: Query/target nodes
- Triangles: Latent/hidden variables
- Hexagons: Decision variables (influence diagrams)

**Custom Shapes:** Domain-specific shapes (e.g., gene icons for biological networks, person silhouettes for social networks) can be registered through the visualization API.

#### Node Labels and Tooltips

Node identity requires textual labels. Label placement strategies balance readability against occlusion:

**Internal Labels:** For large nodes, labels appear inside the node boundary. Font size scales with node size down to a minimum readability threshold.

**External Labels:** For small nodes, labels appear adjacent to nodes with leader lines connecting them. Force-directed label placement separates labels to prevent overlap.

**Adaptive Labeling:** Label visibility adapts to zoom level—only prominent nodes are labeled at overview zoom, with additional labels appearing as users zoom in.

**Tooltips:** Hover interactions reveal full node information: variable name, states, current marginal distribution, evidence status, and statistics (entropy, mutual information with evidence).

#### Node Clustering and Aggregation

Large networks require aggregation to maintain readability:

**Community Aggregation:** Nodes belonging to the same community (detected through modularity optimization or other clustering) can be aggregated into meta-nodes. The meta-node represents the community's collective state and internal connectivity.

**Hierarchical Aggregation:** Networks with natural hierarchy (e.g., organizational structures) can be visualized at multiple levels of detail. Collapsing internal structure shows parent-child relationships without visual clutter.

**Proximity Clustering:** Spatial proximity in the layout triggers aggregation at low zoom levels. As users zoom in, aggregated nodes separate into individual elements.

#### Belief Visualization on Nodes

The distinguishing feature of probabilistic network visualization is the representation of beliefs directly on nodes:

**Probability Bars:** Miniature bar charts embedded within nodes show the full marginal distribution. Bar height encodes probability; bar width scales with node size. Color distinguishes states.

**Pie Charts:** Radial segments represent state probabilities. Pie charts work well for variables with few states (2-4). Beyond 4 states, wedges become difficult to compare.

**Glyph-Based:** Custom glyphs encode distribution properties. A "thermometer" glyph shows the probability of a target state. A "sparkline" glyph shows the full distribution shape.

**Uncertainty Rings:** Concentric rings around nodes encode uncertainty metrics. Ring completeness represents certainty; incomplete rings indicate partial knowledge.

### Edge Visualization

Edges represent dependencies between variables. In probabilistic networks, edges carry rich quantitative information about the nature and strength of these dependencies.

#### Edge Thickness

Edge thickness (stroke width) encodes relationship strength:

**Conditional Probability Magnitude:** The maximum change in child node distribution induced by changing the parent. Strong influences appear as thick edges; weak influences as thin.

**Mutual Information:** The information-theoretic measure of dependency strength. Edges with high mutual information appear prominent.

**Structure Learning Confidence:** When edges are learned from data, thickness represents the posterior probability of edge existence or the strength of statistical dependence.

#### Edge Color

Edge color encodes multiple attributes through hue, saturation, and value:

**Direction:** Standard color schemes distinguish edge direction. Blue-to-red gradients along edge length indicate directionality. Alternatively, directed edges use arrowheads with color indicating influence type.

**Edge Type:** Different hues distinguish deterministic from probabilistic relationships, causal from correlational edges, or positive from negative associations.

**Probability/Certainty:** Saturation encodes edge confidence. High-confidence edges are fully saturated; uncertain edges (from structure learning with limited data) appear desaturated.

#### Edge Style

Line style (solid, dashed, dotted) encodes categorical edge attributes:

**Certainty:** Solid lines for certain edges, dashed for uncertain (e.g., edges present in some but not all bootstrap samples during structure learning).

**Directionality:** Double-headed arrows for undirected edges (Markov Random Fields), single-headed for directed (Bayesian Networks).

**Existence Probability:** Dashed edges represent possible but unconfirmed relationships. Dash density correlates with existence probability.

#### Curved Edges and Bundling

Straight edges in dense networks create occlusion and make individual edges difficult to follow:

**Curved Edges:** Bezier curves or circular arcs reduce edge-edge crossings and improve aesthetics. Control point placement minimizes edge-node overlap while maintaining clear direction indication.

**Edge Bundling:** Edges with similar paths are grouped into bundles, reducing visual clutter. Bundling algorithms identify compatible edges based on graph distance or spatial proximity. Bundling strength is user-controllable.

**Force-Directed Edge Routing:** Edges route around nodes using force-directed methods that repel edges from node interiors while maintaining start and end points.

#### Edge Labels and Tooltips

Edges can be labeled with quantitative attributes:

**Conditional Probability Tables:** For discrete variables, edge labels show the CPD in compact form (e.g., "0.9 | 0.1" for a binary child). Clicking expands to the full table.

**Influence Metrics:** Labels display mutual information, correlation coefficients, or causal effect sizes.

**Structure Learning Statistics:** Labels show p-values, confidence scores, or edge frequencies across bootstrap samples.

**Tooltips:** Hover reveals complete edge information: parent and child variables, CPD summary, influence metrics, and learning statistics.

#### Multi-Edge Visualization

Multiplex networks contain multiple edge types between the same node pairs:

**Parallel Edges:** Multiple edges drawn with slight offsets, distinguished by color or style. Effective for 2-3 edge types; beyond this, visual clutter increases.

**Edge Stacking:** A single visual edge divided into segments representing different edge types. Each segment's thickness encodes the strength of that edge type.

**Temporal Animation:** For temporal multiplex networks, animation cycles through edge types or time slices.

**Interactive Selection:** Only one edge type is visible at a time, with user controls for switching between types.

### Layer and Subgraph Visualization

Complex analysis requires focusing on specific network regions while maintaining context.

#### Highlighting Specific Paths or Structures

**Path Highlighting:** User-selected paths (e.g., causal chains from intervention to outcome) are emphasized through increased opacity and thickness, with other network elements de-emphasized.

**d-Separation Visualization:** When testing conditional independence, paths that are blocked or unblocked given evidence are highlighted in different colors, explaining the d-separation determination.

**Active Trails:** During inference, edges participating in active trails (paths through which influence flows given current evidence) are highlighted, showing how information propagates.

#### Community Visualization

**Contours:** Convex hulls or smooth contours surround community members. Contour color matches community color; contour transparency indicates community coherence.

**Clustered Layouts:** Force-directed layouts with community-aware forces pull community members together while maintaining separation between communities.

**Matrix Views:** For dense communities, adjacency matrices ordered by community membership reveal block structure more clearly than node-link diagrams.

#### Layered Display for Multilayer Networks

Multilayer networks (multiple interaction types or time slices) require visualization that preserves layer identity:

**Vertical Stacking:** Layers arranged vertically with inter-layer edges crossing between layers. Each layer maintains its own layout, with inter-layer edges potentially bundled.

**Merged View:** All layers combined into a single view with edge color encoding layer membership. Effective when inter-layer edges are sparse.

**Small Multiples:** Each layer in a separate panel with aligned node positions across panels, enabling comparison of layer structure.

**3D Extrusion:** Layers positioned along the z-axis in 3D visualization (see Section 4).

#### Context+Focus Techniques

**Fisheye Distortion:** Magnifies a focal region while compressing surrounding regions. The distortion function maintains continuity—there are no abrupt boundaries between focused and contextual regions. Fisheye is effective for navigating large networks while maintaining global context.

**Hyperbolic Geometry:** Nodes are positioned in hyperbolic space and projected onto the 2D display. The exponential expansion of space in hyperbolic geometry naturally creates focus+context—zooming moves the viewport through hyperbolic space, bringing new regions into linear-scale view while compressing distant regions.

**Semantic Zooming:** As users zoom, the level of detail changes. At low zoom, communities are shown as aggregate nodes. At medium zoom, individual nodes appear with labels. At high zoom, full probability distributions and CPDs are visible.

---

## 3D Network Visualization

### When 3D Adds Value

Three-dimensional visualization introduces complexity and occlusion challenges that must be justified by analytical benefits. Lutufi employs 3D visualization selectively when the third dimension carries meaningful information.

#### Large Networks Where 2D Suffers from Occlusion

Networks with thousands of nodes create unavoidable occlusion in 2D. The third dimension provides additional space for node separation:

- **Volume advantage:** 3D space provides cubic scaling of available positions compared to quadratic in 2D
- **Perspective depth cues:** Nodes at different depths appear at different sizes and positions, enabling depth-based clustering
- **Navigation freedom:** Camera rotation around three axes reveals different structural aspects

The threshold for 3D consideration depends on network properties. Dense networks benefit more than sparse networks; networks with natural 3D structure benefit more than abstract graphs.

#### Temporal Networks (Time as Third Dimension)

When network evolution is the analytical focus, positioning time slices along the z-axis creates intuitive temporal visualization:

- **Temporal trajectories:** Individual nodes trace paths through time, showing movement between communities or roles
- **Causal ordering:** The third dimension enforces temporal precedence—earlier times are unambiguously "behind" later times
- **Period comparison:** Slicing through the temporal dimension reveals network structure at specific times

#### Multilayer Networks (Layers as Third Dimension)

Multilayer networks gain clarity when layers are separated in 3D:

- **Layer identity:** Each layer occupies a distinct z-position, preventing edge crossings that would occur in merged 2D layouts
- **Inter-layer edges:** Connections between layers appear as vertical or diagonal lines, clearly distinguished from intra-layer edges
- **Layer comparison:** Viewing along the z-axis projects all layers, revealing node presence across layers

#### Hierarchical Networks (Depth as Meaningful Dimension)

Hierarchical structures with many levels exceed the vertical space available in 2D hierarchical layouts. The third dimension enables deeper hierarchies:

- **Radial hierarchies:** Nodes arranged in concentric spheres rather than circles
- **Tree cones:** Hierarchical trees occupy conical volumes with root at apex and leaves at base
- **Level separation:** Each hierarchy level occupies a distinct z-plane, maintaining clear parent-child relationships

#### VR/AR Applications for Immersive Exploration

Virtual and augmented reality leverage 3D visualization for immersive network exploration:

- **Scale perception:** Stereoscopic depth perception enhances understanding of network structure
- **Natural interaction:** Hand gestures and head tracking provide intuitive navigation
- **Embodied cognition:** Physical movement through network space may improve spatial memory of network structure
- **Collaboration:** Shared virtual environments enable collaborative exploration

### 3D Layout Strategies

#### Force-Directed in 3D

Extension of 2D force-directed algorithms to three dimensions:

**3D Fruchterman-Reingold:** Repulsive forces act along all three axes; attractive forces along edges. The additional dimension reduces local minima and often produces better separation between communities.

**3D ForceAtlas2:** Incorporates 3D-specific optimizations including:
- 3D gravity toward center
- 3D overlap prevention
- Adaptive temperature in three dimensions

**Convergence considerations:** 3D force-directed layouts require more iterations for convergence due to increased degrees of freedom. Initial placement strategies (e.g., based on graph clustering) accelerate convergence.

#### Layer-Based 3D Layouts

**Time Layout:** Nodes are positioned in (x, y, t) space where t is time. The layout problem separates into:
1. Compute 2D layout for each time slice
2. Stack slices along time axis
3. Add temporal forces linking corresponding nodes across consecutive slices

Temporal forces can enforce:
- Temporal stability (nodes don't move abruptly)
- Trajectory smoothness (node paths are smooth curves)
- Event alignment (nodes involved in events are positioned to highlight the event)

**Layer Layout:** For multilayer networks, each layer occupies a z-position. Intra-layer edges use the layer's 2D layout; inter-layer edges connect across z-levels.

**Hierarchy Layout:** Tree layouts extend naturally to 3D:
- **Balloon layout:** Children arranged on spheres around parents
- **Cone tree:** Hierarchical cone with nodes at cone surface
- **Radial 3D:** Radial layouts extended with height encoding hierarchy depth

#### Spherical Layouts

Positioning nodes on a spherical surface provides several advantages:

- **No boundary effects:** Unlike planar layouts, spherical layouts have no edges—every position is equally central
- **Compact representation:** Spherical surface area is efficiently used
- **Geodesic edges:** Great circle paths provide natural edge routing

**Uniform Spherical Distribution:** Nodes distributed uniformly on the sphere surface maximize angular separation. This is achieved through repulsive forces constrained to the sphere surface or through deterministic sampling (e.g., Fibonacci sphere).

**Spherical Projection:** 3D positions are projected onto a sphere, then mapped to 2D using map projections (Mercator, equal-area). This enables spherical visualization on 2D displays.

#### Dimensionality Reduction for 3D Embedding

When network structure lacks natural 3D interpretation, dimensionality reduction can create meaningful 3D embeddings:

**t-SNE in 3D:** t-Distributed Stochastic Neighbor Embedding preserves local neighborhoods while revealing global structure. The 3D embedding often separates clusters more effectively than 2D.

**UMAP in 3D:** Uniform Manifold Approximation and Projection provides faster embedding than t-SNE with competitive quality. UMAP's preservation of both local and global structure suits hierarchical networks.

**Spectral Embedding:** Graph Laplacian eigenvectors provide 3D coordinates that minimize edge length while separating disconnected components.

### 3D Visualization Techniques

#### Node Glyphs in 3D

3D node glyphs provide additional encoding channels:

**Spheres:** The default 3D node glyph. Sphere radius encodes node size; sphere color encodes node attributes. Spheres are computationally efficient and have consistent appearance from all viewing angles.

**Cubes and Boxes:** Axis-aligned cubes can encode directionality or orientation. Non-axis-aligned cubes may cause confusion about precise position.

**Custom Meshes:** Domain-specific 3D models (e.g., molecular structures for biological networks, building models for infrastructure networks) enhance semantic interpretation. Mesh complexity affects rendering performance—LOD strategies are essential.

**Billboards:** 2D sprites oriented to face the camera provide efficient rendering for large networks. Textures encode node information; the 2D limitation is acceptable for distant nodes.

#### Edge Rendering in 3D

3D edges require depth cues for proper interpretation:

**Tubes:** Cylindrical tubes provide clear 3D presence with lighting and shading cues. Tube radius can encode edge weight. Tubes are more expensive to render than lines but provide better depth perception.

**Lines:** Simple line segments with depth cueing (darker/more opaque for closer edges). Lines are efficient but can be difficult to trace through 3D space.

**Ribbons:** Flat ribbons oriented to face the camera provide a compromise between lines and tubes. Ribbons can encode direction through arrowheads or color gradients.

**Bundles:** 3D edge bundling groups similar edges into shared tubes, reducing visual complexity.

#### Transparency and Depth Cues

**Alpha Compositing:** Transparency enables seeing through foreground structures to background elements. Careful opacity assignment prevents the "glass ball" effect where everything becomes equally visible and nothing is clear.

**Depth Fog:** Fog that increases with distance desaturates and fades distant elements, creating atmospheric perspective. This enhances depth perception without requiring transparency.

**Occlusion Culling:** Hidden line removal or making occluded elements partially transparent clarifies spatial relationships.

#### Lighting and Shading for Depth Perception

**Phong Shading:** Specular highlights and diffuse shading provide strong depth cues. Light source positioning affects perceived shape—multiple light sources reduce ambiguity.

**Ambient Occlusion:** Darkening in crevices and corners enhances shape perception. Screen-space ambient occlusion (SSAO) provides efficient approximation of global illumination effects.

**Shadows:** Cast shadows anchor nodes to reference planes and clarify relative heights. Shadow quality trades off against performance—soft shadows are more realistic but expensive.

#### Level-of-Detail (LOD) for Large 3D Networks

**Distance-Based LOD:** Nodes far from the camera render as simple points or billboards; nearby nodes render with full geometry.

**Importance-Based LOD:** Important nodes (high centrality, query targets) maintain full detail regardless of distance; unimportant nodes simplify.

**Aggregate LOD:** Distant regions aggregate into single representative nodes or omitted entirely; nearby regions show full detail.

**Progressive Refinement:** Initial coarse representation refines over time as resources permit, enabling interactive frame rates during camera movement.

### 3D Navigation and Interaction

#### Camera Controls

**Orbit:** Rotate camera around a focal point. Standard mode for inspecting network structure from multiple angles. Orbit center can be the network centroid, a selected node, or user-specified.

**Pan:** Translate camera parallel to the view plane. Enables exploring different network regions while maintaining viewing angle.

**Zoom:** Move camera along the view direction. Distinguished from field-of-view changes—true zoom maintains perspective while changing distance.

**Fly-Through:** First-person navigation with six degrees of freedom (position and orientation). Enables traveling through network volumes.

**Automatic Rotation:** Continuous slow rotation reveals 3D structure that static views cannot convey. User controls rotation speed and axis.

#### Selection and Lasso in 3D

**Ray Picking:** Mouse click casts a ray; the nearest intersected node is selected. Ray visualization (laser pointer) clarifies selection in dense regions.

**Frustum Selection:** Dragging creates a 3D selection volume (frustum from camera through screen rectangle). All nodes within the frustum are selected.

**Lasso in 3D:** Freehand drawing on the screen creates a selection volume. More precise than frustum for irregular regions.

**Sphere Selection:** Click and drag to create a spherical selection region in 3D space. The third dimension is determined by initial click depth.

#### Clipping Planes and Slicing

**Near/Far Clipping:** Standard OpenGL clipping removes elements too close or distant, but adjustable clipping planes enable deliberate sectioning.

**Arbitrary Clipping Planes:** User-defined planes slice through the network, revealing interior structure. Multiple simultaneous clipping planes create complex section views.

**Slice Visualization:** 2D cross-sections through 3D networks rendered as planar slices. Multiple parallel slices create "MRI-like" visualization of network interior.

**Volume Rendering:** Continuous volume representation rather than discrete slicing, with transfer functions mapping data values to color and opacity.

#### First-Person Navigation for Large Graphs

**Movement Controls:** WASD or arrow key movement through the network volume. Speed scales with zoom level—slow for detailed inspection, fast for traversing large distances.

**Teleportation:** Click-to-teleport for rapid movement to points of interest. Prevents motion sickness in VR contexts.

**Waypoints:** User-defined locations that can be rapidly returned to. Automatic waypoint generation at important nodes (high centrality, evidence nodes).

**Minimap:** 2D projection or overview provides global context during first-person navigation. Current position and viewing direction indicated.

#### Bookmarking Viewpoints

**Viewpoint Saving:** Current camera position, orientation, and zoom stored as named viewpoints. Multiple viewpoints capture different analytical perspectives.

**Viewpoint Transitions:** Smooth animation between saved viewpoints for presentation and storytelling. Transition speed and easing configurable.

**Viewpoint Sharing:** Viewpoints exported as URL parameters or shared configuration files, enabling collaboration.

---

## Probabilistic and Uncertainty Visualization

### Belief Visualization

#### Marginal Distribution Display on Nodes

**Bar Chart Glyphs:** Horizontal or vertical bars within node boundaries show probability mass for each state. Bar area is proportional to probability; bar height/width is constant, with length encoding probability. This supports accurate length comparison using position encoding.

**Pie Chart Glyphs:** Radial segments show state probabilities. Effective for 2-4 states; beyond this, angle comparison becomes inaccurate. Pie charts emphasize the part-to-whole relationship.

**Dot Distribution:** Dots within the node area represent probability mass—more dots in a region indicate higher probability. This frequency-based representation supports natural frequency reasoning.

**Box Plot Glyphs:** For continuous variables, box plots show median, quartiles, and range. Outliers may be indicated. Box plots compactly summarize distribution shape.

**Density Plot Glyphs:** Smooth curves showing probability density for continuous variables. Multiple modes and distribution shape are clearly visible.

#### Confidence Intervals and Credible Regions

**Error Bars:** Lines extending from point estimates show confidence/credible interval bounds. Symmetric or asymmetric intervals supported. Error bar caps improve readability.

**Shaded Regions:** Continuous confidence bands around estimated curves. Opacity encodes confidence level—darker regions indicate higher confidence.

**HPD Visualization:** Highest Posterior Density regions shown as shaded ranges containing specified probability mass (e.g., 95% HPD). For multimodal distributions, multiple disconnected regions may be shown.

**Convergence Visualization:** As inference proceeds (e.g., MCMC sampling), confidence intervals narrow. Animation or trail visualization shows convergence history.

#### Variance and Entropy as Visual Encodings

**Variance Mapping:** High variance desaturates node color or increases transparency. This creates visual emphasis on precisely estimated quantities.

**Entropy Rings:** Concentric rings around nodes fill according to entropy. Full ring indicates maximum entropy (uniform distribution); empty ring indicates zero entropy (certain state).

**Uncertainty Heatmaps:** Background coloration shows uncertainty fields across the network. Hot colors indicate high uncertainty; cool colors indicate certainty.

#### Most Probable Explanation (MPE) Highlighting

**State Highlighting:** The most probable state for each node receives visual emphasis (bold border, saturated color) while alternative states are de-emphasized.

**Configuration Highlighting:** For joint MPE queries, the complete configuration (assignment to all query variables) is highlighted as a coherent pattern, revealing the "best explanation" as a gestalt.

**Probability Threshold:** States with probability below a threshold are visually suppressed, decluttering the visualization to focus on plausible states.

### Inference Process Visualization

#### Message Passing Animation

Belief propagation algorithms pass messages between nodes until convergence. Animation of this process reveals information flow:

**Message Visualization:** Messages are shown as particles or waves traveling along edges. Message content (the belief being communicated) can be previewed through glyphs or tooltips.

**Node Activation:** Nodes glow or pulse when receiving or sending messages. Activation intensity correlates with message magnitude.

**Convergence Animation:** As the algorithm converges, message magnitudes stabilize and animation slows, providing visual confirmation of convergence.

**Damping Visualization:** For loopy belief propagation, damping is shown through message blending (incoming message mixed with previous belief).

#### Convergence Visualization

**Residual Plot:** A time series showing the maximum belief change per iteration. Convergence is visually apparent as the curve flattens.

**Belief Trajectory:** Individual node beliefs trace paths through probability space. Trajectories stabilize at convergence.

**Energy Landscape:** For algorithms minimizing an objective (variational inference), the energy surface can be visualized with the current position marked.

#### Evidence Propagation Through the Network

**Propagation Wave:** Evidence entry creates a wave of belief updates propagating through the network. The wave front can be animated, showing reach and speed of influence.

**Active Trail Highlighting:** Edges currently transmitting evidence-induced belief changes are highlighted. Inactive edges (blocked by conditioning) are de-emphasized.

**Impact Visualization:** Magnitude of belief change induced by evidence visualized through node size or color change. High-impact nodes (where evidence causes large belief revision) are prominent.

#### Before/After Comparison Views

**Split Screen:** Left panel shows prior beliefs; right panel shows posterior. Synchronized interaction (pan, zoom, selection) maintains correspondence.

**Difference Visualization:** Direct visualization of belief change (posterior minus prior). Positive changes in one color, negative in another, magnitude through intensity.

**Animation:** Smooth interpolation from prior to posterior states, enabling tracking of how each node's beliefs evolve.

**Small Multiples:** Prior and posterior as adjacent small multiples with linked highlighting.

### Uncertainty Encoding

#### Opacity for Certainty

**Certainty Gradient:** Fully opaque represents complete certainty (point mass, zero variance). Fully transparent represents complete uncertainty (uniform distribution, maximum entropy). Intermediate opacities represent partial knowledge.

**Contextual Opacity:** Nodes with certain evidence remain opaque regardless of their marginal uncertainty. This distinguishes evidence-induced certainty from structural certainty.

**Focus+Context:** Uncertain elements fade into background, focusing attention on reliable knowledge while maintaining context.

#### Error Bars and Confidence Bands

**Standard Error Bars:** Vertical or horizontal lines extending from estimates to show ±1 or ±2 standard errors. Symmetric or asymmetric based on distribution shape.

**Credible Intervals:** Explicit credible interval bounds marked. Multiple confidence levels can be shown (e.g., 50% and 95% intervals as thick and thin lines).

**Gradient Bands:** Continuous confidence visualization with opacity gradient—full opacity at point estimate, fading toward interval bounds.

**Fan Charts:** For time series or sequential predictions, fan charts show expanding prediction intervals into the future.

#### Ensemble Visualization

**Multiple Samples:** When inference produces samples rather than closed-form distributions, individual samples can be overlaid. Opacity per sample is reduced so the ensemble reveals the distribution shape.

**Spaghetti Plots:** For trajectory predictions, multiple sample trajectories create a "spaghetti" visualization of possible futures.

**Percentile Bands:** Ensembles summarized through percentile bands (10th, 25th, 50th, 75th, 90th percentiles) showing the distribution spread without individual samples.

**Glyph Aggregation:** For discrete states, ensemble frequencies shown through stacked bar charts or aggregated pie charts.

#### Hypothetical Scenario Comparison

**Parallel Worlds:** Multiple scenarios (different evidence sets, different interventions) shown as parallel network views. Synchronized interaction enables comparison.

**Difference Highlighting:** Scenarios compared by highlighting where they differ significantly. Agreement regions are de-emphasized.

**Scenario Matrix:** Grid of small multiples showing all pairwise scenario combinations, enabling comprehensive comparison.

**Interactive Scenario Switching:** Single view with controls to switch between scenarios. Smooth transitions animate differences.

---

## Temporal and Dynamic Visualization

### Animation

#### Network Evolution Over Time

**Frame-Based Animation:** Discrete time steps rendered as animation frames. Frame rate controls animation speed; users can pause, step, and scrub.

**Smooth Interpolation:** Between discrete observations, interpolation creates continuous animation. Linear interpolation is simplest; spline interpolation provides smoother motion.

**Temporal Granularity:** Animation adapts to data frequency—slow for gradual changes, fast for rapid dynamics.

**Ghosting/Trails:** Previous frames shown as faded "ghosts," creating motion trails that reveal trajectory history.

#### Flow Animation

**Particle Systems:** Particles flow along edges representing information, influence, or resource diffusion. Particle density correlates with flow magnitude; particle speed with flow velocity.

**Texture Animation:** Animated textures on edges create flowing patterns indicating direction and magnitude. Efficient for dense networks.

**Streamlines:** Curved paths showing flow patterns in continuous fields. Streamlines seeded at sources and traced through the network.

#### Play Controls, Scrubbing, Speed Adjustment

**Transport Controls:** Standard play/pause/stop buttons. Playback direction (forward/backward) and speed (0.25x to 4x) adjustable.

**Scrubber:** Timeline slider enabling direct access to any time point. Current time indicator shows position.

**Bookmarking:** Key moments can be bookmarked for rapid return. Bookmarks shown as markers on the timeline.

**Loop and Bounce:** Playback can loop continuously or bounce between endpoints, supporting continuous monitoring or detailed inspection.

#### Smooth Interpolation Between Time Steps

**Layout Interpolation:** Node positions interpolate between time slices to prevent jarring jumps. Force-directed layouts can be interpolated or recomputed per slice.

**Attribute Interpolation:** Node and edge attributes (size, color, opacity) interpolate smoothly. Discrete state changes can be shown as gradual transitions or abrupt changes.

**Morphing:** Network structure changes (nodes/edges appearing/disappearing) animated through scale/opacity transitions rather than instant appearance.

#### Trails and Trajectories for Moving Nodes

**Trajectory Lines:** Node paths traced as lines through time. Line thickness or color can encode speed or other attributes along the trajectory.

**Comet Tails:** Nodes shown with fading trails behind them, emphasizing current position while showing recent history.

**Ribbon Trajectories:** For multiple related nodes, trajectory ribbons show coordinated movement.

### Small Multiples

#### Time-Slice Comparison

**Grid Layout:** Time slices arranged in a grid (chronological left-to-right, top-to-bottom). Uniform scale enables comparison.

**Aligned Nodes:** Corresponding nodes aligned across panels through consistent layout or post-alignment adjustment.

**Difference Emphasis:** Small multiples can show raw state, difference from baseline, or difference from previous time step.

**Selection Synchronization:** Selection in one panel highlights corresponding elements in all panels.

#### Faceted Views by Attribute

**Attribute Faceting:** Network shown separately for different attribute values (e.g., male vs. female subnetworks, different treatment groups).

**Conditional Faceting:** Panels show network under different conditions (evidence sets, intervention scenarios).

**Facet Arrangement:** Panels arranged to reveal patterns—facets ordered by attribute value, cluster membership, or similarity.

#### Synchronized Interaction Across Views

**Linked Highlighting:** Hover or selection in one view highlights corresponding elements in all views.

**Linked Navigation:** Pan and zoom in one view applies to all views (when spatially meaningful).

**Linked Filtering:** Filter applied in one view applies to all views.

### Timeline Integration

#### Network Views Linked to Timeline

**Timeline Overview:** A separate timeline view showing quantitative summaries over time (e.g., total activation, network density). Clicking the timeline jumps to that time point.

**Network-Timeline Split:** Upper panel shows network; lower panel shows timeline. Synchronized cursors link the views.

**Embedded Timeline:** Timeline rendered as an axis alongside the network, with tick marks for time points.

#### Event Markers on Timeline

**Event Annotation:** Significant events marked on the timeline (interventions, observations, structural changes). Events can be clicked to navigate to that time.

**Event Visualization:** Event markers use distinct icons or colors for different event types.

**Event Density:** Heatmap or histogram showing event frequency over time.

#### Brushing and Linking Between Timeline and Network

**Timeline Brushing:** Selecting a time range on the timeline highlights or filters to show only that period's network state.

**Network-to-Timeline:** Selecting nodes or edges in the network highlights their activity periods on the timeline.

**Temporal Aggregation:** Timeline brushing aggregates network statistics for the selected period (average activation, edge density, etc.).

---

## Causal Visualization

### Causal Graph Visualization

#### Distinguishing Causal from Correlational Edges

**Edge Style Coding:** Causal edges (directed, manipulable) use solid lines; correlational edges (undirected, observational) use dashed lines.

**Edge Color Coding:** Causal edges in one color (e.g., blue); correlational in another (e.g., gray). Confounding edges (common cause relationships) in a third color.

**Arrowhead Emphasis:** Causal edges have prominent arrowheads; correlational edges have small or no arrowheads.

**Double Coding:** Redundant encoding through both style and color ensures distinguishability even in grayscale.

#### Intervention Visualization

**Graph Surgery:** The do-operator removes incoming edges to intervened variables. Visualization shows:
- Removed edges shown as faded or ghosted
- Intervention indicator (lightning bolt, scalpel icon) on intervened nodes
- Intervention value displayed prominently

**Multiple Interventions:** Different colors or icons for different intervention targets. Intervention set shown in a panel or legend.

**Intervention Comparison:** Side-by-side views of original graph and intervened graph, highlighting structural changes.

#### Counterfactual Comparison Views

**Factual vs. Counterfactual:** Split view showing actual outcome alongside counterfactual outcome under different intervention.

**Difference Visualization:** The causal effect shown as the difference between factual and counterfactual panels.

**Trajectory Comparison:** For temporal counterfactuals, trajectories under different interventions shown as different colored paths.

#### Causal Path Highlighting

**Active Causal Paths:** Paths from intervention to outcome along which causal influence flows highlighted prominently.

**Blocked Paths:** Paths blocked by adjustment or structure shown as faded with blocking reason indicated (conditioning on collider, etc.).

**Path Enumeration:** All paths listed in a panel with visual linking to network paths.

### Intervention Effects

#### Before/After Intervention Comparison

**Intervention Effect Panel:** Quantitative display of how intervention changes outcome distributions. Effect size, confidence intervals, and statistical significance shown.

**Network State Comparison:** Network visualizations before and after intervention showing belief updates throughout the graph.

**Effect Propagation:** Animation showing how intervention effects propagate from intervention node through active paths to outcome.

#### Multiple Intervention Scenario Comparison

**Scenario Matrix:** Grid showing all combinations of interventions on multiple variables. Each cell shows the joint effect.

**Pareto Frontier Visualization:** For multi-objective outcomes, Pareto-optimal interventions highlighted in intervention space.

**Interaction Visualization:** When interventions interact (non-additive effects), interaction strength visualized through edge thickness or color between intervention nodes.

#### Treatment Effect Magnitude Visualization

**Effect Size Encoding:** Edge thickness or color encodes the magnitude of causal effect along that path.

**Node Size Encoding:** Nodes resize to reflect their total causal effect on the outcome (total effect = direct + indirect).

**Effect Heatmap:** Background coloring shows effect field—regions of network colored by their contribution to outcome change.

---

## Interactive Features

### Basic Interactions

#### Pan, Zoom, Rotate

**Pan:** Click and drag to translate the view. Panning can be constrained to horizontal/vertical with modifier keys.

**Zoom:** Scroll wheel or pinch gesture zooms in/out. Zoom center can be mouse position (natural) or view center. Zoom limits prevent excessive zoom out (lost context) or in (pixelation).

**Rotate (2D):** For certain layouts, rotation reveals different perspectives. Typically constrained to the plane for 2D visualizations.

**Rotate (3D):** Full 3D rotation with multiple degrees of freedom. Orbit mode rotates around focal point; free mode rotates camera orientation.

**Reset View:** Single action to return to default view position, scale, and orientation.

#### Node and Edge Selection

**Single Selection:** Click to select. Selected elements highlighted with distinct border or glow. Selection clears previous selection unless modifier key held.

**Multiple Selection:** Ctrl/Cmd-click adds to selection; Shift-click selects ranges. Selection set shown in a panel with counts.

**Lasso Selection:** Freehand drawing selects all enclosed elements. Lasso can be additive or subtractive.

**Rectangle Selection:** Drag rectangle to select enclosed elements. Standard in many applications.

**Path Selection:** Click start node, then end node—intermediate nodes on paths between them are selected.

#### Hover Tooltips

**Content:** Tooltips show detailed information: variable name, type, current belief state, evidence status, statistics (entropy, centrality).

**Positioning:** Tooltips positioned to avoid occlusion of the hovered element and cursor. Smart positioning flips tooltip if it would extend beyond viewport.

**Timing:** Delay before appearance prevents flicker during cursor movement. Immediate appearance can be configured.

**Persistence:** Tooltips can be "pinned" to remain visible for reference while interacting elsewhere.

#### Context Menus

**Right-Click Menus:** Context-sensitive menus offer actions appropriate to clicked element:
- Nodes: Set evidence, query marginal, highlight paths, collapse/expand
- Edges: View CPD, test d-separation, remove edge
- Background: Layout options, view settings, reset

**Menu Organization:** Actions grouped by category (Evidence, Query, Layout, Export) with separators.

**Keyboard Shortcuts:** Menu items show keyboard shortcuts for efficiency.

### Advanced Interactions

#### Brushing and Linking Across Multiple Views

**Linked Views:** Multiple visualization panels (network, timeline, distribution plots) linked so interaction in one affects all.

**Brushing:** Selecting a region in one view highlights corresponding elements in all views. Brushing can be:
- **Continuous:** Brush region defines a query; elements satisfying query highlighted
- **Discrete:** Explicit selection of specific elements

**Linking Types:**
- **Highlight linking:** Selected elements emphasized across views
- **Filter linking:** Non-selected elements hidden across views
- **Scale linking:** Zoom/pan synchronized across views

**Data Point Identity:** Linking requires consistent identity mapping between views—nodes must be identifiable across different visual representations.

#### Dynamic Filtering and Masking

**Attribute Filters:** Sliders, checkboxes, or search boxes filter nodes/edges by attributes. Multiple filters combine with AND logic.

**Topology Filters:** Filter by graph properties (degree > threshold, path length from selected node, community membership).

**Probability Filters:** Filter nodes by belief properties (entropy < threshold, probability of state X > threshold).

**Filter Persistence:** Filter configuration can be saved, loaded, and shared.

**Visual Feedback:** Filtered-out elements can be hidden entirely or shown dimmed (masking vs. filtering).

#### Collapsing and Expanding Subgraphs

**Manual Collapse:** User selects nodes to collapse into a meta-node. Meta-node represents the subgraph and can be expanded on demand.

**Automatic Collapse:** Based on community detection or hierarchy, automatically collapse densely connected regions.

**Meta-Node Visualization:** Collapsed subgraphs shown as single nodes with indicators of internal structure (size reflects node count, internal edges shown as self-loop thickness).

**Expand/Collapse Controls:** Click or double-click to toggle expansion state. Animation smooths the transition.

**Hierarchical Navigation:** Breadcrumb trail shows current position in collapse hierarchy; clicking breadcrumbs navigates up.

#### Focus+Context Navigation

**Fisheye Lens:** Distortion-based focus+context with magnification at cursor position and compression elsewhere.

**Overview+Detail:** Separate overview panel showing full network with viewport rectangle indicating current detail view position. Dragging the rectangle pans the detail view.

**Semantic Zooming:** Zoom level determines level of detail—aggregate nodes at low zoom, individual nodes at medium zoom, full probability distributions at high zoom.

**Elastic Presentation:** Network "stretches" to accommodate focus region, maintaining connectivity while creating space for detailed inspection.

#### Search and Highlight

**Text Search:** Search box for finding nodes by name, attribute value, or other text. Search results highlighted and listed.

**Fuzzy Search:** Typo-tolerant search finds similar matches.

**Regular Expression:** Advanced search supports regex patterns.

**Navigate to Results:** Search results can be navigated sequentially, with automatic pan/zoom to center each result.

**Persistent Highlight:** Search results remain highlighted until cleared, supporting reference during other tasks.

### Filtering and Querying Visually

#### Slider-Based Filtering

**Continuous Sliders:** For numeric attributes (degree, centrality, probability), continuous sliders set filter thresholds. Dual sliders set ranges.

**Discrete Sliders:** For ordinal attributes, stepped sliders select specific values or ranges.

**Histogram Integration:** Sliders overlaid on histograms showing data distribution, helping users set meaningful thresholds.

**Live Update:** Visualization updates in real-time as sliders move, providing immediate feedback.

#### Topology Filtering

**Neighborhood Filter:** Select node, filter to show only nodes within k hops. Distance can be visualized through concentric rings.

**Path Filter:** Select start and end nodes, filter to show only nodes/edges on paths between them. Shortest path, all simple paths, or k-shortest paths options.

**Component Filter:** Isolate connected components; option to hide isolated nodes or small components.

**Cycle Filter:** Highlight or filter to cycles in the network. Useful for structure validation (Bayesian networks should be acyclic).

#### Temporal Filtering

**Time Range Selection:** Drag on timeline to select time range. Network visualization updates to show aggregate or endpoint state.

**Time Point Selection:** Click timeline to select specific time point.

**Animation Control:** Play network evolution within selected time range.

**Temporal Aggregation:** Aggregate network statistics over time range (average degree, total activation, etc.).

#### Query Result Visualization

**Query Specification:** Visual query builder for probabilistic queries. Select query variables, evidence variables, and values through point-and-click.

**Result Highlighting:** Query results highlighted on the network—target nodes glow, evidence nodes marked with indicators, marginal distributions updated.

**Comparison Queries:** Multiple queries shown with comparative visualization (e.g., query with different evidence sets shown side-by-side).

**Query History:** Previous queries saved for rapid re-execution or comparison.

---

## Export and Publishing

### Static Image Export

#### PNG, JPEG, TIFF, SVG Formats

**PNG:** Lossless compression with transparency support. Default for web and general use. Supports 8-bit and 16-bit color depth.

**JPEG:** Lossy compression for photographic content. Smaller file sizes but artifacts on text and sharp edges. Not recommended for network diagrams with text labels.

**TIFF:** Lossless format supporting layers and high bit depth. Preferred for publication submission when high quality is required.

**SVG:** Scalable vector graphics preserve quality at any resolution. Supports interactivity and animation (though not all features export). Editable in vector graphics software.

**Format Selection Guidance:**
- Web display: PNG or SVG
- Print publication: TIFF or high-resolution PNG
- Further editing: SVG
- Email/transfer size constrained: JPEG (low quality)

#### Resolution and DPI Settings

**DPI Configuration:** Export DPI configurable (72 for web, 300 for print, 600 for high-quality print).

**Pixel Dimensions:** Absolute pixel dimensions or relative to current view size.

**Vector Output:** SVG and PDF provide resolution-independent output, scaling to any size without quality loss.

**Raster Output:** PNG, JPEG, TIFF require explicit resolution settings. Formula: pixels = inches × DPI.

**Figure Sizing:** Preset sizes for common publication requirements (single column, double column, full page).

#### Color Space

**RGB:** Default for screen display. Wide gamut suitable for most visualization colors.

**CMYK:** Required for print publication. Color conversion may shift some colors (especially bright blues and greens). Soft-proofing preview shows CMYK appearance before export.

**Grayscale:** For black-and-white publication. Color mapping uses luminance preservation.

**Color Profiles:** Embedded color profiles (sRGB, Adobe RGB) ensure consistent color reproduction across devices.

#### Transparent Backgrounds

**Alpha Channel:** PNG and TIFF support transparent backgrounds. Useful for overlaying figures on colored or textured backgrounds.

**Matte Color:** When transparency not supported, background color configurable (typically white).

**Isolation:** Individual elements can be exported with transparent background for compositing.

#### Batch Export for Animations

**Frame Export:** Animation exported as sequence of frames (frame0001.png, frame0002.png, etc.).

**Video Encoding:** Automatic encoding to video formats (MP4, WebM, AVI) with configurable frame rate and codec.

**GIF Animation:** Animated GIF export for web embedding. Limited color palette; dithering options.

**Compression Settings:** Quality/compression tradeoff configurable for video export.

### 3D Export

#### OBJ, GLTF, GLB for 3D Models

**OBJ:** Wavefront OBJ format—widely supported, human-readable, includes geometry and basic materials. Textures referenced externally.

**GLTF/GLB:** Khronos Group standard for 3D assets. GLB is binary (single file); GLTF is JSON with external binaries. Supports:
- Geometry (nodes, edges)
- Materials (colors, transparency)
- Animations (camera paths, node movement)
- Metadata (node attributes, labels)

**Format Selection:**
- Web embedding: GLB
- 3D editing: OBJ or GLTF
- Archival: GLTF (self-describing)

#### Embedding in Web Pages

**Three.js Integration:** GLB files load directly into Three.js scenes. JavaScript API controls camera, lighting, interaction.

**Embed Code Generation:** Export includes HTML snippet for copy-paste embedding.

**Responsiveness:** Embedded viewers responsive to container size. Touch interaction support for mobile.

**Loading Optimization:** Draco compression for reduced file sizes; progressive loading for large models.

#### AR/VR Export Formats

**USDZ:** Apple's Universal Scene Description format for AR Quick Look on iOS. Export includes proper scaling and orientation for AR placement.

**GLB with AR Extensions:** GLB with WebXR extensions for browser-based AR.

**Unity Package:** Export as Unity asset package for custom AR/VR application development.

**Spatial Anchoring:** AR exports include suggested anchor points (e.g., place network on tabletop).

#### 3D Printing Considerations (STL)

**STL Export:** Stereolithography format for 3D printing. Meshes generated from network geometry.

**Printability Checks:**
- Minimum feature size warnings
- Overhang detection
- Structural integrity assessment

**Hollowing:** Option to hollow models to reduce material usage and print time.

**Support Generation:** Indication of where support structures will be needed.

**Scaling:** Physical dimensions configurable (mm, inches). Default scale produces palm-sized models.

### Interactive Export

#### HTML/JavaScript for Web Embedding

**Standalone HTML:** Self-contained HTML file with embedded JavaScript and data. Double-click to open in browser.

**Data Embedding:** Network data embedded as JSON within the HTML or loaded from external file.

**Library Dependencies:** D3.js, Three.js, or Plotly loaded from CDN or bundled.

**Customization:** CSS styling and JavaScript callbacks configurable for integration with existing websites.

#### Standalone HTML Files

**Single File Distribution:** All resources (HTML, CSS, JavaScript, data) bundled into one file for easy sharing.

**Offline Capability:** No internet connection required after download.

**File Size Optimization:** Minified JavaScript, compressed data, optional external asset loading.

#### Jupyter Widget Embedding

**ipywidgets Integration:** Visualization exported as Jupyter widget that can be embedded in notebooks.

**Widget State:** Current view state (camera position, filters, selection) preserved in widget.

**Notebook Compatibility:** Works in Jupyter Notebook, JupyterLab, Google Colab, VS Code notebooks.

**Binder Ready:** Exported widgets include environment specification for Binder deployment.

#### Dashboard Export

**Dash/Streamlit Export:** Python dashboard code generated for deployment with Dash or Streamlit.

**Layout Preserved:** Dashboard layout matches visualization layout with appropriate widget placement.

**Interactivity Preserved:** All interactive features available in dashboard context.

**Deployment Ready:** Generated code includes deployment instructions (Heroku, AWS, etc.).

### Publication-Ready Figures

#### LaTeX Integration

**PGF/TikZ Export:** Native LaTeX graphics format. Text rendered with document fonts; vector quality.

**Font Consistency:** TikZ export uses LaTeX fonts, ensuring figure text matches document text.

**Compilation:** Direct LaTeX compilation or inclusion as external file.

**Matplotlib Compatibility:** Figures exportable as matplotlib objects for further customization in Python before LaTeX integration.

**Raster Fallback:** For complex 3D visualizations, high-resolution PNG with tight bounding box.

#### Journal-Specific Figure Requirements

**Template Presets:** Export presets for common journals:
- Nature (single column: 89 mm, double column: 183 mm)
- Science (single column: 85 mm, double column: 178 mm)
- IEEE (varies by publication type)

**Font Size Compliance:** Minimum font sizes enforced (typically 8 pt after reduction).

**Line Width:** Minimum line widths for visibility (typically 0.5 pt).

**Color Blindness:** Export warning if color scheme not color-blind safe; automatic conversion to safe palette option.

#### Figure Composition and Multi-Panel Layouts

**Subplots:** Multiple visualizations arranged in grid layout. Configurable rows, columns, spacing.

**Shared Elements:** Shared legends, colorbars, and axes across subplots.

**Annotation:** Labels (a, b, c, ...) added automatically for multi-panel figures.

**Insets:** Small inset figures (e.g., zoomed regions) positioned within main figure.

**Export as Unit:** Entire composition exported as single file, or individual panels exported separately.

#### Caption Generation Assistance

**Auto-Caption:** Suggested caption generated based on visualization content:
- Network statistics (nodes, edges, density)
- Evidence and query variables
- Layout algorithm used
- Key findings

**Caption Templates:** User-defined templates with placeholders for auto-generated content.

**Export Metadata:** Structured metadata (JSON) includes all information needed for caption generation.

---

## Technology Stack

### 2D Rendering

#### Matplotlib Integration

**Rationale:** Matplotlib is the de facto standard for Python visualization. Integration ensures familiarity for Python users and compatibility with existing workflows.

**API Design:** `lutufi.viz.matplotlib` module provides functions returning matplotlib Figure and Axes objects.

**Customization:** All matplotlib customization options available (titles, labels, annotations, saving).

**Performance:** Matplotlib suitable for networks up to ~1,000 nodes. Beyond this, recommend other backends.

**Limitations:** Matplotlib's 2D focus limits interactive features; 3D support is basic.

#### Plotly/Dash for Interactive Web

**Rationale:** Plotly provides interactive web-based visualization with minimal deployment friction. Dash enables dashboard construction.

**Interactivity:** Built-in zoom, pan, hover tooltips, selection. Event callbacks for custom interactivity.

**Rendering:** WebGL acceleration for large datasets. SVG fallback for compatibility.

**Deployment:** Plotly figures embeddable in HTML, Jupyter, or Dash apps. Cloud hosting via Plotly Chart Studio.

**Limitations:** Web dependency; offline mode requires additional setup. Less customizable than D3.js for novel visualizations.

#### D3.js for Custom Web Visualizations

**Rationale:** D3.js provides maximum flexibility for custom visualization designs. Used when standard libraries insufficient.

**Integration:** Lutufi generates D3.js-compatible data (JSON) and provides JavaScript visualization components. Users embed in their own D3 applications.

**Component Library:** Reusable D3 components for common Lutufi visualizations.

**Customization:** Full D3 flexibility for advanced users; sensible defaults for rapid deployment.

#### Cairo/Skia for High-Performance Rendering

**Rationale:** For publication-quality raster output and high-performance server-side rendering, low-level graphics libraries provide advantages.

**Cairo:** Vector graphics library with high-quality anti-aliasing, PDF/SVG output, and cross-platform support. Used for print-quality export.

**Skia:** Google's 2D graphics library (used in Chrome). GPU acceleration via Vulkan/Metal/OpenGL. Suitable for high-performance interactive applications.

### 3D Rendering

#### Three.js for Web-Based 3D

**Rationale:** Three.js is the standard for web-based 3D graphics, providing abstraction over WebGL with wide browser support.

**Features:** Scene graph management, lighting and materials, post-processing effects, physics integration, VR support via WebXR.

**Integration:** `lutufi.viz.threejs` generates Three.js scenes. Export as standalone HTML or React components.

**Performance:** WebGL acceleration; instanced rendering for large networks.

#### WebGL/WebGPU for GPU Acceleration

**WebGL 2.0:** Current standard for web GPU acceleration. Lutufi shaders for force-directed layout computation, particle systems for flow visualization, instanced node/edge rendering, picking via render-to-texture.

**WebGPU:** Next-generation web graphics API (successor to WebGL). Lutufi will adopt as browser support increases for improved performance and compute capabilities.

**Compute Shaders:** GPU-based layout algorithms, community detection, and other graph computations.

#### VTK/ParaView for Scientific Visualization

**Rationale:** For scientific applications requiring advanced visualization techniques (volume rendering, complex glyphs), VTK provides enterprise-grade capabilities.

**Integration:** Export to VTK formats (VTU, VTP) for visualization in ParaView or custom VTK applications.

**Pipeline:** VTK's pipeline architecture supports sophisticated data flow and filtering.

#### Unity/Unreal for High-End 3D/VR

**Rationale:** For immersive VR/AR applications and high-fidelity visualization, game engines provide advanced rendering and interaction capabilities.

**Unity Integration:** Lutufi C# API for Unity; network data imported as scriptable objects; visualization components as prefabs.

**Unreal Integration:** Blueprint nodes and C++ API for importing Lutufi models; Niagara for particle-based flow visualization.

**VR/AR:** Native VR/AR support with hand tracking, haptics, and spatial audio. Collaborative VR environments for team-based network analysis.

### Performance Considerations

#### Canvas vs SVG for 2D

**SVG:** DOM-based; excellent for interaction; scales to any resolution; performance degrades with many elements (>5,000).

**Canvas:** Immediate mode rendering; better performance for many elements; lower-level API; interaction requires custom hit detection.

**Selection Guidance:**
- < 1,000 elements: SVG preferred
- 1,000 - 10,000 elements: Canvas with optimizations
- > 10,000 elements: WebGL or aggregation

**Hybrid Approach:** SVG for interactive foreground elements, Canvas for background network.

#### Instanced Rendering for Many Nodes

**Instancing:** Single draw call renders many identical objects with different transforms. Essential for large networks.

**Benefits:** Reduces draw calls from O(n) to O(1), enabling networks with 100,000+ nodes at interactive frame rates.

#### Level-of-Detail for Large Graphs

**LOD Strategies:** Distance-based (farther nodes rendered as points), importance-based (important nodes always high detail), semantic (detail matches analysis focus).

**Transition:** Smooth LOD transitions prevent popping artifacts.

#### Progressive Loading and Streaming

**Chunked Loading:** Large networks loaded in chunks, with progressive rendering.

**Data Streaming:** For dynamic networks, data streams incrementally.

**Priority Loading:** Visible regions loaded first; off-screen regions loaded on demand.

---

## Integration with Analysis Workflow

### Jupyter/Notebook Integration

#### Rich Display in Notebooks

**Protocol:** Lutufi objects implement `_repr_html_()`, `_repr_javascript_()`, and `_repr_png_()` for automatic rich display.

**Priority:** JavaScript (interactive) preferred; HTML fallback; PNG static fallback.

#### Interactive Widgets

**ipywidgets Integration:** Visualization wrapped in ipywidgets with controls for layout algorithm selector, evidence setting widgets, query specification, filter controls.

**Synchronization:** Widget state synchronized between Python kernel and JavaScript frontend.

#### Cell Output Optimization

**Lazy Loading:** Heavy JavaScript libraries loaded only when visualization first displayed.

**Output Caching:** Visualization state cached to avoid recomputation on cell re-execution.

#### Widget State Persistence

**Save State:** Widget state saved with notebook.

**Restore State:** Reopening notebook restores previous state.

### Standalone Applications

#### Desktop GUI Applications (Qt, Electron)

**Qt:** Native desktop application embedding QtWebEngine for visualization; full Python API access; native file dialogs; cross-platform.

**Electron:** Web-based desktop application; HTML/CSS/JavaScript frontend; Python backend via Flask/FastAPI.

**Deployment:** Single-file executable via PyInstaller or electron-builder.

#### Web-Based Dashboards

**Dash:** Python-based web dashboards with Lutufi components.

**Streamlit:** Simpler web app framework; Lutufi visualization functions work directly.

**Voila:** Turns Jupyter notebooks into standalone web applications.

#### Touch Interfaces for Tablets

**Gesture Support:** Touch gestures (pinch to zoom, pan, tap) implemented for tablet use.

**UI Scaling:** Interface elements sized for touch (minimum 44×44 pt targets).

### API Design for Visualization

#### Fluent API for Common Visualizations

**Method Chaining:** Fluent interface for building visualizations with layout, color, size, and highlight methods.

**Configuration Objects:** Alternative to method chaining, supporting serialization and reuse.

#### Declarative Specification (JSON/YAML)

**Specification Format:** Visualizations defined declaratively for reproducibility and sharing.

**Schema Validation:** Visualization specifications validated against JSON Schema.

#### Programmatic Control for Custom Figures

**Matplotlib Integration:** Full access to matplotlib Artists for fine-grained customization.

**D3.js Integration:** Low-level D3 manipulation for web visualizations.

#### Animation API

**Declarative Animation:** Specify animation through configuration with frame properties and transitions.

**Export Formats:** MP4, WebM, GIF, or frame sequences.

---

## Large Network Strategies

### Sampling and Aggregation

#### Node Sampling for Overview

**Random Sampling:** Uniform random sample provides unbiased overview but may miss important structures.

**Importance Sampling:** Sample nodes by importance to ensure key nodes are represented.

**Stratified Sampling:** Sample proportionally from communities to preserve structure.

#### Community-Level Aggregation

**Meta-Graph:** Communities become nodes; edges represent aggregate connectivity.

**Aggregate Statistics:** Meta-nodes encode community statistics.

**Drill-Down:** Clicking a meta-node expands to show internal structure.

#### Edge Bundling and Simplification

**Bundling:** Edges with similar paths grouped into shared curves.

**Sparsification:** Remove edges below a weight threshold.

**Backbone Extraction:** Identify minimal subgraph preserving essential structure.

### Level-of-Detail

#### Multiple Resolution Meshes

**Precomputed LODs:** Multiple versions at different detail levels precomputed.

**Runtime Simplification:** Algorithms simplify network representation in real-time.

#### Distance-Based LOD

**Metric:** Camera distance to node determines detail level.

**Levels:** Distant as points, medium as simple shapes, close as full glyphs.

#### Semantic Zooming

**Zoom Thresholds:** At certain zoom levels, representation changes qualitatively.

### GPU Acceleration

#### Compute Shaders for Layout

**Force Computation:** Node repulsion and edge attraction computed in parallel on GPU.

**Performance:** Enables interactive layout of networks with 100,000+ nodes.

#### Instanced Rendering

**Implementation:** Single geometry instanced for all nodes.

**Benefits:** Reduces GPU draw calls from O(n) to O(1).

#### GPU-Based Picking

**Render-to-Texture:** Scene rendered to off-screen buffer with unique color per object.

**Performance:** Picking in O(1) regardless of scene complexity.

---

## Special Visualizations

### Factor Graph Visualization

#### Bipartite Representation

**Variable Nodes:** Circular nodes representing random variables.

**Factor Nodes:** Square nodes representing factors.

**Bipartite Layout:** Variable and factor nodes arranged to emphasize bipartite structure.

#### Factor Visualization

**Table Views:** Tabular representation of factor values.

**Function Plots:** For continuous factors, surface plots showing factor as function.

**Heatmaps:** Factor values encoded as color heatmap.

#### Message Visualization on Factor Graph Edges

**Message Glyphs:** Edges show glyphs representing messages passed during belief propagation.

**Message Animation:** Messages animate along edges during convergence.

### Learning Visualization

#### Structure Learning Process Animation

**Search Trajectory:** Animation of structure search process.

**Score Landscape:** Visualization of score as function of structure.

#### Parameter Learning Convergence

**Convergence Plots:** Parameter values or log-likelihood over iterations.

**Animation:** Network beliefs animate during parameter updates.

#### Cross-Validation Results

**Fold Comparison:** Model performance across cross-validation folds.

**Learning Curves:** Training and validation performance vs. training set size.

### Comparison Views

#### Side-by-Side Model Comparison

**Structure Comparison:** Two networks shown side-by-side with alignment of corresponding nodes.

**Difference Highlighting:** Direct visualization of difference between models.

#### Diff Visualization Between Networks

**Edge Diff:** Added edges in green, removed in red, preserved in gray.

**Parameter Diff:** Edge thickness or color encodes parameter difference.

#### Ensemble Visualization

**Consensus Network:** Network showing edges present in majority of ensemble members.

**Edge Frequency:** Edge thickness encodes frequency of edge presence.

---

## Design System and Style Guide

### Color Palettes

#### Categorical Palettes

**Default Palette:** ColorBrewer Set2 or Tableau 10.

**Extended Palettes:** Tableau 20 for more categories.

**Semantic Colors:** Reserved colors for specific meanings (blue for causal, red for evidence, etc.).

#### Sequential Palettes

**Single-Hue:** Lightness gradient from white to full color.

**Multi-Hue:** Progression through hues with increasing saturation.

**Perceptually Uniform:** CIELAB or HCL color spaces.

#### Diverging Palettes

**Structure:** Two hues meeting at neutral midpoint.

**Use Cases:** Signed values, deviations from mean.

#### Color Blindness Safe Palettes

**Simulation:** All palettes tested under protanopia, deuteranopia, and tritanopia.

**Safe Defaults:** Default palettes chosen to be distinguishable under common deficiencies.

### Typography

#### Font Selection

**Body Text:** Sans-serif fonts (Inter, Roboto) for screen readability.

**Mathematical Notation:** STIX or Latin Modern Math.

**Monospace:** Fira Code or JetBrains Mono for code.

#### Hierarchy of Information

**Titles:** 14-18 pt, bold.

**Labels:** 10-12 pt, regular.

**Values:** 8-10 pt for annotations.

**Captions:** 8 pt, gray.

### Iconography and Glyphs

#### Standard Node Icons

**Evidence:** Eye icon or filled circle.

**Query:** Target icon or diamond.

**Hidden:** Dashed border.

**Intervention:** Lightning bolt.

#### Uncertainty Glyphs

**Error Bars:** Standard vertical or horizontal lines.

**Entropy Indicator:** Ring or pie chart showing uncertainty.

---

## Accessibility

### Visual Accessibility

#### Color Blindness Simulation and Safe Palettes

**Simulation Modes:** Protanopia, deuteranopia, tritanopia, achromatopsia.

**Alternative Encodings:** Shape, pattern, texture, labels.

#### High Contrast Modes

**Contrast Ratios:** Minimum 4.5:1 for normal text, 3:1 for large text.

**User Preference:** Respect system high contrast settings.

#### Scalable Interfaces

**Vector Graphics:** All visualizations scalable without quality loss.

**Responsive Text:** Text sizes in relative units.

### Non-Visual Accessibility

#### Screen Reader Support for Data Tables

**ARIA Labels:** All interactive elements have descriptive labels.

**Live Regions:** Dynamic updates announced to screen readers.

#### Sonification of Network Properties

**Pitch Mapping:** Node values mapped to pitch.

**Spatial Audio:** Node position mapped to stereo position.

#### Keyboard Navigation

**Focus Management:** Visible focus indicator on all interactive elements.

**Shortcuts:** Keyboard shortcuts for common actions.

---

## Implementation Roadmap

### Phase 1 (MVP): Matplotlib-based 2D, Basic Layouts, Static Export

**Timeline:** Months 1-6

**Deliverables:**
- `plot_network()` function with matplotlib backend
- Force-directed and hierarchical layouts
- Node color/size encoding for beliefs
- Static export to PNG, PDF, SVG
- Basic Jupyter integration

**Success Criteria:**
- Networks up to 500 nodes render in <2 seconds
- Publication-quality output suitable for papers
- All existing Lutufi examples have visualization

### Phase 2: Interactive Plotly, 3D Three.js, Animation

**Timeline:** Months 7-12

**Deliverables:**
- Plotly-based interactive visualizations
- Zoom, pan, hover tooltips
- Three.js 3D network visualization
- Time as third dimension for temporal networks
- Animation API for belief propagation

**Success Criteria:**
- Interactive web visualizations load in <3 seconds
- 3D visualization supports 10,000+ nodes
- Animation plays at 30+ FPS

### Phase 3: GPU Acceleration, VR Support, Advanced Interactions

**Timeline:** Months 13-18

**Deliverables:**
- WebGL compute shaders for layout
- Instanced rendering for large networks
- WebXR VR/AR support
- Brushing and linking across multiple views
- Advanced filtering with real-time updates

**Success Criteria:**
- Real-time layout of 100,000+ nodes
- VR visualization maintains 90 FPS
- Complex multi-view dashboards functional

### Phase 4: AI-Assisted Layout, Automatic Insight Highlighting

**Timeline:** Months 19-24

**Deliverables:**
- ML-based layout quality prediction
- Automatic layout algorithm selection
- AI-assisted insight detection
- Automatic caption generation
- Natural language querying

**Success Criteria:**
- Layout algorithm automatically chosen matches expert selection >80% of time
- Key insights automatically highlighted match manual analysis
- Voice/text queries produce correct visualizations

---

## Key References

### Visualization Research

1. **Munzner, T. (2014).** *Visualization Analysis and Design*. CRC Press.
2. **Heer, J., & Shneiderman, B. (2012).** "Interactive Dynamics for Visual Analysis." *ACM Queue*.

### Network Visualization

3. **Easley, D., & Kleinberg, J. (2010).** *Networks, Crowds, and Markets*. Cambridge University Press.
4. **McInnes, L., Healy, J., & Melville, J. (2018).** "UMAP: Uniform Manifold Approximation and Projection."

### Uncertainty Visualization

5. **Kay, M. (2022).** *Visualizing Uncertainty*. Now Publishers.
6. **Hullman, J. (2020).** "Why Authors Don't Visualize Uncertainty." *IEEE TVCG*.

### 3D Graph Visualization

7. **Yee, K. P., et al. (2001).** "Animated Exploration of Dynamic Graphs with Radial Layout." *IEEE InfoVis*.
8. **Ware, C., & Franck, G. (1996).** "Evaluating Stereo and Motion Cues for Visualizing Information Nets in Three Dimensions."

### Probabilistic Visualization

9. **Gabry, J., et al. (2019).** "Visualization in Bayesian Workflow." *Journal of the Royal Statistical Society: Series A*.
10. **Kale, A., Kay, M., & Hullman, J. (2019).** "Decision-Making Under Uncertainty in Research Synthesis." *CHI*.

### Accessibility in Visualization

11. **Lundgard, A., & Satyanarayan, A. (2021).** "Accessible Visualization via Natural Language Descriptions." *CHI*.

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial outline |
| 1.0 | 2026-03-04 | Wasswa Lutufi Sebbanja | Complete visualization design document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
