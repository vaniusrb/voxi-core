use super::{table_field::TableField, to_sql::ToSQL};
use crate::validate_double_quotes;
use crate::IntoFieldName;
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};
use std::fmt;

/// `TableName` defines the name of a table
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TableName(pub String);

impl TableName {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        validate_double_quotes(&name).unwrap();
        TableName(name)
    }

    /// Create a `TableName`.
    /// ```
    /// # use voxi_core::selections::TableName;
    /// let table_name = TableName::new("TABLE");
    /// assert_eq!(table_name.name(), "TABLE");
    /// ```
    pub fn from(table_name: impl IntoTableName) -> Self {
        table_name.into_table_name()
    }

    /// Get a reference to the table name's name.
    /// ```
    /// # use voxi_core::selections::TableName;
    /// let table_name = TableName::new("TABLE");
    /// assert_eq!(table_name.name(), "TABLE");
    /// ```
    pub fn name(&self) -> &str {
        self.0.as_ref()
    }

    /// Create a TableField from a IntoFieldName.
    pub fn field(&self, field_name: impl IntoFieldName) -> TableField {
        TableField::from(self.clone(), field_name)
    }
}

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToSQL for TableName {
    fn to_sql(
        &self,
        _args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLError> {
        Ok(format!(r#""{self}""#))
    }
}

impl PartialEq<str> for TableName {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<TableName> for str {
    fn eq(&self, other: &TableName) -> bool {
        *self == other.0
    }
}

pub trait IntoTableName {
    fn into_table_name(self) -> TableName;
}

impl IntoTableName for String {
    fn into_table_name(self) -> TableName {
        TableName::new(self)
    }
}

impl IntoTableName for &str {
    fn into_table_name(self) -> TableName {
        TableName::new(self.to_owned())
    }
}

impl IntoTableName for TableName {
    fn into_table_name(self) -> TableName {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolvers::args_resolver_string::ArgsResolverString;

    #[test]
    fn test_into_table_name() {
        let table_name = TableName::from("TABLE");
        assert_eq!(table_name.name(), "TABLE");
    }

    #[test]
    fn test_into_table_equal() {
        let table_name_a = TableName::from("TABLE");
        let table_name_b = TableName::from(String::from("TABLE"));
        assert_eq!(table_name_a, table_name_b);
    }

    #[test]
    fn test_table_to_sql() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table_name = TableName::from("TABLE");
        assert_eq!(
            table_name.to_sql(&mut args_resolver_string).unwrap(),
            "\"TABLE\""
        );
    }

    #[test]
    fn test_table_equal_str() {
        let table_name = TableName::from("TABLE");
        assert_eq!(&table_name, "TABLE");
        assert_eq!("TABLE", &table_name);
    }

    #[test]
    fn test_table_field() {
        let table_name = TableName::from("TABLE");
        let id = table_name.field("ID");
        assert_eq!(id.name(), "ID");
        assert_eq!(id.table().unwrap().name(), "TABLE");
    }
}
