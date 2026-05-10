use crate::core::{
    error::{LutufiError, LutufiResult},
    models::bayesian_network::BayesianNetwork,
    assignment::Assignment,
};

/// Engine for generating samples from a Bayesian Network.
pub struct Sampler;

impl Sampler {
    /// Generate a single sample from the network using forward sampling.
    ///
    /// The network must be valid (all CPTs set) and acyclic.
    pub fn forward_sample(model: &BayesianNetwork) -> LutufiResult<Assignment> {
        use rand::distributions::{Distribution, WeightedIndex};
        
        if !model.is_valid() {
            return Err(LutufiError::InternalError { 
                message: "Cannot sample from an invalid network. Ensure all CPTs are set.".to_string() 
            });
        }

        let mut internal_assignment = Assignment::new();
        let mut result_assignment = Assignment::new();
        let order = model.topological_order()?;
        let mut rng = rand::thread_rng();

        for node_name in order {
            let id = model.id_of(node_name)?;
            let cpd = model.cpd(node_name)?;
            let factor = cpd.as_factor();
            
            // Find probabilities for this node given parents already in assignment
            let reduced = factor.reduce(&internal_assignment)?;
            
            let n = reduced.scope().num_entries();
            let mut probs = Vec::with_capacity(n);
            for i in 0..n {
                probs.push(reduced.value_at(i));
            }
            
            let dist = WeightedIndex::new(&probs).map_err(|e| LutufiError::InternalError { 
                message: format!("Sampling error for variable '{}': {}", node_name, e) 
            })?;
            let sampled_idx = dist.sample(&mut rng);
            
            let var = model.registry().variable(&id).ok_or_else(|| LutufiError::VariableNotFound {
                name: format!("{:?}", id),
                available: format!("{:?}", model.nodes()),
            })?;
            let state_name = match var.domain() {
                crate::core::domain::Domain::Discrete { states } => states[sampled_idx].clone(),
                crate::core::domain::Domain::Binary => crate::core::domain::BINARY_STATES[sampled_idx].to_string(),
                _ => return Err(LutufiError::InternalError { message: "Sampling from continuous domains not yet supported".to_string() }),
            };
            
            internal_assignment.set_discrete(id, sampled_idx)?;
            result_assignment.set(id, state_name);
        }
        
        Ok(result_assignment)
    }
}
