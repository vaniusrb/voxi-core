use super::{
    agg_functions::AggFunction,
    bind_name::{BindName, IntoBindName},
    combination::Combination,
    from::{FromSelect, IntoFrom},
    group_by::{GroupBy, IntoGroupBy},
    join::{IntoJoins, Join},
    logical_expr_where::{IntoLogicalExprWhere, LogicalExprWhere},
    order_by::IntoOrderBy,
    orders::{IntoOrdersBy, OrdersBy},
    query::Query,
    single_select::SingleQuery,
    table_field::{IntoTableField, IntoTablesField},
    table_name::TableName,
    tables_names::TablesNames,
    to_sql::ToSQL,
    value_select::{IntoValueSelect, ValueSelect},
    value_where::IntoValueWhere,
    values_select::{IntoValuesSelect, ValuesSelect},
    LimitOffset, LogicalExprWhereOps, ValueWhere,
};
use crate::{
    resolvers::{
        args_resolver::ArgsResolver, args_resolver_binds::ArgsResolverBindsDecorator,
        args_resolver_string::ArgsResolverString,
    },
    SQLError,
};
use crate::{FieldName, IntoNullableValue, NullableValue};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

/// QueryBuilder allow customize creating of a query SQL (SELECT).
/// # Example
/// ```
/// # use voxi_core::selections::QueryBuilder;
/// # use crate::voxi_core::selections::ToSQL;
/// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
/// let query = QueryBuilder::new().field("ID").from("TABLE").build().unwrap();
/// assert_eq!(args_to_str(query).unwrap(), r#"SELECT "ID" FROM "TABLE""#);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct QueryBuilder {
    distinct: bool,
    columns: ValuesSelect,
    from: Vec<FromSelect>,
    joins: Vec<Join>,
    where_expr: Option<LogicalExprWhere>,
    groups: Vec<GroupBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    having_expr: Option<LogicalExprWhere>,
    orders_by: OrdersBy,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    combination: Option<Box<Combination>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    limit_offset: Option<LimitOffset>,
    binds_values: Vec<(BindName, NullableValue)>,
}

impl QueryBuilder {
    /// Create the `SubQueryBuilder` from a `Query`.
    pub fn from_query(query: impl IntoSelect) -> Self {
        let query = query.into_select();
        Self {
            distinct: query.distinct,
            columns: query.columns,
            from: query.from,
            joins: query.joins,
            where_expr: query.where_expr,
            groups: query.groups,
            having_expr: query.having_expr,
            orders_by: query.orders_by,
            combination: query.combination,
            limit_offset: query.limit_offset,
            binds_values: query.binds_values,
        }
    }

    /// Create the `SubQueryBuilder`.
    pub fn new() -> Self {
        Self {
            distinct: false,
            columns: ValuesSelect::empty(),
            from: Vec::new(),
            joins: Vec::new(),
            where_expr: None,
            groups: Vec::new(),
            having_expr: None,
            orders_by: OrdersBy::empty(),
            combination: None,
            limit_offset: None,
            binds_values: Vec::new(),
        }
    }

    /// Add column to select from query.
    /// If informed string will be considered static value.
    /// If you want select field use the method `field` instead.
    #[must_use]
    pub fn select(mut self, columns: impl IntoValuesSelect) -> Self {
        let values_selects = columns.into_values_select();
        for value_select in values_selects.into_vec() {
            self.columns.push(value_select.into_value_select());
        }
        self
    }

    /// Set or replace column to select from query.
    /// If informed string will be considered static value.
    /// If you want select field use the method `field` instead.
    #[must_use]
    pub fn replace_select(mut self, columns: impl IntoValuesSelect) -> Self {
        self.columns = columns.into_values_select();
        self
    }
    /// Add column field name to select from query.
    /// If informed string will be considered field name.
    /// If you want select static value use the method `field` instead.
    /// ```
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use voxi_core::selections::QueryBuilder;
    /// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = QueryBuilder::new().field("ID").from("TABLE").build().unwrap();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT "ID" FROM "TABLE""#);
    /// ```
    #[must_use]
    pub fn field(self, column: impl IntoTablesField) -> Self {
        let tables_field = column.into_tables_field();
        let mut result = self;
        for table_field in tables_field {
            result = result.select(table_field);
        }
        result
    }

    /// Add value literal to select from query.
    /// ```
    /// # use voxi_core::selections::QueryBuilder;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = QueryBuilder::new().literal("TEXT").from("TABLE").build().unwrap();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT 'TEXT' FROM "TABLE""#);
    /// ```
    #[must_use]
    pub fn literal(self, value: impl IntoValueWhere) -> Self {
        let value_select = ValueSelect::new(value.into_value_where());
        self.select(value_select)
    }

    #[must_use]
    pub fn bind(self, name: impl IntoBindName) -> Self {
        let value_select = ValueSelect::new(ValueWhere::BindParameter(name.into_bind_name()));
        self.select(value_select)
    }

    /// Add column defining SQL function for COUNT(field).
    #[must_use]
    pub fn count(self, column: impl IntoTableField) -> Self {
        self.select(AggFunction::count(column.into_table_field()))
    }

    /// Add column defining SQL function for SUM(field).
    #[must_use]
    pub fn sum(self, column: impl IntoTableField) -> Self {
        self.select(AggFunction::sum(column.into_table_field()))
    }

    /// Add column defining SQL function for MAX(field).
    #[must_use]
    pub fn max(self, column: impl IntoTableField) -> Self {
        self.select(AggFunction::max(column.into_table_field()))
    }

    /// Add column defining SQL function for MAX(field).
    #[must_use]
    pub fn min(self, column: impl IntoTableField) -> Self {
        self.select(AggFunction::min(column.into_table_field()))
    }

    /// Add column defining SQL function for AVG(field).
    #[must_use]
    pub fn avg(self, column: impl IntoTableField) -> Self {
        self.select(AggFunction::avg(column.into_table_field()))
    }

    /// Select all columns (SELECT *) from query.
    #[must_use]
    pub fn all(mut self) -> Self {
        self.columns.push(FieldName::from("*").into_value_select());
        self
    }

    /// Add source (table/sub-query) to from part of select.
    /// ```
    /// # use voxi_core::selections::QueryBuilder;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = QueryBuilder::new().field("ID").from("TABLE").build().unwrap();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT "ID" FROM "TABLE""#);
    /// ```
    #[must_use]
    pub fn from(mut self, from: impl IntoFrom) -> Self {
        self.from.push(from.into_from());
        self
    }

    /// Add join table relation and condition.
    #[must_use]
    pub fn join(mut self, join: impl IntoJoins) -> Self {
        let mut nj = join.into_joins();
        self.joins.append(&mut nj);
        self
    }

    /// Define where condition. If already specified then the new condition will added with `AND` operator.
    #[must_use]
    pub fn where_c(self, expression: impl IntoLogicalExprWhere) -> Self {
        let new_exp = expression.into_logical_expr_where();
        let where_expr = match self.where_expr {
            Some(expr) => expr.and(new_exp),
            None => new_exp,
        };
        Self {
            where_expr: Some(where_expr),
            ..self
        }
    }

    /// Define having condition.
    #[must_use]
    pub fn having_c(self, expression: impl IntoLogicalExprWhere) -> Self {
        Self {
            having_expr: Some(expression.into_logical_expr_where()),
            ..self
        }
    }

    /// Define that SELECT must use DISTINCT clause, to returns only distinct record, without duplicates.
    #[must_use]
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// Add order column.
    #[must_use]
    pub fn order(mut self, order_by: impl IntoOrderBy) -> Self {
        self.orders_by.push(order_by.into_order_by());
        self
    }

    /// Add limit and offset.
    #[must_use]
    pub fn limit_offset(mut self, limit: usize, offset: usize) -> Self {
        self.limit_offset = Some(LimitOffset::new(limit, offset));
        self
    }

    /// Add group column.
    #[must_use]
    pub fn group(mut self, group: impl IntoGroupBy) -> Self {
        self.groups.push(group.into_group_by());
        self
    }

    #[must_use]
    pub fn union(mut self, query: impl IntoSelect) -> Self {
        self.combination = Some(Box::new(Combination::union(query)));
        self
    }

    #[must_use]
    pub fn union_all(mut self, query: impl IntoSelect) -> Self {
        self.combination = Some(Box::new(Combination::union_all(query)));
        self
    }

    #[must_use]
    pub fn except(mut self, query: impl IntoSelect) -> Self {
        self.combination = Some(Box::new(Combination::except(query)));
        self
    }

    #[must_use]
    pub fn intersect(mut self, query: impl IntoSelect) -> Self {
        self.combination = Some(Box::new(Combination::intersect(query)));
        self
    }

    #[must_use]
    pub fn add_bind(mut self, bind_name: impl IntoBindName, value: impl IntoNullableValue) -> Self {
        self.binds_values
            .push((bind_name.into_bind_name(), value.into_nullable_value()));
        self
    }

    /// Without sort column.
    #[must_use]
    pub fn without_sort(mut self) -> Self {
        self.orders_by.clear();
        self
    }

    /// Without offset.
    #[must_use]
    pub fn without_offset(mut self) -> Self {
        self.limit_offset = None;
        self
    }

    // TODO: must to use Result because table or column maybe are empty
    /// ```
    /// # use voxi_core::selections::QueryBuilder;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = QueryBuilder::new().field("ID").from("TABLE").build().unwrap();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT "ID" FROM "TABLE""#);
    /// ```
    pub fn build(self) -> Result<Select, SQLError> {
        if self.columns.is_empty() {
            return Err(SQLError::InvalidQueryBuilderConfiguration(
                "no column has been defined".to_string(),
            ));
        }
        if self.from.is_empty() {
            return Err(SQLError::InvalidQueryBuilderConfiguration(
                "no from source (table/sub-query) has been defined".to_string(),
            ));
        }
        let query = Select {
            distinct: self.distinct,
            columns: self.columns,
            from: self.from,
            joins: self.joins,
            where_expr: self.where_expr,
            groups: self.groups,
            having_expr: self.having_expr,
            orders_by: self.orders_by,
            combination: self.combination,
            limit_offset: self.limit_offset,
            binds_values: self.binds_values,
        };
        Ok(query)
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Select {
    pub columns: ValuesSelect,
    #[serde(rename = "from")]
    pub from: Vec<FromSelect>,
    #[serde(rename = "where")]
    pub where_expr: Option<LogicalExprWhere>,
    #[serde(rename = "having")]
    pub having_expr: Option<LogicalExprWhere>,
    #[serde(rename = "join")]
    pub joins: Vec<Join>,
    #[serde(rename = "group_by")]
    pub groups: Vec<GroupBy>,
    #[serde(rename = "order_by")]
    pub orders_by: OrdersBy,
    #[serde(flatten)]
    pub limit_offset: Option<LimitOffset>,
    pub distinct: bool,
    pub combination: Option<Box<Combination>>,
    pub binds_values: Vec<(BindName, NullableValue)>,
}

impl Query for Select {}

impl ToSQL for Select {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let mut hash = HashMap::new();
        for (bind_name, value) in self.binds_values.iter() {
            hash.insert(bind_name.clone(), value.clone());
        }

        let args_resolver = &mut ArgsResolverBindsDecorator::new(args_resolver, &hash);

        let select_field = self.columns.to_sql(args_resolver)?;

        let from = self
            .from
            .iter()
            .map(|t| t.to_sql(args_resolver))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        let mut full_sql = "SELECT ".to_string();

        if self.distinct {
            full_sql.push_str("DISTINCT ");
        }

        full_sql.push_str(&select_field);

        use std::fmt::Write;
        write!(full_sql, " FROM {from}").unwrap();

        let joins = self
            .joins
            .iter()
            .map(|j| j.to_sql(args_resolver))
            .collect::<Result<Vec<_>, _>>()?
            .join(" ");
        if !joins.is_empty() {
            full_sql.push(' ');
            full_sql.push_str(&joins);
        }

        if let Some(w) = self.where_expr.as_ref() {
            write!(full_sql, " WHERE {}", w.to_sql(args_resolver)?).unwrap();
        }

        if !self.groups.is_empty() {
            let groups = self
                .groups
                .iter()
                .map(|t| t.to_sql(args_resolver))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");
            write!(full_sql, " GROUP BY {groups}").unwrap();
        }

        if let Some(h) = self.having_expr.as_ref() {
            write!(full_sql, " HAVING {}", h.to_sql(args_resolver)?).unwrap();
        }

        if !self.orders_by.is_empty() {
            write!(
                full_sql,
                " ORDER BY {}",
                self.orders_by.to_sql(args_resolver)?
            )
            .unwrap();
        }

        if let Some(limit_offset) = &self.limit_offset {
            full_sql.push(' ');
            full_sql.push_str(&limit_offset.to_sql(args_resolver)?);
        }

        if let Some(combination) = &self.combination {
            full_sql.push(' ');
            full_sql.push_str(&combination.to_sql(args_resolver)?);
        }

        Ok(full_sql)
    }
}

impl Select {
    /// FIXME: remove this constructor!
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        distinct: bool,
        columns: impl IntoValuesSelect,
        tables: Vec<FromSelect>,
        joins: Vec<Join>,
        where_expr: Option<LogicalExprWhere>,
        groups: Vec<GroupBy>,
        having_expr: Option<LogicalExprWhere>,
        orders_by: impl IntoOrdersBy,
        combination: Option<Box<Combination>>,
        limit_offset: Option<LimitOffset>,
        binds_values: Vec<(BindName, NullableValue)>,
    ) -> Self {
        Self {
            distinct,
            columns: columns.into_values_select(),
            from: tables,
            where_expr,
            having_expr,
            joins,
            groups,
            orders_by: orders_by.into_orders_by(),
            combination,
            limit_offset,
            binds_values,
        }
    }

    pub fn tables_names(&self) -> HashSet<&TableName> {
        let tables = match self.where_expr.as_ref() {
            Some(c) => {
                let e: HashSet<&TableName> = self
                    .columns
                    .tables_names()
                    .into_iter()
                    .chain(c.tables_names())
                    .collect();
                e
            }
            None => self.columns.tables_names(),
        };

        // Ignore tables defined in join
        let tables_join = self
            .joins
            .iter()
            .flat_map(|j| j.from().tables_names().into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        tables
            .into_iter()
            .filter(|rt| !tables_join.iter().any(|jt| jt == rt))
            .collect()
    }

    /// Where expression.
    pub fn where_expr(&self) -> Option<&LogicalExprWhere> {
        self.where_expr.as_ref()
    }

    /// From clause.
    pub fn from(&self) -> &[FromSelect] {
        self.from.as_ref()
    }

    pub fn columns(&self) -> &ValuesSelect {
        &self.columns
    }

    pub fn into_boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_sql(&mut ArgsResolverString::new()).unwrap()
        )
    }
}

pub trait IntoSelect {
    fn into_select(self) -> Select;
}

impl IntoSelect for Select {
    fn into_select(self) -> Select {
        self
    }
}

impl IntoSelect for SingleQuery {
    fn into_select(self) -> Select {
        self.into_select()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{selections::ConditionWhereOperation, IntoFieldName};

    #[test]
    fn test_agg_fun() {
        let mut args_resolver_string = ArgsResolverString::new();
        let count = AggFunction::count("*");
        let query = QueryBuilder::new()
            .select(count)
            .from("TABLE")
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT COUNT(*) FROM "TABLE""#
        );
    }

    #[test]
    fn test_distinct() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .distinct()
            .all()
            .from("TABLE")
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT DISTINCT * FROM "TABLE""#
        );
    }

    #[test]
    fn test_error() {
        let result = QueryBuilder::new().from("TABLE").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_field() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .field("ID")
            .from("TABLE")
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT "ID" FROM "TABLE""#
        );
    }

    #[test]
    fn test_literal_text() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .literal("TEXT")
            .from("TABLE")
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT 'TEXT' FROM "TABLE""#
        );
    }

    #[test]
    fn test_literal_number() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .literal(1)
            .from("TABLE")
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT 1 FROM "TABLE""#
        );
    }

    #[test]
    fn test_count() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .count("ID")
            .from("TABLE")
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT COUNT("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_max() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new().max("ID").from("TABLE").build().unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT MAX("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_min() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new().min("ID").from("TABLE").build().unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT MIN("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_sum() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new().sum("ID").from("TABLE").build().unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT SUM("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_avg() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new().avg("ID").from("TABLE").build().unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT AVG("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn where_expr_single() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .avg("ID")
            .from("TABLE")
            .where_c("ID".into_field_name().null())
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT AVG("ID") FROM "TABLE" WHERE "ID" IS NULL"#
        );
    }

    #[test]
    fn where_expr_and() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .avg("ID")
            .from("TABLE")
            .where_c("ID".into_field_name().null())
            .where_c("PRICE".into_field_name().null())
            .build()
            .unwrap();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT AVG("ID") FROM "TABLE" WHERE "ID" IS NULL AND "PRICE" IS NULL"#
        );
    }
}
