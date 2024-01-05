use super::value::ValueTyped;
use crate::{
    selections::{IntoTableField, TableField},
    FieldName, IntoFieldName, IntoValueType, ValueType,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldNameType {
    #[serde(flatten)]
    pub name: FieldName,
    #[serde(rename = "type")]
    pub v_type: ValueType,
}

impl FieldNameType {
    pub fn new(name: impl IntoFieldName, v_type: impl IntoValueType) -> Self {
        Self {
            name: name.into_field_name(),
            v_type: v_type.value_type(),
        }
    }
}

impl IntoValueType for FieldNameType {
    fn value_type(&self) -> ValueType {
        self.v_type
    }
}

impl IntoTableField for FieldNameType {
    fn into_table_field(self) -> TableField {
        self.name.into_table_field()
    }
}

pub trait IntoFieldNameType {
    fn into_field_name_type(self) -> FieldNameType;
}

impl IntoFieldNameType for FieldNameType {
    fn into_field_name_type(self) -> FieldNameType {
        self
    }
}

impl<F, T> IntoFieldNameType for (T, F)
where
    T: IntoValueType,
    F: IntoFieldName,
{
    fn into_field_name_type(self) -> FieldNameType {
        FieldNameType {
            v_type: self.0.value_type(),
            name: self.1.into_field_name(),
        }
    }
}

impl<T> IntoValueType for Option<T>
where
    T: ValueTyped + Clone,
{
    fn value_type(&self) -> ValueType {
        *T::v_type()
    }
}
