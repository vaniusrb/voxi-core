use super::{
    alias::{Alias, IntoAlias},
    select::{IntoSelect, Select},
    table::IntoTable,
    table_name::{IntoTableName, TableName},
    to_sql::ToSQL,
};
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Definition for FROM SQL clause.
/// Can be defined for a table or a sub-query.
/// ```
/// # use crate::voxi_core::selections::ToSQL;
/// # use voxi_core::selections::FromSelect;
/// # use voxi_core::builder::args_resolver_string::args_to_str;
/// let from = FromSelect::from_table("TABLE");
/// assert_eq!(args_to_str(from).unwrap(), r#""TABLE""#);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromSelect {
    alias: Option<Alias>,
    from_type: FromType,
}

pub trait IntoFromSelect {
    fn into_from_select(self) -> FromSelect;
}

impl IntoFromSelect for &str {
    fn into_from_select(self) -> FromSelect {
        self.into_table_name().into_from_select()
    }
}

impl IntoFromSelect for TableName {
    fn into_from_select(self) -> FromSelect {
        FromSelect {
            alias: None,
            from_type: FromType::Table(self),
        }
    }
}

impl IntoFromSelect for Select {
    fn into_from_select(self) -> FromSelect {
        FromSelect {
            alias: None,
            from_type: FromType::Query(self),
        }
    }
}

impl FromSelect {
    /// Create a `FromSelect` from a `IntoFromSelect` implementation.
    /// ```
    /// # use voxi_core::selections::FromType;
    /// # use voxi_core::selections::FromSelect;
    /// # use voxi_core::selections::QueryBuilder;
    /// let query = QueryBuilder::new().all().from("TABLE").build().unwrap();
    /// let from = FromSelect::from(query.clone());
    /// assert_eq!(from.from_type(), &FromType::Query(query));
    /// ```
    pub fn from(from_select: impl IntoFromSelect) -> Self {
        from_select.into_from_select()
    }

    /// Constructor for `FromSelect`.
    pub fn new(from_type: FromType, alias: Option<Alias>) -> Self {
        Self { from_type, alias }
    }

    /// Define FROM informing a table like source of data.
    /// ```
    /// # use voxi_core::selections::FromSelect;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use voxi_core::builder::args_resolver_string::args_to_str;
    /// let from = FromSelect::from_table("TABLE");
    /// assert_eq!(args_to_str(from).unwrap(), r#""TABLE""#);
    /// ```
    pub fn from_table(table: impl IntoTableName) -> FromSelect {
        table.into_table_name().into_from_select()
    }

    /// Define FROM informing a sub-query like source of data.
    /// ```
    /// # use voxi_core::selections::FromSelect;
    /// # use voxi_core::selections::QueryBuilder;
    /// # use voxi_core::selections::FromType;
    /// let query = QueryBuilder::new().all().from("TABLE").build().unwrap();
    /// let from = FromSelect::from_query(query);
    /// assert_eq!(
    ///     from.from_type(),
    ///     &FromType::Query(QueryBuilder::new().all().from("TABLE").build().unwrap())
    /// );
    /// ```
    pub fn from_query(query: impl IntoSelect) -> FromSelect {
        query.into_select().into_from_select()
    }

    /// Set the table alias for TABLE or QUERY used in the FROM.
    /// ```
    /// # use voxi_core::selections::FromSelect;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use voxi_core::builder::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::TableName;
    /// let table_name = TableName::new("TABLE");
    /// let from = FromSelect::from(table_name).with_alias("TAB");
    /// assert_eq!(args_to_str(from).unwrap(), r#""TABLE" "TAB""#);
    /// ```
    #[must_use]
    pub fn with_alias(mut self, alias: impl IntoAlias) -> Self {
        self.alias = Some(alias.into_alias());
        self
    }

    pub fn tables_names(&self) -> HashSet<&TableName> {
        match &self.from_type {
            FromType::Table(t) => {
                let mut h = HashSet::new();
                h.insert(t);
                h
            }
            FromType::Query(q) => q.tables_names(),
        }
    }

    /// Get a reference to the from select's from type.
    pub fn from_type(&self) -> &FromType {
        &self.from_type
    }

    /// Get a reference to the from select's alias.
    pub fn alias(&self) -> Option<&Alias> {
        self.alias.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FromType {
    Table(TableName),
    Query(Select),
}

pub trait IntoFrom {
    fn into_from(self) -> FromSelect;
}

impl IntoFrom for FromSelect {
    fn into_from(self) -> FromSelect {
        self
    }
}

impl IntoFrom for TableName {
    fn into_from(self) -> FromSelect {
        FromSelect::new(FromType::Table(self), None)
    }
}

impl IntoFrom for &str {
    fn into_from(self) -> FromSelect {
        let table = self.into_table();
        table.into_from()
    }
}

impl IntoFrom for Select {
    fn into_from(self) -> FromSelect {
        FromSelect::new(FromType::Query(self), None)
    }
}

pub struct QueryAlias {
    pub query: Select,
    pub alias: Alias,
}

impl QueryAlias {
    pub fn new(query: impl IntoSelect, alias: impl IntoAlias) -> Self {
        Self {
            alias: alias.into_alias(),
            query: query.into_select(),
        }
    }
}

impl IntoFrom for QueryAlias {
    fn into_from(self) -> FromSelect {
        let QueryAlias { alias, query } = self;
        FromSelect::new(FromType::Query(query), Some(alias))
    }
}

impl ToSQL for FromSelect {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let table = match &self.from_type {
            FromType::Table(t) => t.to_sql(args_resolver)?,
            FromType::Query(q) => format!("({})", q.to_sql(args_resolver)?),
        };
        let sql = match &self.alias {
            Some(alias) => format!(r#"{table} "{alias}""#),
            None => table,
        };
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        resolvers::args_resolver_string::ArgsResolverString, selections::select::QueryBuilder,
    };

    use super::*;

    #[test]
    fn test_from_table_impl() {
        let table_name = TableName::from("TABLE");
        let from = FromSelect::from_table(table_name);
        assert_eq!(from.from_type(), &FromType::Table(TableName::from("TABLE")));
    }

    #[test]
    fn test_from_table_new() {
        let table_name = TableName::from("TABLE");
        let from = FromSelect::from(table_name);
        assert_eq!(from.from_type(), &FromType::Table(TableName::from("TABLE")));
    }

    #[test]
    fn test_to_sql_table() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table_name = TableName::from("TABLE");
        let from = FromSelect::from(table_name);
        assert_eq!(
            from.to_sql(&mut args_resolver_string).unwrap(),
            r#""TABLE""#
        );
    }

    #[test]
    fn test_to_sql_table_alias() {
        let mut args_resolver_string = ArgsResolverString::new();
        let table_name = TableName::from("TABLE");
        let from = FromSelect::from(table_name).with_alias("TAB");
        assert_eq!(
            from.to_sql(&mut args_resolver_string).unwrap(),
            r#""TABLE" "TAB""#
        );
    }

    #[test]
    fn test_from_query_impl() {
        let query = QueryBuilder::new().all().from("TABLE").build().unwrap();
        let from = FromSelect::from_query(query);
        assert_eq!(
            from.from_type(),
            &FromType::Query(QueryBuilder::new().all().from("TABLE").build().unwrap())
        );
    }

    #[test]
    fn test_from_query_new() {
        let query = QueryBuilder::new().all().from("TABLE").build().unwrap();
        let from = FromSelect::from(query.clone());
        assert_eq!(from.from_type(), &FromType::Query(query));
    }

    #[test]
    fn test_to_sql_query() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new().all().from("TABLE").build().unwrap();
        let from = FromSelect::from(query);
        assert_eq!(
            from.to_sql(&mut args_resolver_string).unwrap(),
            r#"(SELECT * FROM "TABLE")"#
        );
    }

    #[test]
    fn test_to_sql_query_alias() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new().all().from("TABLE").build().unwrap();
        let from = FromSelect::from(query).with_alias("TAB");
        assert_eq!(
            from.to_sql(&mut args_resolver_string).unwrap(),
            r#"(SELECT * FROM "TABLE") "TAB""#
        );
    }
}
