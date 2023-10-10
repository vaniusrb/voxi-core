use derive_more::Display;
use error_stack::Context;

#[derive(Display, Debug)]
pub enum CoreError {
    #[display(fmt = "conversion error: `{}`, provided string: `{}`", _0, _1)]
    Conversion(String, String),
    #[display(fmt = "parse json error: `{:?}`", _0)]
    ParseJson(serde_json::Value), // serde_json::error::Error
    #[display(fmt = "convert to json error")]
    ConvertToJson, // serde_json::error::Error
    #[display(fmt = "field name not found: `{}` available fields are: `{}`", _0, _1)]
    FieldNameNotFound(String, String),
}

impl Context for CoreError {}

// https://www.reddit.com/r/rust/comments/wr562q/using_errorstack_crate_instead_of_anyhow_in/
// Yeah it's not going to be super convenient but you will need to define your own result type where your error
// is basically struct Wrapper<C>(Report<C>) and then implement the trait for your wrapper
