use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
    IntoValueType, NullableValue,
};

impl IntoValue for serde_json::Value {
    fn into_value(self) -> Value {
        Value::Json(self)
    }

    fn value_type() -> Option<ValueType> {
        Some(ValueType::Json)
    }
}

impl ValueToSQL for serde_json::Value {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for serde_json::Value {
    fn v_type() -> &'static ValueType {
        &ValueType::Json
    }
}

impl TryFrom<Value> for serde_json::Value {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Json(v) => Ok(v),
            _ => Err("not uuid value".into()),
        }
    }
}

impl TryFrom<&Value> for serde_json::Value {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Json(v) => Ok(v.clone()),
            _ => Err("not uuid value".into()),
        }
    }
}

impl TryFrom<NullableValue> for serde_json::Value {
    type Error = String;

    fn try_from(value: NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Json(v)) => Ok(v.clone()),
            Some(v) => Err(format!("not uuid value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl TryFrom<&NullableValue> for serde_json::Value {
    type Error = String;

    fn try_from(value: &NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Json(v)) => Ok(v.clone()),
            Some(v) => Err(format!("not uuid value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl From<&serde_json::Value> for Value {
    fn from(value: &serde_json::Value) -> Self {
        value.clone().into_value()
    }
}

impl From<serde_json::Value> for Value {
    fn from(other: serde_json::Value) -> Self {
        Value::Json(other)
    }
}

impl PartialEq<serde_json::Value> for Value {
    fn eq(&self, other: &serde_json::Value) -> bool {
        match self {
            Self::Json(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for serde_json::Value {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Json(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<serde_json::Value> for &Value {
    fn eq(&self, other: &serde_json::Value) -> bool {
        match self {
            Value::Json(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for serde_json::Value {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::Json(l0) => l0 == self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_value_decimal() {
        {
            let s = json!({"value": "test"});
            let value_s = s.into_value();
            let new_s = serde_json::Value::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = json!({"value": "test 2"});
            let new_s: serde_json::Value = s.clone();
            assert!(new_s == s);
        }

        {
            let s = json!({"value": "test 3"});
            let value_s = s.into_value();
            let new_s: serde_json::Value = value_s.clone().try_into().unwrap();

            assert!(new_s.into_value() == value_s);
        }

        {
            let s = json!({"value": "test 4"});
            let value_s = s.into_value();
            let new_s = serde_json::Value::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = json!({"value": "test 5"});
            let v1: serde_json::Value = s1.clone();
            assert!(v1 == s1);

            let s2: serde_json::Value = v1.clone();
            let v2: serde_json::Value = s2.clone();

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);
        }
    }
}
