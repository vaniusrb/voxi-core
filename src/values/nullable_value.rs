use super::{IntoValue, Value};
use crate::{CoreError, IntoValueType, ValueType};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{fmt, hash::Hasher};
use uuid::Uuid;

// TODO: add comment
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NullableValue {
    #[serde(rename = "s")]
    String(Option<String>),
    #[serde(rename = "u")]
    Uuid(Option<Uuid>),
    #[serde(rename = "i4")]
    Int32(Option<i32>),
    #[serde(rename = "i8")]
    Int64(Option<i64>),
    #[serde(rename = "f")]
    Decimal(Option<Decimal>),
    #[serde(rename = "b")]
    Boolean(Option<bool>),
    #[serde(rename = "t")]
    Date(Option<NaiveDate>),
    #[serde(rename = "d")]
    DateTime(Option<NaiveDateTime>),
    #[serde(rename = "j")]
    Json(Option<serde_json::Value>),
}

impl std::hash::Hash for NullableValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NullableValue::String(value) => value.hash(state),
            NullableValue::Uuid(value) => value.hash(state),
            NullableValue::Int32(value) => value.hash(state),
            NullableValue::Int64(value) => value.hash(state),
            NullableValue::Decimal(value) => value.hash(state),
            NullableValue::Boolean(value) => value.hash(state),
            NullableValue::Date(value) => value.hash(state),
            NullableValue::DateTime(value) => value.hash(state),
            NullableValue::Json(value) => value.as_ref().map(|v| v.to_string()).hash(state),
        }
    }
}

impl IntoValueType for NullableValue {
    fn value_type(&self) -> ValueType {
        match self {
            NullableValue::String(_) => ValueType::String,
            NullableValue::Uuid(_) => ValueType::Uuid,
            NullableValue::Int32(_) => ValueType::Int32,
            NullableValue::Int64(_) => ValueType::Int64,
            NullableValue::Decimal(_) => ValueType::Decimal,
            NullableValue::Boolean(_) => ValueType::Boolean,
            NullableValue::Date(_) => ValueType::Date,
            NullableValue::DateTime(_) => ValueType::DateTime,
            NullableValue::Json(_) => ValueType::Json,
        }
    }
}

impl NullableValue {
    pub fn from(value: impl IntoValue) -> Self {
        let value = value.into_value();
        match value.value_type() {
            ValueType::String => Self::String(Some(value.try_into().unwrap())),
            ValueType::Uuid => Self::Uuid(Some(value.try_into().unwrap())),
            ValueType::Int32 => Self::Int32(Some(value.try_into().unwrap())),
            ValueType::Int64 => Self::Int64(Some(value.try_into().unwrap())),
            ValueType::Decimal => Self::Decimal(Some(value.try_into().unwrap())),
            ValueType::Boolean => Self::Boolean(Some(value.try_into().unwrap())),
            ValueType::Date => Self::Date(Some(value.try_into().unwrap())),
            ValueType::DateTime => Self::DateTime(Some(value.try_into().unwrap())),
            ValueType::Json => Self::Json(Some(value.try_into().unwrap())),
        }
    }

    pub fn null(value_type: impl IntoValueType) -> Self {
        match value_type.value_type() {
            ValueType::String => Self::String(None),
            ValueType::Uuid => Self::Uuid(None),
            ValueType::Int32 => Self::Int32(None),
            ValueType::Int64 => Self::Int64(None),
            ValueType::Decimal => Self::Decimal(None),
            ValueType::Boolean => Self::Boolean(None),
            ValueType::Date => Self::Date(None),
            ValueType::DateTime => Self::DateTime(None),
            ValueType::Json => Self::Json(None),
        }
    }

    pub fn value(&self) -> Option<Value> {
        match self {
            NullableValue::String(value) => value.as_ref().map(|v| v.clone().into_value()),
            NullableValue::Uuid(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Int32(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Int64(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Decimal(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Boolean(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Date(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::DateTime(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Json(value) => value.as_ref().map(|v| v.clone().into_value()),
        }
    }

    pub fn sql(&self) -> String {
        match self.value() {
            Some(value) => value.sql(),
            None => "NULL".to_string(),
        }
    }

    pub fn into_opt(self) -> Option<Value> {
        match self {
            NullableValue::String(value) => value.as_ref().map(|v| v.clone().into_value()),
            NullableValue::Uuid(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Int32(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Int64(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Decimal(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Boolean(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Date(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::DateTime(value) => value.as_ref().map(|v| v.into_value()),
            NullableValue::Json(value) => value.as_ref().map(|v| v.clone().into_value()),
        }
    }

    pub fn into_opt_val<T>(self) -> error_stack::Result<Option<T>, CoreError>
    where
        T: TryFrom<Value>,
        <T as TryFrom<Value>>::Error: std::fmt::Debug,
    {
        let value = match self.into_opt() {
            Some(value) => value,
            None => return Ok(None),
        };
        let result = value.clone().try_into();

        let type_name = std::any::type_name::<T>();
        let ret: T = result
            // TODO: create error like "unexpected field type"
            .map_err(|_| {
                CoreError::Conversion(
                    format!("conversion error from type {type_name}"),
                    value.to_string(),
                )
            })?;
        Ok(Some(ret))
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
            (None, ValueType::Json) => NullableValue::Json(None),
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
            (Some(value), ValueType::Json) => {
                NullableValue::Json(Some(value.into_value().try_into().unwrap()))
            }
        }
    }
}

impl<V> IntoNullableValue for Option<V>
where
    V: IntoValue,
{
    fn into_nullable_value(self) -> NullableValue {
        match self {
            Some(value) => NullableValue::from(value),
            None => {
                let value_type = V::value_type().unwrap_or(ValueType::String);
                NullableValue::null(value_type)
            }
        }
    }
}

impl fmt::Display for NullableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value() {
            Some(value) => value.fmt(f),
            None => write!(f, ""),
        }
    }
}
