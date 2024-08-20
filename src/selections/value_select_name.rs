use super::{IntoTableField, ValueSelect, ValueWhere};
use crate::FieldName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Deserialize, Hash)]
pub struct ValueSelectName {
    #[serde(rename = "n")]
    pub name: FieldName,
    #[serde(rename = "vs")]
    pub value_select: Option<ValueSelect>,
}

impl IntoTableField for ValueSelectName {
    fn into_table_field(self) -> super::TableField {
        if let Some(value_select) = self.value_select {
            if let ValueWhere::TableField(tf) = value_select.value_where {
                return tf;
            }
        }
        self.name.into_table_field()
    }
}
