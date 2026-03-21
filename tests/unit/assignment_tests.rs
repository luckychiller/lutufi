//! A (possibly partial) assignment of values to variables.
//!
//! Assignments are used throughout Lutufi to represent:
//! - **Evidence**: observed values provided to inference
//! - **States**: the current configuration when evaluating a factor
//! - **Queries**: partial assignments used to index factor tables
//!
//! An assignment is partial — not every variable needs to be assigned.
//! Unassigned variables are treated as latent during inference.

use lutufi_core::core::{assignment::Assignment, variable::VariableId};

    fn make_id() -> VariableId {
        VariableId::new()
    }

    #[test]
    fn empty_assignment() {
        let a = Assignment::new();
        assert!(a.is_empty());
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn set_and_get() {
        let mut a = Assignment::new();
        let id = make_id();
        a.set(id, "high");
        assert_eq!(a.get(&id), Some("high"));
        assert!(a.is_assigned(&id));
    }

    #[test]
    fn overwrite_existing() {
        let mut a = Assignment::new();
        let id = make_id();
        a.set(id, "low");
        a.set(id, "high"); // overwrite
        assert_eq!(a.get(&id), Some("high"));
        assert_eq!(a.len(), 1); // still only one entry
    }

    #[test]
    fn get_unassigned_returns_none() {
        let a = Assignment::new();
        let id = make_id();
        assert_eq!(a.get(&id), None);
        assert!(!a.is_assigned(&id));
    }

    #[test]
    fn unset_removes_value() {
        let mut a = Assignment::new();
        let id = make_id();
        a.set(id, "medium");
        let removed = a.unset(&id);
        assert_eq!(removed, Some("medium".to_string()));
        assert!(!a.is_assigned(&id));
        assert!(a.is_empty());
    }

    #[test]
    fn iterate_over_empty_assignment() {
        let a = Assignment::new();
        let count = a.iter().count();
        assert_eq!(count, 0); // must not panic
    }

    #[test]
    fn iterate_over_assignment() {
        let mut a = Assignment::new();
        let id1 = make_id();
        let id2 = make_id();
        a.set(id1, "yes");
        a.set(id2, "no");
        assert_eq!(a.len(), 2);
        let count = a.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn merge_assignments() {
        let mut a = Assignment::new();
        let mut b = Assignment::new();
        let id1 = make_id();
        let id2 = make_id();
        a.set(id1, "low");
        b.set(id2, "high");
        a.merge(&b);
        assert_eq!(a.get(&id1), Some("low"));
        assert_eq!(a.get(&id2), Some("high"));
    }

    #[test]
    fn merge_overwrites_conflicts() {
        let mut a = Assignment::new();
        let mut b = Assignment::new();
        let id = make_id();
        a.set(id, "original");
        b.set(id, "replacement");
        a.merge(&b);
        assert_eq!(a.get(&id), Some("replacement"));
    }
