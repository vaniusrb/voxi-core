use super::{
    into_value::IntoValue,
    value::Value,
    value::{ValueToSQL, ValueTyped},
    value_type::ValueType,
};
use crate::RoxiTypeError;
use rust_decimal::Decimal;

impl IntoValue for Decimal {
    fn into_value(self) -> Value {
        Value::Decimal(self)
    }
}

impl ValueToSQL for Decimal {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ValueTyped for Decimal {
    fn v_type() -> &'static ValueType {
        &ValueType::Decimal
    }
}

impl TryFrom<Value> for Decimal {
    type Error = RoxiTypeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Decimal(v) => Ok(v),
            _ => Err(RoxiTypeError::Conversion("not decimal value".to_string())),
        }
    }
}

impl TryFrom<&Value> for Decimal {
    type Error = RoxiTypeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Decimal(v) => Ok(*v),
            _ => Err(RoxiTypeError::Conversion("not decimal value".to_string())),
        }
    }
}

impl From<Decimal> for Value {
    fn from(other: Decimal) -> Self {
        Value::Decimal(other)
    }
}

impl From<&Decimal> for Value {
    fn from(value: &Decimal) -> Self {
        value.into_value()
    }
}

impl PartialEq<Decimal> for Value {
    fn eq(&self, other: &Decimal) -> bool {
        match self {
            Value::Decimal(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<Value> for Decimal {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Decimal(l0) => l0 == self,
            _ => false,
        }
    }
}

impl PartialEq<Decimal> for &Value {
    fn eq(&self, other: &Decimal) -> bool {
        match self {
            Value::Decimal(l0) => l0 == other,
            _ => false,
        }
    }
}

impl PartialEq<&Value> for Decimal {
    fn eq(&self, other: &&Value) -> bool {
        match other {
            Value::Decimal(l0) => l0 == self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_value_decimal() {
        {
            let s = dec!(100);
            let value_s = s.into_value();
            let new_s = Decimal::try_from(value_s.clone()).unwrap();

            assert!(new_s == value_s);
        }

        {
            let s = dec!(100);
            let new_s: Decimal = s;
            assert!(new_s == s);
        }

        {
            let s = dec!(100);
            let value_s = s.into_value();
            let new_s: Decimal = value_s.clone().try_into().unwrap();
            assert!(new_s.into_value() == value_s);
        }

        {
            let s = dec!(100);
            let value_s = s.into_value();
            let new_s = Decimal::try_from(value_s.clone()).unwrap();

            assert!(new_s.into_value() == value_s);
        }

        {
            let s1 = dec!(100);
            let v1: Decimal = s1;
            assert!(v1 == s1);

            let s2: Decimal = v1;
            let v2: Decimal = s2;

            assert_eq!(s1, s2);
            assert_eq!(v1, v2);
        }
    }
}
