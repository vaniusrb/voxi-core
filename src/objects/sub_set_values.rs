use crate::{CoreError, ValueType};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::json_to_value;
use crate::v_to_json;
use crate::FieldNameType;
use crate::IntoNullableValue;
use crate::TypedOptionValue;

/// TODO: this is very similar to Record
#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct SubsetValues {
    values: HashMap<String, TypedOptionValue>,
}

impl PartialEq for SubsetValues {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self.values) == format!("{:?}", other.values)
    }
}

impl Hash for SubsetValues {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self.values).hash(state);
    }
}

impl SubsetValues {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
        }
    }

    pub fn from_json(
        object_j: &serde_json::Value,
        fields: Vec<&FieldNameType>,
    ) -> Result<SubsetValues, CoreError> {
        object_j_to_subset_values(object_j, fields)
    }

    pub fn from_object<T: Serialize + DeserializeOwned>(
        object: &T,
        fields: Vec<&FieldNameType>,
    ) -> Result<SubsetValues, CoreError> {
        let object_j = serde_json::to_value(object).unwrap();
        object_j_to_subset_values(&object_j, fields)
    }

    pub fn add(&mut self, field_name: &str, v_type: ValueType, opt_value: impl IntoNullableValue) {
        let opt_value = opt_value.into_nullable_value();
        let typed_option_value = TypedOptionValue { v_type, opt_value };
        self.values.insert(field_name.into(), typed_option_value);
    }

    pub fn values(&self) -> &HashMap<String, TypedOptionValue> {
        &self.values
    }

    pub fn add_from_object(
        &mut self,
        field_name: &str,
        v_type: ValueType,
        object: &impl Serialize,
    ) -> Result<(), CoreError> {
        let value = serde_json::to_value(object).unwrap();
        let object_j = value.as_object().unwrap();
        let value_j = match object_j.get(field_name) {
            Some(value_j) => value_j.clone(),
            None => {
                let fields = object_j
                    .iter()
                    .map(|(k, _)| k.clone())
                    .collect::<Vec<_>>()
                    .join(",");
                return Err(CoreError::FieldNameNotFound(field_name.into(), fields));
            }
        };
        let value = json_to_value(value_j, v_type)?;

        self.add(field_name, v_type, value);
        Ok(())
    }

    pub fn by_name(&self, name: &str) -> Option<&TypedOptionValue> {
        self.values.get(name)
    }

    pub fn object_j(&self) -> serde_json::Value {
        let object_j = json!({});
        subset_values_to_object_j(self, object_j)
    }

    pub fn object<T: Serialize + DeserializeOwned>(&self) -> T {
        let object_j = self.object_j();
        serde_json::from_value(object_j).unwrap()
    }
}

impl Default for SubsetValues {
    fn default() -> Self {
        Self::new()
    }
}

pub fn object_to_subset_values<T: Serialize>(
    object: &T,
    fields: Vec<&FieldNameType>,
) -> Result<SubsetValues, CoreError> {
    let object_j = serde_json::to_value(object)?;
    object_j_to_subset_values(&object_j, fields)
}

// pub fn fields_attribs_to_names_types(field_attribs: &FieldsAttribs) -> Vec<FieldNameType> {
//     field_attribs
//         .to_vec()
//         .into_iter()
//         .map(|fa| fa.into_field_name_type())
//         .collect()
// }

pub fn object_j_to_subset_values(
    object_j: &serde_json::Value,
    fields: Vec<&FieldNameType>,
) -> Result<SubsetValues, CoreError> {
    let mut subset_values = SubsetValues::new();
    let map_j = object_j.as_object().unwrap();
    for field in fields {
        let field_name = field.name.to_string();
        let v_type = field.v_type;
        let opt_value = map_j
            .get(&field_name)
            .map(|v| json_to_value(v.clone(), v_type).map(|nv| nv.into_opt()))
            .transpose()?
            .flatten();
        subset_values.add(&field_name, v_type, opt_value);
    }
    Ok(subset_values)
}

// TODO: create unit test
pub fn subset_values_to_object_j(
    subset_values: &SubsetValues,
    mut object_j: serde_json::Value,
) -> serde_json::Value {
    let map_j = object_j.as_object_mut().unwrap();
    for (name, opt_value) in subset_values.values() {
        match opt_value.opt_value.value() {
            Some(value) => {
                let value_j = v_to_json(value);
                map_j.insert(name.to_string(), value_j);
            }
            None => {
                map_j.remove(name);
            }
        }
    }
    object_j
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn add_test() {
        let mut subset_values = SubsetValues::new();

        subset_values.add("code", ValueType::Int32, Some(1));
        subset_values.add("description", ValueType::String, Some("description"));

        assert_eq!(
            subset_values.values().get("code").unwrap().opt_value,
            1.into_nullable_value()
        );
        assert_eq!(
            subset_values.values().get("description").unwrap().opt_value,
            "description".into_nullable_value()
        );
    }

    #[test]
    fn add_from_object_test() {
        // #[derive(Serialize, Deserialize, FieldName, FieldType)]
        #[derive(Serialize, Deserialize)]
        struct Object {
            code: i32,
            description: String,
        }

        let object = Object {
            code: 1,
            description: "description".to_string(),
        };

        // ObjectFieldType::Code::

        let mut subset_values = SubsetValues::new();

        subset_values
            .add_from_object("code", ValueType::Int32, &object)
            .unwrap();
        subset_values
            .add_from_object("description", ValueType::String, &object)
            .unwrap();

        assert_eq!(
            subset_values.values().get("code").unwrap().opt_value,
            1.into_nullable_value()
        );
        assert_eq!(
            subset_values.values().get("description").unwrap().opt_value,
            "description".into_nullable_value()
        );
    }
}
