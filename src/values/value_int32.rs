use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
};

impl IntoValue for i32 {
    fn into_value(self) -> Value {
        Value::Int32(self)
    }

    fn value_type() -> Option<ValueType> {
        Some(ValueType::Int32)
    }
}

impl ValueToSQL for i32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for i32 {
    fn v_type() -> &'static ValueType {
        &ValueType::Int32
    }
}

impl TryFrom<Value> for i32 {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int32(v) => Ok(v),
            _ => Err("not i32 value".into()),
        }
    }
}

impl TryFrom<&Value> for i32 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int32(v) => Ok(*v),
            _ => Err("not i32 value".into()),
        }
    }
}

impl From<&i32> for Value {
    fn from(value: &i32) -> Self {
        value.into_value()
    }
}

impl From<i32> for Value {
    fn from(other: i32) -> Self {
        Value::Int32(other)
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Int32(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for i32 {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Int32(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<i32> for &Value {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Value::Int32(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for i32 {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::Int32(l0) => l0 == self,
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
            let s = 100i32;
            let value_s = s.into_value();
            let new_s = i32::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = 100i32;
            let new_s: i32 = s;
            assert!(new_s == s);
        }

        {
            let s = 100i32;
            let value_s = s.into_value();
            let new_s: i32 = value_s.clone().try_into().unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s = 100i32;
            let value_s = s.into_value();
            let new_s = i32::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = 100i32;
            let v1: i32 = s1;
            assert!(v1 == s1);

            let s2: i32 = v1;
            let v2: i32 = s2;

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);

            assert_eq!(serde_json::to_string_pretty(&v1).unwrap(), "100");
        }
    }
}
