use super::{
    agg_functions::AggFunction,
    bind_name::{BindName, IntoBindName},
    case::{case_condition::CaseCondition, case_value::CaseValue},
    single_select::SingleQuery,
    string_functions::StringFunction,
    table_field::TableField,
    to_sql::ToSQL,
    ArithmeticExprWhere,
};
use crate::IntoNullableValue;
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use crate::{FieldName, NullableValue};
use serde::{Deserialize, Serialize};

// TODO: add comment
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueWhere {
    LiteralValue(NullableValue),
    FieldName(TableField),
    Expression(ArithmeticExprWhere),
    BindParameter(BindName),
    SingleQuery(Box<SingleQuery>),
    AggFunction(AggFunction),
    CaseCondition(Box<CaseCondition>),
    CaseValue(Box<CaseValue>),
    StringFunction(Box<StringFunction>),
}

impl ValueWhere {
    pub fn bind(name: impl IntoBindName) -> ValueWhere {
        ValueWhere::BindParameter(name.into_bind_name())
    }
}

impl ToSQL for ValueWhere {
    // TODO: add comment
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        match self {
            ValueWhere::LiteralValue(v) => v.to_sql(args_resolver),
            ValueWhere::FieldName(f) => f.to_sql(args_resolver),
            ValueWhere::Expression(e) => e.to_sql(args_resolver),
            ValueWhere::CaseCondition(c) => c.to_sql(args_resolver),
            ValueWhere::CaseValue(c) => c.to_sql(args_resolver),
            ValueWhere::AggFunction(f) => f.to_sql(args_resolver),
            ValueWhere::StringFunction(f) => f.to_sql(args_resolver),
            ValueWhere::SingleQuery(sq) => sq.to_sql(args_resolver).map(|s| format!("({s})")),
            ValueWhere::BindParameter(bn) => args_resolver
                .add_bind(bn.clone())
                .ok_or_else(|| SQLError::BindNameNotFound(bn.name().to_string()))?
                .to_sql(args_resolver),
        }
    }
}

pub trait IntoValueWhere {
    fn into_value_where(self) -> ValueWhere;
}

impl IntoValueWhere for ValueWhere {
    fn into_value_where(self) -> ValueWhere {
        self
    }
}

impl IntoValueWhere for TableField {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::FieldName(self)
    }
}

impl IntoValueWhere for &TableField {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::FieldName(self.clone())
    }
}

impl IntoValueWhere for &FieldName {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::FieldName(TableField {
            table: None,
            field_name: self.clone(),
        })
    }
}

impl IntoValueWhere for FieldName {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::FieldName(TableField {
            table: None,
            field_name: self,
        })
    }
}

impl<T> IntoValueWhere for T
where
    T: IntoNullableValue,
{
    fn into_value_where(self) -> ValueWhere {
        let value = self.into_nullable_value();
        ValueWhere::LiteralValue(value)
    }
}

impl IntoValueWhere for ArithmeticExprWhere {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::Expression(self)
    }
}

impl IntoValueWhere for SingleQuery {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::SingleQuery(Box::new(self))
    }
}

impl IntoValueWhere for AggFunction {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::AggFunction(self)
    }
}

impl IntoValueWhere for StringFunction {
    fn into_value_where(self) -> ValueWhere {
        ValueWhere::StringFunction(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::{FieldName, IntoFieldName, IntoNullableValue};

    use crate::selections::{
        agg_functions::AggFunction,
        single_select::SingleSelectBuilder,
        string_functions::StringFunction,
        table_field::{IntoTableField, TableField},
        value_where::{IntoValueWhere, ValueWhere},
        ArithmeticExprWhere, IntoArithmeticExprWhere,
    };

    #[test]
    fn test_into_value_value_where() {
        let value = ValueWhere::LiteralValue("FIELD".into_nullable_value()).into_value_where();
        assert_eq!(
            value,
            ValueWhere::LiteralValue("FIELD".into_nullable_value())
        );
    }

    #[test]
    fn test_into_value_table_field() {
        let value = TableField::new("FIELD".to_string()).into_value_where();
        assert_eq!(
            value,
            ValueWhere::FieldName(TableField::new("FIELD".to_string()))
        );
    }

    #[test]
    fn test_into_value_field_name() {
        let value = FieldName::new("FIELD".to_string()).into_value_where();
        assert_eq!(
            value,
            ValueWhere::FieldName(FieldName::new("FIELD".to_string()).into_table_field())
        );
    }

    #[test]
    fn test_into_value_where_i64() {
        let value = 100i64.into_value_where();
        assert_eq!(
            value,
            ValueWhere::LiteralValue(100i64.into_nullable_value())
        );
    }

    #[test]
    fn test_into_value_where_arithmetic_expr_where() {
        let func =
            ArithmeticExprWhere::add("FIELD_A".into_value_where(), "FIELD_B".into_value_where());
        let value = func.into_value_where();
        assert_eq!(
            value,
            ArithmeticExprWhere::Add(
                "FIELD_A"
                    .into_value_where()
                    .into_arithmetic_expr_where()
                    .boxed(),
                "FIELD_B"
                    .into_value_where()
                    .into_arithmetic_expr_where()
                    .boxed()
            )
            .into_value_where()
        );
    }

    #[test]
    fn test_into_value_where_single_query() {
        let query = SingleSelectBuilder::literal("TEXT").from("TABLE").build();
        let value = query.clone().into_value_where();
        assert_eq!(value, ValueWhere::SingleQuery(Box::new(query)));
    }

    #[test]
    fn test_into_value_where_agg_function() {
        let func = AggFunction::count("PRICE");
        let value = func.clone().into_value_where();
        assert_eq!(value, ValueWhere::AggFunction(func));
    }

    #[test]
    fn test_into_value_where_string_function() {
        let func = StringFunction::Upper("NAME".into_field_name().into_value_where());
        let value = func.clone().into_value_where();
        assert_eq!(value, ValueWhere::StringFunction(Box::new(func)));
    }

    #[test]
    fn test_into_value_where_str() {
        let value = "VALUE".into_value_where();
        assert_eq!(
            value,
            ValueWhere::LiteralValue("VALUE".to_string().into_nullable_value())
        );
    }
}

#[cfg(test)]
mod test_sql {
    use crate::{FieldName, IntoFieldName, IntoNullableValue};

    use crate::{
        resolvers::args_resolver_string::ArgsResolverString,
        selections::{
            agg_functions::AggFunction, single_select::SingleSelectBuilder,
            string_functions::StringFunction, table_field::TableField, to_sql::ToSQL,
            ArithmeticExprWhere, IntoValueWhere, ValueWhere,
        },
    };

    #[test]
    fn test_into_value_literal() {
        let mut args_resolver_string = ArgsResolverString::new();

        let value = ValueWhere::LiteralValue("TEXT".into_nullable_value()).into_value_where();
        assert_eq!(value.to_sql(&mut args_resolver_string).unwrap(), "'TEXT'");
    }

    #[test]
    fn test_into_value_table_field() {
        let mut args_resolver_string = ArgsResolverString::new();

        let value = TableField::new("FIELD".to_string()).into_value_where();
        assert_eq!(
            value.to_sql(&mut args_resolver_string).unwrap(),
            r#""FIELD""#
        );

        let value = TableField::new("TABLE.FIELD".to_string()).into_value_where();
        assert_eq!(
            value.to_sql(&mut args_resolver_string).unwrap(),
            r#""TABLE"."FIELD""#
        );
    }

    #[test]
    fn test_into_value_field_name() {
        let mut args_resolver_string = ArgsResolverString::new();

        let value = FieldName::new("FIELD".to_string()).into_value_where();
        assert_eq!(
            value.to_sql(&mut args_resolver_string).unwrap(),
            r#""FIELD""#
        );
    }

    #[test]
    fn test_into_value_where_i64() {
        let mut args_resolver_string = ArgsResolverString::new();

        let value = 100i64.into_value_where();
        assert_eq!(value.to_sql(&mut args_resolver_string).unwrap(), r#"100"#);
    }

    #[test]
    fn test_into_value_where_arithmetic_expr_where() {
        let mut args_resolver_string = ArgsResolverString::new();

        let value = ArithmeticExprWhere::add(
            "FIELD_A".into_field_name().into_value_where(),
            "FIELD_B".into_field_name().into_value_where(),
        )
        .into_value_where();
        assert_eq!(
            value.to_sql(&mut args_resolver_string).unwrap(),
            r#""FIELD_A" + "FIELD_B""#
        );

        let value =
            ArithmeticExprWhere::add("FIELD_A".into_value_where(), "FIELD_B".into_value_where())
                .into_value_where();
        assert_eq!(
            value.to_sql(&mut args_resolver_string).unwrap(),
            r#"'FIELD_A' + 'FIELD_B'"#
        );

        let value = ArithmeticExprWhere::add(100i64.into_value_where(), 200i64.into_value_where())
            .into_value_where();
        assert_eq!(
            value.to_sql(&mut args_resolver_string).unwrap(),
            r#"100 + 200"#
        );
    }

    #[test]
    fn test_into_value_where_single_query() {
        let query = SingleSelectBuilder::literal("TEXT").from("TABLE").build();
        let value = query.clone().into_value_where();
        assert_eq!(value, ValueWhere::SingleQuery(Box::new(query)));
    }

    #[test]
    fn test_into_value_where_agg_function() {
        let func = AggFunction::count("PRICE");
        let value = func.clone().into_value_where();
        assert_eq!(value, ValueWhere::AggFunction(func));
    }

    #[test]
    fn test_into_value_where_string_function() {
        let func = StringFunction::Upper("NAME".into_field_name().into_value_where());
        let value = func.clone().into_value_where();
        assert_eq!(value, ValueWhere::StringFunction(Box::new(func)));
    }

    #[test]
    fn test_into_value_where_str() {
        let value = "VALUE".into_value_where();
        assert_eq!(
            value,
            ValueWhere::LiteralValue("VALUE".to_string().into_nullable_value())
        );
    }
}
