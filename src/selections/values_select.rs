use super::{IntoValueSelect, TableName, TablesNames, ToSQL, ValueSelect};
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// TODO: add comment
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValuesSelect {
    pub values_select: Vec<ValueSelect>,
}

impl ValuesSelect {
    pub fn new(values_select: Vec<ValueSelect>) -> Self {
        Self { values_select }
    }

    /// Get a reference to the values select's values select.
    pub fn values_select(&self) -> &[ValueSelect] {
        self.values_select.as_ref()
    }

    /// Get owned vec to the values select's values select.
    pub fn into_vec(self) -> Vec<ValueSelect> {
        self.values_select
    }

    pub(crate) fn empty() -> ValuesSelect {
        ValuesSelect {
            values_select: vec![],
        }
    }

    pub(crate) fn push(&mut self, value_select: impl IntoValueSelect) {
        self.values_select.push(value_select.into_value_select());
    }

    pub(crate) fn len(&self) -> usize {
        self.values_select.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait IntoValuesSelect {
    fn into_values_select(self) -> ValuesSelect;
}

impl IntoValuesSelect for ValuesSelect {
    fn into_values_select(self) -> ValuesSelect {
        self
    }
}

impl<T> IntoValuesSelect for Vec<T>
where
    T: IntoValueSelect,
{
    fn into_values_select(self) -> ValuesSelect {
        let values_select = self
            .into_iter()
            .map(|v| v.into_value_select())
            .collect::<Vec<_>>();
        ValuesSelect { values_select }
    }
}

impl<T> IntoValuesSelect for T
where
    T: IntoValueSelect,
{
    fn into_values_select(self) -> ValuesSelect {
        vec![self].into_values_select()
    }
}

impl ToSQL for Vec<ValueSelect> {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let sql = self
            .iter()
            .map(|v| v.to_sql(args_resolver))
            .collect::<Result<Vec<_>, _>>()?
            .join(",");
        Ok(sql)
    }
}

impl TablesNames for Vec<ValueSelect> {
    fn tables_names(&self) -> HashSet<&TableName> {
        self.iter()
            .flat_map(|v| v.tables_names())
            .collect::<HashSet<_>>()
    }
}

impl TablesNames for ValuesSelect {
    fn tables_names(&self) -> HashSet<&TableName> {
        self.values_select.tables_names()
    }
}

impl ToSQL for ValuesSelect {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        self.values_select.to_sql(args_resolver)
    }
}
