use crate::{builder::args_resolver::ArgsResolver, SQLRoxiError};
use crate::{FieldName, IntoNullableValue, NullableValue, TypedOptionValue, Value};

/// Trait to generate SQL string.
/// Allow define argument (bind) calling `args_resolver.add_arg()`
pub trait ToSQL {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLRoxiError>;
}

impl ToSQL for FieldName {
    fn to_sql(&self, _args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLRoxiError> {
        let lit = self.to_string();
        if lit == "*" {
            return Ok(lit);
        }
        Ok(format!(r#""{self}""#))
    }
}

impl ToSQL for TypedOptionValue {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLRoxiError> {
        if let Some(value) = self.opt_value.value().as_ref() {
            Ok(value.to_sql(args_resolver)?)
        } else {
            Ok(String::from("NULL"))
        }
    }
}

impl ToSQL for NullableValue {
    /// `Value` type works like a literal value, then must call `args_resolver`.
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLRoxiError> {
        Ok(args_resolver.add_arg(self.clone()))
    }
}

impl ToSQL for Value {
    /// `Value` type works like a literal value, then must call `args_resolver`.
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLRoxiError> {
        let value = self.clone().into_nullable_value();
        Ok(args_resolver.add_arg(value))
    }
}
