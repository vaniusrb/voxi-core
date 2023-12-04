use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
    NullableValue,
};
use crate::IntoValueType;

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Int64(self)
    }

    fn value_type() -> Option<ValueType> {
        Some(ValueType::Int64)
    }
}

impl ValueToSQL for i64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for i64 {
    fn v_type() -> &'static ValueType {
        &ValueType::Int64
    }
}

impl TryFrom<Value> for i64 {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(v) => Ok(v),
            _ => Err(format!("not i64 value! type is {:?}", value.value_type())),
        }
    }
}

impl TryFrom<&Value> for i64 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(v) => Ok(*v),
            _ => Err(format!(
                "not i64 value! type is {:?}",
                value.clone().value_type()
            )),
        }
    }
}

impl TryFrom<NullableValue> for i64 {
    type Error = String;

    fn try_from(value: NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Int64(v)) => Ok(*v),
            Some(v) => Err(format!("not i64 value! type is {:?}", v.value_type())),
            _ => Err("value is null".into()),
        }
    }
}

impl TryFrom<&NullableValue> for i64 {
    type Error = String;

    fn try_from(value: &NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Int64(v)) => Ok(*v),
            Some(v) => Err(format!("not i64 value! type is {:?}", v.value_type())),
            _ => Err("value is null".into()),
        }
    }
}

impl From<i64> for Value {
    fn from(other: i64) -> Self {
        Value::Int64(other)
    }
}

impl From<&i64> for Value {
    fn from(value: &i64) -> Self {
        value.into_value()
    }
}

impl PartialEq<i64> for Value {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Self::Int64(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for i64 {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Int64(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<i64> for &Value {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Value::Int64(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for i64 {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::Int64(l0) => l0 == self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IntoValue;

    #[test]
    fn test_value_integer() {
        {
            let s = 100i64;
            let value_s = s.into_value();
            let new_s = i64::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = 100i64;
            let new_s: i64 = s;
            assert!(new_s == s);
        }

        {
            let s = 100i64;
            let value_s = s.into_value();
            let new_s: i64 = value_s.clone().try_into().unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s = 100i64;
            let value_s = s.into_value();
            let new_s = i64::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = 100i64;
            let v1: i64 = s1;
            assert!(v1 == s1);

            let s2: i64 = v1;
            let v2: i64 = s2;

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);
        }
    }
}
