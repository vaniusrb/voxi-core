use super::{
    agg_functions::AggFunction,
    bind_name::{BindName, IntoBindName},
    combination::Combination,
    from::{FromSelect, IntoFrom},
    group_by::GroupBy,
    join::{IntoJoin, Join},
    logical_expr_where::{IntoLogicalExprWhere, LogicalExprWhere},
    order_by::IntoOrderBy,
    orders::OrdersBy,
    select::Select,
    table_field::IntoTableField,
    table_name::TableName,
    to_sql::ToSQL,
    value_select::{IntoValueSelect, ValueSelect},
    value_where::IntoValueWhere,
    IntoSelect, LimitOffset, Query,
};
use crate::{
    resolvers::{args_resolver::ArgsResolver, args_resolver_string::ArgsResolverString},
    SQLError,
};
use crate::{IntoNullableValue, NullableValue};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt};

// TODO: add comment
pub struct SingleSelectBuilder {
    distinct: bool,
    column: ValueSelect,
    tables: Vec<FromSelect>,
    where_expr: Option<LogicalExprWhere>,
    having_expr: Option<LogicalExprWhere>,
    joins: Vec<Join>,
    groups: Vec<GroupBy>,
    orders_by: OrdersBy,
    combination: Option<Box<Combination>>,
    limit_offset: Option<LimitOffset>,
    binds_values: Vec<(BindName, NullableValue)>,
}

impl SingleSelectBuilder {
    /// Create the `SubQueryBuilder` from a `Query`.
    pub fn from_query(query: impl IntoSelect, column: impl IntoValueSelect) -> Self {
        let query = query.into_select();
        Self {
            distinct: query.distinct,
            column: column.into_value_select(),
            tables: query.from,
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

    /// Create `SingleSelectBuilder` specifying the single column.
    /// ```
    /// # use voxi_core::selections::SingleSelectBuilder;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = SingleSelectBuilder::field("ID")
    ///     .distinct()
    ///     .from("TABLE")
    ///     .build();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT DISTINCT "ID" FROM "TABLE""#);
    /// ```
    pub fn field(column: impl IntoTableField) -> Self {
        Self::select(column.into_table_field())
    }

    /// Create `SingleSelectBuilder` specifying the single column.
    /// ```
    /// # use voxi_core::selections::SingleSelectBuilder;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = SingleSelectBuilder::field("ID")
    ///     .distinct()
    ///     .from("TABLE")
    ///     .build();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT DISTINCT "ID" FROM "TABLE""#);
    /// ```
    pub fn literal(value: impl IntoValueWhere) -> Self {
        Self::select(value.into_value_where().into_value_select())
    }

    /// Create `SingleSelectBuilder` specifying the single column.
    pub fn select(column: impl IntoValueSelect) -> Self {
        Self {
            distinct: false,
            column: column.into_value_select(),
            where_expr: None,
            having_expr: None,
            joins: Vec::new(),
            tables: Vec::new(),
            groups: Vec::new(),
            orders_by: OrdersBy::empty(),
            combination: None,
            limit_offset: None,
            binds_values: Vec::new(),
        }
    }

    /// Define that SELECT must use DISTINCT clause, to returns only distinct record, without duplicates.
    /// ```
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use voxi_core::selections::SingleSelectBuilder;
    /// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = SingleSelectBuilder::field("ID")
    ///     .distinct()
    ///     .from("TABLE")
    ///     .build();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT DISTINCT "ID" FROM "TABLE""#);
    /// ```
    #[must_use]
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// Add column defining SQL function for COUNT(field).
    pub fn count(column: impl IntoTableField) -> Self {
        Self::select(AggFunction::count(column.into_table_field()))
    }

    /// Add column defining SQL function for SUM(field).
    pub fn sum(column: impl IntoTableField) -> Self {
        Self::select(AggFunction::sum(column.into_table_field()))
    }

    /// Add column defining SQL function for MAX(field).
    pub fn max(column: impl IntoTableField) -> Self {
        Self::select(AggFunction::max(column.into_table_field()))
    }

    /// Add column defining SQL function for MAX(field).
    pub fn min(column: impl IntoTableField) -> Self {
        Self::select(AggFunction::min(column.into_table_field()))
    }

    /// Add column defining SQL function for AVG(field).
    pub fn avg(column: impl IntoTableField) -> Self {
        Self::select(AggFunction::avg(column.into_table_field()))
    }

    /// Add table to from part of select.
    /// ```
    /// # use voxi_core::selections::SingleSelectBuilder;
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use crate::voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let query = SingleSelectBuilder::field("ID")
    ///     .distinct()
    ///     .from("TABLE")
    ///     .build();
    /// assert_eq!(args_to_str(query).unwrap(), r#"SELECT DISTINCT "ID" FROM "TABLE""#);
    /// ```
    #[must_use]
    pub fn from(mut self, from: impl IntoFrom) -> Self {
        self.tables.push(from.into_from());
        self
    }

    /// Add join table relation and condition.
    #[must_use]
    pub fn join(mut self, join: impl IntoJoin) -> Self {
        self.joins.push(join.into_join());
        self
    }

    /// Define where condition.
    #[must_use]
    pub fn where_c(self, expression: impl IntoLogicalExprWhere) -> Self {
        Self {
            where_expr: Some(expression.into_logical_expr_where()),
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

    /// Add limit and offset.
    #[must_use]
    pub fn limit_offset(mut self, limit: usize, offset: usize) -> Self {
        self.limit_offset = Some(LimitOffset::new(limit, offset));
        self
    }

    /// Add sort column.
    #[must_use]
    pub fn without_sort(mut self) -> Self {
        self.orders_by.clear();
        self
    }
    /// Add sort column.
    #[must_use]
    pub fn sort(mut self, sort: impl IntoOrderBy) -> Self {
        self.orders_by.push(sort.into_order_by());
        self
    }

    #[must_use]
    pub fn add_bind(mut self, bind_name: impl IntoBindName, value: impl IntoNullableValue) -> Self {
        self.binds_values
            .push((bind_name.into_bind_name(), value.into_nullable_value()));
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
    pub fn build(self) -> SingleQuery {
        let query = Select::new(
            self.distinct,
            vec![self.column],
            self.tables,
            self.joins,
            self.where_expr,
            self.groups,
            self.having_expr,
            self.orders_by,
            self.combination,
            self.limit_offset,
            self.binds_values,
        );
        SingleQuery::new(query)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleQuery {
    query: Select,
}

impl ToSQL for SingleQuery {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        self.query.to_sql(args_resolver)
    }
}
impl Query for SingleQuery {}

impl fmt::Display for SingleQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_sql(&mut ArgsResolverString::new()).unwrap()
        )
    }
}

impl SingleQuery {
    pub fn new(query: Select) -> Self {
        Self { query }
    }

    pub fn tables_names(&self) -> HashSet<&TableName> {
        self.query.tables_names()
    }

    /// Get a reference to the single query's sub query.
    pub fn sub_query(&self) -> &Select {
        &self.query
    }

    /// Get owned sub query.
    pub fn into_select(self) -> Select {
        self.query
    }
    pub fn into_boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resolvers::args_resolver_string::ArgsResolverString,
        selections::{
            condition_where::{ConditionWhere, ConditionWhereOperation},
            from::QueryAlias,
            logical_expr_where::LogicalExprWhereOps,
            table::Table,
        },
    };

    #[test]
    fn test_single_select() {
        let exchange = Table::new("EXCHANGE").with_alias("exc");
        let exc_id = exchange.field("id");
        let exc_name = exchange.field("name");

        let symbol = Table::new("SYMBOL").with_alias("sym");

        let sym_id = symbol.field("id");
        let sym_description = symbol.field("description");
        let sym_name = symbol.field("name");
        let sym_exchange = symbol.field("exchange");

        let mut select_builder = SingleSelectBuilder::select(sym_id.clone())
            .from(symbol)
            .join(Join::inner(exchange, exc_id.equal(sym_exchange)));

        select_builder = select_builder.where_c(
            sym_description
                .equal("USD")
                .and(exc_name.equal("PASSFOLIO")),
        );

        let mut args_resolver_string = ArgsResolverString::new();

        let single_select = select_builder.build();

        println!(
            "1: {}",
            single_select.to_sql(&mut args_resolver_string).unwrap()
        );

        let sub_query = QueryAlias::new(single_select.clone(), "SUB");

        let select_from_sub_query = SingleSelectBuilder::count("*").from(sub_query).build();
        select_from_sub_query
            .to_sql(&mut args_resolver_string)
            .unwrap();
        println!(
            "2: {}",
            select_from_sub_query
                .to_sql(&mut args_resolver_string)
                .unwrap()
        );

        let full_where = sym_id
            .equal(1)
            .or(sym_name.include(vec!["SYMBOL.USD", "SYMBOL.BRL"]))
            .exp()
            .and(ConditionWhere::exists(single_select));

        println!(
            "3: {}",
            full_where.to_sql(&mut args_resolver_string).unwrap()
        );

        // TODO: unit test with and without alias
    }

    #[test]
    fn test_distinct() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = SingleSelectBuilder::field("ID")
            .distinct()
            .from("TABLE")
            .build();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT DISTINCT "ID" FROM "TABLE""#
        );
    }

    #[test]
    fn test_count() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = SingleSelectBuilder::count("ID").from("TABLE").build();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT COUNT("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_max() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = SingleSelectBuilder::max("ID").from("TABLE").build();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT MAX("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_min() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = SingleSelectBuilder::min("ID").from("TABLE").build();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT MIN("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_sum() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = SingleSelectBuilder::sum("ID").from("TABLE").build();
        assert_eq!(
            query.to_sql(&mut args_resolver_string).unwrap(),
            r#"SELECT SUM("ID") FROM "TABLE""#
        );
    }

    #[test]
    fn test_avg() {
        use crate::resolvers::args_resolver_string::args_to_str;
        let query = SingleSelectBuilder::avg("ID").from("TABLE").build();
        assert_eq!(
            args_to_str(query).unwrap(),
            r#"SELECT AVG("ID") FROM "TABLE""#
        );
    }
}
