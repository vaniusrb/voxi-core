use crate::{IntoValue, IntoValueType, ValueType};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    hash::Hasher,
};
use uuid::Uuid;

// TODO: add comment
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Value {
    #[serde(rename = "s")]
    String(String),
    #[serde(rename = "u")]
    Uuid(Uuid),
    #[serde(rename = "i4")]
    Int32(i32),
    #[serde(rename = "i8")]
    Int64(i64),
    #[serde(rename = "f")]
    Decimal(Decimal),
    #[serde(rename = "b")]
    Boolean(bool),
    #[serde(rename = "d")]
    Date(NaiveDate),
    #[serde(rename = "t")]
    DateTime(NaiveDateTime),
    #[serde(rename = "j")]
    Json(serde_json::Value),
}

impl std::hash::Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::String(value) => value.hash(state),
            Value::Uuid(value) => value.hash(state),
            Value::Int32(value) => value.hash(state),
            Value::Int64(value) => value.hash(state),
            Value::Decimal(value) => value.hash(state),
            Value::Boolean(value) => value.hash(state),
            Value::Date(value) => value.hash(state),
            Value::DateTime(value) => value.hash(state),
            Value::Json(value) => value.to_string().hash(state),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(v) => v.fmt(f),
            Value::Uuid(v) => v.fmt(f),
            Value::Int32(v) => v.fmt(f),
            Value::Int64(v) => v.fmt(f),
            Value::Decimal(v) => v.fmt(f),
            Value::Boolean(v) => v.fmt(f),
            Value::Date(v) => v.fmt(f),
            Value::DateTime(v) => v.fmt(f),
            Value::Json(v) => v.fmt(f),
        }
    }
}

impl Value {
    pub fn new(value: impl IntoValue) -> Self {
        value.into_value()
    }

    pub fn sql(&self) -> String {
        match self {
            Value::String(v) => v.to_sql(),
            Value::Uuid(v) => v.to_sql(),
            Value::Int32(v) => v.to_sql(),
            Value::Int64(v) => v.to_sql(),
            Value::Decimal(v) => v.to_sql(),
            Value::Boolean(v) => v.to_sql(),
            Value::Date(v) => v.to_sql(),
            Value::DateTime(v) => v.to_sql(),
            Value::Json(v) => v.to_sql(),
        }
    }
}

impl IntoValueType for Value {
    fn value_type(&self) -> ValueType {
        match self {
            Value::String(_) => ValueType::String,
            Value::Uuid(_) => ValueType::Uuid,
            Value::Int32(_) => ValueType::Int32,
            Value::Int64(_) => ValueType::Int64,
            Value::Decimal(_) => ValueType::Decimal,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Date(_) => ValueType::Date,
            Value::DateTime(_) => ValueType::DateTime,
            Value::Json(_) => ValueType::Json,
        }
    }
}

// TODO: add comment
pub trait ValueTyped {
    fn v_type() -> &'static ValueType;
}

// TODO: add comment
pub trait ValueToSQL: Display + std::fmt::Debug {
    fn to_sql(&self) -> String;
}

impl PartialEq for dyn ValueToSQL {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v: Value = 1i32.into();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    }
}
