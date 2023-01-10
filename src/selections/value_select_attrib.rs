use super::{
    values_select::{IntoValuesSelect, ValuesSelect},
    IntoValueSelect, ValueSelect,
};
use crate::{
    selections::{FieldAttribs, FieldsAttribs, IntoFieldsAttribs},
    IntoFieldName,
};
use crate::{ValueType, ValueTyped};
use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, Sub},
    sync::Arc,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ValuesSelectAttribs {
    inner: Arc<Vec<ValueSelectAttrib>>,
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

impl ValuesSelectAttribs {
    pub fn new(values: Vec<ValueSelectAttrib>) -> Self {
        Self {
            inner: Arc::new(values),
        }
    }

    pub fn into_vec(self) -> Vec<ValueSelectAttrib> {
        (*self.inner).clone()
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

impl IntoValuesSelectAttribs for FieldsAttribs {
    fn into_values_select_attribs(self) -> ValuesSelectAttribs {
        let values = self
            .to_vec()
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

// `ValueSelectAttrib` contains a `ValueSelect` (expression used in SELECT's columns) with an assigned `FieldAttribs`.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ValueSelectAttrib {
    pub value_select: ValueSelect,
    pub field_attrib: FieldAttribs,
}

pub trait IntoValueSelectAttrib {
    fn into_value_select_attrib(self) -> ValueSelectAttrib;
}

impl IntoValueSelectAttrib for ValueSelectAttrib {
    fn into_value_select_attrib(self) -> ValueSelectAttrib {
        self
    }
}

impl IntoValueSelectAttrib for FieldAttribs {
    fn into_value_select_attrib(self) -> ValueSelectAttrib {
        ValueSelectAttrib {
            value_select: self.name.clone().into_value_select(),
            field_attrib: self,
        }
    }
}

impl ValueSelectAttrib {
    /// Add `ValueSelect` that can be an expression.
    pub fn new_t(
        name: &str,
        title: &str,
        into_value_select: impl IntoValueSelect,
        value_type: ValueType,
    ) -> Self {
        Self {
            field_attrib: FieldAttribs::new_t(name, title, value_type),
            value_select: into_value_select.into_value_select(),
        }
    }

    /// Add `ValueSelect` that can be an expression.
    pub fn new<T: ValueTyped>(
        name: &str,
        title: &str,
        into_value_select: impl IntoValueSelect,
    ) -> Self {
        Self {
            field_attrib: FieldAttribs::new::<T>(name, title),
            value_select: into_value_select.into_value_select(),
        }
    }

    /// Add field name.
    pub fn field<T: ValueTyped>(name: &str, title: &str) -> Self {
        Self {
            field_attrib: FieldAttribs::new::<T>(name, title),
            value_select: name.into_field_name().into_value_select(),
        }
    }

    pub fn field_attrib(&self) -> &FieldAttribs {
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
