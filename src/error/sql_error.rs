use error_stack_derive::ErrorStack;

// TODO: add comment
#[derive(ErrorStack, Clone, Debug, PartialEq, Eq)]
pub enum SQLRoxiError {
    #[error_message(&format!("conversion error: `{unnamed0}`"))]
    Conversion(String),
    #[error_message(&format!("parse json error: `{unnamed0}`"))]
    ParseJson(String),
    #[error_message(&format!("query builder invalid configuration: `{unnamed0}`"))]
    InvalidQueryBuilderConfiguration(String),
    #[error_message(&format!("error to resolve SQL: `{unnamed0}`"))]
    SQLResolver(String),
    #[error_message(&format!("field name not found: `{unnamed1}` available fields are: `{unnamed0}`", ))]
    FieldNameNotFound(String, String),
    #[error_message(&format!("bind name not found: `{unnamed0}`"))]
    BindNameNotFound(String),
}

impl From<serde_json::error::Error> for SQLRoxiError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::ParseJson(e.to_string())
    }
}
