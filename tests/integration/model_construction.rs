/// Integration test: construct the Asia network (Lauritzen & Spiegelhalter 1988).
///
/// This is the canonical 8-node medical Bayesian network used as a
/// benchmark throughout the PGM literature. If this test passes,
/// the representation layer is correct.
///
/// Network structure:
///   Asia → Tuberculosis
///   Smoking → Lung Cancer
///   Smoking → Bronchitis
///   Tuberculosis → "Tuberculosis or Cancer" (TbOrCa)
///   Lung Cancer → TbOrCa
///   TbOrCa → Positive X-Ray (XRay)
///   TbOrCa → Dyspnoea
///   Bronchitis → Dyspnoea
///
/// CPT values from the original paper.
#[cfg(test)]
mod asia_network_tests {
    use lutufi_core::core::{
        domain::Domain,
        factor::ConditionalProbabilityTable,
        models::bayesian_network::BayesianNetwork,
    };

    fn build_asia_network() -> BayesianNetwork {
        let mut bn = BayesianNetwork::new();

        // Add all 8 variables
        bn.add_variable("Asia",       Domain::binary()).unwrap();
        bn.add_variable("Tuberculosis", Domain::binary()).unwrap();
        bn.add_variable("Smoking",    Domain::binary()).unwrap();
        bn.add_variable("LungCancer", Domain::binary()).unwrap();
        bn.add_variable("Bronchitis", Domain::binary()).unwrap();
        bn.add_variable("TbOrCa",     Domain::binary()).unwrap();
        bn.add_variable("XRay",       Domain::binary()).unwrap();
        bn.add_variable("Dyspnoea",   Domain::binary()).unwrap();

        // Add edges
        bn.add_edge("Asia",         "Tuberculosis").unwrap();
        bn.add_edge("Smoking",      "LungCancer").unwrap();
        bn.add_edge("Smoking",      "Bronchitis").unwrap();
        bn.add_edge("Tuberculosis", "TbOrCa").unwrap();
        bn.add_edge("LungCancer",   "TbOrCa").unwrap();
        bn.add_edge("TbOrCa",       "XRay").unwrap();
        bn.add_edge("TbOrCa",       "Dyspnoea").unwrap();
        bn.add_edge("Bronchitis",   "Dyspnoea").unwrap();

        // Set CPTs (values from Lauritzen & Spiegelhalter 1988)

        // P(Asia): 0.01 visited Asia, 0.99 didn't
        let asia_var = bn.variable("Asia").unwrap().clone();
        bn.set_cpd("Asia", ConditionalProbabilityTable::from_values(
            &asia_var, &[],
            vec![vec![0.99], vec![0.01]],  // [F, T]
        ).unwrap()).unwrap();

        // P(Smoking): 0.5 smoker, 0.5 non-smoker
        let smoking_var = bn.variable("Smoking").unwrap().clone();
        bn.set_cpd("Smoking", ConditionalProbabilityTable::from_values(
            &smoking_var, &[],
            vec![vec![0.5], vec![0.5]],
        ).unwrap()).unwrap();

        // P(Tuberculosis | Asia):
        //              Asia=F    Asia=T
        // Tb=F          0.99      0.95
        // Tb=T          0.01      0.05
        let tb_var = bn.variable("Tuberculosis").unwrap().clone();
        let asia_var2 = bn.variable("Asia").unwrap().clone();
        bn.set_cpd("Tuberculosis", ConditionalProbabilityTable::from_values(
            &tb_var, &[&asia_var2],
            vec![
                vec![0.99, 0.95],
                vec![0.01, 0.05],
            ],
        ).unwrap()).unwrap();

        // P(LungCancer | Smoking):
        //              Smoking=F  Smoking=T
        // LC=F           0.99       0.90
        // LC=T           0.01       0.10
        let lc_var = bn.variable("LungCancer").unwrap().clone();
        let smoking_var2 = bn.variable("Smoking").unwrap().clone();
        bn.set_cpd("LungCancer", ConditionalProbabilityTable::from_values(
            &lc_var, &[&smoking_var2],
            vec![
                vec![0.99, 0.90],
                vec![0.01, 0.10],
            ],
        ).unwrap()).unwrap();

        // P(Bronchitis | Smoking):
        //              Smoking=F  Smoking=T
        // Br=F           0.70       0.40
        // Br=T           0.30       0.60
        let br_var = bn.variable("Bronchitis").unwrap().clone();
        let smoking_var3 = bn.variable("Smoking").unwrap().clone();
        bn.set_cpd("Bronchitis", ConditionalProbabilityTable::from_values(
            &br_var, &[&smoking_var3],
            vec![
                vec![0.70, 0.40],
                vec![0.30, 0.60],
            ],
        ).unwrap()).unwrap();

        // P(TbOrCa | Tuberculosis, LungCancer):
        // This is a deterministic OR: TbOrCa=T if either Tb=T or LC=T
        //              Tb=F,LC=F  Tb=F,LC=T  Tb=T,LC=F  Tb=T,LC=T
        // TbOrCa=F       1.0        0.0        0.0        0.0
        // TbOrCa=T       0.0        1.0        1.0        1.0
        let tborca_var = bn.variable("TbOrCa").unwrap().clone();
        let tb_var2 = bn.variable("Tuberculosis").unwrap().clone();
        let lc_var2 = bn.variable("LungCancer").unwrap().clone();
        bn.set_cpd("TbOrCa", ConditionalProbabilityTable::from_values(
            &tborca_var, &[&tb_var2, &lc_var2],
            vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 1.0, 1.0],
            ],
        ).unwrap()).unwrap();

        // P(XRay | TbOrCa):
        //              TbOrCa=F  TbOrCa=T
        // XRay=F         0.95      0.02
        // XRay=T         0.05      0.98
        let xray_var = bn.variable("XRay").unwrap().clone();
        let tborca_var2 = bn.variable("TbOrCa").unwrap().clone();
        bn.set_cpd("XRay", ConditionalProbabilityTable::from_values(
            &xray_var, &[&tborca_var2],
            vec![
                vec![0.95, 0.02],
                vec![0.05, 0.98],
            ],
        ).unwrap()).unwrap();

        // P(Dyspnoea | TbOrCa, Bronchitis):
        //              TbOrCa=F,Br=F  TbOrCa=F,Br=T  TbOrCa=T,Br=F  TbOrCa=T,Br=T
        // Dy=F            0.90           0.20           0.30           0.10
        // Dy=T            0.10           0.80           0.70           0.90
        let dy_var = bn.variable("Dyspnoea").unwrap().clone();
        let tborca_var3 = bn.variable("TbOrCa").unwrap().clone();
        let br_var2 = bn.variable("Bronchitis").unwrap().clone();
        bn.set_cpd("Dyspnoea", ConditionalProbabilityTable::from_values(
            &dy_var, &[&tborca_var3, &br_var2],
            vec![
                vec![0.90, 0.20, 0.30, 0.10],
                vec![0.10, 0.80, 0.70, 0.90],
            ],
        ).unwrap()).unwrap();

        bn
    }

    #[test]
    fn asia_network_constructs_without_error() {
        let bn = build_asia_network();
        assert_eq!(bn.nodes().len(), 8);
        assert_eq!(bn.edges().len(), 8);
        assert!(bn.is_valid(), "Asia network should be valid: {:?}", bn.validate());
    }

    #[test]
    fn asia_network_topological_order_is_valid() {
        let bn = build_asia_network();
        let order = bn.topological_order().unwrap();
        assert_eq!(order.len(), 8);

        // In topological order, parents always come before children.
        // Verify this for the known edges.
        let edges = bn.edges();
        for (parent, child) in edges {
            let parent_pos = order.iter().position(|n| *n == parent).unwrap();
            let child_pos = order.iter().position(|n| *n == child).unwrap();
            assert!(
                parent_pos < child_pos,
                "Parent '{}' at position {} should come before child '{}' at position {} \
                 in topological order",
                parent, parent_pos, child, child_pos
            );
        }
    }

    #[test]
    fn asia_network_markov_blanket_dyspnoea() {
        let bn = build_asia_network();
        let blanket = bn.markov_blanket("Dyspnoea").unwrap();
        // Dyspnoea's blanket: TbOrCa (parent), Bronchitis (parent)
        // Dyspnoea has no children in this network.
        assert!(blanket.iter().any(|&n| n == "TbOrCa"));
        assert!(blanket.iter().any(|&n| n == "Bronchitis"));
        assert_eq!(blanket.len(), 2);
    }

    #[test]
    fn asia_network_cycle_detection() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("A", Domain::binary()).unwrap();
        bn.add_variable("B", Domain::binary()).unwrap();
        bn.add_variable("C", Domain::binary()).unwrap();

        bn.add_edge("A", "B").unwrap();
        bn.add_edge("B", "C").unwrap();

        // This edge would create the cycle A → B → C → A
        let result = bn.add_edge("C", "A");
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("cycle"), "Error should mention cycle: {}", msg);
        assert!(msg.contains("DynamicBayesianNetwork") || msg.contains("DBN"),
            "Error should suggest DBN: {}", msg);
    }

    #[test]
    fn cpt_does_not_normalize_gives_informative_error() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("A", Domain::binary()).unwrap();
        let a_var = bn.variable("A").unwrap().clone();

        let result = ConditionalProbabilityTable::from_values(
            &a_var, &[],
            vec![vec![0.3], vec![0.8]], // 0.3 + 0.8 = 1.1
        );

        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("1.1") || msg.contains("1.10"),
            "Error should show the actual sum: {}", msg);
        assert!(msg.contains("A"), "Error should name the variable: {}", msg);
    }
}