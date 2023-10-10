use derive_more::Display;

#[derive(Display, Debug)]
pub enum SQLError {
    #[display(fmt = "core error: `{}`", _0)]
    CoreError(crate::CoreError),
    #[display(fmt = "conversion error: `{}`", _0)]
    Conversion(String),
    #[display(fmt = "parse json error: `{}`", _0)]
    ParseJson(serde_json::error::Error),
    #[display(fmt = "query builder invalid configuration: `{}`", _0)]
    InvalidQueryBuilderConfiguration(String),
    #[display(fmt = "error to resolve SQL: `{}`", _0)]
    SQLResolver(String),
    #[display(fmt = "field name not found: `{}` available fields are: `{}`", _0, _1)]
    FieldNameNotFound(String, String),
    #[display(fmt = "bind name not found: `{}`", _0)]
    BindNameNotFound(String),
}
