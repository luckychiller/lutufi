use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    variable::VariableId,
};

/// Represents a context-specific independence (CSI) relation.
///
/// CSI allows a parent to only affect a child in certain contexts
/// (specific values of other parents), dramatically reducing CPT size.
///
/// Example: P(Y | X, Z) where Z only matters when X=1.
/// CSI tree: X=0 → Y independent of Z; X=1 → Y depends on Z.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSpecificIndependence {
    /// The child variable ID.
    child_id: VariableId,
    /// All parent variable IDs.
    parent_ids: Vec<VariableId>,
    /// The parent ID that becomes irrelevant in certain contexts.
    irrelevant_parent: VariableId,
    /// The context variable ID (the one whose value determines relevance).
    context_variable: VariableId,
    /// Context value(s) where the irrelevant_parent becomes irrelevant.
    context_values: HashSet<usize>,
    /// Simplified CPT data (only for configurations where irrelevant_parent matters).
    reduced_cpt: Option<TabularFactor>,
    child_name: String,
    irrelevant_parent_name: String,
    context_variable_name: String,
}

impl ContextSpecificIndependence {
    /// Create a new CSI relation.
    pub fn new(
        child_id: VariableId,
        parent_ids: Vec<VariableId>,
        irrelevant_parent: VariableId,
        context_variable: VariableId,
        context_values: HashSet<usize>,
        parent_names: &HashMap<VariableId, String>,
    ) -> Self {
        ContextSpecificIndependence {
            child_id,
            parent_ids,
            irrelevant_parent,
            context_variable,
            context_values,
            reduced_cpt: None,
            child_name: parent_names.get(&child_id).cloned().unwrap_or_default(),
            irrelevant_parent_name: parent_names.get(&irrelevant_parent).cloned().unwrap_or_default(),
            context_variable_name: parent_names.get(&context_variable).cloned().unwrap_or_default(),
        }
    }

    /// Returns the child variable ID.
    pub fn child_id(&self) -> VariableId { self.child_id }
    /// Returns the irrelevant parent variable ID.
    pub fn irrelevant_parent(&self) -> VariableId { self.irrelevant_parent }
    /// Returns the context variable ID.
    pub fn context_variable(&self) -> VariableId { self.context_variable }
    /// Returns the set of context values where the parent is irrelevant.
    pub fn context_values(&self) -> &HashSet<usize> { &self.context_values }

    /// Check if the irrelevant parent is active for a given context.
    pub fn is_parent_active(&self, context_value: usize) -> bool {
        !self.context_values.contains(&context_value)
    }

    /// Compute the number of entries saved by this CSI relation.
    pub fn entries_saved(&self, child_domain_size: usize, domain_sizes: &HashMap<VariableId, usize>) -> usize {
        let total_entries: usize = self.parent_ids.iter()
            .map(|id| domain_sizes.get(id).copied().unwrap_or(2))
            .product::<usize>()
            * child_domain_size;

        let _irrelevant_idx = self.parent_ids.iter()
            .position(|&id| id == self.irrelevant_parent)
            .unwrap_or(0);
        let irrelevant_size = domain_sizes.get(&self.irrelevant_parent).copied().unwrap_or(2);
        let context_size = domain_sizes.get(&self.context_variable).copied().unwrap_or(2);
        let context_count = self.context_values.len();
        let fraction_reduced = context_count as f64 / context_size as f64;

        (total_entries as f64 * fraction_reduced * (1.0 - 1.0 / irrelevant_size as f64)) as usize
    }

    /// Reduce a full CPT by removing the irrelevant parent in the given context.
    pub fn reduce_cpt(&mut self, full_cpt: &TabularFactor, domain_sizes: &HashMap<VariableId, usize>) -> LutufiResult<()> {
        let scope = full_cpt.scope();
        let _child_pos = scope.variable_ids().iter()
            .position(|&id| id == self.child_id)
            .ok_or_else(|| LutufiError::InternalError {
                message: "Child not found in CPT scope".to_string(),
            })?;
        let context_pos = scope.variable_ids().iter()
            .position(|&id| id == self.context_variable)
            .ok_or_else(|| LutufiError::InternalError {
                message: "Context variable not found in CPT scope".to_string(),
            })?;
        let irrelevant_pos = scope.variable_ids().iter()
            .position(|&id| id == self.irrelevant_parent)
            .ok_or_else(|| LutufiError::InternalError {
                message: "Irrelevant parent not found in CPT scope".to_string(),
            })?;

        let mut reduced_vars: Vec<VariableId> = scope.variable_ids().to_vec();
        reduced_vars.remove(irrelevant_pos);
        let reduced_sizes: Vec<usize> = reduced_vars.iter()
            .map(|id| domain_sizes.get(id).copied().unwrap_or(2))
            .collect();
        let reduced_scope = Scope::from_ids_and_sizes(reduced_vars, reduced_sizes.clone());

        let mut new_log_values = vec![f64::NEG_INFINITY; reduced_scope.num_entries()];
        let num_entries = scope.num_entries();
        let sizes = scope.sizes();

        for i in 0..num_entries {
            let multi = crate::core::factor::multi_index_from_flat(i, sizes);
            let context_val = multi[context_pos];
            if !self.context_values.contains(&context_val) {
                continue;
            }

            let reduced_multi: Vec<usize> = multi.iter()
                .enumerate()
                .filter(|(j, _)| *j != irrelevant_pos)
                .map(|(_, &v)| v)
                .collect();

            let mut flat = 0;
            let mut stride = 1;
            for j in (0..reduced_multi.len()).rev() {
                flat += reduced_multi[j] * stride;
                if j > 0 { stride *= reduced_sizes[j]; }
            }

            new_log_values[flat] = crate::core::factor::log_sum_exp(
                new_log_values[flat],
                full_cpt.log_value_at(i),
            );
        }

        self.reduced_cpt = Some(TabularFactor::from_log_values(reduced_scope, new_log_values)?);
        Ok(())
    }
}

/// Manages all CSI relations in a CPT, enabling efficient storage
/// and computation of context-specific independencies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CsiManager {
    relations: Vec<ContextSpecificIndependence>,
}

impl CsiManager {
    /// Create an empty CSI manager.
    pub fn new() -> Self { CsiManager { relations: Vec::new() } }

    /// Add a CSI relation to the manager.
    pub fn add_relation(&mut self, csi: ContextSpecificIndependence) {
        self.relations.push(csi);
    }

    /// Returns all managed CSI relations.
    pub fn relations(&self) -> &[ContextSpecificIndependence] { &self.relations }

    /// Compute the total number of CPT entries saved by all managed relations.
    pub fn total_entries_saved(&self, child_domain_size: usize, domain_sizes: &HashMap<VariableId, usize>) -> usize {
        self.relations.iter()
            .map(|r| r.entries_saved(child_domain_size, domain_sizes))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::variable::Variable;
    use crate::core::domain::Domain;

    fn make_var_id(name: &str) -> VariableId {
        Variable::new(name, Domain::binary()).id()
    }

    #[test]
    fn test_csi_creation() {
        let c = make_var_id("C");
        let p1 = make_var_id("P1");
        let p2 = make_var_id("P2");
        let mut names = HashMap::new();
        names.insert(c, "C".to_string());
        names.insert(p1, "P1".to_string());
        names.insert(p2, "P2".to_string());

        let mut ctx_vals = HashSet::new();
        ctx_vals.insert(0);

        let csi = ContextSpecificIndependence::new(
            c, vec![p1, p2], p2, p1, ctx_vals, &names,
        );

        assert!(!csi.is_parent_active(0));
        assert!(csi.is_parent_active(1));
    }

    #[test]
    fn test_csi_manager() {
        let mut manager = CsiManager::new();
        assert!(manager.relations().is_empty());

        let c = make_var_id("C");
        let p1 = make_var_id("P1");
        let p2 = make_var_id("P2");
        let mut names = HashMap::new();
        names.insert(c, "C".to_string());
        names.insert(p1, "P1".to_string());
        names.insert(p2, "P2".to_string());

        let mut ctx_vals = HashSet::new();
        ctx_vals.insert(0);

        let csi = ContextSpecificIndependence::new(
            c, vec![p1, p2], p2, p1, ctx_vals, &names,
        );
        manager.add_relation(csi);
        assert_eq!(manager.relations().len(), 1);
    }
}
