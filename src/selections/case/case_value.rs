use super::when_value::{IntoWhenValue, WhenValue};
use crate::{
    resolvers::args_resolver::ArgsResolver,
    selections::{
        to_sql::ToSQL,
        value_where::{IntoValueWhere, ValueWhere},
    },
    SQLError,
};
use serde::{Deserialize, Serialize};

/// Definition for CASE SQL condition structure where a input value is evaluated and it value is checked for each WHEN.
/// This case syntax example:
/// ```text
/// CASE input_expression
//     WHEN (value_where) THEN (value_where) [ ...n ]
///    [ ELSE (value_where) ]
/// END
/// ```
/// `CaseValueBuilder` is the object that allows build a `CaseValue` object.
/// # Example
/// ```
/// # use voxi_core::selections::CaseValueBuilder;
/// # use voxi_core::selections::TableField;
/// # use voxi_core::selections::WhenValue;
/// # use crate::voxi_core::builder::args_resolver_string::args_to_str;
/// # use voxi_core::selections::ToSQL;
/// let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
/// let when_1 = WhenValue::new(1, "One");
/// let when_2 = WhenValue::new(2, "Two");
/// let case = case_builder.when_value(when_1).when_value(when_2).build();
/// assert_eq!(
///     args_to_str(case).unwrap(),
///     r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' END"#
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CaseValueBuilder {
    input: ValueWhere,
    whens: Vec<WhenValue>,
    else_case: Option<ValueWhere>,
}

impl CaseValueBuilder {
    /// Create `CaseValueBuilder` to allow build a `CaseValue` object.
    /// ```
    /// # use voxi_core::selections::WhenValue;
    /// # use crate::voxi_core::builder::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::selections::CaseValueBuilder;
    /// # use voxi_core::selections::ToSQL;
    /// let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
    /// let when_1 = WhenValue::new(1, "One");
    /// let when_2 = WhenValue::new(2, "Two");
    /// let case = case_builder.when_value(when_1).when_value(when_2).build();
    /// assert_eq!(
    ///     args_to_str(case).unwrap(),
    ///     r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' END"#
    /// );
    /// ```
    pub fn new(input: impl IntoValueWhere) -> Self {
        Self {
            input: input.into_value_where(),
            whens: Vec::new(),
            else_case: None,
        }
    }

    /// Add a definition for WHEN condition for a CASE structure.
    /// ```
    /// # use voxi_core::selections::WhenValue;
    /// # use crate::voxi_core::builder::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::selections::CaseValueBuilder;
    /// # use voxi_core::selections::ToSQL;
    /// let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
    /// let when_1 = WhenValue::new(1, "One");
    /// let when_2 = WhenValue::new(2, "Two");
    /// let case = case_builder.when_value(when_1).when_value(when_2).build();
    /// assert_eq!(
    ///     args_to_str(case).unwrap(),
    ///     r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' END"#
    /// );
    /// ```
    #[must_use]
    pub fn when_value(mut self, when_value: impl IntoWhenValue) -> Self {
        self.whens.push(when_value.into_when_value());
        self
    }

    /// Define the ELSE condition for a CASE structure.
    /// ```
    /// # use voxi_core::selections::WhenValue;
    /// # use crate::voxi_core::builder::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::selections::CaseValueBuilder;
    /// # use voxi_core::selections::ToSQL;
    /// let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
    /// let when_1 = WhenValue::new(1, "One");
    /// let when_2 = WhenValue::new(2, "Two");
    /// let case = case_builder
    ///     .when_value(when_1)
    ///     .when_value(when_2)
    ///     .else_case("Unknown")
    ///     .build();
    /// assert_eq!(
    ///     args_to_str(case).unwrap(),
    ///     r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' ELSE 'Unknown' END"#
    /// );
    /// ```
    #[must_use]
    pub fn else_case(mut self, value_where: impl IntoValueWhere) -> Self {
        self.else_case = Some(value_where.into_value_where());
        self
    }

    /// Create `CaseValue` object
    /// ```
    /// # use voxi_core::selections::WhenValue;
    /// # use crate::voxi_core::builder::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::selections::CaseValueBuilder;
    /// # use crate::voxi_core::selections::ToSQL;
    /// let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
    /// let when_1 = WhenValue::new(1, "One");
    /// let when_2 = WhenValue::new(2, "Two");
    /// let case = case_builder
    ///     .when_value(when_1)
    ///     .when_value(when_2)
    ///     .else_case("Unknown")
    ///     .build();
    /// assert_eq!(
    ///     args_to_str(case).unwrap(),
    ///     r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' ELSE 'Unknown' END"#
    /// );
    /// ```
    pub fn build(self) -> CaseValue {
        CaseValue {
            input: self.input,
            whens: self.whens,
            else_case: self.else_case,
        }
    }
}

/// Definition for CASE SQL condition structure where a input value is evaluated and it value is checked for each WHEN.
/// This case syntax example:
/// ```text
/// CASE input_expression
//     WHEN (value_where) THEN (value_where) [ ...n ]
///    [ ELSE (value_where) ]
/// END
/// ```
/// Example:
/// ```
/// # use voxi_core::selections::CaseValueBuilder;
/// # use voxi_core::selections::WhenValue;
/// # use voxi_core::builder::args_resolver_string::args_to_str;
/// # use voxi_core::selections::TableField;
/// # use voxi_core::selections::ToSQL;
/// let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
/// let when_1 = WhenValue::new(1, "One");
/// let when_2 = WhenValue::new(2, "Two");
/// let case = case_builder.when_value(when_1).when_value(when_2).build();
/// assert_eq!(
///     args_to_str(case).unwrap(),
///     r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' END"#
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseValue {
    input: ValueWhere,
    whens: Vec<WhenValue>,
    else_case: Option<ValueWhere>,
}

impl CaseValue {
    pub fn new(input: ValueWhere, whens: Vec<WhenValue>, else_case: Option<ValueWhere>) -> Self {
        Self {
            input,
            whens,
            else_case,
        }
    }
}

impl ToSQL for CaseValue {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let mut sql = format!("CASE {} ", self.input.to_sql(args_resolver)?);
        let whens = self
            .whens
            .iter()
            .map(|w| w.to_sql(args_resolver))
            .collect::<Result<Vec<_>, _>>()?
            .join(" ");
        sql.push_str(&whens);
        if let Some(else_case) = &self.else_case {
            use std::fmt::Write;
            write!(sql, " ELSE {}", else_case.to_sql(args_resolver)?).unwrap();
        }
        sql.push_str(" END");
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resolvers::args_resolver_string::ArgsResolverString, selections::table_field::TableField,
    };

    #[test]
    fn test_case_to_sql_without_else() {
        let mut args_resolver_string = ArgsResolverString::new();
        let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
        let when_1 = WhenValue::new(1, "One");
        let when_2 = WhenValue::new(2, "Two");
        let case = case_builder.when_value(when_1).when_value(when_2).build();
        assert_eq!(
            case.to_sql(&mut args_resolver_string).unwrap(),
            r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' END"#
        );
    }

    #[test]
    fn test_case_to_sql_with_else() {
        let mut args_resolver_string = ArgsResolverString::new();
        let case_builder = CaseValueBuilder::new(TableField::new("TABLE.STATUS"));
        let when_1 = WhenValue::new(1, "One");
        let when_2 = WhenValue::new(2, "Two");
        let case = case_builder
            .when_value(when_1)
            .when_value(when_2)
            .else_case("Unknown")
            .build();

        assert_eq!(
            case.to_sql(&mut args_resolver_string).unwrap(),
            r#"CASE "TABLE"."STATUS" WHEN 1 THEN 'One' WHEN 2 THEN 'Two' ELSE 'Unknown' END"#
        );
    }
}
