#[cfg(test)]
mod tests {
    use lutufi_core::core::{
        domain::Domain,
        models::bayesian_network::BayesianNetwork,
    };

    #[test]
    fn test_d_sep_chain() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("A", Domain::binary()).unwrap();
        bn.add_variable("B", Domain::binary()).unwrap();
        bn.add_variable("C", Domain::binary()).unwrap();
        bn.add_edge("A", "B").unwrap();
        bn.add_edge("B", "C").unwrap();

        // A -> B -> C
        // A and C are NOT d-separated given nothing
        assert!(!bn.d_separated("A", "C", &[]).unwrap());
        // A and C ARE d-separated given B
        assert!(bn.d_separated("A", "C", &["B"]).unwrap());
    }

    #[test]
    fn test_d_sep_fork() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("A", Domain::binary()).unwrap();
        bn.add_variable("B", Domain::binary()).unwrap();
        bn.add_variable("C", Domain::binary()).unwrap();
        bn.add_edge("B", "A").unwrap();
        bn.add_edge("B", "C").unwrap();

        // A <- B -> C
        // A and C are NOT d-separated given nothing
        assert!(!bn.d_separated("A", "C", &[]).unwrap());
        // A and C ARE d-separated given B
        assert!(bn.d_separated("A", "C", &["B"]).unwrap());
    }

    #[test]
    fn test_d_sep_collider() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("A", Domain::binary()).unwrap();
        bn.add_variable("B", Domain::binary()).unwrap();
        bn.add_variable("C", Domain::binary()).unwrap();
        bn.add_edge("A", "C").unwrap();
        bn.add_edge("B", "C").unwrap();

        // A -> C <- B
        // A and B ARE d-separated given nothing
        assert!(bn.d_separated("A", "B", &[]).unwrap());
        // A and B are NOT d-separated given C
        assert!(!bn.d_separated("A", "B", &["C"]).unwrap());

        // Test descendant of collider
        bn.add_variable("D", Domain::binary()).unwrap();
        bn.add_edge("C", "D").unwrap();
        // A and B are NOT d-separated given D
        assert!(!bn.d_separated("A", "B", &["D"]).unwrap());
    }

    #[test]
    fn test_d_sep_asia() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("Asia", Domain::binary()).unwrap();
        bn.add_variable("Tb", Domain::binary()).unwrap();
        bn.add_variable("Smoking", Domain::binary()).unwrap();
        bn.add_variable("Lc", Domain::binary()).unwrap();
        bn.add_variable("Br", Domain::binary()).unwrap();
        bn.add_variable("TbOrCa", Domain::binary()).unwrap();
        bn.add_variable("XRay", Domain::binary()).unwrap();
        bn.add_variable("Dysp", Domain::binary()).unwrap();

        bn.add_edge("Asia", "Tb").unwrap();
        bn.add_edge("Smoking", "Lc").unwrap();
        bn.add_edge("Smoking", "Br").unwrap();
        bn.add_edge("Tb", "TbOrCa").unwrap();
        bn.add_edge("Lc", "TbOrCa").unwrap();
        bn.add_edge("TbOrCa", "XRay").unwrap();
        bn.add_edge("TbOrCa", "Dysp").unwrap();
        bn.add_edge("Br", "Dysp").unwrap();

        // Asia and Smoking are d-separated given nothing
        assert!(bn.d_separated("Asia", "Smoking", &[]).unwrap());
        // Asia and Smoking are NOT d-separated given Dyspnoea (opened by collider TbOrCa)
        assert!(!bn.d_separated("Asia", "Smoking", &["Dysp"]).unwrap());
        // Tb and Br are d-separated given nothing
        assert!(bn.d_separated("Tb", "Br", &[]).unwrap());
        // Tb and Br are NOT d-separated given Dyspnoea
        assert!(!bn.d_separated("Tb", "Br", &["Dysp"]).unwrap());
    }
}
