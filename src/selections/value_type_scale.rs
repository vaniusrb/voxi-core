use crate::{IntoValueType, ValueType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValueTypeScale {
    #[serde(rename = "type")]
    pub type_: ValueType,
    pub scale: Option<u32>,
}

pub trait IntoValueTypeScale {
    fn into_value_type_scale(self) -> ValueTypeScale;
}

impl IntoValueTypeScale for ValueTypeScale {
    fn into_value_type_scale(self) -> ValueTypeScale {
        self
    }
}

impl IntoValueType for ValueTypeScale {
    fn value_type(&self) -> ValueType {
        self.type_
    }
}
