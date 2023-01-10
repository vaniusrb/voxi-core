use super::{IntoValue, Value};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct NullableValue {
    value: Option<Value>,
}

impl NullableValue {
    pub fn from(value: impl IntoValue) -> Self {
        Self {
            value: Some(value.into_value()),
        }
    }

    pub fn null() -> Self {
        Self { value: None }
    }

    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    pub fn sql(&self) -> String {
        match self.value.as_ref() {
            Some(value) => value.sql(),
            None => "NULL".to_string(),
        }
    }
}

pub trait IntoNullableValue {
    fn into_nullable_value(self) -> NullableValue;
}

impl IntoNullableValue for NullableValue {
    fn into_nullable_value(self) -> NullableValue {
        self
    }
}

impl<V> IntoNullableValue for V
where
    V: IntoValue,
{
    fn into_nullable_value(self) -> NullableValue {
        NullableValue {
            value: Some(self.into_value()),
        }
    }
}

impl<V> IntoNullableValue for Option<V>
where
    V: IntoValue,
{
    fn into_nullable_value(self) -> NullableValue {
        NullableValue {
            value: self.map(|v| v.into_value()),
        }
    }
}

impl fmt::Display for NullableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value.as_ref() {
            Some(value) => value.fmt(f),
            None => write!(f, "NULL"),
        }
    }
}
