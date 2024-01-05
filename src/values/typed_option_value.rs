use std::fmt;

use crate::{IntoNullableValue, IntoValueType, NullableValue, ValueType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TypedOptionValue {
    #[serde(rename = "type")]
    pub v_type: ValueType,
    pub opt_value: NullableValue,
}

pub trait IntoTypedOptionValue {
    fn typed_option_value(self) -> TypedOptionValue;
}

impl IntoTypedOptionValue for TypedOptionValue {
    fn typed_option_value(self) -> TypedOptionValue {
        self
    }
}

impl<T: IntoValueType + IntoNullableValue> IntoTypedOptionValue for T {
    fn typed_option_value(self) -> TypedOptionValue {
        TypedOptionValue {
            v_type: self.value_type(),
            opt_value: self.into_nullable_value(),
        }
    }
}

impl fmt::Display for TypedOptionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.opt_value.fmt(f)
    }
}
