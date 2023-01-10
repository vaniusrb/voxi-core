use super::ToSQL;
use dyn_clonable::clonable;
use std::fmt::{Debug, Display};

#[clonable]
pub trait Query: ToSQL + Clone + Display + Debug {}
