use crate::{
    resolvers::args_resolver::ArgsResolver,
    selections::{
        to_sql::ToSQL,
        value_where::{IntoValueWhere, ValueWhere},
    },
    SQLError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhenValue {
    when_value_where: ValueWhere,
    then_value_where: ValueWhere,
}

impl WhenValue {
    pub fn new(
        when_value_where: impl IntoValueWhere,
        then_value_where: impl IntoValueWhere,
    ) -> Self {
        Self {
            when_value_where: when_value_where.into_value_where(),
            then_value_where: then_value_where.into_value_where(),
        }
    }
}

impl ToSQL for WhenValue {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let sql = format!(
            "WHEN {} THEN {}",
            self.when_value_where.to_sql(args_resolver)?,
            self.then_value_where.to_sql(args_resolver)?
        );
        Ok(sql)
    }
}

pub trait IntoWhenValue {
    fn into_when_value(self) -> WhenValue;
}

impl IntoWhenValue for WhenValue {
    fn into_when_value(self) -> WhenValue {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolvers::args_resolver_string::ArgsResolverString;

    #[test]
    fn test_case_to_sql() {
        let mut args_resolver_string = ArgsResolverString::new();

        let when = WhenValue::new(1, "One");
        assert_eq!(
            when.to_sql(&mut args_resolver_string).unwrap(),
            "WHEN 1 THEN 'One'"
        );
        let when = WhenValue::new(2, "Two");
        assert_eq!(
            when.to_sql(&mut args_resolver_string).unwrap(),
            "WHEN 2 THEN 'Two'"
        );
    }
}
