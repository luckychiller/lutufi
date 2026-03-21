use lutufi_core::core::{domain::Domain, variable::Variable};


#[test]
fn variable_creation() {
    let domain = Domain::discrete(vec!["sick", "healthy"]).unwrap();
    let var = Variable::new("disease", domain);
    assert_eq!(var.name(), "disease");
    assert_eq!(var.domain().size(), Some(2));
}

#[test]
fn variable_ids_are_unique() {
    let d = Domain::binary();
    let v1 = Variable::new("a", d.clone());
    let v2 = Variable::new("a", d.clone()); // same name, different ID
    assert_ne!(v1.id(), v2.id());
}

#[test]
fn variable_validate_value() {
    let domain = Domain::discrete(vec!["yes", "no"]).unwrap();
    let var = Variable::new("v", domain);
    assert!(var.validate_value("yes").is_ok());
    assert!(var.validate_value("no").is_ok());
    let err = var.validate_value("maybe").unwrap_err();
    assert!(format!("{}", err).contains("maybe"));
}
