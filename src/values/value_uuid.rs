use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
    IntoValueType, NullableValue,
};
use uuid::Uuid;

impl IntoValue for Uuid {
    fn into_value(self) -> Value {
        Value::Uuid(self)
    }

    fn value_type() -> Option<ValueType> {
        Some(ValueType::Uuid)
    }
}

impl ValueToSQL for Uuid {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for Uuid {
    fn v_type() -> &'static ValueType {
        &ValueType::Uuid
    }
}

impl TryFrom<Value> for Uuid {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uuid(v) => Ok(v),
            _ => Err("not uuid value".into()),
        }
    }
}

impl TryFrom<&Value> for Uuid {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uuid(v) => Ok(*v),
            _ => Err("not uuid value".into()),
        }
    }
}

impl TryFrom<NullableValue> for Uuid {
    type Error = String;

    fn try_from(value: NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Uuid(v)) => Ok(v),
            Some(v) => Err(format!("not uuid value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl TryFrom<&NullableValue> for Uuid {
    type Error = String;

    fn try_from(value: &NullableValue) -> Result<Self, Self::Error> {
        match value.value() {
            Some(Value::Uuid(v)) => Ok(v),
            Some(v) => Err(format!("not uuid value! type is {:?}", v.value_type())),
            None => Err("value is null".into()),
        }
    }
}

impl From<&Uuid> for Value {
    fn from(value: &Uuid) -> Self {
        value.into_value()
    }
}

impl From<Uuid> for Value {
    fn from(other: Uuid) -> Self {
        Value::Uuid(other)
    }
}

impl PartialEq<Uuid> for Value {
    fn eq(&self, other: &Uuid) -> bool {
        match self {
            Self::Uuid(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for Uuid {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Uuid(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<Uuid> for &Value {
    fn eq(&self, other: &Uuid) -> bool {
        match self {
            Value::Uuid(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for Uuid {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::Uuid(l0) => l0 == self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_decimal() {
        {
            let s = Uuid::new_v4();
            let value_s = s.into_value();
            let new_s = Uuid::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = Uuid::new_v4();
            let new_s: Uuid = s;
            assert!(new_s == s);
        }

        {
            let s = Uuid::new_v4();
            let value_s = s.into_value();
            let new_s: Uuid = value_s.clone().try_into().unwrap();

            assert!(new_s.into_value() == value_s);
        }

        {
            let s = Uuid::new_v4();
            let value_s = s.into_value();
            let new_s = Uuid::try_from(value_s.clone()).unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = Uuid::new_v4();
            let v1: Uuid = s1;
            assert!(v1 == s1);

            let s2: Uuid = v1;
            let v2: Uuid = s2;

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);
        }
    }
}
