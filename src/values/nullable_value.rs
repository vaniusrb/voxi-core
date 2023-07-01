use crate::ValueType;

use super::{IntoValue, Value};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
// pub struct NullableValue {
//     #[serde(flatten)]
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(default)]
//     value: Option<Value>,
// }

// TODO: add comment
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NullableValue {
    String(Option<String>),
    Uuid(Option<Uuid>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    Decimal(Option<Decimal>),
    Boolean(Option<bool>),
    Date(Option<NaiveDate>),
    DateTime(Option<NaiveDateTime>),
}

impl NullableValue {
    pub fn from(value: impl IntoValue) -> Self {
        match value.into_value() {
            Value::String(value) => Self::String(Some(value)),
            Value::Uuid(value) => Self::Uuid(Some(value)),
            Value::Int32(value) => Self::Int32(Some(value)),
            Value::Int64(value) => Self::Int64(Some(value)),
            Value::Decimal(value) => Self::Decimal(Some(value)),
            Value::Boolean(value) => Self::Boolean(Some(value)),
            Value::Date(value) => Self::Date(Some(value)),
            Value::DateTime(value) => Self::DateTime(Some(value)),
        }
    }

    pub fn null(value_type: ValueType) -> Self {
        match value_type {
            ValueType::String => Self::String(None),
            ValueType::Uuid => Self::Uuid(None),
            ValueType::Int32 => Self::Int32(None),
            ValueType::Int64 => Self::Int64(None),
            ValueType::Decimal => Self::Decimal(None),
            ValueType::Boolean => Self::Boolean(None),
            ValueType::Date => Self::Date(None),
            ValueType::DateTime => Self::DateTime(None),
        }
    }

    pub fn value(&self) -> Option<Value> {
        match self {
            NullableValue::String(value) => value.as_ref().map(|v| v.clone().into_value()),
            NullableValue::Uuid(value) => value.as_ref().map(|v| (*v).into_value()),
            NullableValue::Int32(value) => value.as_ref().map(|v| (*v).into_value()),
            NullableValue::Int64(value) => value.as_ref().map(|v| (*v).into_value()),
            NullableValue::Decimal(value) => value.as_ref().map(|v| (*v).into_value()),
            NullableValue::Boolean(value) => value.as_ref().map(|v| (*v).into_value()),
            NullableValue::Date(value) => value.as_ref().map(|v| (*v).into_value()),
            NullableValue::DateTime(value) => value.as_ref().map(|v| (*v).into_value()),
        }
    }

    pub fn sql(&self) -> String {
        match self.value() {
            Some(value) => value.sql(),
            None => "NULL".to_string(),
        }
    }

    pub fn into_opt(self) -> Option<Value> {
        self.value()
    }

    pub fn is_null(&self) -> bool {
        matches!(
            self,
            NullableValue::String(None)
                | NullableValue::Uuid(None)
                | NullableValue::Int32(None)
                | NullableValue::Int64(None)
                | NullableValue::Decimal(None)
                | NullableValue::Boolean(None)
                | NullableValue::Date(None)
                | NullableValue::DateTime(None)
        )
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
        NullableValue::from(self)
    }
}

pub trait IntoNullableValueType {
    fn into_nullable_value(self, value_t: ValueType) -> NullableValue;
}

impl IntoNullableValueType for NullableValue {
    fn into_nullable_value(self, _: ValueType) -> NullableValue {
        self
    }
}

impl<V> IntoNullableValueType for Option<V>
where
    V: IntoValue,
{
    fn into_nullable_value(self, value_t: ValueType) -> NullableValue {
        match (self, value_t) {
            (None, ValueType::String) => NullableValue::String(None),
            (None, ValueType::Uuid) => NullableValue::Uuid(None),
            (None, ValueType::Int32) => NullableValue::Int32(None),
            (None, ValueType::Int64) => NullableValue::Int64(None),
            (None, ValueType::Decimal) => NullableValue::Decimal(None),
            (None, ValueType::Boolean) => NullableValue::Boolean(None),
            (None, ValueType::Date) => NullableValue::Date(None),
            (None, ValueType::DateTime) => NullableValue::DateTime(None),
            (Some(value), ValueType::String) => {
                NullableValue::String(Some(value.into_value().try_into().unwrap()))
            }
            (Some(value), ValueType::Uuid) => {
                NullableValue::Uuid(Some(value.into_value().try_into().unwrap()))
            }
            (Some(value), ValueType::Int32) => {
                NullableValue::Int32(Some(value.into_value().try_into().unwrap()))
            }
            (Some(value), ValueType::Int64) => {
                NullableValue::Int64(Some(value.into_value().try_into().unwrap()))
            }
            (Some(value), ValueType::Decimal) => {
                NullableValue::Decimal(Some(value.into_value().try_into().unwrap()))
            }
            (Some(value), ValueType::Boolean) => {
                NullableValue::Boolean(Some(value.into_value().try_into().unwrap()))
            }
            (Some(value), ValueType::Date) => {
                NullableValue::Date(Some(value.into_value().try_into().unwrap()))
            }
            (Some(value), ValueType::DateTime) => {
                NullableValue::DateTime(Some(value.into_value().try_into().unwrap()))
            }
        }
    }
}

// impl<V> IntoNullableValue for Option<V>
// where
//     V: IntoValue,
// {
//     fn into_nullable_value(self) -> NullableValue {
//         NullableValue {
//             value: self.map(|v| v.into_value()),
//         }
//     }
// }

impl fmt::Display for NullableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value() {
            Some(value) => value.fmt(f),
            None => write!(f, "NULL"),
        }
    }
}
