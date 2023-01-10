use super::table_field::{IntoTableField, TableField};
use crate::{builder::args_resolver::ArgsResolver, selections::to_sql::ToSQL, SQLRoxiError};
use serde::{Deserialize, Serialize};

/// Definition for SQL functions MIN, MAX, AVG, SUM and COUNT.
/// # Example
/// ```
/// # use roxi_sql::selections::AggFunction;
/// let min = AggFunction::min("PRICE");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AggFunction {
    table_field: TableField,
    agg_type: AggFunctionType,
}

impl AggFunction {
    /// Define SQL function for MIN(field).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::AggFunction;
    /// let min = AggFunction::min("PRICE");
    /// ```
    pub fn min(table_field: impl IntoTableField) -> Self {
        Self {
            table_field: table_field.into_table_field(),
            agg_type: AggFunctionType::Min,
        }
    }

    /// Define SQL function for MAX(field).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::AggFunction;
    /// let max = AggFunction::max("PRICE");
    /// ```
    pub fn max(table_field: impl IntoTableField) -> Self {
        Self {
            table_field: table_field.into_table_field(),
            agg_type: AggFunctionType::Max,
        }
    }

    /// Define SQL function for AVG(field).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::AggFunction;
    /// let avg = AggFunction::avg("PRICE");
    /// ```
    pub fn avg(table_field: impl IntoTableField) -> Self {
        Self {
            table_field: table_field.into_table_field(),
            agg_type: AggFunctionType::Avg,
        }
    }

    /// Define SQL function for SUM(field).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::AggFunction;
    /// let sum = AggFunction::sum("PRICE");
    /// ```
    pub fn sum(table_field: impl IntoTableField) -> Self {
        Self {
            table_field: table_field.into_table_field(),
            agg_type: AggFunctionType::Sum,
        }
    }

    /// Define SQL function for COUNT(field).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::AggFunction;
    /// let count = AggFunction::count("PRICE");
    /// ```
    pub fn count(table_field: impl IntoTableField) -> Self {
        Self {
            table_field: table_field.into_table_field(),
            agg_type: AggFunctionType::Count,
        }
    }

    /// Get a reference to the agg function's agg type.
    /// ```
    /// # use roxi_sql::selections::AggFunction;
    /// # use roxi_sql::selections::AggFunctionType;
    /// let agg_fun = AggFunction::avg("*");
    /// assert_eq!(agg_fun.agg_type(), &AggFunctionType::Avg);
    /// ```
    pub fn agg_type(&self) -> &AggFunctionType {
        &self.agg_type
    }

    /// Get a reference to the agg function's table field.
    /// # Example
    /// ```
    /// # use roxi_sql::selections::AggFunction;
    /// let agg_fun = AggFunction::avg("*");
    /// assert_eq!(agg_fun.table_field(), "*");
    /// ```
    pub fn table_field(&self) -> &TableField {
        &self.table_field
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AggFunctionType {
    Min,
    Max,
    Avg,
    Sum,
    Count,
}

impl ToSQL for AggFunction {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLRoxiError> {
        let op = match &self.agg_type {
            AggFunctionType::Min => "MIN",
            AggFunctionType::Max => "MAX",
            AggFunctionType::Avg => "AVG",
            AggFunctionType::Sum => "SUM",
            AggFunctionType::Count => "COUNT",
        };
        Ok(format!(
            "{}({})",
            op,
            self.table_field.to_sql(args_resolver)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::args_resolver_string::ArgsResolverString;

    #[test]
    fn test_count_function() {
        let mut args_resolver_string = ArgsResolverString::new();
        let agg_fun = AggFunction::count("*");
        assert_eq!(agg_fun.table_field(), "*");
        assert_eq!(agg_fun.agg_type(), &AggFunctionType::Count);
        assert_eq!(
            agg_fun.to_sql(&mut args_resolver_string).unwrap(),
            "COUNT(*)"
        );
    }

    #[test]
    fn test_min_function() {
        let mut args_resolver_string = ArgsResolverString::new();
        let agg_fun = AggFunction::min("*");
        assert_eq!(agg_fun.table_field(), "*");
        assert_eq!(agg_fun.agg_type(), &AggFunctionType::Min);
        assert_eq!(agg_fun.to_sql(&mut args_resolver_string).unwrap(), "MIN(*)");
    }

    #[test]
    fn test_max_function() {
        let mut args_resolver_string = ArgsResolverString::new();
        let agg_fun = AggFunction::max("*");
        assert_eq!(agg_fun.table_field(), "*");
        assert_eq!(agg_fun.agg_type(), &AggFunctionType::Max);
        assert_eq!(agg_fun.to_sql(&mut args_resolver_string).unwrap(), "MAX(*)");
    }

    #[test]
    fn test_avg_function() {
        let mut args_resolver_string = ArgsResolverString::new();
        let agg_fun = AggFunction::avg("*");
        assert_eq!(agg_fun.table_field(), "*");
        assert_eq!(agg_fun.agg_type(), &AggFunctionType::Avg);
        assert_eq!(agg_fun.to_sql(&mut args_resolver_string).unwrap(), "AVG(*)");
    }

    #[test]
    fn test_sum_function() {
        let mut args_resolver_string = ArgsResolverString::new();
        let agg_fun = AggFunction::sum("*");
        assert_eq!(agg_fun.table_field(), "*");
        assert_eq!(agg_fun.agg_type(), &AggFunctionType::Sum);
        assert_eq!(agg_fun.to_sql(&mut args_resolver_string).unwrap(), "SUM(*)");
    }
}
