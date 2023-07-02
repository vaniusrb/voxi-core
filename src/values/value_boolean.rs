use crate::CoreError;

use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
};

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Boolean(self)
    }

    fn value_type() -> Option<ValueType> {
        Some(ValueType::Boolean)
    }
}

impl ValueToSQL for bool {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for bool {
    fn v_type() -> &'static ValueType {
        &ValueType::Boolean
    }
}

impl TryFrom<Value> for bool {
    type Error = CoreError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(v) => Ok(v),
            _ => Err(CoreError::Conversion(
                "not boolean value".to_string(),
                value.to_string(),
            )),
        }
    }
}

impl TryFrom<&Value> for bool {
    type Error = CoreError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(v) => Ok(*v),
            _ => Err(CoreError::Conversion(
                "not boolean value".to_string(),
                value.to_string(),
            )),
        }
    }
}

impl From<bool> for Value {
    fn from(other: bool) -> Self {
        Value::Boolean(other)
    }
}

impl From<&bool> for Value {
    fn from(value: &bool) -> Self {
        value.into_value()
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Value::Boolean(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for bool {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Boolean(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<bool> for &Value {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Value::Boolean(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for bool {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::Boolean(l0) => l0 == self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_value_boolean() {
        {
            let s = true;
            let value_s = s.into_value();
            let new_s = bool::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = true;
            let new_s: bool = s;
            assert!(new_s == s);
        }

        {
            let s = true;
            let value_s = s.into_value();
            let new_s: bool = value_s.clone().try_into().unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s = true;
            let value_s = s.into_value();
            let new_s = bool::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = true;
            let v1: bool = s1;
            assert!(v1 == s1);

            let s2: bool = v1;
            let v2: bool = s2;

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);
        }
    }
}
