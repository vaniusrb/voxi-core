use super::{IntoValue, Value};
use crate::{CoreError, IntoValueType, ValueType};
use serde::{Deserialize, Serialize};
use std::fmt;

// TODO: add comment
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NullableValue {
    String(Option<Value>),
    Uuid(Option<Value>),
    Int32(Option<Value>),
    Int64(Option<Value>),
    Decimal(Option<Value>),
    Boolean(Option<Value>),
    Date(Option<Value>),
    DateTime(Option<Value>),
    Json(Option<Value>),
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
            ValueType::String => Self::String(Some(value)),
            ValueType::Uuid => Self::Uuid(Some(value)),
            ValueType::Int32 => Self::Int32(Some(value)),
            ValueType::Int64 => Self::Int64(Some(value)),
            ValueType::Decimal => Self::Decimal(Some(value)),
            ValueType::Boolean => Self::Boolean(Some(value)),
            ValueType::Date => Self::Date(Some(value)),
            ValueType::DateTime => Self::DateTime(Some(value)),
            ValueType::Json => Self::Json(Some(value)),
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

    pub fn value(&self) -> Option<&Value> {
        match self {
            NullableValue::String(value) => value.as_ref(),
            NullableValue::Uuid(value) => value.as_ref(),
            NullableValue::Int32(value) => value.as_ref(),
            NullableValue::Int64(value) => value.as_ref(),
            NullableValue::Decimal(value) => value.as_ref(),
            NullableValue::Boolean(value) => value.as_ref(),
            NullableValue::Date(value) => value.as_ref(),
            NullableValue::DateTime(value) => value.as_ref(),
            NullableValue::Json(value) => value.as_ref(),
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
            NullableValue::String(value) => value,
            NullableValue::Uuid(value) => value,
            NullableValue::Int32(value) => value,
            NullableValue::Int64(value) => value,
            NullableValue::Decimal(value) => value,
            NullableValue::Boolean(value) => value,
            NullableValue::Date(value) => value,
            NullableValue::DateTime(value) => value,
            NullableValue::Json(value) => value,
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
        let ret: T = value
            .try_into()
            // TODO: create error like "unexpected field type"
            .map_err(|_| CoreError::Conversion("conversion error".to_string(), "".to_string()))?;
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
            (Some(value), ValueType::String) => NullableValue::String(Some(value.into_value())),
            (Some(value), ValueType::Uuid) => NullableValue::Uuid(Some(value.into_value())),
            (Some(value), ValueType::Int32) => NullableValue::Int32(Some(value.into_value())),
            (Some(value), ValueType::Int64) => NullableValue::Int64(Some(value.into_value())),
            (Some(value), ValueType::Decimal) => NullableValue::Decimal(Some(value.into_value())),
            (Some(value), ValueType::Boolean) => NullableValue::Boolean(Some(value.into_value())),
            (Some(value), ValueType::Date) => NullableValue::Date(Some(value.into_value())),
            (Some(value), ValueType::DateTime) => NullableValue::DateTime(Some(value.into_value())),
            (Some(value), ValueType::Json) => NullableValue::Json(Some(value.into_value())),
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
            None => write!(f, "NULL"),
        }
    }
}
