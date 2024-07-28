use super::{
    alias::{Alias, IntoAlias},
    from::{FromSelect, FromType, IntoFrom},
    table_field::TableField,
    table_name::TableName,
    to_sql::ToSQL,
};
use crate::IntoFieldName;
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};
use std::fmt;

/// `Table` represents a definition for a table, with a table name and alias (optional).
/// Allow define alias by informing a second word after a space.
/// ```
/// # use voxi_core::selections::Table;
/// let table = Table::new("TABLE tab");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table {
    pub name: TableName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alias: Option<Alias>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.alias.as_ref() {
            Some(alias) => write!(f, "{} AS {}", self.name, alias),
            None => write!(f, "{}", self.name),
        }
    }
}

impl Table {
    /// Create a `Table` from a `IntoTable` implementation.
    /// ```
    /// # use voxi_core::selections::Table;
    /// let table = Table::new("TABLE").with_alias("TAB");
    /// assert_eq!(table.name(), "TABLE");
    /// ```
    pub fn new(name: impl IntoTable) -> Self {
        name.into_table()
    }

    /// Create a `Table` from a `IntoTable` implementation and an alias.
    /// ```
    /// # use voxi_core::selections::Alias;
    /// # use voxi_core::selections::Table;
    /// let table = Table::from("TABLE", "TAB");
    /// assert_eq!(table, "TABLE AS TAB");
    /// assert_eq!(table.name(), "TABLE");
    /// assert_eq!(table.alias(), Some(&Alias::from("TAB")));
    /// ```
    pub fn from(name: impl IntoTable, alias: &str) -> Self {
        name.into_table().with_alias(alias)
    }

    /// Create a field from a table definition
    /// ```
    /// # use voxi_core::selections::Table;
    /// let table = Table::new("TABLE TAB");
    /// let field = table.field("ID");
    /// assert_eq!(field.name(), "ID");
    /// ```
    pub fn field(&self, name: impl IntoFieldName) -> TableField {
        TableField {
            table: Some(self.clone()),
            field_name: name.into_field_name(),
        }
    }

    /// Set the table name's alias.
    /// ```
    /// # use voxi_core::selections::Table;
    /// # use voxi_core::selections::Alias;
    /// let table = Table::new("TABLE").with_alias("TAB");
    /// assert_eq!(table.name(), "TABLE");
    /// assert_eq!(table.alias(), Some(&Alias::from("TAB")));
    /// ```
    #[must_use]
    pub fn with_alias(self, alias: impl IntoAlias) -> Self {
        Self {
            alias: Some(alias.into_alias()),
            ..self
        }
    }

    #[must_use]
    pub fn without_alias(self) -> Self {
        Self {
            alias: None,
            ..self
        }
    }

    /// Get a reference to the table's name.
    /// ```
    /// # use voxi_core::selections::Table;
    /// let table = Table::new("TABLE");
    /// assert_eq!(table.name(), "TABLE");
    /// ```
    pub fn name(&self) -> &TableName {
        &self.name
    }

    /// Get a reference to the table's alias.
    /// ```
    /// # use voxi_core::selections::Table;
    /// # use voxi_core::selections::Alias;
    /// let table = Table::new("TABLE").with_alias("TAB");
    /// assert_eq!(table.alias(), Some(&Alias::from("TAB")));
    /// ```
    pub fn alias(&self) -> Option<&Alias> {
        self.alias.as_ref()
    }
}

impl ToSQL for Table {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let sql = match self.alias.as_ref() {
            Some(alias) => {
                format!(
                    "{} AS {}",
                    self.name.to_sql(args_resolver)?,
                    alias.to_sql(args_resolver)?
                )
            }
            None => self.name.to_sql(args_resolver)?,
        };
        Ok(sql)
    }
}

pub trait IntoTable {
    fn into_table(self) -> Table;
}

impl IntoTable for Table {
    fn into_table(self) -> Table {
        self
    }
}

impl IntoTable for TableName {
    fn into_table(self) -> Table {
        Table::new(self.name())
    }
}

impl IntoTable for TableField {
    fn into_table(self) -> Table {
        self.table.unwrap_or_else(|| Table::new(""))
    }
}

impl IntoTable for &str {
    fn into_table(self) -> Table {
        let mut sp = self.split(' ');
        let name = sp.next().unwrap();

        let mut table = Table {
            name: TableName::from(name),
            alias: None,
        };
        if let Some(alias) = sp.next() {
            table = table.with_alias(alias);
        };
        table
    }
}

impl PartialEq<&str> for Table {
    #[allow(clippy::cmp_owned)]
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<Table> for &str {
    #[allow(clippy::cmp_owned)]
    fn eq(&self, other: &Table) -> bool {
        *self == other.to_string()
    }
}

impl IntoFrom for Table {
    fn into_from(self) -> FromSelect {
        let Table { name, alias } = self;
        FromSelect::new(FromType::Table(name), alias)
    }
}

#[cfg(test)]
mod tests {
    use crate::resolvers::args_resolver_string::ArgsResolverString;

    use super::*;

    #[test]
    fn test_into_table_without_alias() {
        let table = Table::new("TABLE");
        assert_eq!(table.name(), "TABLE");
        assert_eq!(table.alias(), None);
    }

    #[test]
    fn test_into_table_with_alias_new() {
        let table = Table::new("TABLE TAB");
        assert_eq!(table.name(), "TABLE");
        assert_eq!(table.alias(), Some(&Alias::from("TAB")));
    }

    #[test]
    fn test_into_table_with_alias() {
        let table = Table::from("TABLE", "TAB");
        assert_eq!(table.name(), "TABLE");
        assert_eq!(table.alias(), Some(&Alias::from("TAB")));
    }

    #[test]
    fn test_table_with_alias() {
        let table = Table::new("TABLE").with_alias("TAB");
        assert_eq!(table.name(), "TABLE");
        assert_eq!(table.alias(), Some(&Alias::from("TAB")));
    }

    #[test]
    fn test_equal_with_alias() {
        let table = Table::from("TABLE", "TAB");
        assert_eq!(table, "TABLE AS TAB");
        assert_eq!(table.name(), "TABLE");
        assert_eq!(table.alias(), Some(&Alias::from("TAB")));
    }

    #[test]
    fn test_field() {
        let table = Table::new("TABLE TAB");
        let field = table.field("ID");
        assert_eq!(field.name(), "ID");
        assert_eq!(field.table().unwrap().name(), "TABLE");
        assert_eq!(
            field.table().unwrap().alias().unwrap(),
            &Alias::new("TAB".to_string())
        );
    }

    #[test]
    fn test_field_to_sql_without_alias() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table = Table::new("TABLE");
        assert_eq!(
            table.to_sql(&mut args_resolver_string).unwrap(),
            r#""TABLE""#
        );
    }

    #[test]
    fn test_field_to_sql_with_alias() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table = Table::new("TABLE TAB");
        assert_eq!(
            table.to_sql(&mut args_resolver_string).unwrap(),
            r#""TABLE" AS "TAB""#
        );
    }
}
