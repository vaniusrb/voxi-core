use crate::RoxiTypeError;
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
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for &Value {
    fn into_value(self) -> Value {
        self.clone()
    }
}

pub trait TryValueFromString: ValueToSQL {
    type Return: ValueToSQL;
    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError>;
}

impl TryValueFromString for String {
    type Return = String;

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        Ok(value.to_string())
    }
}

impl TryValueFromString for Uuid {
    type Return = Uuid;

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        let v: Uuid = Uuid::from_slice(value.as_bytes())
            .map_err(|e| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

impl TryValueFromString for i32 {
    type Return = i32;

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        let v: i32 = value
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

pub fn try_value_from_string(value: &str, value_type: &ValueType) -> Result<Value, RoxiTypeError> {
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

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        let v: i64 = value
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

impl TryValueFromString for bool {
    type Return = bool;

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        let v: bool = value
            .to_string()
            .parse()
            .map_err(|e: ParseBoolError| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

impl TryValueFromString for NaiveDateTime {
    type Return = NaiveDateTime;

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        let d = value
            .to_string()
            .parse::<DateTime<Local>>()
            .map_err(|e| RoxiTypeError::Conversion(e.to_string()))?
            .naive_local();
        Ok(d)
    }
}

impl TryValueFromString for NaiveDate {
    type Return = NaiveDate;

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        let d = NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map_err(|e| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(d)
    }
}

impl TryValueFromString for Decimal {
    type Return = Decimal;

    fn try_value_from_string(value: &str) -> Result<Self::Return, RoxiTypeError> {
        let v: Decimal =
            Decimal::from_str(value).map_err(|e| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

// pub trait TryStringIntoValue2 {
//     fn try_value_from_string2<T: TryValueFromString>(&self) -> Result<T, String>;
// }

// impl TryStringIntoValue2 for String {
//     fn try_value_from_string2<T: TryValueFromString>(&self) -> Result<T, RoxiTypeError> {
//         let t = T::try_value_from_string(self)? as T ;
//         Ok(t)
//     }
// }

// impl<T: TryValueFromString>  for String {
//     fn try_value_from_string2(&self) -> Result<T, RoxiTypeError> {
//         let t = T::try_value_from_string(self)?;
//         Ok(t)
//     }
// }

//-------------------
pub trait TryStringIntoValue<T: ValueToSQL> {
    fn try_string_into_value(&self) -> Result<T, RoxiTypeError>;
}

impl TryStringIntoValue<String> for String {
    fn try_string_into_value(&self) -> Result<String, RoxiTypeError> {
        Ok(self.to_string())
    }
}

impl TryStringIntoValue<NaiveDate> for String {
    fn try_string_into_value(&self) -> Result<NaiveDate, RoxiTypeError> {
        let d = NaiveDate::parse_from_str(self, "%Y-%m-%d")
            .map_err(|e| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(d)
    }
}

impl TryStringIntoValue<NaiveDateTime> for String {
    fn try_string_into_value(&self) -> Result<NaiveDateTime, RoxiTypeError> {
        let d = self
            .to_string()
            .parse::<DateTime<Local>>()
            .map_err(|e| RoxiTypeError::Conversion(e.to_string()))?
            .naive_local();
        Ok(d)
    }
}

impl TryStringIntoValue<i32> for String {
    fn try_string_into_value(&self) -> Result<i32, RoxiTypeError> {
        let v: i32 = self
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

impl TryStringIntoValue<i64> for String {
    fn try_string_into_value(&self) -> Result<i64, RoxiTypeError> {
        let v: i64 = self
            .to_string()
            .parse()
            .map_err(|e: ParseIntError| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

impl TryStringIntoValue<bool> for String {
    fn try_string_into_value(&self) -> Result<bool, RoxiTypeError> {
        let v: bool = self
            .to_string()
            .parse()
            .map_err(|e: ParseBoolError| RoxiTypeError::Conversion(e.to_string()))?;
        Ok(v)
    }
}

impl TryStringIntoValue<Decimal> for String {
    fn try_string_into_value(&self) -> Result<Decimal, RoxiTypeError> {
        let v: Decimal =
            Decimal::from_str(self).map_err(|e| RoxiTypeError::Conversion(e.to_string()))?;
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
//     fn try_into_value_gen(&self) -> Result<T, RoxiTypeError> {
//         match T::v_type() {
//             ValueType::String => Box::new(self.clone()) as Box<dyn CustomValue>,
//             ValueType::Uuid => {
//                 let uuid = Uuid::from_str(self).map_err(|e| RoxiTypeError::Conversion(e.to_string()))?;
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
