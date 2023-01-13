use super::nullable_value::{IntoNullableValue, NullableValue};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct NullValue {}

impl NullValue {
    pub fn new() -> Self {
        Self {}
    }

    pub fn sql(&self) -> String {
        self.to_string()
    }
}

impl Default for NullValue {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoNullableValue for NullValue {
    fn into_nullable_value(self) -> NullableValue {
        NullableValue::null()
    }
}

impl fmt::Display for NullValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NULL")
    }
}
