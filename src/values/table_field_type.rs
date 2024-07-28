use crate::{
    selections::{IntoTableField, TableField},
    IntoValueType, ValueType,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TableFieldType {
    pub name: TableField,
    #[serde(rename = "type")]
    pub v_type: ValueType,
}

impl TableFieldType {
    pub fn new(name: impl IntoTableField, v_type: impl IntoValueType) -> Self {
        Self {
            name: name.into_table_field(),
            v_type: v_type.value_type(),
        }
    }
}

impl IntoValueType for TableFieldType {
    fn value_type(&self) -> ValueType {
        self.v_type
    }
}

impl IntoTableField for TableFieldType {
    fn into_table_field(self) -> TableField {
        self.name.into_table_field()
    }
}

pub trait IntoTableFieldType {
    fn into_field_name_type(self) -> TableFieldType;
}

impl IntoTableFieldType for TableFieldType {
    fn into_field_name_type(self) -> TableFieldType {
        self
    }
}

impl<F, T> IntoTableFieldType for (T, F)
where
    T: IntoValueType,
    F: IntoTableField,
{
    fn into_field_name_type(self) -> TableFieldType {
        TableFieldType {
            v_type: self.0.value_type(),
            name: self.1.into_table_field(),
        }
    }
}

// impl<T> IntoValueType for Option<T>
// where
//     T: ValueTyped + Clone,
// {
//     fn value_type(&self) -> ValueType {
//         *T::v_type()
//     }
// }
