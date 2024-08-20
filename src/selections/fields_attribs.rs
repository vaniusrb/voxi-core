use super::{
    FieldAttribs, IntoFieldAttribs, IntoFieldAttsLimit, IntoTableFieldAlias, ValueSelectName,
};
use crate::selections::FieldAttsLimit;
use crate::{
    resolvers::args_resolver::ArgsResolver,
    selections::{
        table_field::IntoTablesField,
        value_select_attrib::{ValueSelectAttrib, ValuesSelectAttribs},
        values_select::{IntoValuesSelect, ValuesSelect},
        IntoTableField, IntoValueSelect, TableField, ToSQL,
    },
    SQLError,
};
use crate::{IntoFieldName, IntoValueType, ValueType};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::sync::Arc;

// TODO: Add comment
pub struct FieldsAttribsBuilder {
    fields_attribs: Vec<FieldAttribs>,
}

impl FieldsAttribsBuilder {
    pub fn new() -> Self {
        Self {
            fields_attribs: vec![],
        }
    }

    /// Create new `FieldsAttribsBuilder` by reusing a existing `FieldAttribs`.
    pub fn from(fields_attribs: impl IntoFieldsAttribs) -> Self {
        Self {
            fields_attribs: fields_attribs.into_fields_attribs().to_vec(),
        }
    }

    pub fn add_str(
        &mut self,
        name: &str,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
        nullable: bool,
    ) {
        let value_attrib = FieldAttribs {
            value_select_name: ValueSelectName {
                name: name.into_field_name(),
                value_select: value_select.map(|vs| vs.into_value_select()),
            },
            value_type: ValueType::String,
            title: title.to_owned(),
            nullable,
            calculated: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    pub fn add_dec(
        &mut self,
        name: &str,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
        nullable: bool,
    ) {
        let value_attrib = FieldAttribs {
            value_select_name: ValueSelectName {
                name: name.into_field_name(),
                value_select: value_select.map(|vs| vs.into_value_select()),
            },
            value_type: ValueType::Decimal,
            title: title.to_owned(),
            nullable,
            calculated: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    pub fn add_dat(
        &mut self,
        name: &str,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
        nullable: bool,
    ) {
        let value_attrib = FieldAttribs {
            value_select_name: ValueSelectName {
                name: name.into_field_name(),
                value_select: value_select.map(|vs| vs.into_value_select()),
            },
            value_type: ValueType::Date,
            title: title.to_owned(),
            nullable,
            calculated: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    pub fn add_tim(
        &mut self,
        name: &str,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
        nullable: bool,
    ) {
        let value_attrib = FieldAttribs {
            value_select_name: ValueSelectName {
                name: name.into_field_name(),
                value_select: value_select.map(|vs| vs.into_value_select()),
            },
            value_type: ValueType::DateTime,
            title: title.to_owned(),
            nullable,
            calculated: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    pub fn add_jso(
        &mut self,
        name: &str,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
        nullable: bool,
    ) {
        let value_attrib = FieldAttribs {
            value_select_name: ValueSelectName {
                name: name.into_field_name(),
                value_select: value_select.map(|vs| vs.into_value_select()),
            },
            value_type: ValueType::Json,
            title: title.to_owned(),
            nullable,
            calculated: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    pub fn add(
        &mut self,
        value_type: impl IntoValueType,
        name: &str,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
        nullable: bool,
    ) {
        let value_attrib = FieldAttribs {
            value_select_name: ValueSelectName {
                name: name.into_field_name(),
                value_select: value_select.map(|vs| vs.into_value_select()),
            },
            value_type: value_type.value_type(),
            title: title.to_owned(),
            nullable,
            calculated: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    pub fn with_add(
        mut self,
        value_type: impl IntoValueType,
        name: &str,
        title: &str,
        value_select: Option<impl IntoValueSelect>,
        nullable: bool,
    ) -> Self {
        let value_attrib = FieldAttribs {
            value_select_name: ValueSelectName {
                name: name.into_field_name(),
                value_select: value_select.map(|vs| vs.into_value_select()),
            },
            value_type: value_type.value_type(),
            title: title.to_owned(),
            nullable,
            calculated: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
        self
    }

    pub fn build(self) -> FieldsAttribs {
        FieldsAttribs::new(self.fields_attribs)
    }
}

impl Default for FieldsAttribsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Add comment
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct FieldsAttsLimit {
    pub fields_attribs: Vec<FieldAttsLimit>,
}

pub trait IntoVecFieldAttsLimit {
    fn into_vec_field_atts_limit(self) -> Vec<FieldAttsLimit>;
}

impl<T: IntoFieldAttsLimit> IntoVecFieldAttsLimit for Vec<T> {
    fn into_vec_field_atts_limit(self) -> Vec<FieldAttsLimit> {
        self.into_iter()
            .map(|fa| fa.into_field_atts_limit())
            .collect()
    }
}

impl FieldsAttsLimit {
    pub fn new(fields_attribs: impl IntoVecFieldAttsLimit) -> Self {
        Self {
            fields_attribs: fields_attribs.into_vec_field_atts_limit(),
        }
    }

    pub fn len(&self) -> usize {
        self.fields_attribs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields_attribs.is_empty()
    }

    // FIXME: ValueSelectAttrib lost its reason to exists, because now FieldAttribs contains a ValueSelect
    /// Convert to `ValuesSelectAttribs` defining `TableField` from given table name.
    pub fn into_values_select_attribs_table(self, table: &str) -> ValuesSelectAttribs {
        let fs = self
            .fields_attribs
            .into_iter()
            .map(|a| {
                ValueSelectAttrib::new(
                    a.value_type,
                    a.value_select_name.name.as_ref(),
                    &a.title,
                    TableField::from(table, a.value_select_name.name.clone()),
                )
            })
            .collect::<Vec<_>>();
        ValuesSelectAttribs::new(fs)
    }
}

// TODO: Add comment
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct FieldsAttribs {
    pub fields_attribs: Vec<FieldAttribs>,
}

impl std::fmt::Display for FieldsAttribs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .fields_attribs
            .iter()
            .enumerate()
            .map(|(i, f)| format!("{i}:{}", f.clone().into_table_field_alias()))
            .collect::<Vec<_>>()
            .join(", ");
        f.write_str(&s)
    }
}

impl FieldsAttribs {
    pub fn new(fields_attribs: impl IntoVecFieldAttribs) -> Self {
        let fields_attribs = fields_attribs.into_vec_field_attribs();

        // for fa in &fields_attribs {
        //     if let Some(value_select) = &fa.value_select_name.value_select {
        //         if let super::ValueWhere::LiteralValue(literal) = &value_select.value_where {
        //             panic!("FieldAttrib with LiteralValue {literal:?}");
        //         }
        //     }
        // }
        Self { fields_attribs }
    }

    pub fn push(&mut self, field_attribs: impl IntoFieldAttribs) {
        self.fields_attribs.push(field_attribs.into_field_attribs());
    }

    pub fn len(&self) -> usize {
        self.fields_attribs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields_attribs.is_empty()
    }

    pub fn empty() -> Self {
        Self::new(Vec::<FieldAttribs>::new())
    }

    pub fn to_vec(&self) -> Vec<FieldAttribs> {
        self.fields_attribs.clone()
    }

    pub fn vec_ref(&self) -> &Vec<FieldAttribs> {
        &self.fields_attribs
    }

    pub fn to_vec_rc(&self) -> Vec<Arc<FieldAttribs>> {
        self.clone().into_vec_rc()
    }

    pub fn into_vec_rc(self) -> Vec<Arc<FieldAttribs>> {
        self.fields_attribs.into_iter().map(Arc::new).collect()
    }

    /// Try find field attribs by field name.
    pub fn field_attribs_by_name(
        &self,
        name: impl IntoFieldName,
    ) -> error_stack::Result<FieldAttribs, SQLError> {
        let name = name.into_field_name();
        self.fields_attribs
            .iter()
            .find(|f| f.value_select_name.name == name)
            .cloned()
            .ok_or_else(|| {
                let fields = self
                    .fields_attribs
                    .iter()
                    .map(|k| k.clone().into_field_name().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                SQLError::FieldNameNotFound(name.to_string(), fields).into()
            })
    }

    pub fn next_field_attribs(&mut self) -> Option<FieldAttribs> {
        if self.fields_attribs.is_empty() {
            return None;
        }
        Some(self.fields_attribs.remove(0))
    }

    pub fn fields_attribs(&self) -> &[FieldAttribs] {
        self.fields_attribs.as_ref()
    }
}

pub trait IntoVecFieldAttribs {
    fn into_vec_field_attribs(self) -> Vec<FieldAttribs>;
}

impl<F> IntoVecFieldAttribs for Vec<F>
where
    F: IntoFieldAttribs,
{
    fn into_vec_field_attribs(self) -> Vec<FieldAttribs> {
        self.into_iter().map(|fa| fa.into_field_attribs()).collect()
    }
}

impl ToSQL for FieldsAttribs {
    fn to_sql(&self, _args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let sql = self
            .to_vec()
            .iter()
            // FIXME: put between quotes
            .map(|f| f.value_select_name.name.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Ok(sql)
    }
}

impl Add<FieldsAttribs> for FieldsAttribs {
    type Output = FieldsAttribs;

    fn add(self, rhs: FieldsAttribs) -> Self::Output {
        let mut a = self.to_vec();
        let mut b = rhs.to_vec();
        a.append(&mut b);
        FieldsAttribs::new(a)
    }
}

pub trait IntoFieldsAttribs {
    fn into_fields_attribs(self) -> FieldsAttribs;
}

impl IntoFieldsAttribs for FieldsAttribs {
    fn into_fields_attribs(self) -> FieldsAttribs {
        self
    }
}

impl IntoFieldsAttribs for &FieldsAttribs {
    fn into_fields_attribs(self) -> FieldsAttribs {
        self.clone()
    }
}

impl<F> IntoFieldsAttribs for Vec<F>
where
    F: IntoFieldAttribs,
{
    fn into_fields_attribs(self) -> FieldsAttribs {
        let vec = self
            .into_iter()
            .map(|v| v.into_field_attribs())
            .collect::<Vec<_>>();
        FieldsAttribs::new(vec)
    }
}

impl IntoTablesField for FieldsAttribs {
    fn into_tables_field(self) -> Vec<TableField> {
        self.into_fields_attribs()
            .to_vec()
            .into_iter()
            .map(|f| f.value_select_name.name.into_table_field())
            .collect::<Vec<_>>()
    }
}

// impl<F> IntoFieldsAttribs for &[F]
// where
//     F: IntoFieldAttribs + Clone,
// {
//     fn into_fields_attribs(self) -> FieldsAttribs {
//         let vec = self
//             .iter()
//             .map(|v| v.clone().into_field_attribs())
//             .collect::<Vec<_>>();
//         FieldsAttribs::new(vec)
//     }
// }

impl<F> IntoFieldsAttribs for &[&F]
where
    F: IntoFieldAttribs + Clone,
{
    fn into_fields_attribs(self) -> FieldsAttribs {
        let vec = self
            .iter()
            .map(|v| (*v).clone().into_field_attribs())
            .collect::<Vec<_>>();
        FieldsAttribs::new(vec)
    }
}

impl IntoFieldsAttribs for FieldAttribs {
    fn into_fields_attribs(self) -> FieldsAttribs {
        FieldsAttribs::new(vec![self])
    }
}

impl IntoValuesSelect for FieldsAttribs {
    fn into_values_select(self) -> ValuesSelect {
        let fields = self
            .to_vec()
            .iter()
            .map(|f| f.clone().into_value_select())
            .collect::<Vec<_>>();
        fields.into_values_select()
    }
}
