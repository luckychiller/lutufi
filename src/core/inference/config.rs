/// Configuration for inference algorithm selection.
///
/// All thresholds are user-configurable with sensible defaults.
/// This allows users to tune algorithm selection for their specific
/// model sizes and performance requirements.
#[derive(Debug, Clone)]
pub struct InferenceConfig {
    /// Maximum node count for which exact inference (Junction Tree)
    /// is preferred over approximate methods. For models at or below
    /// this size, Junction Tree is automatically selected by Auto mode.
    ///
    /// Default: 15 (roadmap Sprint 3.4 recommendation).
    pub exact_max_nodes: usize,

    /// Maximum node count for which Variable Elimination is considered
    /// viable. Beyond this threshold, VE will not be selected by Auto.
    ///
    /// Default: 30.
    pub ve_max_nodes: usize,

    /// Treewidth threshold above which a warning is issued recommending
    /// approximate inference instead of exact.
    ///
    /// Default: 15.
    pub max_tolerable_treewidth: usize,

    /// Cost multiplier for exact inference strategies in auto-selection.
    /// Higher values make exact methods less preferred by the cost estimator.
    ///
    /// Default: 200 (cost per node for JT).
    pub exact_cost_per_node: usize,

    /// Cost multiplier for approximate inference strategies.
    /// Lower values make approximate methods more preferred.
    ///
    /// Default: 10 (cost per node for LBP).
    pub approx_cost_per_node: usize,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            exact_max_nodes: 15,
            ve_max_nodes: 30,
            max_tolerable_treewidth: 15,
            exact_cost_per_node: 200,
            approx_cost_per_node: 10,
        }
    }
}

impl InferenceConfig {
    /// Create a new config with all defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum node count for exact inference preference.
    pub fn with_exact_max_nodes(mut self, max: usize) -> Self {
        self.exact_max_nodes = max;
        self
    }

    /// Set the maximum node count for Variable Elimination viability.
    pub fn with_ve_max_nodes(mut self, max: usize) -> Self {
        self.ve_max_nodes = max;
        self
    }

    /// Set the maximum tolerable treewidth.
    pub fn with_max_tolerable_treewidth(mut self, tw: usize) -> Self {
        self.max_tolerable_treewidth = tw;
        self
    }
}
