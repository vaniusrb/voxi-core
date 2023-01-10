use super::{IntoOrderBy, OrderBy, ToSQL};
use crate::{builder::args_resolver::ArgsResolver, SQLRoxiError};
use serde::{Deserialize, Serialize};

// TODO: add comment
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrdersBy {
    orders_by: Vec<OrderBy>,
}

impl OrdersBy {
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn new(orders: Vec<OrderBy>) -> Self {
        Self { orders_by: orders }
    }

    pub fn len(&self) -> usize {
        self.orders_by.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn from(orders_by: impl IntoOrdersBy) -> Self {
        orders_by.into_orders_by()
    }

    pub fn push(&mut self, order_by: impl IntoOrderBy) {
        self.orders_by.push(order_by.into_order_by());
    }
}

impl ToSQL for OrdersBy {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLRoxiError> {
        let sql = self
            .orders_by
            .iter()
            .map(|t| t.to_sql(args_resolver))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        Ok(sql)
    }
}

pub trait IntoOrdersBy {
    fn into_orders_by(self) -> OrdersBy;
}

impl IntoOrdersBy for Vec<OrderBy> {
    fn into_orders_by(self) -> OrdersBy {
        OrdersBy::new(self)
    }
}

impl IntoOrdersBy for OrdersBy {
    fn into_orders_by(self) -> OrdersBy {
        self
    }
}
