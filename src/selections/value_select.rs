use super::{
    agg_functions::AggFunction,
    alias::{Alias, IntoAlias},
    case::{case_condition::CaseCondition, case_value::CaseValue},
    single_select::SingleQuery,
    table_field::TableField,
    table_name::TableName,
    tables_names::TablesNames,
    to_sql::ToSQL,
    value_where::ValueWhere,
    ArithmeticExprWhere, IntoValueWhere,
};
use crate::{builder::args_resolver::ArgsResolver, IntoFieldName, SQLRoxiError};
use crate::{FieldName, NullableValue};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Definition for column in SELECT. Could be a literal value or single query, per example.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueSelect {
    pub value_where: ValueWhere,
    pub alias: Option<Alias>,
}

impl ValueSelect {
    pub fn into_value_where(self) -> ValueWhere {
        self.value_where
    }

    pub fn try_field_name(&self) -> Option<FieldName> {
        match self.alias.as_ref() {
            Some(alias) => Some(alias.to_string().into_field_name()),
            None => match &self.value_where {
                ValueWhere::LiteralValue(field_name) => {
                    Some(field_name.to_string().into_field_name())
                }
                ValueWhere::FieldName(field_name) => Some(field_name.clone().into_field_name()),
                _ => None,
            },
        }
    }

    /// Create a `ValueSelect` from a `IntoValueSelect` implementation.
    pub fn from(value_select: impl IntoValueSelect) -> Self {
        value_select.into_value_select()
    }

    pub fn new(value_where: ValueWhere) -> Self {
        Self {
            value_where,
            alias: None,
        }
    }

    /// Set the table alias for TABLE or QUERY used in the FROM.
    #[must_use]
    pub fn with_alias(mut self, alias: impl IntoAlias) -> Self {
        self.alias = Some(alias.into_alias());
        self
    }

    /// Get a reference to the value select's value select type.
    pub fn value_select_type(&self) -> &ValueWhere {
        &self.value_where
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ValueSelectType {
    LiteralValue(NullableValue),
    FieldName(TableField),
    Expression(ArithmeticExprWhere),
    SingleQuery(SingleQuery),
    AggFunction(AggFunction),
    CaseCondition(Box<CaseCondition>),
    CaseValue(Box<CaseValue>),
}

pub trait IntoValueSelect {
    fn into_value_select(self) -> ValueSelect;
}

impl IntoValueSelect for ValueSelect {
    fn into_value_select(self) -> ValueSelect {
        self
    }
}

impl<T> IntoValueSelect for T
where
    T: IntoValueWhere,
{
    fn into_value_select(self) -> ValueSelect {
        ValueSelect {
            value_where: self.into_value_where(),
            alias: None,
        }
    }
}

impl<T> IntoValueSelect for (T, &str)
where
    T: IntoValueWhere,
{
    fn into_value_select(self) -> ValueSelect {
        self.0.into_value_select().with_alias(self.1)
    }
}

impl<T> IntoValueSelect for (T, String)
where
    T: IntoValueWhere,
{
    fn into_value_select(self) -> ValueSelect {
        self.0.into_value_select().with_alias(self.1)
    }
}

impl ToSQL for ValueSelect {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLRoxiError> {
        let column = self.value_select_type().to_sql(args_resolver)?;
        let sql = match &self.alias {
            Some(alias) => format!(r#"{column} AS "{alias}""#),
            None => column,
        };
        Ok(sql)
    }
}

impl TablesNames for ValueSelect {
    fn tables_names(&self) -> HashSet<&TableName> {
        // TODO: implement
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::args_resolver_string::ArgsResolverString;
    use crate::{IntoNullableValue, Value};

    #[test]
    fn test_value_select_type() {
        let v: Value = "1".into();
        let t = ValueSelect::from(v.clone());
        assert_eq!(
            t.value_select_type(),
            &ValueWhere::LiteralValue(v.into_nullable_value())
        );
    }

    #[test]
    fn test_value_select_value() {
        let v: Value = "1".into();
        let t = ValueSelect::from(v.clone());
        assert_eq!(
            t.value_select_type(),
            &ValueWhere::LiteralValue(v.into_nullable_value())
        );
    }

    #[test]
    fn test_value_to_sql_without_alias() {
        let mut args_resolver_string = ArgsResolverString::new();

        let v: Value = "text".into();
        let t = ValueSelect::from(v);
        assert_eq!(t.to_sql(&mut args_resolver_string).unwrap(), r#"'text'"#);

        let v: Value = 1.into();
        let t = ValueSelect::from(v);
        assert_eq!(t.to_sql(&mut args_resolver_string).unwrap(), r#"1"#);
    }

    #[test]
    fn test_value_to_sql_with_alias() {
        let mut args_resolver_string = ArgsResolverString::new();

        let v: Value = "text".into();
        let t = ValueSelect::from(v).with_alias("alias");
        assert_eq!(
            t.to_sql(&mut args_resolver_string).unwrap(),
            r#"'text' AS "alias""#
        );

        let v: Value = 1.into();
        let t = ValueSelect::from(v).with_alias("alias");
        assert_eq!(
            t.to_sql(&mut args_resolver_string).unwrap(),
            r#"1 AS "alias""#
        );
    }
}
