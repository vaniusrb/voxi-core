use super::{
    select::{IntoSelect, Select},
    to_sql::ToSQL,
    value_where::{IntoValueWhere, ValueWhere},
    values_where::{IntoValuesListWhere, ValuesListWhere},
    ArithmeticExprWhere,
};
use crate::{builder::args_resolver::ArgsResolver, SQLRoxiError};
use serde::{Deserialize, Serialize};

// TODO: add comments
pub trait ConditionWhereOperation {
    fn diff(self, value: impl IntoValueWhere) -> ConditionWhere;
    fn null(self) -> ConditionWhere;
    fn equal(self, value: impl IntoValueWhere) -> ConditionWhere;
    fn like(self, value: impl IntoValueWhere) -> ConditionWhere;
    fn greater(self, value: impl IntoValueWhere) -> ConditionWhere;
    fn less(self, value: impl IntoValueWhere) -> ConditionWhere;
    fn greater_or_equal(self, value: impl IntoValueWhere) -> ConditionWhere;
    fn less_or_equal(self, value: impl IntoValueWhere) -> ConditionWhere;
    fn include(self, values: impl IntoValuesListWhere) -> ConditionWhere;
}

pub trait IntoConditionWhere {
    fn into_condition_where(self) -> ConditionWhere;
}

impl IntoConditionWhere for ConditionWhere {
    fn into_condition_where(self) -> ConditionWhere {
        self
    }
}

impl<T> ConditionWhereOperation for T
where
    T: IntoValueWhere,
{
    // TODO: add comment
    fn diff(self, into_value: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::df(self, into_value)
    }

    fn equal(self, into_value: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::eq(self, into_value)
    }

    fn like(self, value: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::like(self, value)
    }

    fn greater(self, into_value: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::gt(self, into_value)
    }

    fn less(self, into_value: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::ls(self, into_value)
    }

    fn greater_or_equal(self, into_value: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::ge(self, into_value)
    }

    fn less_or_equal(self, into_value: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::le(self, into_value)
    }

    fn include(self, into_values_condition: impl IntoValuesListWhere) -> ConditionWhere {
        ConditionWhere::inc(self, into_values_condition)
    }

    fn null(self) -> ConditionWhere {
        ConditionWhere::null(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionWhere {
    Expression(ArithmeticExprWhere),
    ConditionEq(ValueWhere, ValueWhere),
    ConditionNull(ValueWhere),
    ConditionDf(ValueWhere, ValueWhere),
    ConditionGt(ValueWhere, ValueWhere),
    ConditionLs(ValueWhere, ValueWhere),
    ConditionGe(ValueWhere, ValueWhere),
    ConditionLe(ValueWhere, ValueWhere),
    ConditionLk(ValueWhere, ValueWhere),
    ConditionIn(ValueWhere, ValuesListWhere),
    ConditionBetween(ValueWhere, ValueWhere, ValueWhere),
    Exists(Box<Select>),
}

impl ToSQL for ConditionWhere {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLRoxiError> {
        let sql = match self {
            ConditionWhere::Expression(f) => format!("{} IS NULL", f.to_sql(args_resolver)?),
            ConditionWhere::ConditionNull(f) => format!("{} IS NULL", f.to_sql(args_resolver)?),
            ConditionWhere::ConditionEq(f, v) => {
                format!(
                    "{} = {}",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionDf(f, v) => {
                format!(
                    "{} <> {}",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionGt(f, v) => {
                format!(
                    "{} > {}",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionLs(f, v) => {
                format!(
                    "{} < {}",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionGe(f, v) => {
                format!(
                    "{} >= {}",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionLe(f, v) => {
                format!(
                    "{} <= {}",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionLk(f, v) => {
                format!(
                    "{} LIKE {}",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionIn(f, v) => {
                format!(
                    "{} IN ({})",
                    f.to_sql(args_resolver)?,
                    v.to_sql(args_resolver)?
                )
            }
            ConditionWhere::ConditionBetween(f, v1, v2) => {
                format!(
                    "{} BETWEEN {} AND {}",
                    f.to_sql(args_resolver)?,
                    v1.to_sql(args_resolver)?,
                    v2.to_sql(args_resolver)?
                )
            }
            ConditionWhere::Exists(s) => format!("EXISTS ({})", s.to_sql(args_resolver)?),
        };
        Ok(sql)
    }
}

impl ConditionWhere {
    /// Condition xxx IS NULL
    pub fn null(into_value_where: impl IntoValueWhere) -> ConditionWhere {
        ConditionWhere::ConditionNull(into_value_where.into_value_where())
    }

    /// Define condition "equal to", like format `<ValueWhere> = <ValueWhere>`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::eq("TEXT_1", "TEXT_2");
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' = 'TEXT_2'"#);
    /// ```
    pub fn eq(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValueWhere,
    ) -> ConditionWhere {
        ConditionWhere::ConditionEq(
            value_where_a.into_value_where(),
            value_where_b.into_value_where(),
        )
    }

    /// Define condition "different from", like format `<ValueWhere> <> <ValueWhere>`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::df("TEXT_1", "TEXT_2");
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' <> 'TEXT_2'"#);
    /// ```
    pub fn df(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValueWhere,
    ) -> ConditionWhere {
        ConditionWhere::ConditionDf(
            value_where_a.into_value_where(),
            value_where_b.into_value_where(),
        )
    }

    /// Define condition "greater than", like format `<ValueWhere> > <ValueWhere>`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::gt("TEXT_1", "TEXT_2");
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' > 'TEXT_2'"#);
    /// ```
    pub fn gt(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValueWhere,
    ) -> ConditionWhere {
        ConditionWhere::ConditionGt(
            value_where_a.into_value_where(),
            value_where_b.into_value_where(),
        )
    }

    /// Define condition "greater or equal than", like format `<ValueWhere> >= <ValueWhere>`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::ge("TEXT_1", "TEXT_2");
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' >= 'TEXT_2'"#);
    /// ```
    pub fn ge(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValueWhere,
    ) -> ConditionWhere {
        ConditionWhere::ConditionGe(
            value_where_a.into_value_where(),
            value_where_b.into_value_where(),
        )
    }

    /// Condition xxx < xxx
    /// Define condition "less than", like format `<ValueWhere> > <ValueWhere>`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::ge("TEXT_1", "TEXT_2");
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' >= 'TEXT_2'"#);
    /// ```
    pub fn ls(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValueWhere,
    ) -> ConditionWhere {
        ConditionWhere::ConditionLs(
            value_where_a.into_value_where(),
            value_where_b.into_value_where(),
        )
    }

    /// Define condition "less or equal than", like format `<ValueWhere> <= <ValueWhere>`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::le("TEXT_1", "TEXT_2");
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' <= 'TEXT_2'"#);
    /// ```
    pub fn le(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValueWhere,
    ) -> ConditionWhere {
        ConditionWhere::ConditionLe(
            value_where_a.into_value_where(),
            value_where_b.into_value_where(),
        )
    }

    /// Define "include in", like format `<ValueWhere> IN (<ValueWhere>,...)`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::inc("TEXT_1", vec!["TEXT_2", "TEXT_3"]);
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' IN ('TEXT_2','TEXT_3')"#);
    /// ```
    pub fn inc(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValuesListWhere,
    ) -> ConditionWhere {
        let values = value_where_b.into_values();
        ConditionWhere::ConditionIn(value_where_a.into_value_where(), values)
    }

    /// Define text condition `LIKE`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::ConditionWhere;
    /// # use roxi_sql::builder::args_resolver_string::args_to_str;
    /// let c1 = ConditionWhere::like("TEXT_1", "TEXT%");
    /// assert_eq!(args_to_str(c1).unwrap(), r#"'TEXT_1' LIKE 'TEXT%'"#);
    /// ```
    pub fn like(
        value_where_a: impl IntoValueWhere,
        value_where_b: impl IntoValueWhere,
    ) -> ConditionWhere {
        ConditionWhere::ConditionLk(
            value_where_a.into_value_where(),
            value_where_b.into_value_where(),
        )
    }

    /// Condition EXISTS (xxx)
    /// Define "exist in sub-query", like format `EXISTS (<query>)`.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::QueryBuilder;
    /// # use roxi_sql::selections::ConditionWhere;
    /// let query = QueryBuilder::new().field("FIELD").from("TABLE").build().unwrap();
    /// let c1 = ConditionWhere::exists(query);
    /// ```
    pub fn exists(into_sub_query: impl IntoSelect) -> ConditionWhere {
        ConditionWhere::Exists(Box::new(into_sub_query.into_select()))
    }
}

#[cfg(test)]
mod tests {
    use crate::FieldName;

    use super::*;
    use crate::{
        builder::args_resolver_string::{args_to_str, ArgsResolverString},
        selections::{
            logical_expr_where::LogicalExprWhere, select::QueryBuilder,
            single_select::SingleSelectBuilder, LogicalExprWhereOps, TableField,
        },
    };

    #[test]
    fn expression_test() {
        let c1 = super::ConditionWhere::eq("id", 1i64);
        let c2 = super::ConditionWhere::inc("name", vec!["USD", "BRL"]);
        let c3 = super::ConditionWhere::gt("price", "1000");

        // (id = 1 or name = "USD" ) and price > 1000;

        let _ = LogicalExprWhere::and(LogicalExprWhere::or(c1, c2), c3);

        let id = FieldName::from("SYMBOL.id");
        let name = FieldName::from("SYMBOL.name");
        let price = FieldName::from("SYMBOL.price");

        //ValueInteger::new(100).add();

        let sql = id
            .equal(1)
            .or(name.include(vec!["SYMBOL.USD", "SYMBOL.BRL"]))
            .exp()
            .and(price.equal("1000"));

        // id.equal(1).or(name().equal("usd"))
        let mut args_resolver_string = ArgsResolverString::new();

        // TODO: remove print and assert_eq
        println!("{}", sql.to_sql(&mut args_resolver_string).unwrap());

        // let v_id = Value::try_from(1i64).unwrap();
        // let v_name = Value::try_from("USD").unwrap();

        // let id = ValueName::new("id");
        // let name = ValueName::new("name");
        // let e = (id & v_id) & (name & v_name);
    }

    #[test]
    fn test_eq() {
        let mut args_resolver_string = ArgsResolverString::new();
        let c1 = ConditionWhere::eq("TEXT_1", "TEXT_2");
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' = 'TEXT_2'"#
        );
        let c2 =
            ConditionWhere::ConditionEq("TEXT_1".into_value_where(), "TEXT_2".into_value_where());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_df() {
        let mut args_resolver_string = ArgsResolverString::new();
        let c1 = ConditionWhere::df("TEXT_1", "TEXT_2");
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' <> 'TEXT_2'"#
        );
        let c2 =
            ConditionWhere::ConditionDf("TEXT_1".into_value_where(), "TEXT_2".into_value_where());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_gt() {
        let mut args_resolver_string = ArgsResolverString::new();
        let c1 = ConditionWhere::gt("TEXT_1", "TEXT_2");
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' > 'TEXT_2'"#
        );
        let c2 =
            ConditionWhere::ConditionGt("TEXT_1".into_value_where(), "TEXT_2".into_value_where());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_ls() {
        let mut args_resolver_string = ArgsResolverString::new();
        let c1 = ConditionWhere::ls("TEXT_1", "TEXT_2");
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' < 'TEXT_2'"#
        );
        let c2 =
            ConditionWhere::ConditionLs("TEXT_1".into_value_where(), "TEXT_2".into_value_where());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_ge() {
        let mut args_resolver_string = ArgsResolverString::new();
        let c1 = ConditionWhere::ge("TEXT_1", "TEXT_2");
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' >= 'TEXT_2'"#
        );
        let c2 =
            ConditionWhere::ConditionGe("TEXT_1".into_value_where(), "TEXT_2".into_value_where());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_le() {
        let mut args_resolver_string = ArgsResolverString::new();
        let c1 = ConditionWhere::le("TEXT_1", "TEXT_2");
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' <= 'TEXT_2'"#
        );
        let c2 =
            ConditionWhere::ConditionLe("TEXT_1".into_value_where(), "TEXT_2".into_value_where());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_null() {
        let mut args_resolver_string = ArgsResolverString::new();
        let c1 = ConditionWhere::null("TEXT_1");
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' IS NULL"#
        );
        let c2 = ConditionWhere::ConditionNull("TEXT_1".into_value_where());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_in_vec() {
        let mut args_resolver_string = ArgsResolverString::new();
        let vec = vec!["TEXT_1", "TEXT_2"];
        let c1 = ConditionWhere::inc("TEXT_1", vec.clone());
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' IN ('TEXT_1','TEXT_2')"#
        );
        let c2 = ConditionWhere::ConditionIn("TEXT_1".into_value_where(), vec.into_values());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_in_sub_query() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = SingleSelectBuilder::literal("TEXT_1").from("TABLE").build();
        let c1 = ConditionWhere::inc("TEXT_1", query.clone());
        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"'TEXT_1' IN (SELECT 'TEXT_1' FROM "TABLE")"#
        );
        let c2 = ConditionWhere::ConditionIn("TEXT_1".into_value_where(), query.into_values());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_exists() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .field("FIELD")
            .from("TABLE")
            .build()
            .unwrap();
        let c1 = ConditionWhere::exists(query.clone());

        assert_eq!(
            c1.to_sql(&mut args_resolver_string).unwrap(),
            r#"EXISTS (SELECT "FIELD" FROM "TABLE")"#
        );
        let c2 = ConditionWhere::Exists(Box::new(query));
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_equal_op() {
        let id = &TableField::new("SYMBOL.id");
        let cond = id.equal(1);
        assert_eq!(
            cond,
            ConditionWhere::ConditionEq(id.into_value_where(), 1.into_value_where())
        );
        assert_eq!(args_to_str(cond).unwrap(), r#""SYMBOL"."id" = 1"#);
    }

    #[test]
    fn test_diff_op() {
        let id = &TableField::new("SYMBOL.id");
        let cond = id.diff(1);
        assert_eq!(
            cond,
            ConditionWhere::ConditionDf(id.into_value_where(), 1.into_value_where())
        );
        assert_eq!(args_to_str(cond).unwrap(), r#""SYMBOL"."id" <> 1"#);
    }

    #[test]
    fn test_greater_op() {
        let id = &TableField::new("SYMBOL.id");
        let cond = id.greater(1);
        assert_eq!(
            cond,
            ConditionWhere::ConditionGt(id.into_value_where(), 1.into_value_where())
        );
        assert_eq!(args_to_str(cond).unwrap(), r#""SYMBOL"."id" > 1"#);
    }

    #[test]
    fn test_less_op() {
        let id = &TableField::new("SYMBOL.id");
        let cond = id.less(1);
        assert_eq!(
            cond,
            ConditionWhere::ConditionLs(id.into_value_where(), 1.into_value_where())
        );
        assert_eq!(args_to_str(cond).unwrap(), r#""SYMBOL"."id" < 1"#);
    }

    #[test]
    fn test_greater_eq_op() {
        let id = &TableField::new("SYMBOL.id");
        let cond = id.greater_or_equal(1);
        assert_eq!(
            cond,
            ConditionWhere::ConditionGe(id.into_value_where(), 1.into_value_where())
        );
        assert_eq!(args_to_str(cond).unwrap(), r#""SYMBOL"."id" >= 1"#);
    }

    #[test]
    fn test_less_eq_op() {
        let id = &TableField::new("SYMBOL.id");
        let cond = id.less_or_equal(1);
        assert_eq!(
            cond,
            ConditionWhere::ConditionLe(id.into_value_where(), 1.into_value_where())
        );
        assert_eq!(args_to_str(cond).unwrap(), r#""SYMBOL"."id" <= 1"#);
    }

    #[test]
    fn test_include_op() {
        let id = &TableField::new("SYMBOL.id");
        let cond = id.include(vec![1, 2, 3]);
        assert_eq!(
            cond,
            ConditionWhere::ConditionIn(
                id.into_value_where(),
                vec![
                    1.into_value_where(),
                    2.into_value_where(),
                    3.into_value_where()
                ]
                .into_values()
            )
        );
        assert_eq!(args_to_str(cond).unwrap(), r#""SYMBOL"."id" IN (1,2,3)"#);
    }
}
