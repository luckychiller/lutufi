#[cfg(test)]
mod tests {
    use lutufi_core::core::error::LutufiError;

    #[test]
    fn error_display_messages() {
        let err = LutufiError::VariableNotFound {
            name: "TestVar".to_string(),
            available: "A, B, C".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("TestVar"));
        assert!(msg.contains("A, B, C"));

        let err2 = LutufiError::CyclicGraph {
            from: "A".to_string(),
            to: "B".to_string(),
            cycle: "A -> B -> A".to_string(),
        };
        let msg2 = format!("{}", err2);
        assert!(msg2.contains("A -> B"));
        assert!(msg2.contains("cycle: A -> B -> A"));
    }
}
