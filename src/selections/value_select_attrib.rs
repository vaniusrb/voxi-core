use super::{
    fields_attribs::FieldsAttsLimit,
    values_select::{IntoValuesSelect, ValuesSelect},
    DbValueType, FieldAttsLimit, IntoDbValueType, IntoValueSelect, ValueSelect,
};
use crate::{
    selections::{FieldsAttribs, IntoFieldsAttribs},
    IntoFieldName,
};
use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, Sub},
    sync::Arc,
};

// FIXME: ValueSelectAttrib lost its reason to exists, because now FieldAttribs contains a ValueSelect

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ValuesSelectAttribs {
    inner: Arc<Vec<ValueSelectAttrib>>,
}

impl ValuesSelectAttribs {
    pub fn new(values: Vec<ValueSelectAttrib>) -> Self {
        Self {
            inner: Arc::new(values),
        }
    }

    pub fn into_vec(self) -> Vec<ValueSelectAttrib> {
        (*self.inner).clone()
    }

    pub fn as_vec(&self) -> Arc<Vec<ValueSelectAttrib>> {
        self.inner.clone()
    }
}

impl Add<ValuesSelectAttribs> for ValuesSelectAttribs {
    type Output = ValuesSelectAttribs;

    fn add(self, rhs: ValuesSelectAttribs) -> Self::Output {
        let mut s = self.into_vec();
        let mut o = rhs.into_vec();
        s.append(&mut o);
        ValuesSelectAttribs::new(s)
    }
}

impl Sub<ValuesSelectAttribs> for ValuesSelectAttribs {
    type Output = ValuesSelectAttribs;

    fn sub(self, rhs: ValuesSelectAttribs) -> Self::Output {
        let mut s = self.into_vec();
        let o = rhs.into_vec();
        s.retain(|x| !o.contains(x));
        ValuesSelectAttribs::new(s)
    }
}

pub trait IntoValuesSelectAttribs {
    fn into_values_select_attribs(self) -> ValuesSelectAttribs;
}

impl IntoValuesSelectAttribs for ValuesSelectAttribs {
    fn into_values_select_attribs(self) -> ValuesSelectAttribs {
        self
    }
}

impl IntoValuesSelectAttribs for FieldsAttsLimit {
    fn into_values_select_attribs(self) -> ValuesSelectAttribs {
        let values = self
            .fields_attribs
            .into_iter()
            .map(|a| a.into_value_select_attrib())
            .collect::<Vec<_>>();
        ValuesSelectAttribs::new(values)
    }
}

impl<P> IntoValuesSelectAttribs for Vec<P>
where
    P: IntoValueSelectAttrib,
{
    fn into_values_select_attribs(self) -> ValuesSelectAttribs {
        let values = self
            .into_iter()
            .map(|v| v.into_value_select_attrib())
            .collect::<Vec<_>>();
        ValuesSelectAttribs {
            inner: Arc::new(values),
        }
    }
}

// FIXME: ValueSelectAttrib lost its reason to exists, because now FieldAttribs contains a ValueSelect

// `ValueSelectAttrib` contains a `ValueSelect` (expression used in SELECT's columns) with an assigned `FieldAttribs`.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ValueSelectAttrib {
    pub field_attrib: FieldAttsLimit,
    pub value_select: ValueSelect,
}

pub trait IntoValueSelectAttrib {
    fn into_value_select_attrib(self) -> ValueSelectAttrib;
}

impl IntoValueSelectAttrib for ValueSelectAttrib {
    fn into_value_select_attrib(self) -> ValueSelectAttrib {
        self
    }
}

impl IntoValueSelectAttrib for FieldAttsLimit {
    fn into_value_select_attrib(self) -> ValueSelectAttrib {
        ValueSelectAttrib {
            value_select: self.value_select_name.name.clone().into_value_select(),
            field_attrib: self,
        }
    }
}

impl ValueSelectAttrib {
    /// Add `ValueSelect` that can be an expression.
    pub fn new(
        value_type: impl IntoDbValueType,
        name: &str,
        title: &str,
        into_value_select: impl IntoValueSelect,
    ) -> Self {
        let into_value_select = into_value_select.into_value_select();
        Self {
            field_attrib: FieldAttsLimit::new(
                value_type,
                name,
                title,
                Some(into_value_select.clone()),
            ),
            value_select: into_value_select.into_value_select(),
        }
    }

    /// Add string field.
    pub fn field_str(
        name: &str,
        title: &str,
        size: u16,
        into_value_select: impl IntoValueSelect,
    ) -> Self {
        let into_value_select = into_value_select.into_value_select();
        Self {
            field_attrib: FieldAttsLimit::new(
                DbValueType::String(size),
                name,
                title,
                Some(into_value_select.clone()),
            ),
            value_select: name.into_field_name().into_value_select(),
        }
    }

    /// Add decimal field.
    pub fn field_dec(
        name: &str,
        title: &str,
        precision: u8,
        scale: u8,
        into_value_select: impl IntoValueSelect,
    ) -> Self {
        let into_value_select = into_value_select.into_value_select();
        Self {
            field_attrib: FieldAttsLimit::new(
                DbValueType::Decimal(precision, scale),
                name,
                title,
                Some(into_value_select.clone()),
            ),
            value_select: name.into_field_name().into_value_select(),
        }
    }

    /// Add date field.
    pub fn field_dat(name: &str, title: &str, into_value_select: impl IntoValueSelect) -> Self {
        let into_value_select = into_value_select.into_value_select();
        Self {
            field_attrib: FieldAttsLimit::new(
                DbValueType::Date,
                name,
                title,
                Some(into_value_select.clone()),
            ),
            value_select: name.into_field_name().into_value_select(),
        }
    }

    pub fn field_attrib(&self) -> &FieldAttsLimit {
        &self.field_attrib
    }
}

impl IntoValuesSelect for ValuesSelectAttribs {
    fn into_values_select(self) -> ValuesSelect {
        let values = self
            .inner
            .iter()
            .map(|v| v.value_select.clone())
            .collect::<Vec<_>>();
        ValuesSelect::new(values)
    }
}

impl IntoValueSelect for ValueSelectAttrib {
    fn into_value_select(self) -> ValueSelect {
        ValueSelect {
            value_where: self.value_select.value_where,
            alias: None,
        }
    }
}

impl IntoFieldsAttribs for ValuesSelectAttribs {
    fn into_fields_attribs(self) -> FieldsAttribs {
        (&self).into_fields_attribs()
    }
}

impl IntoFieldsAttribs for &ValuesSelectAttribs {
    fn into_fields_attribs(self) -> FieldsAttribs {
        let attribs = self
            .inner
            .iter()
            .map(|v| v.field_attrib.clone())
            .collect::<Vec<_>>();
        FieldsAttribs::new(attribs)
    }
}
