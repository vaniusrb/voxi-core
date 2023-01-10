use crate::NullableValue;

use super::args_resolver::ArgsResolver;
use crate::{selections::to_sql::ToSQL, SQLError};

// TODO: add comment
pub struct ArgsResolverString {}

impl ArgsResolverString {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ArgsResolverString {
    fn default() -> Self {
        Self::new()
    }
}

impl ArgsResolver for ArgsResolverString {
    fn add_arg(&mut self, value: NullableValue) -> String {
        value.sql()
    }

    fn add_bind(
        &mut self,
        _bind_name: crate::selections::bind_name::BindName,
    ) -> Option<NullableValue> {
        None
    }
}

/// Expand value arguments (binds) in a plain String
/// # Example
/// ```
/// # use voxi_core::selections::ConditionWhere;
/// # use voxi_core::builder::args_resolver_string::args_to_str;
/// let c1 = ConditionWhere::eq("TEXT_1", "TEXT_2");
/// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' = 'TEXT_2'"#);
/// ```
pub fn args_to_str(to_sql: impl ToSQL) -> Result<String, SQLError> {
    to_sql.to_sql(&mut ArgsResolverString::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FieldName;

    #[test]
    fn test_field_name_to_sql() {
        let mut args_resolver_string = ArgsResolverString::new();
        let field = FieldName::from("ID");
        assert_eq!(field.to_sql(&mut args_resolver_string).unwrap(), r#""ID""#);
        let field = FieldName::from("NAME");
        assert_eq!(
            field.to_sql(&mut args_resolver_string).unwrap(),
            r#""NAME""#
        );
    }
}
