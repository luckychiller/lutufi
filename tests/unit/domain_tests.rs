use lutufi_core::core::domain::Domain;

    
    #[test]
    fn discrete_domain_creation() {
        let domain = Domain::discrete(vec!["low", "medium", "high"]).unwrap();
        assert_eq!(domain.size(), Some(3));
        assert!(domain.is_discrete());
    }

    #[test]
    fn discrete_domain_empty_fails() {
        let result = Domain::discrete(Vec::<String>::new());
        assert!(result.is_err());
    }

    #[test]
    fn discrete_domain_contains() {
        let domain = Domain::discrete(vec!["yes", "no"]).unwrap();
        assert!(domain.contains("yes"));
        assert!(domain.contains("no"));
        assert!(!domain.contains("maybe"));
        assert!(!domain.contains("Yes")); // case sensitive
    }

    #[test]
    fn discrete_domain_index_of() {
        let domain = Domain::discrete(vec!["a", "b", "c"]).unwrap();
        assert_eq!(domain.index_of("a"), Some(0));
        assert_eq!(domain.index_of("b"), Some(1));
        assert_eq!(domain.index_of("c"), Some(2));
        assert_eq!(domain.index_of("d"), None);
    }

    #[test]
    fn binary_domain_size() {
        let domain = Domain::binary();
        assert_eq!(domain.size(), Some(2));
        assert!(domain.is_discrete());
    }

    #[test]
    fn binary_domain_contains() {
        let domain = Domain::binary();
        assert!(domain.contains("true"));
        assert!(domain.contains("false"));
        assert!(!domain.contains("yes"));
        assert!(!domain.contains("True")); // case sensitive
    }

    #[test]
    fn binary_domain_index_of() {
        let domain = Domain::binary();
        assert_eq!(domain.index_of("false"), Some(0));
        assert_eq!(domain.index_of("true"), Some(1));
        assert_eq!(domain.index_of("yes"), None);
    }

    #[test]
    fn continuous_domain_size_is_none() {
        let domain = Domain::continuous(Some(0.0), Some(1.0));
        assert_eq!(domain.size(), None);
        assert!(!domain.is_discrete());
    }

    #[test]
    fn continuous_domain_contains_bounded() {
        let domain = Domain::continuous(Some(0.0), Some(1.0));
        assert!(domain.contains("0.5"));
        assert!(domain.contains("0.0"));
        assert!(domain.contains("1.0"));
        assert!(!domain.contains("1.5"));
        assert!(!domain.contains("-0.1"));
        assert!(!domain.contains("not_a_number"));
    }

    #[test]
    fn single_state_discrete_domain() {
        // Edge case: a domain with one state is valid
        let domain = Domain::discrete(vec!["only"]).unwrap();
        assert_eq!(domain.size(), Some(1));
        assert!(domain.contains("only"));
    }