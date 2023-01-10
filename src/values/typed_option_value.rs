use crate::{NullableValue, ValueType};
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
