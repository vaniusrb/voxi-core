use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
    NullableValue,
};
use crate::IntoValueType;
use chrono::NaiveDateTime;

impl IntoValue for NaiveDateTime {
    fn into_value(self) -> Value {
        Value::DateTime(self)
    }
}

impl ValueToSQL for NaiveDateTime {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for NaiveDateTime {
    fn v_type() -> &'static ValueType {
        &ValueType::DateTime
    }
}

impl TryFrom<Value> for NaiveDateTime {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(v) => Ok(v),
            _ => Err("not date time value".into()),
        }
    }
}

impl TryFrom<&Value> for NaiveDateTime {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(v) => Ok(*v),
            _ => Err("not date time value".into()),
        }
    }
}

impl TryFrom<NullableValue> for NaiveDateTime {
    type Error = String;

    fn try_from(value: NullableValue) -> Result<Self, Self::Error> {
        match value.value().cloned() {
            Some(Value::DateTime(v)) => Ok(v),
            Some(v) => Err(format!("not datetime value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl TryFrom<&NullableValue> for NaiveDateTime {
    type Error = String;

    fn try_from(value: &NullableValue) -> Result<Self, Self::Error> {
        match value.value().cloned() {
            Some(Value::DateTime(v)) => Ok(v),
            Some(v) => Err(format!("not datetime value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl From<NaiveDateTime> for Value {
    fn from(other: NaiveDateTime) -> Self {
        Value::DateTime(other)
    }
}

impl From<&NaiveDateTime> for Value {
    fn from(value: &NaiveDateTime) -> Self {
        value.into_value()
    }
}

impl PartialEq<NaiveDateTime> for Value {
    fn eq(&self, other: &NaiveDateTime) -> bool {
        match self {
            Self::DateTime(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for NaiveDateTime {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::DateTime(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<NaiveDateTime> for &Value {
    fn eq(&self, other: &NaiveDateTime) -> bool {
        match self {
            Value::DateTime(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for NaiveDateTime {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::DateTime(l0) => l0 == self,
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
            let s = Local::now().naive_local();
            let value_s = s.into_value();
            let new_s = NaiveDateTime::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = Local::now().naive_local();
            let new_s: NaiveDateTime = s;
            assert!(new_s == s);
        }

        {
            let s = Local::now().naive_local();
            let value_s = s.into_value();
            let new_s: NaiveDateTime = value_s.clone().try_into().unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s = Local::now().naive_local();
            let value_s = s.into_value();
            let new_s = NaiveDateTime::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = Local::now().naive_local();
            let v1: NaiveDateTime = s1;
            assert!(v1 == s1);

            let s2: NaiveDateTime = v1;
            let v2: NaiveDateTime = s2;

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);
        }
    }
}
