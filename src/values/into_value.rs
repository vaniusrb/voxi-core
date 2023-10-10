use crate::CoreError;
use crate::{Value, ValueToSQL, ValueType};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use std::{
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};
use uuid::Uuid;

pub trait IntoValue {
    fn into_value(self) -> Value;

    fn value_type() -> Option<ValueType>;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }

    fn value_type() -> Option<ValueType> {
        None
    }
}

impl IntoValue for &Value {
    fn into_value(self) -> Value {
        self.clone()
    }

    fn value_type() -> Option<ValueType> {
        None
    }
}

pub trait TryValueFromString: ValueToSQL {
    type Return: ValueToSQL;
    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError>;
}

impl TryValueFromString for String {
    type Return = String;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        Ok(value.to_string())
    }
}

impl TryValueFromString for Uuid {
    type Return = Uuid;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        let v: Uuid = Uuid::from_slice(value.as_bytes())
            .map_err(|e| CoreError::Conversion(e.to_string(), value.to_string()))?;
        Ok(v)
    }
}

impl TryValueFromString for i32 {
    type Return = i32;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        let v: i32 = value
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| CoreError::Conversion(e.to_string(), value.to_string()))?;
        Ok(v)
    }
}

pub fn try_value_from_string(
    value: &str,
    value_type: ValueType,
) -> error_stack::Result<Value, CoreError> {
    let value = match value_type {
        ValueType::String => String::try_value_from_string(value)?.into_value(),
        ValueType::Uuid => Uuid::try_value_from_string(value)?.into_value(),
        ValueType::Int32 => i32::try_value_from_string(value)?.into_value(),
        ValueType::Int64 => i64::try_value_from_string(value)?.into_value(),
        ValueType::Decimal => Decimal::try_value_from_string(value)?.into_value(),
        ValueType::Boolean => bool::try_value_from_string(value)?.into_value(),
        ValueType::Date => NaiveDate::try_value_from_string(value)?.into_value(),
        ValueType::DateTime => NaiveDateTime::try_value_from_string(value)?.into_value(),
    };
    Ok(value)
}

impl TryValueFromString for i64 {
    type Return = i64;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        let v: i64 = value
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| CoreError::Conversion(e.to_string(), value.to_string()))?;
        Ok(v)
    }
}

impl TryValueFromString for bool {
    type Return = bool;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        let v: bool = value
            .to_string()
            .parse()
            .map_err(|e: ParseBoolError| CoreError::Conversion(e.to_string(), value.to_string()))?;
        Ok(v)
    }
}

impl TryValueFromString for NaiveDateTime {
    type Return = NaiveDateTime;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        let d = value
            .to_string()
            .parse::<DateTime<Local>>()
            .map_err(|e| CoreError::Conversion(e.to_string(), value.to_string()))?
            .naive_local();
        Ok(d)
    }
}

impl TryValueFromString for NaiveDate {
    type Return = NaiveDate;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        let d = NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map_err(|e| CoreError::Conversion(e.to_string(), value.to_string()))?;
        Ok(d)
    }
}

impl TryValueFromString for Decimal {
    type Return = Decimal;

    fn try_value_from_string(value: &str) -> Result<Self::Return, CoreError> {
        let v: Decimal = Decimal::from_str(value)
            .map_err(|e| CoreError::Conversion(e.to_string(), value.to_string()))?;
        Ok(v)
    }
}

pub trait TryStringIntoValue<T: ValueToSQL> {
    fn try_string_into_value(&self) -> Result<T, CoreError>;
}

impl TryStringIntoValue<String> for String {
    fn try_string_into_value(&self) -> Result<String, CoreError> {
        Ok(self.to_string())
    }
}

impl TryStringIntoValue<NaiveDate> for String {
    fn try_string_into_value(&self) -> Result<NaiveDate, CoreError> {
        let d = NaiveDate::parse_from_str(self, "%Y-%m-%d")
            .map_err(|e| CoreError::Conversion(e.to_string(), self.clone()))?;
        Ok(d)
    }
}

impl TryStringIntoValue<NaiveDateTime> for String {
    fn try_string_into_value(&self) -> Result<NaiveDateTime, CoreError> {
        let d = self
            .to_string()
            .parse::<DateTime<Local>>()
            .map_err(|e| CoreError::Conversion(e.to_string(), self.clone()))?
            .naive_local();
        Ok(d)
    }
}

impl TryStringIntoValue<i32> for String {
    fn try_string_into_value(&self) -> Result<i32, CoreError> {
        let v: i32 = self
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| CoreError::Conversion(e.to_string(), self.clone()))?;
        Ok(v)
    }
}

impl TryStringIntoValue<i64> for String {
    fn try_string_into_value(&self) -> Result<i64, CoreError> {
        let v: i64 = self
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| CoreError::Conversion(e.to_string(), self.clone()))?;
        Ok(v)
    }
}

impl TryStringIntoValue<bool> for String {
    fn try_string_into_value(&self) -> Result<bool, CoreError> {
        let v: bool = self
            .to_string()
            .parse()
            .map_err(|e: ParseBoolError| CoreError::Conversion(e.to_string(), self.clone()))?;
        Ok(v)
    }
}

impl TryStringIntoValue<Decimal> for String {
    fn try_string_into_value(&self) -> Result<Decimal, CoreError> {
        let v: Decimal = Decimal::from_str(self)
            .map_err(|e| CoreError::Conversion(e.to_string(), self.clone()))?;
        Ok(v)
    }
}

// TODO: create try down cast
// search in old commits

// pub trait CustomValueTyped: CustomValue + ValueTyped {}

// pub trait TryStringIntoValueGen<T: CustomValueTyped> {
//     fn try_into_value_gen(&self) -> Result<T, String>;
// }

// impl<T: CustomValueTyped> TryStringIntoValueGen<T> for String {
//     fn try_into_value_gen(&self) -> Result<T, VoxiTypeError> {
//         match T::v_type() {
//             ValueType::String => Box::new(self.clone()) as Box<dyn CustomValue>,
//             ValueType::Uuid => {
//                 let uuid = Uuid::from_str(self).map_err(|e| VoxiTypeError::Conversion(e.to_string()))?;
//                 let t: T = uuid as T;
//                 return Ok(uuid);
//                 //Box::new(uuid) as Box<dyn CustomValue>
//             }
//             ValueType::Int32 => todo!(),
//             ValueType::Int64 => todo!(),
//             ValueType::Decimal => todo!(),
//             ValueType::Boolean => todo!(),
//             ValueType::Date => todo!(),
//             ValueType::DateTime => todo!(),
//         };
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    // pub fn load<T>(value: String) -> Result<T, String>
    // where
    //     T: CustomValue,
    // {
    //     let r = value.try_into_value_gen().unwrap();
    //     Ok(r)
    // }

    #[test]
    pub fn test_string() {
        let s1 = String::from("text");
        let s2: String = s1.try_string_into_value().unwrap();
        assert_eq!(s1, s2);
    }

    #[test]
    pub fn test_i32() {
        let i = i32::try_value_from_string("1").unwrap();
        assert_eq!(i, 1);
    }

    #[test]
    pub fn test_i64() {
        let i: i64 = "1".to_string().try_string_into_value().unwrap();
        assert_eq!(i, 1);
    }

    #[test]
    pub fn test_decimal() {
        let i: Decimal = "1".to_string().try_string_into_value().unwrap();
        assert_eq!(i, dec!(1));
    }

    #[test]
    pub fn test_bool() {
        let b: bool = "true".to_string().try_string_into_value().unwrap();
        assert!(b);
        let b: bool = "false".to_string().try_string_into_value().unwrap();
        assert!(!b);
    }
}
