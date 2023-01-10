use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum SQLError {
    #[error("core error: `{0}`")]
    CoreError(#[from] crate::CoreError),
    #[error("conversion error: `{0}`")]
    Conversion(String),
    #[error("parse json error: `{0}`")]
    ParseJson(#[from] serde_json::error::Error),
    #[error("query builder invalid configuration: `{0}`")]
    InvalidQueryBuilderConfiguration(String),
    #[error("error to resolve SQL: `{0}`")]
    SQLResolver(String),
    #[error("field name not found: `{1}` available fields are: `{0}`")]
    FieldNameNotFound(String, String),
    #[error("bind name not found: `{0}`")]
    BindNameNotFound(String),
}
