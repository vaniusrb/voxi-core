use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    NullableValue,
};
use crate::{IntoValueType, ValueType};
use chrono::NaiveDate;

impl IntoValue for NaiveDate {
    fn into_value(self) -> Value {
        Value::Date(self)
    }
}

impl ValueToSQL for NaiveDate {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for NaiveDate {
    fn v_type() -> &'static ValueType {
        &ValueType::Date
    }
}

impl TryFrom<Value> for NaiveDate {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Date(v) => Ok(v),
            _ => Err("not date value".into()),
        }
    }
}

impl TryFrom<&Value> for NaiveDate {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Date(v) => Ok(*v),
            _ => Err("not date value".into()),
        }
    }
}

impl From<NaiveDate> for Value {
    fn from(other: NaiveDate) -> Self {
        Value::Date(other)
    }
}

impl TryFrom<NullableValue> for NaiveDate {
    type Error = String;

    fn try_from(value: NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Date(v)) => Ok(v),
            Some(v) => Err(format!("not date value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl TryFrom<&NullableValue> for NaiveDate {
    type Error = String;

    fn try_from(value: &NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Date(v)) => Ok(v),
            Some(v) => Err(format!("not date value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl From<&NaiveDate> for Value {
    fn from(value: &NaiveDate) -> Self {
        value.into_value()
    }
}

impl PartialEq<NaiveDate> for Value {
    fn eq(&self, other: &NaiveDate) -> bool {
        match self {
            Self::Date(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for NaiveDate {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Date(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<NaiveDate> for &Value {
    fn eq(&self, other: &NaiveDate) -> bool {
        match self {
            Value::Date(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for NaiveDate {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::Date(l0) => l0 == self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_value_decimal() {
        {
            let s = Local::today().naive_local();
            let value_s = s.into_value();
            let new_s = NaiveDate::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = Local::today().naive_local();
            let new_s: NaiveDate = s;
            assert!(new_s == s);
        }

        {
            let s = Local::today().naive_local();
            let value_s = s.into_value();
            let new_s: NaiveDate = value_s.clone().try_into().unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s = Local::today().naive_local();
            let value_s = s.into_value();
            let new_s = NaiveDate::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = Local::today().naive_local();
            let v1: NaiveDate = s1;
            assert!(v1 == s1);

            let s2: NaiveDate = v1;
            let v2: NaiveDate = s2;

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);
        }
    }
}
