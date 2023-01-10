/// Check if content contain double quote
pub fn validate_double_quotes(content: &str) -> Result<(), String> {
    if content.contains('\"') {
        return Err(String::from("Content cannot contain double quotes!"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_double_quotes_ok_test() {
        let r = validate_double_quotes("TABLE");
        assert_eq!(r, Ok(()));
    }

    #[test]
    fn validate_double_quotes_error_test() {
        let r = validate_double_quotes(r#"TABLE" DELETE FROM TABLE "#);
        assert_eq!(
            r,
            Err(String::from("Content cannot contain double quotes!"))
        );
    }
}
