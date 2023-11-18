use crate::values::into_value::try_value_from_string;
use crate::{CoreError, IntoNullableValue};
use crate::{IntoValue, NullableValue, Value, ValueType};
use chrono::{NaiveDate, NaiveDateTime};
use error_stack::ResultExt;
use log::error;
use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Map};
use uuid::Uuid;

/// Try convert a single json value field to `NullableValue`
pub fn json_to_value(
    value_j: serde_json::Value,
    value_type: ValueType,
) -> error_stack::Result<NullableValue, CoreError> {
    if value_j.is_null() {
        return Ok(NullableValue::null(value_type));
    }
    let result = match value_type {
        ValueType::String => serde_json::from_value::<String>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
        ValueType::Uuid => serde_json::from_value::<Uuid>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
        ValueType::Int32 => serde_json::from_value::<i32>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
        ValueType::Int64 => serde_json::from_value::<i64>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
        ValueType::Decimal => serde_json::from_value::<Decimal>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
        ValueType::Boolean => serde_json::from_value::<bool>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
        ValueType::Date => serde_json::from_value::<NaiveDate>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
        ValueType::DateTime => serde_json::from_value::<NaiveDateTime>(value_j.clone())
            .change_context_lazy(|| CoreError::ParseJson(value_j))?
            .into_value(),
            ValueType::Json => value_j.clone().into_value(),
    };
    Ok(result.into_nullable_value())
}

pub fn value_from_object<T: Serialize>(object: &T, field_name: &str) -> serde_json::Value {
    json_to_map(object).get(field_name).cloned().unwrap()
}

pub fn json_to_map<T: Serialize>(object: &T) -> Map<String, serde_json::Value> {
    let value_j = serde_json::to_value(object).unwrap();
    value_j.as_object().cloned().unwrap()
}

pub fn json_to_str(value_j: serde_json::Value, value_type: ValueType) -> String {
    let nullable = json_to_value(value_j, value_type).unwrap();
    nullable
        .into_opt()
        .map(|v| v.to_string())
        .unwrap_or_default()
}

/// Convert a `NullableValue` to a single json value
pub fn value_to_json(value: &NullableValue) -> error_stack::Result<serde_json::Value, CoreError> {
    let v = match value.value() {
        Some(v) => v_to_json(v)?,
        None => {
            serde_json::to_value(Option::<String>::None).change_context(CoreError::ConvertToJson)?
        }
    };
    Ok(v)
}

/// Convert a `Value` to a single json value
pub fn v_to_json(value: &Value) -> error_stack::Result<serde_json::Value, CoreError> {
    let v = match value {
        Value::String(v) => serde_json::to_value(v),
        Value::Uuid(v) => serde_json::to_value(v),
        Value::Int32(v) => serde_json::to_value(v),
        Value::Int64(v) => serde_json::to_value(v),
        Value::Decimal(v) => serde_json::to_value(v),
        Value::Boolean(v) => serde_json::to_value(v),
        Value::Date(v) => serde_json::to_value(v),
        Value::DateTime(v) => serde_json::to_value(v),
        Value::Json(v) => Ok(v.clone()),
    }
    .map_err(|e| CoreError::Conversion(e.to_string(), value.to_string()))?;
    Ok(v)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DiffValues {
    pub old: serde_json::Value,
    pub new: serde_json::Value,
}

pub fn fields_names_from_object<T: Serialize>(value: &T) -> Vec<String> {
    let object = serde_json::to_value(value).unwrap();
    object
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>()
}

pub fn modified_fields_name<A: Serialize, B: Serialize>(old: A, new: B) -> Vec<String> {
    let mut fields_name = vec![];
    let old = serde_json::to_value(old).unwrap();
    let new = serde_json::to_value(new).unwrap();

    let old_fields = old.as_object().unwrap();
    let new_fields = new.as_object().unwrap();

    for (key, new_value) in new_fields {
        if old_fields.get(key) != Some(new_value) {
            fields_name.push(key.clone());
        }
    }
    fields_name
}

/// Given two objects return a pair with old and new values, only the modified values.
pub fn diff_object<T: Serialize>(old: T, new: T) -> DiffValues {
    let old = serde_json::to_value(old).unwrap();
    let new = serde_json::to_value(new).unwrap();
    let mut diff_old = json!({});
    let mut diff_new = json!({});
    let map_old = diff_old.as_object_mut().unwrap();
    let map_new = diff_new.as_object_mut().unwrap();

    let old_fields = old.as_object().unwrap();
    let new_fields = new.as_object().unwrap();

    for (key, value_old) in old.as_object().cloned().unwrap() {
        if new_fields.get(&key).is_none() {
            map_old.insert(key.clone(), value_old.clone());
        }
    }

    for (key, value_new) in new.as_object().cloned().unwrap() {
        let (value_old, value_new) = match old_fields.get(&key).cloned() {
            Some(value_old) if value_old == value_new => (None, None),
            Some(value_old) => (Some(value_old), Some(value_new)),
            None => (None, Some(value_new)),
        };
        if let Some(value_old) = value_old {
            map_old.insert(key.clone(), value_old.clone());
        }
        if let Some(value_new) = value_new {
            map_new.insert(key.clone(), value_new.clone());
        }
    }
    DiffValues {
        old: diff_old,
        new: diff_new,
    }
}

/// Given an object `T` update its field value from `String`
pub fn set_field_from_str<T: Serialize + DeserializeOwned>(
    object: &T,
    field_name: &str,
    value_s: Option<String>,
    value_type: ValueType,
) -> error_stack::Result<T, CoreError> {
    let mut object_j = serde_json::to_value(object).change_context(CoreError::ConvertToJson)?;
    let map_j = object_j.as_object_mut().ok_or(CoreError::Conversion(
        String::from("field is not object"),
        serde_json::to_value(object).unwrap().to_string(),
    ))?;
    let value_s = match value_s {
        Some(value_s) => value_s,
        None => {
            map_j.remove(field_name);
            let new_object: T =
                serde_json::from_value(object_j).change_context(CoreError::ConvertToJson)?;
            return Ok(new_object);
        }
    };
    let value_j = {
        if value_s.is_empty() {
            json!(Option::<String>::None)
        } else {
            try_value_from_string(&value_s, value_type)                
                .and_then(|value| v_to_json(&value))
                .map_err(|e|
                    CoreError::Conversion(e.to_string(), format!("error extracting value from field name `{field_name}` type `{value_type}`: {e}"))
                    )?
        }
    };
    map_j.insert(field_name.to_string(), value_j);
    let new_object: T =
        serde_json::from_value(object_j).change_context(CoreError::ConvertToJson)?;
    Ok(new_object)
}

/// Given an object `T` returns its field value to `String`
// TODO: returns serde_json::Error
pub fn get_field_to_str<T: Serialize + DeserializeOwned>(
    object: &T,
    field_name: &str,
    value_type: ValueType,
) -> Option<String> {
    let object_j = serde_json::to_value(object).unwrap();
    let value_j = match object_j.get(field_name) {
        Some(value_j) => value_j.clone(),
        None => return None,
    };
    let value = match json_to_value(value_j.clone(), value_type) {
        Ok(value) => value,
        Err(e) => {
            error!(
                "error to get json value of {value_j:?} field {field_name}: {}",
                e.to_string()
            );
            return None;
        }
    };
    value.into_opt().map(|v| v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    pub fn modified_fields_name_test() {
        let a = json!({ "a": "1", "b": "2" });
        let b = json!({ "a": "1", "b": "2" });
        assert_eq!(modified_fields_name(a, b), Vec::<String>::new());

        let a = json!({ "a": "1" });
        let b = json!({ "a": "1", "b": "2" });
        assert_eq!(modified_fields_name(a, b), vec![String::from("b")]);

        let a = json!({ "a": "1", "b": "1" });
        let b = json!({ "a": "1", "b": "2" });
        assert_eq!(modified_fields_name(a, b), vec![String::from("b")]);

        let a = json!({ "a": "1", "b": "1" });
        let b = json!({ "a": "2", "b": "2" });
        assert_eq!(
            modified_fields_name(a, b),
            vec![String::from("a"), String::from("b")]
        );
    }

    #[derive(Serialize, Deserialize, Clone)]
    struct Test {
        a: String,
        b: String,
        c: Option<String>,
    }

    #[test]
    fn diff_json_test() {
        let old = Test {
            a: String::from("a"),
            b: String::from("b"),
            c: None,
        };
        let new = Test {
            a: String::from("a"),
            b: String::from("b"),
            c: String::from("c").into(),
        };
        let r = DiffValues {
            old: json!({ "c": null }),
            new: json!({ "c": "c"}),
        };
        assert_eq!(diff_object(old, new), r);

        let old = Test {
            a: String::from("a"),
            b: String::from("a"),
            c: None,
        };
        let new = Test {
            a: String::from("a"),
            b: String::from("b"),
            c: String::from("c").into(),
        };
        let r = DiffValues {
            old: json!({ "b": "a", "c": null }),
            new: json!({ "b": "b", "c": "c"}),
        };
        assert_eq!(diff_object(old, new), r);

        let old = Test {
            a: String::from("a"),
            b: String::from("b"),
            c: String::from("c").into(),
        };
        let new = Test {
            a: String::from("a"),
            b: String::from("b"),
            c: None,
        };
        let r = DiffValues {
            old: json!({ "c": "c"}),
            new: json!({ "c": null }),
        };
        assert_eq!(diff_object(old, new), r);
    }
}
