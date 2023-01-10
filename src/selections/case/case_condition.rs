use super::when_condition::WhenCondition;
use crate::{
    resolvers::args_resolver::ArgsResolver,
    selections::{
        to_sql::ToSQL,
        value_where::{IntoValueWhere, ValueWhere},
    },
    SQLError,
};
use serde::{Deserialize, Serialize};

/// Definition for CASE SQL condition structure where is evaluated a boolean condition for each WHEN.
/// This case syntax example:
/// ```text
/// CASE
///    WHEN (logical_expression) THEN (value_where) [ ...n ]
///    [ ELSE (value_where) ]
/// END`
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CaseConditionBuilder {
    whens: Vec<WhenCondition>,
    else_case: Option<ValueWhere>,
}

impl CaseConditionBuilder {
    pub fn new() -> Self {
        Self {
            whens: Vec::new(),
            else_case: None,
        }
    }

    #[must_use]
    pub fn when_value(mut self, when_value: impl IntoWhenCondition) -> Self {
        self.whens.push(when_value.into_when_condition());
        self
    }

    #[must_use]
    pub fn else_case(mut self, value_where: impl IntoValueWhere) -> Self {
        self.else_case = Some(value_where.into_value_where());
        self
    }

    pub fn build(self) -> CaseCondition {
        CaseCondition {
            whens: self.whens,
            else_case: self.else_case,
        }
    }
}

impl Default for CaseConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Definition for CASE SQL condition structure where is evaluated a boolean condition for each WHEN.
/// This case syntax example:
/// ```text
/// CASE
///    WHEN (logical_expression) THEN (value_where) [ ...n ]
///    [ ELSE (value_where) ]
/// END`
/// ```
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseCondition {
    whens: Vec<WhenCondition>,
    else_case: Option<ValueWhere>,
}

impl ToSQL for CaseCondition {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let mut sql = "CASE ".to_string();
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

pub trait IntoWhenCondition {
    fn into_when_condition(self) -> WhenCondition;
}

impl IntoWhenCondition for WhenCondition {
    fn into_when_condition(self) -> WhenCondition {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolvers::args_resolver_string::ArgsResolverString;
    use crate::selections::condition_where::ConditionWhereOperation;
    use crate::selections::table_field::IntoTableField;

    #[test]
    fn test_case_to_sql_without_else() {
        let mut args_resolver_string = ArgsResolverString::new();
        let case_builder = CaseConditionBuilder::new();
        let field_status = "TABLE.STATUS".into_table_field();

        let cond_1 = field_status.clone().equal(1);
        let when_1 = WhenCondition::new(cond_1, "One");

        let cond_2 = field_status.equal(2);
        let when_2 = WhenCondition::new(cond_2, "Two");

        let case = case_builder.when_value(when_1).when_value(when_2).build();

        assert_eq!(
            case.to_sql(&mut args_resolver_string).unwrap(),
            r#"CASE WHEN "TABLE"."STATUS" = 1 THEN 'One' WHEN "TABLE"."STATUS" = 2 THEN 'Two' END"#
        );
    }

    #[test]
    fn test_case_to_sql_with_else() {
        let mut args_resolver_string = ArgsResolverString::new();
        let case_builder = CaseConditionBuilder::new();
        let field_status = "TABLE.STATUS".into_table_field();

        let cond_1 = field_status.clone().equal(1);
        let when_1 = WhenCondition::new(cond_1, "One");

        let cond_2 = field_status.equal(2);
        let when_2 = WhenCondition::new(cond_2, "Two");

        let case = case_builder
            .when_value(when_1)
            .when_value(when_2)
            .else_case("Unknown")
            .build();

        assert_eq!(
            case.to_sql(&mut args_resolver_string).unwrap(),
            r#"CASE WHEN "TABLE"."STATUS" = 1 THEN 'One' WHEN "TABLE"."STATUS" = 2 THEN 'Two' ELSE 'Unknown' END"#
        );
    }
}
