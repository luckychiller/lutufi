#[cfg(test)]
mod tests {
    use lutufi_core::core::{
        graph::{DirectedVariableGraph, UndirectedVariableGraph},
        variable::VariableId,
    };

    #[test]
    fn directed_graph_basic_ops() {
        let mut g = DirectedVariableGraph::new();
        let v1 = VariableId::new();
        let v2 = VariableId::new();
        
        g.add_node(v1);
        g.add_node(v2);
        assert_eq!(g.node_count(), 2);
        
        g.add_edge(&v1, &v2, "V1", "V2").unwrap();
        assert_eq!(g.edges().len(), 1);
        assert_eq!(g.parents(&v2), vec![v1]);
        assert_eq!(g.children(&v1), vec![v2]);
        
        g.remove_edge(&v1, &v2);
        assert_eq!(g.edges().len(), 0);
    }

    #[test]
    fn directed_graph_cycle_detection() {
        let mut g = DirectedVariableGraph::new();
        let v1 = VariableId::new();
        let v2 = VariableId::new();
        
        g.add_node(v1);
        g.add_node(v2);
        g.add_edge(&v1, &v2, "V1", "V2").unwrap();
        
        let result = g.add_edge(&v2, &v1, "V2", "V1");
        assert!(result.is_err());
    }

    #[test]
    fn undirected_graph_basic_ops() {
        let mut g = UndirectedVariableGraph::new();
        let v1 = VariableId::new();
        let v2 = VariableId::new();
        
        g.add_node(v1);
        g.add_node(v2);
        
        g.add_edge(&v1, &v2);
        assert_eq!(g.neighbors(&v1), vec![v2]);
        assert_eq!(g.neighbors(&v2), vec![v1]);
        
        g.remove_edge(&v1, &v2);
        assert_eq!(g.neighbors(&v1).len(), 0);
    }
}
