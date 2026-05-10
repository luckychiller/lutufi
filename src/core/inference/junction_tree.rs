use std::collections::{BTreeSet, HashMap, HashSet};
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    graph::UndirectedVariableGraph,
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
};

/// Exact inference engine using the Junction Tree (JT) algorithm.
/// 
/// The Junction Tree algorithm (also known as Clique Tree Propagation)
/// compiles the model into a tree of cliques, allowing for efficient
/// repeated queries via message passing.
#[derive(Debug, Clone)]
pub struct JunctionTreeEngine {
    model: BayesianNetwork,
    _clique_scopes: Vec<Scope>,
    adjacency: Vec<Vec<usize>>,
    separator_scopes: HashMap<(usize, usize), Scope>,
    clique_potentials: Vec<TabularFactor>,
    treewidth: usize,
}

impl JunctionTreeEngine {
    /// Create a new Junction Tree engine for a model.
    /// 
    /// This performs the expensive compilation step (moralization, triangulation,
    /// and clique tree construction).
    pub fn new(model: &BayesianNetwork) -> LutufiResult<Self> {
        let moral = Self::moralize(model);
        let clique_scopes = Self::triangulate(model, &moral)?;
        let treewidth = clique_scopes.iter().map(|scope| scope.len()).max().unwrap_or(0).saturating_sub(1);
        let adjacency = Self::build_junction_tree(&clique_scopes)?;
        let separator_scopes = Self::build_separator_scopes(&clique_scopes, &adjacency);
        let clique_potentials = Self::build_clique_potentials(model, &clique_scopes)?;

        Ok(Self {
            model: model.clone(),
            _clique_scopes: clique_scopes,
            adjacency,
            separator_scopes,
            clique_potentials,
            treewidth,
        })
    }

    /// The treewidth of the compiled junction tree.
    pub fn treewidth(&self) -> usize {
        self.treewidth
    }

    /// The model this engine was compiled from.
    pub fn model(&self) -> &BayesianNetwork {
        &self.model
    }

    /// Query the marginal probabilities for specific variables.
    pub fn query(
        &self,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<TabularFactor> {
        let query_ids = variables.iter().map(|&name| self.model.id_of(name)).collect::<LutufiResult<HashSet<VariableId>>>()?;
        let mut potentials = self.clique_potentials.clone();
        for potential in potentials.iter_mut() {
            *potential = potential.reduce(evidence)?;
        }

        let beliefs = self.calibrate(potentials)?;
        if let Some(clique_idx) = beliefs.iter().position(|belief| belief.scope().contains_all(&query_ids)) {
            let vars_to_sum_out: Vec<VariableId> = beliefs[clique_idx].scope().variable_ids()
                .iter()
                .filter(|&&id| !query_ids.contains(&id))
                .copied()
                .collect();
            let result = beliefs[clique_idx].marginalize(&vars_to_sum_out)?;
            let mut normalized = result;
            normalized.normalize();
            return Ok(normalized);
        }

        Self::exact_query_from_beliefs(beliefs, &query_ids)
    }

    fn moralize(model: &BayesianNetwork) -> UndirectedVariableGraph {
        let mut moral = UndirectedVariableGraph::new();
        for id in model.graph.node_ids() {
            moral.add_node(id);
        }

        for (parent, child) in model.graph.edges() {
            moral.add_edge(&parent, &child);
        }

        for id in model.graph.node_ids() {
            let parents = model.graph.parents(&id);
            for i in 0..parents.len() {
                for j in i + 1..parents.len() {
                    moral.add_edge(&parents[i], &parents[j]);
                }
            }
        }

        moral
    }

    fn triangulate(model: &BayesianNetwork, moral: &UndirectedVariableGraph) -> LutufiResult<Vec<Scope>> {
        let mut graph = moral.clone();

        // Use reverse topological order for elimination to guarantee family preservation.
        // When a variable X is eliminated, all its parents Pa(X) are still in the graph
        // (parents come after children in reverse topological order), so the elimination
        // clique {X} ∪ Neighbors(X) contains the family {X} ∪ Pa(X).
        let topo = model.topological_order()?;
        let order: Vec<VariableId> = topo.iter()
            .rev()
            .filter_map(|&name| model.id_of(name).ok())
            .collect();

        let mut cliques: Vec<HashSet<VariableId>> = Vec::new();
        for &eliminate in &order {
            let neighbors = graph.neighbors(&eliminate);
            let mut clique: HashSet<VariableId> = neighbors.iter().cloned().collect();
            clique.insert(eliminate);
            cliques.push(clique);

            for i in 0..neighbors.len() {
                for j in i + 1..neighbors.len() {
                    graph.add_edge(&neighbors[i], &neighbors[j]);
                }
            }
            graph.remove_node(&eliminate);
        }

        let mut scopes: Vec<Scope> = Vec::new();
        for clique in cliques {
            let mut ids: Vec<VariableId> = clique.into_iter().collect();
            ids.sort_by_key(|id| id.to_string());
            let sizes = ids.iter().map(|id| {
                model.variables().get(id).map(|v| v.domain().size().unwrap_or(0)).unwrap_or(0)
            }).collect();
            scopes.push(Scope::from_ids_and_sizes(ids, sizes));
        }

        Ok(scopes)
    }

    fn build_junction_tree(cliques: &[Scope]) -> LutufiResult<Vec<Vec<usize>>> {
        let n = cliques.len();
        if n == 0 { return Ok(Vec::new()); }
        if n == 1 { return Ok(vec![Vec::new()]); }

        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut in_tree = vec![false; n];
        let mut max_weight = vec![0; n];
        let mut best_neighbor = vec![usize::MAX; n];

        // Start with the first clique
        in_tree[0] = true;
        for j in 1..n {
            max_weight[j] = Self::scope_intersection_size(&cliques[0], &cliques[j]);
            best_neighbor[j] = 0;
        }

        for _ in 1..n {
            let mut u = usize::MAX;
            let mut weight = -1;

            for j in 0..n {
                if !in_tree[j] && (max_weight[j] as i32) > weight {
                    weight = max_weight[j] as i32;
                    u = j;
                }
            }

            if u == usize::MAX { break; } // Should not happen if graph is connected

            in_tree[u] = true;
            let v = best_neighbor[u];
            adjacency[u].push(v);
            adjacency[v].push(u);

            for j in 0..n {
                if !in_tree[j] {
                    let w = Self::scope_intersection_size(&cliques[u], &cliques[j]);
                    if w > max_weight[j] {
                        max_weight[j] = w;
                        best_neighbor[j] = u;
                    }
                }
            }
        }

        Ok(adjacency)
    }

    fn build_separator_scopes(
        cliques: &[Scope],
        adjacency: &[Vec<usize>],
    ) -> HashMap<(usize, usize), Scope> {
        let mut separators = HashMap::new();
        for i in 0..cliques.len() {
            for &j in &adjacency[i] {
                if i == j { continue; }
                let intersection = Self::scope_intersection(&cliques[i], &cliques[j]);
                separators.insert((i, j), intersection);
            }
        }
        separators
    }

    fn build_clique_potentials(
        model: &BayesianNetwork,
        clique_scopes: &[Scope],
    ) -> LutufiResult<Vec<TabularFactor>> {
        let mut potentials: Vec<TabularFactor> = clique_scopes
            .iter()
            .map(|scope| TabularFactor::identity(scope.clone()))
            .collect::<LutufiResult<_>>()?;

        for cpd in model.cpd_iter() {
            let candidate = cpd.as_factor();
            let clique_index = clique_scopes.iter()
                .position(|scope| scope.contains_all(&candidate.scope().variable_ids().iter().cloned().collect()))
                .ok_or_else(|| LutufiError::InternalError { message: "No clique contains a CPT scope".to_string() })?;
            potentials[clique_index] = potentials[clique_index].multiply(candidate)?;
        }

        Ok(potentials)
    }

    fn scope_intersection_size(a: &Scope, b: &Scope) -> usize {
        let set_a: HashSet<_> = a.variable_ids().iter().copied().collect();
        b.variable_ids().iter().filter(|&&id| set_a.contains(&id)).count()
    }

    fn scope_intersection(a: &Scope, b: &Scope) -> Scope {
        let intersection: BTreeSet<VariableId> = a.variable_ids()
            .iter()
            .chain(b.variable_ids().iter())
            .copied()
            .collect();

        let vars: Vec<VariableId> = intersection.into_iter().filter(|id| a.contains(id) && b.contains(id)).collect();
        let sizes = vars.iter().map(|id| {
            a.size_of(id).or_else(|| b.size_of(id)).unwrap_or(0)
        }).collect();
        Scope::from_ids_and_sizes(vars, sizes)
    }

    fn calibrate(&self, potentials: Vec<TabularFactor>) -> LutufiResult<Vec<TabularFactor>> {
        let original = potentials.clone();
        let mut messages: HashMap<(usize, usize), TabularFactor> = HashMap::new();
        self.collect_messages(0, None, &original, &mut messages)?;
        self.distribute_messages(0, None, &original, &mut messages)?;

        let mut beliefs = original;
        for i in 0..beliefs.len() {
            for &neighbor in &self.adjacency[i] {
                if let Some(msg) = messages.get(&(neighbor, i)) {
                    beliefs[i] = beliefs[i].multiply(msg)?;
                }
            }
        }
        Ok(beliefs)
    }

    fn collect_messages(
        &self,
        node: usize,
        parent: Option<usize>,
        original: &[TabularFactor],
        messages: &mut HashMap<(usize, usize), TabularFactor>,
    ) -> LutufiResult<()> {
        for &child in &self.adjacency[node] {
            if Some(child) == parent {
                continue;
            }
            self.collect_messages(child, Some(node), original, messages)?;
            let separator = self.separator_scopes.get(&(child, node)).ok_or_else(|| LutufiError::InternalError {
                message: format!("Missing separator scope for child={} node={}", child, node),
            })?;
            let message = self.compute_message(child, node, original, messages, separator)?;
            messages.insert((child, node), message);
        }
        Ok(())
    }

    fn distribute_messages(
        &self,
        node: usize,
        parent: Option<usize>,
        original: &[TabularFactor],
        messages: &mut HashMap<(usize, usize), TabularFactor>,
    ) -> LutufiResult<()> {
        for &child in &self.adjacency[node] {
            if Some(child) == parent {
                continue;
            }
            let separator = self.separator_scopes.get(&(node, child)).ok_or_else(|| LutufiError::InternalError {
                message: format!("Missing separator scope for node={} child={}", node, child),
            })?;
            let message = self.compute_message(node, child, original, messages, separator)?;
            messages.insert((node, child), message);
            self.distribute_messages(child, Some(node), original, messages)?;
        }
        Ok(())
    }

    fn compute_message(
        &self,
        sender: usize,
        receiver: usize,
        original: &[TabularFactor],
        messages: &HashMap<(usize, usize), TabularFactor>,
        separator: &Scope,
    ) -> LutufiResult<TabularFactor> {
        let mut product = original[sender].clone();
        for &neighbor in &self.adjacency[sender] {
            if neighbor == receiver { continue; }
            if let Some(msg) = messages.get(&(neighbor, sender)) {
                product = product.multiply(msg)?;
            }
        }

        let vars_to_marginalize: Vec<VariableId> = product.scope().variable_ids()
            .iter()
            .filter(|&&id| !separator.contains(&id))
            .copied()
            .collect();

        let message = product.marginalize(&vars_to_marginalize)?;
        Ok(message)
    }

    fn exact_query_from_beliefs(
        beliefs: Vec<TabularFactor>,
        query_ids: &HashSet<VariableId>,
    ) -> LutufiResult<TabularFactor> {
        let reduced_factors: Vec<TabularFactor> = beliefs.into_iter().filter(|belief| {
            !belief.scope().variable_ids().is_empty()
        }).collect();
        let order = Self::determine_elimination_order_from_factors(&reduced_factors, query_ids)?;
        let factors = Self::eliminate_variables(reduced_factors, &order, false)?;
        let product = Self::multiply_factors(factors)?;
        let mut final_factor = product;
        final_factor.normalize();
        Ok(final_factor)
    }

    fn determine_elimination_order_from_factors(
        factors: &[TabularFactor],
        query_ids: &HashSet<VariableId>,
    ) -> LutufiResult<Vec<VariableId>> {
        let mut all_vars = HashSet::new();
        for factor in factors {
            for &id in factor.scope().variable_ids() {
                if !query_ids.contains(&id) {
                    all_vars.insert(id);
                }
            }
        }

        let mut remaining_vars = all_vars;
        let mut order = Vec::new();
        while !remaining_vars.is_empty() {
            let mut best = *remaining_vars.iter().next().ok_or_else(|| LutufiError::InternalError {
                message: "Empty remaining variables set in elimination order".to_string(),
            })?;
            let mut best_fill = usize::MAX;
            for &var in &remaining_vars {
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
                let neighbors: Vec<_> = neighbors.into_iter().collect();
                for i in 0..neighbors.len() {
                    for j in i + 1..neighbors.len() {
                        let v1 = neighbors[i];
                        let v2 = neighbors[j];
                        if !factors.iter().any(|factor| factor.scope().contains(&v1) && factor.scope().contains(&v2)) {
                            fill += 1;
                        }
                    }
                }
                if fill < best_fill {
                    best_fill = fill;
                    best = var;
                }
            }
            order.push(best);
            remaining_vars.remove(&best);
        }
        Ok(order)
    }

    fn eliminate_variables(
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
            let mut p_iter = containing.into_iter();
            let first = p_iter.next().ok_or_else(|| LutufiError::InternalError {
                message: "Empty containing factors in elimination".to_string(),
            })?;
            let mut product = first;
            for factor in p_iter {
                product = product.multiply(&factor)?;
            }
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

    fn multiply_factors(factors: Vec<TabularFactor>) -> LutufiResult<TabularFactor> {
        if factors.is_empty() {
            return TabularFactor::identity(Scope::from_ids_and_sizes(vec![], vec![]));
        }
        let mut p_iter = factors.into_iter();
        let first = p_iter.next().ok_or_else(|| LutufiError::InternalError {
            message: "Empty factors in multiply_factors".to_string(),
        })?;
        let mut product = first;
        for factor in p_iter {
            product = product.multiply(&factor)?;
        }
        Ok(product)
    }
}
