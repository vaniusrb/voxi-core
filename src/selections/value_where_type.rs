use super::{IntoTableField, IntoValueWhere, ValueWhere};
use crate::{FieldNameType, IntoValueType, ValueType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValueWhereType {
    pub value_where: ValueWhere,
    #[serde(rename = "type")]
    pub v_type: ValueType,
}

impl ValueWhereType {
    pub fn new(value_where: impl IntoValueWhere, v_type: impl IntoValueType) -> Self {
        Self {
            value_where: value_where.into_value_where(),
            v_type: v_type.value_type(),
        }
    }
}

impl IntoValueWhere for ValueWhereType {
    fn into_value_where(self) -> ValueWhere {
        self.value_where
    }
}

impl IntoValueType for ValueWhereType {
    fn value_type(&self) -> ValueType {
        self.v_type
    }
}

pub trait IntoValueWhereType {
    fn into_value_where_type(self) -> ValueWhereType;
}

impl<F, T> IntoValueWhereType for (T, F)
where
    T: IntoValueType,
    F: IntoValueWhere,
{
    fn into_value_where_type(self) -> ValueWhereType {
        ValueWhereType {
            v_type: self.0.value_type(),
            value_where: self.1.into_value_where(),
        }
    }
}

impl IntoValueWhereType for ValueWhereType {
    fn into_value_where_type(self) -> ValueWhereType {
        self
    }
}

impl IntoValueWhereType for FieldNameType {
    fn into_value_where_type(self) -> ValueWhereType {
        ValueWhereType {
            value_where: ValueWhere::TableField(self.name.into_table_field()),
            v_type: self.v_type,
        }
    }
}
