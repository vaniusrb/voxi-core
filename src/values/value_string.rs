use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
    IntoValueType, NullableValue,
};

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        Value::String(self.to_owned())
    }
}

impl ValueToSQL for String {
    fn to_sql(&self) -> String {
        format!("'{self}'")
    }
}

impl ValueTyped for String {
    fn v_type() -> &'static ValueType {
        &ValueType::String
    }
}

impl TryFrom<Value> for String {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v),
            v => Err(format!("not string value! type is {:?}", v.value_type())),
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v.into()),
            v => Err(format!(
                "not string value! type is {:?}",
                v.clone().value_type()
            )),
        }
    }
}

impl TryFrom<NullableValue> for String {
    type Error = String;

    fn try_from(value: NullableValue) -> Result<Self, Self::Error> {
        match value.value().cloned() {
            Some(Value::String(v)) => Ok(v),
            Some(v) => Err(format!("not string value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl TryFrom<&NullableValue> for String {
    type Error = String;

    fn try_from(value: &NullableValue) -> Result<Self, Self::Error> {
        match value.value().cloned() {
            Some(Value::String(v)) => Ok(v),
            Some(v) => Err(format!("not string value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_owned())
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::String(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::String(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<String> for &Value {
    fn eq(&self, other: &String) -> bool {
        match self {
            Value::String(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for String {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::String(l0) => l0 == self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_string() {
        {
            let s = String::from("hello");
            let value_s = s.into_value();
            let new_s = String::try_from(value_s.clone()).unwrap();
            assert!(new_s == value_s);
        }

        {
            let s = String::from("hello");
            let value_s = s.into_value();
            let new_s: String = value_s.clone().try_into().unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s = String::from("hello");
            let value_s = s.into_value();
            let new_s = String::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }
    }
}
