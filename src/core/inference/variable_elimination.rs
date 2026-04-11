use std::collections::HashSet;
use crate::core::{
    assignment::Assignment,
    error::LutufiResult,
    factor::TabularFactor,
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
};

/// Heuristic for choosing the next variable to eliminate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EliminationHeuristic {
    /// Minimum degree: choose the variable with the fewest neighbors in the elimination graph.
    MinDegree,
    /// Minimum fill: choose the variable whose elimination adds the fewest fill-in edges.
    MinFill,
}

/// Mode for inference: Marginal (sum-out), MAP (max-out), or MPE (max-out all).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferenceMode {
    /// Compute marginal probabilities P(Q | E).
    Marginal,
    /// Compute Maximum A Posteriori assignment argmax P(Q | E).
    Map,
    /// Compute Most Probable Explanation argmax P(all | E).
    Mpe,
}

/// Exact inference engine using the Variable Elimination (VE) algorithm.
#[derive(Debug, Clone)]
pub struct VariableEliminationEngine<'a> {
    model: &'a BayesianNetwork,
}

impl<'a> VariableEliminationEngine<'a> {
    /// Create a new Variable Elimination engine for a model.
    pub fn new(model: &'a BayesianNetwork) -> Self {
        VariableEliminationEngine { model }
    }

    /// Query the marginal probabilities for specific variables.
    pub fn query(
        &self,
        variables: &[&str],
        evidence: &Assignment,
        heuristic: EliminationHeuristic,
    ) -> LutufiResult<TabularFactor> {
        let query_ids = self.variable_ids(variables)?;
        let mut factors = self.collect_reduced_factors(evidence)?;
        let elimination_order = self.determine_elimination_order(&query_ids, &mut factors, heuristic)?;
        let reduced_factors = self.eliminate_variables(factors, &elimination_order, false)?;
        let mut final_factor = self.multiply_factors(reduced_factors)?;
        final_factor.normalize();
        Ok(final_factor)
    }

    /// Perform a MAP or MPE query using max-product elimination.
    pub fn query_map(
        &self,
        variables: &[&str],
        evidence: &Assignment,
        heuristic: EliminationHeuristic,
        mode: InferenceMode,
    ) -> LutufiResult<TabularFactor> {
        let query_ids = if matches!(mode, InferenceMode::Mpe) {
            self.variables_to_maximize(evidence)?
        } else {
            self.variable_ids(variables)?
        };

        let mut factors = self.collect_reduced_factors(evidence)?;
        let elimination_order = self.determine_elimination_order(&query_ids, &mut factors, heuristic)?;
        let reduced_factors = self.eliminate_variables(factors, &elimination_order, true)?;
        let final_factor = self.multiply_factors(reduced_factors)?;
        Ok(final_factor)
    }

    /// Estimate the treewidth of the model using a specific heuristic.
    pub fn estimate_treewidth(&self, heuristic: EliminationHeuristic) -> LutufiResult<usize> {
        let mut factors = self.collect_reduced_factors(&Assignment::new())?;
        let elimination_order = self.determine_elimination_order(&HashSet::new(), &mut factors, heuristic)?;
        let mut max_clique = 0;

        for &variable in &elimination_order {
            let (containing, not_containing): (Vec<_>, Vec<_>) = factors.into_iter()
                .partition(|f| f.scope().contains(&variable));
            
            if containing.is_empty() {
                factors = not_containing;
                continue;
            }

            let product = containing.into_iter().reduce(|acc, factor| acc.multiply(&factor).unwrap()).unwrap();
            max_clique = max_clique.max(product.scope().len());
            let marginalized = product.marginalize(&[variable])?;
            factors = not_containing;
            if !marginalized.scope().is_empty() {
                factors.push(marginalized);
            }
        }

        Ok(max_clique.saturating_sub(1))
    }

    fn variable_ids(&self, variables: &[&str]) -> LutufiResult<HashSet<VariableId>> {
        variables.iter().map(|&name| self.model.id_of(name)).collect()
    }

    fn variables_to_maximize(&self, evidence: &Assignment) -> LutufiResult<HashSet<VariableId>> {
        let mut vars = HashSet::new();
        for &id in self.model.variables.keys() {
            if evidence.get_discrete(&id).is_err() {
                vars.insert(id);
            }
        }
        Ok(vars)
    }

    fn collect_reduced_factors(&self, evidence: &Assignment) -> LutufiResult<Vec<TabularFactor>> {
        let mut factors = Vec::new();
        for cpd in self.model.cpd_iter() {
            factors.push(cpd.as_factor().reduce(evidence)?);
        }
        Ok(factors)
    }

    fn determine_elimination_order(
        &self,
        query_ids: &HashSet<VariableId>,
        factors: &mut [TabularFactor],
        heuristic: EliminationHeuristic,
    ) -> LutufiResult<Vec<VariableId>> {
        let mut all_vars = HashSet::new();
        for factor in factors.iter() {
            for &id in factor.scope().variable_ids() {
                if !query_ids.contains(&id) {
                    all_vars.insert(id);
                }
            }
        }

        let mut order = Vec::new();
        let mut remaining_vars = all_vars;
        
        while !remaining_vars.is_empty() {
            let var = match heuristic {
                EliminationHeuristic::MinDegree => self.choose_min_degree(&remaining_vars, factors),
                EliminationHeuristic::MinFill => self.choose_min_fill(&remaining_vars, factors),
            };
            order.push(var);
            remaining_vars.remove(&var);
        }

        Ok(order)
    }

    fn eliminate_variables(
        &self,
        mut factors: Vec<TabularFactor>,
        order: &[VariableId],
        max_product: bool,
    ) -> LutufiResult<Vec<TabularFactor>> {
        for &variable in order {
            let (containing, not_containing): (Vec<_>, Vec<_>) = factors.into_iter()
                .partition(|f| f.scope().contains(&variable));
            
            if containing.is_empty() {
                factors = not_containing;
                continue;
            }

            let product = containing.into_iter().reduce(|acc, factor| acc.multiply(&factor).unwrap()).unwrap();
            let marginalized = if max_product {
                product.max_marginalize(&[variable])?
            } else {
                product.marginalize(&[variable])?
            };
            
            factors = not_containing;
            if !marginalized.scope().is_empty() {
                factors.push(marginalized);
            }
        }
        Ok(factors)
    }

    fn multiply_factors(&self, factors: Vec<TabularFactor>) -> LutufiResult<TabularFactor> {
        if factors.is_empty() {
            // Return identity factor if no factors left
            return TabularFactor::identity(crate::core::factor::Scope::from_ids_and_sizes(vec![], vec![]));
        }
        let product = factors.into_iter().reduce(|acc, factor| acc.multiply(&factor).unwrap()).unwrap();
        Ok(product)
    }

    fn choose_min_degree(&self, vars: &HashSet<VariableId>, factors: &[TabularFactor]) -> VariableId {
        let mut best_var = *vars.iter().next().unwrap();
        let mut min_degree = usize::MAX;

        for &var in vars {
            let mut neighbors = HashSet::new();
            for factor in factors {
                if factor.scope().contains(&var) {
                    for &v in factor.scope().variable_ids() {
                        if v != var {
                            neighbors.insert(v);
                        }
                    }
                }
            }
            if neighbors.len() < min_degree {
                min_degree = neighbors.len();
                best_var = var;
            }
        }
        best_var
    }

    fn choose_min_fill(&self, vars: &HashSet<VariableId>, factors: &[TabularFactor]) -> VariableId {
        let mut best_var = *vars.iter().next().unwrap();
        let mut best_fill = usize::MAX;

        for &var in vars {
            let mut neighbors = HashSet::new();
            for factor in factors {
                if factor.scope().contains(&var) {
                    for &v in factor.scope().variable_ids() {
                        if v != var {
                            neighbors.insert(v);
                        }
                    }
                }
            }
            
            let mut fill = 0;
            let neighbor_list: Vec<_> = neighbors.into_iter().collect();
            for i in 0..neighbor_list.len() {
                for j in i + 1..neighbor_list.len() {
                    let v1 = neighbor_list[i];
                    let v2 = neighbor_list[j];
                    let already_neighbors = factors.iter().any(|factor| factor.scope().contains(&v1) && factor.scope().contains(&v2));
                    if !already_neighbors {
                        fill += 1;
                    }
                }
            }
            if fill < best_fill {
                best_fill = fill;
                best_var = var;
            }
        }
        best_var
    }
}
