use crate::{resolvers::args_resolver::ArgsResolver, SQLError};

use super::{
    table_field::{IntoTableField, TableField},
    to_sql::ToSQL,
};
use serde::{Deserialize, Serialize};

/// Definition for SQL GROUP BY.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupBy {
    table_field: TableField,
}

impl GroupBy {
    /// Create a `GroupBy` from a `IntoGroupBy` implementation.
    /// ```
    /// # use voxi_core::selections::GroupBy;
    /// # use voxi_core::selections::TableField;
    /// let group_by = GroupBy::new("TABLE.ID");
    /// assert_eq!(group_by.table_field(), &TableField::new("TABLE.ID"));
    /// ```
    pub fn new(group_by: impl IntoGroupBy) -> Self {
        group_by.into_group_by()
    }

    /// Get a reference to the group by's table field.
    /// ```
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::selections::GroupBy;
    /// let group_by = GroupBy::new("TABLE.ID");
    /// assert_eq!(group_by.table_field(), &TableField::new("TABLE.ID"));
    /// ```
    pub fn table_field(&self) -> &TableField {
        &self.table_field
    }
}

pub trait IntoGroupBy {
    fn into_group_by(self) -> GroupBy;
}

impl IntoGroupBy for TableField {
    fn into_group_by(self) -> GroupBy {
        GroupBy { table_field: self }
    }
}

impl IntoGroupBy for &str {
    fn into_group_by(self) -> GroupBy {
        GroupBy {
            table_field: self.into_table_field(),
        }
    }
}

impl ToSQL for GroupBy {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        self.table_field().to_sql(args_resolver)
    }
}

#[cfg(test)]
mod tests {
    use crate::resolvers::args_resolver_string::ArgsResolverString;

    use super::*;

    #[test]
    fn test_group_by_new() {
        let group_by = GroupBy::new("TABLE.ID");
        assert_eq!(group_by.table_field(), &TableField::new("TABLE.ID"));
    }

    #[test]
    fn test_group_into_str() {
        let group_by = GroupBy::new("TABLE.ID");
        assert_eq!(group_by, "TABLE.ID".into_group_by());
    }

    #[test]
    fn test_group_table_field_str() {
        let group_by = GroupBy::new("TABLE.ID");
        assert_eq!(group_by, TableField::new("TABLE.ID").into_group_by());
    }

    #[test]
    fn test_group_to_sql() {
        let mut args_resolver_string = ArgsResolverString::new();
        let group_by = GroupBy::new("TABLE.ID");
        assert_eq!(
            group_by.to_sql(&mut args_resolver_string).unwrap(),
            r#""TABLE"."ID""#
        );
    }
}
