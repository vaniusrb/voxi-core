use super::nullable_value::{IntoNullableValue, NullableValue};
use crate::ValueType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct NullValue {
    value_type: ValueType,
}

impl NullValue {
    pub fn new(value_type: ValueType) -> Self {
        Self { value_type }
    }

    pub fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoNullableValue for NullValue {
    fn into_nullable_value(self) -> NullableValue {
        NullableValue::null(self.value_type)
    }
}

impl fmt::Display for NullValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NULL")
    }
}
