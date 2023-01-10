use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum CoreError {
    #[error("conversion error: `{0}`")]
    Conversion(String),
    #[error("parse json error: `{0}`")]
    ParseJson(#[from] serde_json::Error),
    #[error("field name not found: `{0}` available fields are: `{1}`")]
    FieldNameNotFound(String, String),
}
