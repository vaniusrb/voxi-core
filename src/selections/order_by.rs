use super::{
    table_field::{IntoTableField, TableField},
    to_sql::ToSQL,
};
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};

/// Definition for SQL `ORDER BY`.
/// # Example
/// ```
/// # use voxi_core::selections::OrderBy;
/// let order_by = OrderBy::desc("PRICE");
/// assert_eq!(order_by.table_field(), "PRICE");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderBy {
    table_field: TableField,
    #[serde(rename = "type")]
    order_by_type: OrderByType,
}

impl OrderBy {
    /// Define SQL `ORDER BY` field with `ASC` order.
    /// # Example
    /// ```
    /// # use voxi_core::selections::OrderBy;
    /// let order_by = OrderBy::asc("PRICE");
    /// ```
    pub fn asc(table_field: impl IntoTableField) -> Self {
        Self {
            table_field: table_field.into_table_field(),
            order_by_type: OrderByType::Asc,
        }
    }

    /// Define SQL `ORDER BY` field with `DESC` order.
    /// # Example
    /// ```
    /// # use voxi_core::selections::OrderBy;
    /// let order_by = OrderBy::desc("PRICE");
    /// assert_eq!(order_by.table_field(), "PRICE");
    /// ```
    pub fn desc(table_field: impl IntoTableField) -> Self {
        Self {
            table_field: table_field.into_table_field(),
            order_by_type: OrderByType::Desc,
        }
    }

    /// Get a reference to the order by's order by type.
    /// # Example
    /// ```
    /// # use crate::voxi_core::selections::OrderBy;
    /// # use crate::voxi_core::selections::OrderByType;
    /// let sort = OrderBy::desc("ID");
    /// assert_eq!(sort.order_by_type(), &OrderByType::Desc);
    /// ```
    pub fn order_by_type(&self) -> &OrderByType {
        &self.order_by_type
    }

    /// Get a reference to the order by's table field.
    /// # Example
    /// ```
    /// # use crate::voxi_core::selections::OrderBy;
    /// let sort = OrderBy::desc("ID");
    /// assert_eq!(sort.table_field(), "ID");
    /// ```
    pub fn table_field(&self) -> &TableField {
        &self.table_field
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderByType {
    Asc,
    Desc,
}

impl ToSQL for OrderBy {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let sort_t = match self.order_by_type {
            OrderByType::Asc => "ASC",
            OrderByType::Desc => "DESC",
        };
        Ok(format!(
            "{} {}",
            self.table_field.to_sql(args_resolver)?,
            sort_t
        ))
    }
}

pub trait IntoOrderBy {
    fn into_order_by(self) -> OrderBy;
}

impl IntoOrderBy for OrderBy {
    fn into_order_by(self) -> OrderBy {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::resolvers::args_resolver_string::ArgsResolverString;

    use super::*;

    #[test]
    fn test_order_by_asc() {
        let mut args_resolver_string = ArgsResolverString::new();
        let sort = OrderBy::asc("ID");
        assert_eq!(sort.table_field(), "ID");
        assert_eq!(sort.order_by_type(), &OrderByType::Asc);
        assert_eq!(
            sort.to_sql(&mut args_resolver_string).unwrap(),
            r#""ID" ASC"#
        );
    }

    #[test]
    fn test_order_by_desc() {
        let mut args_resolver_string = ArgsResolverString::new();
        let sort = OrderBy::desc("ID");
        assert_eq!(sort.table_field(), "ID");
        assert_eq!(sort.order_by_type(), &OrderByType::Desc);
        assert_eq!(
            sort.to_sql(&mut args_resolver_string).unwrap(),
            r#""ID" DESC"#
        );
    }
}
