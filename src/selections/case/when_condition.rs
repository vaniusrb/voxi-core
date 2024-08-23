use crate::{
    resolvers::args_resolver::ArgsResolver,
    selections::{
        logical_expr_where::{IntoLogicalExprWhere, LogicalExprWhere},
        to_sql::ToSQL,
        value_where::{IntoValueWhere, ValueWhere},
    },
    SQLError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhenCondition {
    when_condition: LogicalExprWhere,
    then_value_where: ValueWhere,
}

impl WhenCondition {
    pub fn new(
        when_condition: impl IntoLogicalExprWhere,
        then_value_where: impl IntoValueWhere,
    ) -> Self {
        Self {
            when_condition: when_condition.into_logical_expr_where(),
            then_value_where: then_value_where.into_value_where(),
        }
    }
}

impl ToSQL for WhenCondition {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLError> {
        let sql = format!(
            "WHEN {} THEN {}",
            self.when_condition.to_sql(args_resolver)?,
            self.then_value_where.to_sql(args_resolver)?
        );
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resolvers::args_resolver_string::ArgsResolverString,
        selections::{condition_where::ConditionWhereOperation, table_field::IntoTableField},
    };

    #[test]
    fn test_case_to_sql() {
        let mut args_resolver_string = ArgsResolverString::new();

        let field_status = "TABLE.STATUS".into_table_field();

        let cond_1 = field_status.clone().equal(1);
        let when_1 = WhenCondition::new(cond_1, "One");
        assert_eq!(
            when_1.to_sql(&mut args_resolver_string).unwrap(),
            r#"WHEN "TABLE"."STATUS" = 1 THEN 'One'"#
        );

        let cond_2 = field_status.equal(2);
        let when_2 = WhenCondition::new(cond_2, "Two");
        assert_eq!(
            when_2.to_sql(&mut args_resolver_string).unwrap(),
            r#"WHEN "TABLE"."STATUS" = 2 THEN 'Two'"#
        );
    }
}
