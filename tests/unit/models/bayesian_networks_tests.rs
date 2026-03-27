#[test]
fn single_node_self_loop_is_rejected() {
    let mut bn = BayesianNetwork::new();
    bn.add_node("A", Domain::Binary).unwrap();
    let result = bn.add_edge("A", "A");
    assert!(matches!(result, Err(LutufiError::CyclicGraph { .. })));
}

#[test]
fn two_node_cycle_is_rejected() {
    let mut bn = BayesianNetwork::new();
    bn.add_node("A", Domain::Binary).unwrap();
    bn.add_node("B", Domain::Binary).unwrap();
    bn.add_edge("A", "B").unwrap();
    let result = bn.add_edge("B", "A");
    assert!(matches!(result, Err(LutufiError::CyclicGraph { .. })));
}

#[test]
fn three_node_cycle_is_rejected() {
    let mut bn = BayesianNetwork::new();
    bn.add_node("A", Domain::Binary).unwrap();
    bn.add_node("B", Domain::Binary).unwrap();
    bn.add_node("C", Domain::Binary).unwrap();
    bn.add_edge("A", "B").unwrap();
    bn.add_edge("B", "C").unwrap();
    let result = bn.add_edge("C", "A");
    assert!(matches!(result, Err(LutufiError::CyclicGraph { .. })));
}

#[test]
fn cycle_error_message_includes_cycle_path() {
    let mut bn = BayesianNetwork::new();
    bn.add_node("A", Domain::Binary).unwrap();
    bn.add_node("B", Domain::Binary).unwrap();
    bn.add_edge("A", "B").unwrap();
    let err = bn.add_edge("B", "A").unwrap_err();
    let message = err.to_string();
    assert!(message.contains("A"), "Error should mention node A");
    assert!(message.contains("B"), "Error should mention node B");
    assert!(message.contains("DynamicBayesianNetwork"), 
            "Error should suggest using DBN for feedback loops");
}

#[test]
fn diamond_graph_is_valid_dag() {
    // A -> B, A -> C, B -> D, C -> D — this is a valid DAG
    let mut bn = BayesianNetwork::new();
    bn.add_node("A", Domain::Binary).unwrap();
    bn.add_node("B", Domain::Binary).unwrap();
    bn.add_node("C", Domain::Binary).unwrap();
    bn.add_node("D", Domain::Binary).unwrap();
    bn.add_edge("A", "B").unwrap();
    bn.add_edge("A", "C").unwrap();
    bn.add_edge("B", "D").unwrap();
    bn.add_edge("C", "D").unwrap();
    // No error — diamond is valid
}

#[test]
fn topological_order_of_chain_is_correct() {
    let mut bn = BayesianNetwork::new();
    bn.add_node("A", Domain::Binary).unwrap();
    bn.add_node("B", Domain::Binary).unwrap();
    bn.add_node("C", Domain::Binary).unwrap();
    bn.add_edge("A", "B").unwrap();
    bn.add_edge("B", "C").unwrap();
    let order = bn.topological_order().unwrap();
    // A must come before B, B must come before C
    let pos_a = order.iter().position(|n| n == "A").unwrap();
    let pos_b = order.iter().position(|n| n == "B").unwrap();
    let pos_c = order.iter().position(|n| n == "C").unwrap();
    assert!(pos_a < pos_b);
    assert!(pos_b < pos_c);
}

#[test]
fn asia_network_constructs_without_error() {
    // The Asia network from Lauritzen & Spiegelhalter 1988
    // This is a ground truth test — if this fails, the model is wrong
    let mut bn = BayesianNetwork::new();
    bn.add_node("Asia", Domain::Binary).unwrap();
    bn.add_node("Tuberculosis", Domain::Binary).unwrap();
    bn.add_node("Smoking", Domain::Binary).unwrap();
    bn.add_node("LungCancer", Domain::Binary).unwrap();
    bn.add_node("Bronchitis", Domain::Binary).unwrap();
    bn.add_node("TbOrCancer", Domain::Binary).unwrap();
    bn.add_node("XRay", Domain::Binary).unwrap();
    bn.add_node("Dyspnea", Domain::Binary).unwrap();
    
    bn.add_edge("Asia", "Tuberculosis").unwrap();
    bn.add_edge("Smoking", "LungCancer").unwrap();
    bn.add_edge("Smoking", "Bronchitis").unwrap();
    bn.add_edge("Tuberculosis", "TbOrCancer").unwrap();
    bn.add_edge("LungCancer", "TbOrCancer").unwrap();
    bn.add_edge("TbOrCancer", "XRay").unwrap();
    bn.add_edge("TbOrCancer", "Dyspnea").unwrap();
    bn.add_edge("Bronchitis", "Dyspnea").unwrap();
    
    // Model constructs successfully
    assert_eq!(bn.nodes().count(), 8);
    assert_eq!(bn.edges().len(), 8);
}
