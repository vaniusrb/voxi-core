use super::{FieldAttribs, IntoFieldAttribs};
use crate::IntoFieldName;
use crate::ValueTyped;
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
use serde::{Deserialize, Serialize};
use std::{ops::Add, rc::Rc};

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

    pub fn add<T: ValueTyped>(&mut self, name: &str, title: &str) {
        let value_attrib = FieldAttribs {
            name: name.into_field_name(),
            value_type: *T::v_type(),
            title: title.to_owned(),
            nullable: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    pub fn add_nullable<T: ValueTyped>(&mut self, name: &str, title: &str) {
        let value_attrib = FieldAttribs {
            name: name.into_field_name(),
            value_type: *T::v_type(),
            title: title.to_owned(),
            nullable: true,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
    }

    #[must_use]
    pub fn with_add<T: ValueTyped>(mut self, name: &str, title: &str) -> Self {
        let value_attrib = FieldAttribs {
            name: name.into_field_name(),
            value_type: *T::v_type(),
            title: title.to_owned(),
            nullable: false,
            alignment: Default::default(),
        };
        self.fields_attribs.push(value_attrib);
        self
    }

    #[must_use]
    pub fn with_add_nullable<T: ValueTyped>(mut self, name: &str, title: &str) -> Self {
        let value_attrib = FieldAttribs {
            name: name.into_field_name(),
            value_type: *T::v_type(),
            title: title.to_owned(),
            nullable: true,
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
pub struct FieldsAttribs {
    fields_attribs: Vec<FieldAttribs>,
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

impl FieldsAttribs {
    /// Convert to `ValuesSelectAttribs` defining `TableField` from given table name.
    pub fn into_values_select_attribs_table(self, table: &str) -> ValuesSelectAttribs {
        let fs = self
            .to_vec()
            .into_iter()
            .map(|a| {
                ValueSelectAttrib::new_t(
                    &a.name.to_string(),
                    &a.title,
                    TableField::from(table, a.name.clone()),
                    a.value_type,
                )
            })
            .collect::<Vec<_>>();
        ValuesSelectAttribs::new(fs)
    }

    pub fn new(fields_attribs: impl IntoVecFieldAttribs) -> Self {
        Self {
            fields_attribs: fields_attribs.into_vec_field_attribs(),
        }
    }

    pub fn empty() -> Self {
        Self::new(Vec::<FieldAttribs>::new())
    }

    pub fn to_vec(&self) -> Vec<FieldAttribs> {
        self.fields_attribs.clone()
    }

    pub fn to_vec_rc(&self) -> Vec<Rc<FieldAttribs>> {
        self.clone().into_vec_rc()
    }

    pub fn into_vec_rc(self) -> Vec<Rc<FieldAttribs>> {
        self.fields_attribs.into_iter().map(Rc::new).collect()
    }

    /// Try find field attribs by field name.
    pub fn field_attribs_by_name(
        &self,
        name: impl IntoFieldName,
    ) -> Result<FieldAttribs, SQLError> {
        let name = name.into_field_name();
        self.fields_attribs
            .iter()
            .find(|f| f.name == name)
            .cloned()
            .ok_or_else(|| {
                let fields = self
                    .fields_attribs
                    .iter()
                    .map(|k| k.clone().into_field_name().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                SQLError::FieldNameNotFound(name.to_string(), fields)
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

impl ToSQL for FieldsAttribs {
    fn to_sql(&self, _args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let sql = self
            .to_vec()
            .iter()
            // FIXME: put between quotes
            .map(|f| f.name.to_string())
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
            .map(|f| f.name.into_table_field())
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
