use super::{
    single_select::SingleQuery,
    to_sql::ToSQL,
    value_where::{IntoValueWhere, ValueWhere},
};
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};

/// Represents a list of `ValueWhere`, to be used to represent a list `IN (n1, n2..)`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValuesWhere(Vec<ValueWhere>);

impl ToSQL for ValuesWhere {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLError> {
        let sql: String = self
            .0
            .iter()
            .map(|v| v.to_sql(args_resolver))
            .collect::<Result<Vec<_>, _>>()?
            .join(",");
        Ok(sql)
    }
}

impl<P: IntoValueWhere> IntoValuesWhere for Vec<P> {
    fn into_values_where(self) -> ValuesWhere {
        let values = self
            .into_iter()
            .map(|i| i.into_value_where())
            .collect::<Vec<_>>();
        ValuesWhere(values)
    }
}

pub trait IntoValuesWhere {
    fn into_values_where(self) -> ValuesWhere;
}

/// Represents a list of values, that can be used in "IN" condition.
/// There are two distinct uses, in to represent a value list or a sub-query.
/// ```text
///     IN (1, 2, 3)
///     IN (SELECT FIELD FROM TABLE)
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValuesListWhere {
    List(ValuesWhere),
    SingleSelect(Box<SingleQuery>),
}

impl ToSQL for ValuesListWhere {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLError> {
        match self {
            ValuesListWhere::List(values) => values.to_sql(args_resolver),
            ValuesListWhere::SingleSelect(single_select) => single_select.to_sql(args_resolver),
        }
    }
}

pub trait IntoValuesListWhere {
    fn into_values(self) -> ValuesListWhere;
}

impl<P: IntoValueWhere> IntoValuesListWhere for Vec<P> {
    fn into_values(self) -> ValuesListWhere {
        ValuesListWhere::List(self.into_values_where())
    }
}

impl IntoValuesListWhere for SingleQuery {
    fn into_values(self) -> ValuesListWhere {
        ValuesListWhere::SingleSelect(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::IntoNullableValue;

    use super::*;

    #[test]
    fn test_into_values() {
        let l = vec![
            1i32.into_value_where(),
            2i32.into_value_where(),
            3i32.into_value_where(),
        ]
        .into_values_where();
        let r = vec![
            ValueWhere::LiteralValue(1i32.into_nullable_value()),
            ValueWhere::LiteralValue(2i32.into_nullable_value()),
            ValueWhere::LiteralValue(3i32.into_nullable_value()),
        ];
        assert_eq!(l, ValuesWhere(r));
    }
}
