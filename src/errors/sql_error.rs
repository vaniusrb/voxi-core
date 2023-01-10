use thiserror::Error as ThisError;

// TODO: add comment
#[derive(ThisError, Debug)]
pub enum SQLError {
    #[error("type error: `{0}`")]
    RoxiTypeError(#[from] crate::CoreError),
    #[error("conversion error: `{0}`")]
    Conversion(String),
    #[error("parse json error: `{0}`")]
    ParseJson(String),
    #[error("query builder invalid configuration: `{0}`")]
    InvalidQueryBuilderConfiguration(String),
    #[error("error to resolve SQL: `{0}`")]
    SQLResolver(String),
    #[error("field name not found: `{1}` available fields are: `{0}`")]
    FieldNameNotFound(String, String),
    #[error("bind name not found: `{0}`")]
    BindNameNotFound(String),
}

impl From<serde_json::error::Error> for SQLError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::ParseJson(e.to_string())
    }
}
