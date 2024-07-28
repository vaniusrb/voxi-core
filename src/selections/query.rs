use super::{ToSQL, ValuesSelect};
use dyn_clonable::clonable;
use std::fmt::{Debug, Display};

#[clonable]
pub trait Query: ToSQL + Clone + Display + Debug {
    fn columns(&self) -> &ValuesSelect;
}
