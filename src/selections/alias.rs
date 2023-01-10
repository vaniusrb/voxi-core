use super::to_sql::ToSQL;
use crate::validate_double_quotes;
use crate::{builder::args_resolver::ArgsResolver, SQLRoxiError};
use core::fmt;
use serde::{Deserialize, Serialize};

/// Alias for table name, field name or query.
/// Cannot contains double quotes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alias {
    alias: String,
}

impl Alias {
    /// Create a alias for table name, field name or query.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::Alias;
    /// let alias = Alias::new("TAB".to_string());
    /// assert_eq!(alias.alias(), "TAB");
    /// ```
    pub fn new(alias: String) -> Self {
        validate_double_quotes(&alias).unwrap();
        Self { alias }
    }

    /// Create a alias for table name, field name or query.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::Alias;
    /// let alias = Alias::from("TAB");
    /// assert_eq!(alias.alias(), "TAB");
    /// ```
    pub fn from(alias: impl IntoAlias) -> Self {
        alias.into_alias()
    }

    /// Get a reference to the alias's alias.
    pub fn alias(&self) -> &str {
        self.alias.as_ref()
    }
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.alias)
    }
}

impl ToSQL for Alias {
    fn to_sql(
        &self,
        _args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLRoxiError> {
        Ok(format!(r#""{}""#, self.alias))
    }
}

pub trait IntoAlias {
    fn into_alias(self) -> Alias;
}

impl IntoAlias for Alias {
    fn into_alias(self) -> Alias {
        self
    }
}

impl IntoAlias for String {
    fn into_alias(self) -> Alias {
        Alias::new(self)
    }
}

impl IntoAlias for &str {
    fn into_alias(self) -> Alias {
        Alias::new(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::args_resolver_string::ArgsResolverString;

    #[test]
    fn test_alias_new() {
        let alias = Alias::new("TAB".to_string());
        assert_eq!(alias.alias(), "TAB");
        let alias = Alias::new("id".to_string());
        assert_eq!(alias.alias(), "id");
    }

    #[test]
    fn test_alias_from() {
        let alias = Alias::from("TAB");
        assert_eq!(alias.alias(), "TAB");
        let alias = Alias::from("id".to_string());
        assert_eq!(alias.alias(), "id");
        let alias = Alias::from(alias);
        assert_eq!(alias.alias(), "id");
    }

    #[test]
    fn test_to_sql() {
        let mut args_resolver_string = ArgsResolverString::new();
        let alias = Alias::from("TAB");
        assert_eq!(alias.to_sql(&mut args_resolver_string).unwrap(), r#""TAB""#);
    }
}
