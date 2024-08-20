use super::{FieldNameType, IntoFieldNameType};
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

impl std::fmt::Display for TableFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{} ({})", self.name, self.v_type);
        f.write_str(&s)
    }
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
    fn into_table_field_type(self) -> TableFieldType;
}

impl IntoTableFieldType for TableFieldType {
    fn into_table_field_type(self) -> TableFieldType {
        self
    }
}

impl IntoFieldNameType for TableFieldType {
    fn into_field_name_type(self) -> FieldNameType {
        FieldNameType {
            name: self.name.field_name,
            v_type: self.v_type,
        }
    }
}

impl<F, T> IntoTableFieldType for (T, F)
where
    T: IntoValueType,
    F: IntoTableField,
{
    fn into_table_field_type(self) -> TableFieldType {
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
