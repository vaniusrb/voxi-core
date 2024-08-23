use super::ToSQL;
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};

/// Definition for SQL `LIMIT` and `OFFSET`.
/// # Example
/// ```
/// # use voxi_core::selections::LimitOffset;
/// # use voxi_core::resolvers::args_resolver_string::args_to_str;
/// let limit_offset = LimitOffset::new(0, 30);
/// assert_eq!(args_to_str(limit_offset).unwrap(), "LIMIT 0 OFFSET 30");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LimitOffset {
    pub limit: usize,
    pub offset: usize,
}

impl LimitOffset {
    pub fn new(limit: usize, offset: usize) -> Self {
        Self { limit, offset }
    }

    pub fn page(&self) -> usize {
        if self.limit == 0 {
            1
        } else {
            (self.offset / self.limit) + 1
        }
    }

    pub fn set_page(&mut self, page: usize) {
        self.offset = self.limit * (page - 1);
    }
}

impl ToSQL for LimitOffset {
    fn to_sql(
        &self,
        _args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLError> {
        Ok(format!("LIMIT {} OFFSET {}", self.limit, self.offset))
    }
}

pub trait IntoLimitOffset {
    fn into_limit_offset(self) -> LimitOffset;
}

impl IntoLimitOffset for LimitOffset {
    fn into_limit_offset(self) -> LimitOffset {
        self
    }
}

impl IntoLimitOffset for (usize, usize) {
    fn into_limit_offset(self) -> LimitOffset {
        LimitOffset::new(self.0, self.1)
    }
}

impl IntoLimitOffset for (i32, i32) {
    fn into_limit_offset(self) -> LimitOffset {
        LimitOffset::new(self.0 as usize, self.1 as usize)
    }
}

impl IntoLimitOffset for (u32, u32) {
    fn into_limit_offset(self) -> LimitOffset {
        LimitOffset::new(self.0 as usize, self.1 as usize)
    }
}
