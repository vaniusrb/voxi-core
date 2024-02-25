use crate::{IntoValueType, ValueType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DbValueType {
    String(u16),
    Uuid,
    Int32(u8),
    Int64(u8),
    Decimal(u8, u8),
    Boolean,
    Date,
    DateTime,
    Json,
}

impl DbValueType {
    pub fn is_string(&self) -> bool {
        matches!(&self, DbValueType::String(_))
    }

    pub fn scale(&self) -> Option<u8> {
        match self {
            DbValueType::Decimal(_, scale) => Some(*scale),
            _ => None,
        }
    }
}

impl IntoValueType for DbValueType {
    fn value_type(&self) -> ValueType {
        match self {
            DbValueType::String(_) => ValueType::String,
            DbValueType::Uuid => ValueType::Uuid,
            DbValueType::Int32(_) => ValueType::Int32,
            DbValueType::Int64(_) => ValueType::Int64,
            DbValueType::Decimal(_, _) => ValueType::Decimal,
            DbValueType::Boolean => ValueType::Boolean,
            DbValueType::Date => ValueType::Date,
            DbValueType::DateTime => ValueType::DateTime,
            DbValueType::Json => ValueType::Json,
        }
    }
}

pub trait IntoDbValueType {
    fn into_db_value_type(self) -> DbValueType;
}

impl IntoDbValueType for DbValueType {
    fn into_db_value_type(self) -> DbValueType {
        self
    }
}
