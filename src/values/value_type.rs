use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Enum to represents field type, can be used in ListView.
/// Here can be defined "column" attributes, like date time/numeric format, blank when zero, etc.
/// It isn't paired with json value, it is more close to sqlx types.
#[derive(Copy, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    String,
    Uuid,
    Int32,
    Int64,
    Decimal,
    Boolean,
    Date,
    DateTime,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait IntoValueType {
    fn value_type(&self) -> ValueType;
}

impl IntoValueType for ValueType {
    fn value_type(&self) -> ValueType {
        *self
    }
}

impl IntoValueType for i32 {
    fn value_type(&self) -> ValueType {
        ValueType::Int32
    }
}

impl IntoValueType for i64 {
    fn value_type(&self) -> ValueType {
        ValueType::Int64
    }
}

impl IntoValueType for bool {
    fn value_type(&self) -> ValueType {
        ValueType::Boolean
    }
}

impl IntoValueType for String {
    fn value_type(&self) -> ValueType {
        ValueType::String
    }
}

impl IntoValueType for &str {
    fn value_type(&self) -> ValueType {
        ValueType::String
    }
}

impl IntoValueType for Uuid {
    fn value_type(&self) -> ValueType {
        ValueType::Uuid
    }
}

impl IntoValueType for Decimal {
    fn value_type(&self) -> ValueType {
        ValueType::Decimal
    }
}

impl IntoValueType for NaiveDate {
    fn value_type(&self) -> ValueType {
        ValueType::Date
    }
}

impl IntoValueType for NaiveDateTime {
    fn value_type(&self) -> ValueType {
        ValueType::DateTime
    }
}
