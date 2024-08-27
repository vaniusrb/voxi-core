use crate::validate_double_quotes;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `FieldName` represents a definition for field name
/// ```
/// # use voxi_core::FieldName;
/// let field_name = FieldName::new("NAME");
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldName(pub String);

impl PartialEq<&str> for FieldName {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for FieldName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<FieldName> for &str {
    fn eq(&self, other: &FieldName) -> bool {
        self == &other.0
    }
}

impl PartialEq<FieldName> for str {
    fn eq(&self, other: &FieldName) -> bool {
        self == other.0
    }
}

pub trait IntoCowFieldName<'a> {
    fn into_cow_field_name(self) -> Cow<'a, FieldName>;
}

impl<'a> IntoCowFieldName<'a> for &str {
    fn into_cow_field_name(self) -> Cow<'a, FieldName> {
        Cow::Owned(FieldName(self.to_string()))
    }
}

impl<'a> IntoCowFieldName<'a> for &String {
    fn into_cow_field_name(self) -> Cow<'a, FieldName> {
        Cow::Owned(FieldName(self.to_owned()))
    }
}

impl<'a> IntoCowFieldName<'a> for String {
    fn into_cow_field_name(self) -> Cow<'a, FieldName> {
        Cow::Owned(FieldName(self))
    }
}

impl<'a> IntoCowFieldName<'a> for &'a FieldName {
    fn into_cow_field_name(self) -> Cow<'a, FieldName> {
        Cow::Borrowed(self)
    }
}

impl<'a> IntoCowFieldName<'a> for FieldName {
    fn into_cow_field_name(self) -> Cow<'a, FieldName> {
        Cow::Owned(self)
    }
}

impl AsRef<FieldName> for FieldName {
    fn as_ref(&self) -> &FieldName {
        self
    }
}

impl AsRef<str> for FieldName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn cow_field_test() {
        fn receives_field_name<'a>(field_name: impl IntoCowFieldName<'a>) {
            let cow: Cow<'a, FieldName> = field_name.into_cow_field_name();
            let field_name: &FieldName = cow.borrow();
            println!("field name: {field_name}");
        }

        let field_name = FieldName("foo".into());
        receives_field_name(&field_name);
        receives_field_name(field_name);
    }

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
