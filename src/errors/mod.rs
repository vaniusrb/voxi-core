pub mod core_error;
pub use core_error::CoreError;

#[cfg(feature = "sql")]
pub mod sql_error;
#[cfg(feature = "sql")]
pub use sql_error::SQLError;
