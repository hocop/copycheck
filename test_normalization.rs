#[cfg(test)]
mod tests {
    use super::extract_base_identifier;
    use super::normalize_identifiers;

    #[test]
    fn test_extract_base_identifier() {
        assert_eq!(extract_base_identifier("matrix[0][0]"), "matrix");
        assert_eq!(extract_base_identifier("matrix"), "matrix");
        assert_eq!(extract_base_identifier("matrix[0]"), "matrix");
        assert_eq!(extract_base_identifier("var1"), "var1");
        assert_eq!(extract_base_identifier("data['key']"), "data");
    }

    #[test]
    fn test_normalize_identifiers() {
        // Test with array expressions
        let tokens = vec!["matrix".to_string(), "[".to_string(), "0".to_string(), "]".to_string()];
        let normalized = normalize_identifiers(&tokens);
        assert_eq!(normalized, ["matrix_array".to_string(), "[".to_string(), "<NUM>".to_string(), "]".to_string()]);

        // Test with numeric values
        let tokens = vec!["123".to_string(), "+".to_string(), "456".to_string()];
        let normalized = normalize_identifiers(&tokens);
        assert_eq!(normalized, ["<NUM>".to_string(), "+".to_string(), "<NUM>".to_string()]);

        // Test with normal variables
        let tokens = vec!["var1".to_string(), "=".to_string(), "var2".to_string()];
        let normalized = normalize_identifiers(&tokens);
        assert_eq!(normalized, ["var1".to_string(), "=".to_string(), "var2".to_string()]);
    }
}
