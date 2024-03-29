use crate::validate_double_quotes;
use core::fmt;
use serde::{Deserialize, Serialize};

/// `FieldName` represents a definition for field name
/// ```
/// # use voxi_core::FieldName;
/// let field_name = FieldName::new("NAME");
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldName(pub String);

impl From<String> for FieldName {
    fn from(value: String) -> Self {
        FieldName(value)
    }
}

impl From<&String> for FieldName {
    fn from(value: &String) -> Self {
        FieldName(value.clone())
    }
}

impl From<&str> for FieldName {
    fn from(value: &str) -> Self {
        FieldName(value.into())
    }
}

impl From<FieldName> for String {
    fn from(value: FieldName) -> Self {
        value.0
    }
}

impl From<&FieldName> for String {
    fn from(value: &FieldName) -> Self {
        value.0.clone()
    }
}

impl FieldName {
    // TODO: add comment
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        validate_double_quotes(&name).unwrap();
        Self(name)
    }

    pub fn from(into_field_name: impl IntoFieldName) -> FieldName {
        into_field_name.into_field_name()
    }

    /// Get a reference to the field name's name.
    pub fn name(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait IntoFieldName {
    fn into_field_name(self) -> FieldName;
}

impl IntoFieldName for &str {
    fn into_field_name(self) -> FieldName {
        FieldName::new(self.to_owned())
    }
}

impl IntoFieldName for String {
    fn into_field_name(self) -> FieldName {
        FieldName::new(self)
    }
}

impl IntoFieldName for FieldName {
    fn into_field_name(self) -> FieldName {
        self
    }
}

impl PartialEq<str> for FieldName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<FieldName> for str {
    fn eq(&self, other: &FieldName) -> bool {
        self == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_name_new() {
        let field = FieldName::from("NAME");
        assert_eq!(field.name(), "NAME");
        let field = FieldName::from("ID");
        assert_eq!(field.name(), "ID");
    }

    #[test]
    fn test_field_into_field_name() {
        let field = "ID".into_field_name();
        assert_eq!(field.name(), "ID");
        let field = "NAME".into_field_name();
        assert_eq!(field.name(), "NAME");
    }

    #[test]
    fn test_field_name_equal() {
        let field = "ID".into_field_name();
        assert_eq!(field, FieldName::from(String::from("ID")));
    }
}
