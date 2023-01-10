use super::{
    table::{IntoTable, Table},
    to_sql::ToSQL,
};
use crate::{builder::args_resolver::ArgsResolver, SQLRoxiError};
use crate::{FieldName, IntoFieldName};
use serde::{Deserialize, Serialize};
use std::fmt;

/// `TableField` represents a field that can contains a table definition, mainly to be used in `SELECT` or `WHERE` clause
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableField {
    pub table: Option<Table>,
    pub field_name: FieldName,
}

impl IntoFieldName for TableField {
    fn into_field_name(self) -> FieldName {
        self.field_name
    }
}

impl TableField {
    /// Create a `TableField`, that can contais a table definition.
    /// It is possible to inform table delimited by a dot.
    /// ```
    /// # use crate::roxi_sql::selections::ToSQL;
    /// # use roxi_sql::selections::TableField;
    /// # use crate::roxi_sql::builder::args_resolver_string::args_to_str;
    /// let table_field = TableField::new("TABLE.FIELD");
    /// assert_eq!(args_to_str(table_field).unwrap(), r#""TABLE"."FIELD""#);
    /// ```
    pub fn new(table_field: impl IntoTableField) -> Self {
        table_field.into_table_field()
    }

    /// TODO: comment
    pub fn from(table: impl IntoTable, field: impl IntoFieldName) -> Self {
        Self {
            table: Some(table.into_table()),
            field_name: field.into_field_name(),
        }
    }

    /// Get a reference to the table field's table.
    pub fn table(&self) -> Option<&Table> {
        self.table.as_ref()
    }

    /// Get a reference to the table field's name.
    pub fn name(&self) -> &FieldName {
        &self.field_name
    }

    pub fn with_table(mut self, table: impl IntoTable) -> Self {
        self.table = Some(table.into_table());
        self
    }
}

impl fmt::Display for TableField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.table.as_ref() {
            Some(table) => write!(f, "{}.{}", table.name(), self.name()),
            None => write!(f, "{}", self.name()),
        }
    }
}

impl ToSQL for TableField {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLRoxiError> {
        let sql = if let Some(table) = self.table.as_ref() {
            if let Some(alias) = table.alias() {
                format!(
                    "{}.{}",
                    alias.to_sql(args_resolver)?,
                    self.field_name.to_sql(args_resolver)?
                )
            } else {
                format!(
                    "{}.{}",
                    table.to_sql(args_resolver)?,
                    self.field_name.to_sql(args_resolver)?
                )
            }
        } else {
            self.field_name.to_sql(args_resolver)?
        };
        Ok(sql)
    }
}

pub trait IntoTableField {
    fn into_table_field(self) -> TableField;
}

pub trait IntoTablesField {
    fn into_tables_field(self) -> Vec<TableField>;
}

impl IntoTablesField for Vec<TableField> {
    fn into_tables_field(self) -> Vec<TableField> {
        self
    }
}

impl<T> IntoTablesField for T
where
    T: IntoTableField,
{
    fn into_tables_field(self) -> Vec<TableField> {
        vec![self.into_table_field()]
    }
}

impl<T, F> IntoTableField for (T, F)
where
    T: IntoTable,
    F: IntoFieldName,
{
    fn into_table_field(self) -> TableField {
        TableField {
            table: Some(self.0.into_table()),
            field_name: self.1.into_field_name(),
        }
    }
}

impl IntoTableField for &str {
    fn into_table_field(self) -> TableField {
        if self.contains('.') {
            let mut sp = self.split('.');

            let table = sp.next().map(|s| s.into_table()).unwrap();

            let field = sp.next().map(|s| s.into_field_name()).unwrap_or_default();
            TableField {
                table: Some(table),
                field_name: field,
            }
        } else {
            TableField {
                table: None,
                field_name: self.into_field_name(),
            }
        }
    }
}

impl PartialEq<TableField> for str {
    #[allow(clippy::cmp_owned)]
    fn eq(&self, other: &TableField) -> bool {
        other.to_string() == self
    }
}

impl PartialEq<str> for TableField {
    #[allow(clippy::cmp_owned)]
    fn eq(&self, other: &str) -> bool {
        other == self.to_string()
    }
}

impl IntoTableField for String {
    fn into_table_field(self) -> TableField {
        let s = &self[..];
        s.into_table_field()
    }
}

impl IntoTableField for TableField {
    fn into_table_field(self) -> TableField {
        self
    }
}

impl IntoTableField for FieldName {
    fn into_table_field(self) -> TableField {
        TableField {
            table: None,
            field_name: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::args_resolver_string::ArgsResolverString;

    macro_rules! t_field {
        ($a:expr) => {{
            TableField::new($a)
        }};
    }

    // macro_rules! table {
    //     ($a:expr) => {{
    //         Table::new($a)
    //     }};
    // }

    // macro_rules! field {
    //     ($a:expr) => {{
    //         Field::new($a)
    //     }};
    // }

    #[test]
    fn t_field_test() {
        let t = t_field!("TABLE.FIELD");
        assert_eq!(t, TableField::new("TABLE.FIELD"));
    }

    #[test]
    fn test_field_without_table() {
        let table_field = TableField::new("FIELD");
        assert_eq!(table_field.name(), &"FIELD".into_field_name());
        assert_eq!(table_field.table(), None);
    }

    #[test]
    fn test_field_with_table() {
        let table_field = TableField::new("TABLE.FIELD");
        assert_eq!(table_field.name(), &"FIELD".into_field_name());
        assert_eq!(table_field.table(), Some(&Table::new("TABLE")));
    }

    #[test]
    fn test_field_from() {
        let table_field = TableField::from("TABLE TAB", "FIELD");
        assert_eq!(table_field.name(), &"FIELD".into_field_name());
        assert_eq!(table_field.table(), Some(&Table::new("TABLE TAB")));
        assert_eq!(table_field.table().unwrap(), &Table::new("TABLE TAB"));
    }

    #[test]
    fn test_field_to_sql_without_table() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table_field = TableField::new("FIELD");
        assert_eq!(
            table_field.to_sql(&mut args_resolver_string).unwrap(),
            r#""FIELD""#
        );
    }

    #[test]
    fn test_field_to_sql_with_table() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table_field = TableField::new("TABLE.FIELD");
        assert_eq!(
            table_field.to_sql(&mut args_resolver_string).unwrap(),
            r#""TABLE"."FIELD""#
        );
    }

    #[test]
    fn test_field_to_sql_with_table_alias() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table = Table::new("TABLE TAB");
        let table_field = TableField::from(table, "FIELD");
        assert_eq!(
            table_field.to_sql(&mut args_resolver_string).unwrap(),
            r#""TAB"."FIELD""#
        );
    }

    #[test]
    fn test_field_to_string() {
        let table_field_a = TableField::new("table.field");
        assert_eq!(table_field_a.to_string(), "table.field");
        let table_field_b = TableField::new("field");
        assert_eq!(table_field_b.to_string(), "field");
    }
}
