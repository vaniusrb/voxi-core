use super::value_type_scale::{DbValueType, IntoDbValueType};
use crate::selections::{IntoTableField, IntoValueSelect, TableField, ValueSelect, ValueWhere};
use crate::{FieldName, FieldNameType, IntoFieldName, IntoFieldNameType, IntoValueType, ValueType};
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
pub struct FieldAttsLimit {
    pub name: FieldName,
    pub title: String,
    pub value_select: Option<ValueSelect>,
    #[serde(rename = "type")]
    #[serde(flatten)]
    pub value_type: DbValueType,
    pub nullable: bool,
    #[serde(
        skip_serializing_if = "Alignment::is_default",
        default = "Alignment::default"
    )]
    pub alignment: Alignment,
}

impl FieldAttsLimit {
    /// Create a `FieldAttribs` definition
    pub fn new(
        value_type: impl IntoDbValueType,
        name: impl IntoFieldName,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
    ) -> Self {
        Self {
            name: name.into_field_name(),
            value_type: value_type.into_db_value_type(),
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

pub trait IntoFieldAttsLimit {
    fn into_field_atts_limit(self) -> FieldAttsLimit;
}

impl IntoFieldAttsLimit for FieldAttsLimit {
    fn into_field_atts_limit(self) -> FieldAttsLimit {
        self
    }
}

impl IntoFieldAttribs for FieldAttsLimit {
    fn into_field_attribs(self) -> FieldAttribs {
        FieldAttribs {
            name: self.name,
            title: self.title,
            value_select: self.value_select,
            value_type: self.value_type.value_type(),
            nullable: self.nullable,
            alignment: self.alignment,
        }
    }
}

impl IntoValueSelect for FieldAttsLimit {
    fn into_value_select(self) -> ValueSelect {
        ValueSelect::new(ValueWhere::FieldName(self.name.into_table_field()))
    }
}

/// Definition for field with name, title, type and nullable attributes
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Deserialize, Hash)]
pub struct FieldAttribs {
    pub name: FieldName,
    pub title: String,
    pub value_select: Option<ValueSelect>,
    #[serde(rename = "type")]
    // #[serde(flatten)]
    pub value_type: ValueType,
    pub nullable: bool,
    #[serde(
        skip_serializing_if = "Alignment::is_default",
        default = "Alignment::default"
    )]
    pub alignment: Alignment,
}

impl FieldAttribs {
    /// Create a `FieldAttribs` definition
    pub fn new(
        value_type: impl IntoValueType,
        name: impl IntoFieldName,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
    ) -> Self {
        Self {
            name: name.into_field_name(),
            value_type: value_type.value_type(),
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
            v_type: self.value_type.value_type(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::selections::{value_type_scale::DbValueType, FieldAttribs};

    #[test]
    fn test_serialize() {
        let value = FieldAttribs::new(
            DbValueType::String(32),
            "name",
            "Title",
            Option::<String>::None,
        );
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
