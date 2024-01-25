use std::ops::Add;

use super::{
    condition_where::{ConditionWhere, IntoConditionWhere},
    to_sql::ToSQL,
};
use crate::{resolvers::args_resolver::ArgsResolver, SQLError};
use serde::{Deserialize, Serialize};

// TODO: add comment
pub trait IntoLogicalExprWhere {
    fn into_logical_expr_where(self) -> LogicalExprWhere;
}

impl IntoLogicalExprWhere for ConditionWhere {
    fn into_logical_expr_where(self) -> LogicalExprWhere {
        LogicalExprWhere::Condition(Box::new(self))
    }
}

impl IntoLogicalExprWhere for LogicalExprWhere {
    fn into_logical_expr_where(self) -> LogicalExprWhere {
        self
    }
}

impl<L> IntoLogicalExprWhere for Vec<L>
where
    L: IntoLogicalExprWhere,
{
    fn into_logical_expr_where(self) -> LogicalExprWhere {
        let mut where_expr: Option<LogicalExprWhere> = None;
        for new in self {
            let e = match where_expr {
                Some(prev) => prev.and(new.into_logical_expr_where()),
                None => new.into_logical_expr_where(),
            };
            where_expr = Some(e);
        }
        where_expr.unwrap()
    }
}

impl IntoLogicalExprWhere for Box<LogicalExprWhere> {
    fn into_logical_expr_where(self) -> LogicalExprWhere {
        *self
    }
}

pub trait LogicalExprWhereOps {
    // TODO: maybe must be IntoLogicalExprWhere, because I can have (expression) and (expression)
    fn or(self, condition: impl IntoLogicalExprWhere) -> LogicalExprWhere;
    fn and(self, condition: impl IntoLogicalExprWhere) -> LogicalExprWhere;
    fn not(self) -> LogicalExprWhere;
    /// Given current logical condition transforms in an expression, where his content will became into "( )".
    fn exp(self) -> LogicalExprWhere;
}

impl<T> LogicalExprWhereOps for T
where
    T: IntoLogicalExprWhere,
{
    fn or(self, condition: impl IntoLogicalExprWhere) -> LogicalExprWhere {
        LogicalExprWhere::Or(
            Box::new(self.into_logical_expr_where()),
            Box::new(condition.into_logical_expr_where()),
        )
    }

    fn and(self, condition: impl IntoLogicalExprWhere) -> LogicalExprWhere {
        LogicalExprWhere::And(
            Box::new(self.into_logical_expr_where()),
            Box::new(condition.into_logical_expr_where()),
        )
    }

    fn not(self) -> LogicalExprWhere {
        LogicalExprWhere::Not(Box::new(self.into_logical_expr_where()))
    }

    fn exp(self) -> LogicalExprWhere {
        LogicalExprWhere::Expression(Box::new(self.into_logical_expr_where()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogicalExprWhere {
    Condition(Box<ConditionWhere>),
    Not(Box<LogicalExprWhere>),
    Expression(Box<LogicalExprWhere>),
    And(Box<LogicalExprWhere>, Box<LogicalExprWhere>),
    Or(Box<LogicalExprWhere>, Box<LogicalExprWhere>),
}

// https://www.reddit.com/r/rust/comments/vdggo6/sqlx_postgres_result_to_json/
// #[cfg(feature = "sqlx")]
// impl sqlx::Type<sqlx::Postgres> for LogicalExprWhere {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         PgType::Json
//     }
// }

impl LogicalExprWhere {
    pub fn condition(condition: impl IntoConditionWhere) -> LogicalExprWhere {
        LogicalExprWhere::Condition(Box::new(condition.into_condition_where()))
    }

    pub fn or(e1: impl IntoLogicalExprWhere, e2: impl IntoLogicalExprWhere) -> LogicalExprWhere {
        LogicalExprWhere::Or(
            Box::new(e1.into_logical_expr_where()),
            Box::new(e2.into_logical_expr_where()),
        )
    }

    pub fn and(e1: impl IntoLogicalExprWhere, e2: impl IntoLogicalExprWhere) -> LogicalExprWhere {
        LogicalExprWhere::And(
            Box::new(e1.into_logical_expr_where()),
            Box::new(e2.into_logical_expr_where()),
        )
    }
}

impl ToSQL for LogicalExprWhere {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLError> {
        let sql = match self {
            LogicalExprWhere::Condition(c) => c.to_sql(args_resolver)?,
            LogicalExprWhere::Not(e) => format!("NOT {}", e.to_sql(args_resolver)?),
            LogicalExprWhere::Expression(e) => format!("({})", e.to_sql(args_resolver)?),
            LogicalExprWhere::And(e1, e2) => {
                format!(
                    "{} AND {}",
                    e1.to_sql(args_resolver)?,
                    e2.to_sql(args_resolver)?
                )
            }
            LogicalExprWhere::Or(e1, e2) => {
                format!(
                    "{} OR {}",
                    e1.to_sql(args_resolver)?,
                    e2.to_sql(args_resolver)?
                )
            }
        };
        Ok(sql)
    }
}

impl Add<ConditionWhere> for ConditionWhere {
    type Output = LogicalExprWhere;

    fn add(self, rhs: ConditionWhere) -> Self::Output {
        LogicalExprWhere::and(self, rhs)
    }
}

impl Add<ConditionWhere> for LogicalExprWhere {
    type Output = LogicalExprWhere;

    fn add(self, rhs: ConditionWhere) -> Self::Output {
        self.and(rhs)
    }
}

impl Add<LogicalExprWhere> for ConditionWhere {
    type Output = LogicalExprWhere;

    fn add(self, rhs: LogicalExprWhere) -> Self::Output {
        LogicalExprWhere::and(self, rhs)
    }
}

impl Add<LogicalExprWhere> for LogicalExprWhere {
    type Output = LogicalExprWhere;

    fn add(self, rhs: LogicalExprWhere) -> Self::Output {
        LogicalExprWhere::and(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::FieldName;

    use super::LogicalExprWhereOps;
    use crate::{
        resolvers::args_resolver_string::args_to_str,
        selections::{ConditionWhereOperation, LogicalExprWhere},
    };

    #[test]
    fn test_condition() {
        let field_a = &FieldName::new("FIELD_A");
        let field_b = &FieldName::new("FIELD_B");
        let cond_a = field_a.equal(field_b);
        let log_expr = LogicalExprWhere::condition(cond_a);

        assert_eq!(args_to_str(log_expr).unwrap(), r#""FIELD_A" = "FIELD_B""#);
    }

    #[test]
    fn test_not() {
        let field_a = &FieldName::new("FIELD_A");
        let field_b = &FieldName::new("FIELD_B");
        let cond_a = field_a.equal(field_b);
        let log_expr = cond_a.not();

        assert_eq!(
            args_to_str(log_expr).unwrap(),
            r#"NOT "FIELD_A" = "FIELD_B""#
        );
    }

    #[test]
    fn test_expr() {
        let field_a = &FieldName::new("FIELD_A");
        let field_b = &FieldName::new("FIELD_B");
        let cond_a = field_a.equal(field_b);
        let log_expr = cond_a.exp();

        assert_eq!(args_to_str(log_expr).unwrap(), r#"("FIELD_A" = "FIELD_B")"#);
    }

    #[test]
    fn test_and() {
        let field_a = &FieldName::new("FIELD_A");
        let field_b = &FieldName::new("FIELD_B");
        let field_c = &FieldName::new("FIELD_C");
        let cond_a = field_a.equal(field_b);
        let cond_b = field_b.equal(field_c);
        let log_expr = cond_a.and(cond_b);

        assert_eq!(
            args_to_str(log_expr).unwrap(),
            r#""FIELD_A" = "FIELD_B" AND "FIELD_B" = "FIELD_C""#
        );
    }

    #[test]
    fn test_or() {
        let field_a = &FieldName::new("FIELD_A");
        let field_b = &FieldName::new("FIELD_B");
        let field_c = &FieldName::new("FIELD_C");
        let cond_a = field_a.equal(field_b);
        let cond_b = field_b.equal(field_c);
        let log_expr = cond_a.or(cond_b);

        assert_eq!(
            args_to_str(log_expr).unwrap(),
            r#""FIELD_A" = "FIELD_B" OR "FIELD_B" = "FIELD_C""#
        );
    }
}
