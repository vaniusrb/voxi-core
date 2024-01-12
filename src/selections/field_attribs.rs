use super::value_type_scale::IntoValueTypeScale;
use super::ValueTypeScale;
use crate::selections::{IntoTableField, IntoValueSelect, TableField, ValueSelect, ValueWhere};
use crate::{FieldName, FieldNameType, IntoFieldName, IntoFieldNameType, ValueTyped};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Deserialize, Hash)]
pub enum Alignment {
    #[default]
    Default,
    Left,
    Right,
}

impl Alignment {
    pub fn is_default(&self) -> bool {
        matches!(self, Alignment::Default)
    }
}

/// Definition for field with name, title, type and nullable attributes
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Deserialize, Hash)]
pub struct FieldAttribs {
    #[serde(flatten)]
    pub name: FieldName,
    pub title: String,
    pub value_select: Option<ValueSelect>,
    #[serde(rename = "type")]
    pub value_type: ValueTypeScale,
    pub nullable: bool,
    #[serde(
        skip_serializing_if = "Alignment::is_default",
        default = "Alignment::default"
    )]
    pub alignment: Alignment,
}

impl FieldAttribs {
    /// Create a `FieldAttribs` definition
    pub fn new<T: ValueTyped>(
        name: impl IntoFieldName,
        title: &str,
        scale: Option<u32>,
        value_select: Option<impl IntoValueSelect>,
    ) -> Self {
        Self {
            name: name.into_field_name(),
            value_type: ValueTypeScale {
                type_: *T::v_type(),
                scale,
            },
            title: title.to_owned(),
            nullable: true,
            alignment: Default::default(),
            value_select: value_select.map(|vs| vs.into_value_select()),
        }
    }

    pub fn new_t(
        name: impl IntoFieldName,
        title: &str,
        value_type: impl IntoValueTypeScale,
        value_select: Option<impl IntoValueSelect>,
    ) -> Self {
        Self {
            name: name.into_field_name(),
            value_type: value_type.into_value_type_scale(),
            title: title.to_owned(),
            nullable: true,
            alignment: Default::default(),
            value_select: value_select.map(|vs| vs.into_value_select()),
        }
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_alignment_right(mut self) -> Self {
        self.alignment = Alignment::Right;
        self
    }

    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }
}

// TODO: Add comment
pub trait IntoFieldAttribs {
    fn into_field_attribs(self) -> FieldAttribs;
}

impl IntoFieldAttribs for FieldAttribs {
    fn into_field_attribs(self) -> FieldAttribs {
        self
    }
}

impl IntoFieldAttribs for Arc<FieldAttribs> {
    fn into_field_attribs(self) -> FieldAttribs {
        (*self).clone()
    }
}

impl IntoFieldAttribs for &FieldAttribs {
    fn into_field_attribs(self) -> FieldAttribs {
        self.clone()
    }
}

impl IntoValueSelect for FieldAttribs {
    fn into_value_select(self) -> ValueSelect {
        ValueSelect::new(ValueWhere::FieldName(self.name.into_table_field()))
    }
}

impl IntoFieldName for FieldAttribs {
    fn into_field_name(self) -> FieldName {
        self.name
    }
}

impl IntoTableField for FieldAttribs {
    fn into_table_field(self) -> TableField {
        TableField::new(self.name)
    }
}

impl IntoFieldNameType for FieldAttribs {
    fn into_field_name_type(self) -> FieldNameType {
        FieldNameType {
            name: self.name,
            v_type: self.value_type.type_,
        }
    }
}

impl IntoValueTypeScale for FieldAttribs {
    fn into_value_type_scale(self) -> ValueTypeScale {
        self.value_type
    }
}

#[cfg(test)]
mod tests {
    use crate::selections::FieldAttribs;

    #[test]
    fn test_serialize() {
        let value = FieldAttribs::new::<String>("name", "Title", None, Option::<String>::None);
        let json = serde_json::to_string_pretty(&value).unwrap();
        println!("{json}");
        let exp = r#"{
  "name": "name",
  "title": "Title",
  "type": "String",
  "nullable": true
}"#;
        assert_eq!(json, exp);
    }
}
