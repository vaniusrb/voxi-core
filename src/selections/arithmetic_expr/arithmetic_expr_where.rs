use crate::FieldName;
use crate::IntoNullableValue;
use crate::Value;
use crate::{
    resolvers::args_resolver::ArgsResolver,
    selections::{TableField, ToSQL, ValueWhere},
    SQLError,
};
use serde::{Deserialize, Serialize};

/// Represents arithmetic operations, included expressions.
/// # Example
/// ```
/// # use voxi_core::selections::TableField;
/// # use voxi_core::resolvers::args_resolver_string::args_to_str;
/// # use voxi_core::selections::ArithmeticExprWhere;
/// let op = ArithmeticExprWhere::add(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
/// assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" + "FIELD_B""#)
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArithmeticExprWhere {
    ValueWhere(Box<ValueWhere>),
    Expression(Box<ArithmeticExprWhere>),
    Add(Box<ArithmeticExprWhere>, Box<ArithmeticExprWhere>),
    Subtract(Box<ArithmeticExprWhere>, Box<ArithmeticExprWhere>),
    Multiply(Box<ArithmeticExprWhere>, Box<ArithmeticExprWhere>),
    Divide(Box<ArithmeticExprWhere>, Box<ArithmeticExprWhere>),
}

impl ArithmeticExprWhere {
    // Add operation.
    /// # Example
    /// ```
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::ArithmeticExprWhere;
    /// let op = ArithmeticExprWhere::add(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
    /// assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" + "FIELD_B""#)
    /// ```
    pub fn add(
        a: impl IntoArithmeticExprWhere,
        b: impl IntoArithmeticExprWhere,
    ) -> ArithmeticExprWhere {
        let a = a.into_arithmetic_expr_where().boxed();
        let b = b.into_arithmetic_expr_where().boxed();
        ArithmeticExprWhere::Add(a, b)
    }

    /// Subtract operation.
    /// # Example
    /// ```
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::ArithmeticExprWhere;
    /// let op = ArithmeticExprWhere::subtract(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
    /// assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" - "FIELD_B""#)
    /// ```
    pub fn subtract(
        a: impl IntoArithmeticExprWhere,
        b: impl IntoArithmeticExprWhere,
    ) -> ArithmeticExprWhere {
        let a = a.into_arithmetic_expr_where().boxed();
        let b = b.into_arithmetic_expr_where().boxed();
        ArithmeticExprWhere::Subtract(a, b)
    }

    /// Subtract operation.
    /// # Example
    /// ```
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::ArithmeticExprWhere;
    /// let op = ArithmeticExprWhere::multiply(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
    /// assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" * "FIELD_B""#)
    /// ```
    pub fn multiply(
        a: impl IntoArithmeticExprWhere,
        b: impl IntoArithmeticExprWhere,
    ) -> ArithmeticExprWhere {
        let a = a.into_arithmetic_expr_where().boxed();
        let b = b.into_arithmetic_expr_where().boxed();
        ArithmeticExprWhere::Multiply(a, b)
    }

    /// Divide operation.
    /// # Example
    /// ```
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::ArithmeticExprWhere;
    /// let op = ArithmeticExprWhere::divide(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
    /// assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" / "FIELD_B""#)
    /// ```
    pub fn divide(
        a: impl IntoArithmeticExprWhere,
        b: impl IntoArithmeticExprWhere,
    ) -> ArithmeticExprWhere {
        let a = a.into_arithmetic_expr_where().boxed();
        let b = b.into_arithmetic_expr_where().boxed();
        ArithmeticExprWhere::Divide(a, b)
    }

    /// Create a arithmetic expression
    /// # Example
    /// ```
    /// # use voxi_core::selections::TableField;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// # use voxi_core::selections::ArithmeticExprWhere;
    /// let add = ArithmeticExprWhere::add(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
    /// let exp = ArithmeticExprWhere::expression(add);
    /// assert_eq!(args_to_str(&exp.clone()).unwrap(),String::from(r#"("FIELD_A" + "FIELD_B")"#));
    /// let div = ArithmeticExprWhere::divide(exp, TableField::new("FIELD_C"));
    /// assert_eq!(args_to_str(&div).unwrap(), String::from(r#"("FIELD_A" + "FIELD_B") / "FIELD_C""#));
    /// ```
    pub fn expression(a: impl IntoArithmeticExprWhere) -> ArithmeticExprWhere {
        ArithmeticExprWhere::Expression(a.into_arithmetic_expr_where().boxed())
    }

    /// Create a boxed expression, for internal use.
    pub fn boxed_expression(self) -> Box<ArithmeticExprWhere> {
        let expression = ArithmeticExprWhere::Expression(Box::new(self));
        Box::new(expression)
    }

    /// Create a boxed self, for internal use.
    pub(crate) fn boxed(self) -> Box<ArithmeticExprWhere> {
        Box::new(self)
    }
}

pub trait IntoArithmeticExprWhere {
    fn into_arithmetic_expr_where(self) -> ArithmeticExprWhere;
}

impl ToSQL for ArithmeticExprWhere {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLError> {
        let sql = match self {
            ArithmeticExprWhere::ValueWhere(c) => c.to_sql(args_resolver)?,
            ArithmeticExprWhere::Expression(e) => format!("({})", e.to_sql(args_resolver)?),
            ArithmeticExprWhere::Add(e1, e2) => {
                format!(
                    "{} + {}",
                    e1.to_sql(args_resolver)?,
                    e2.to_sql(args_resolver)?
                )
            }
            ArithmeticExprWhere::Subtract(e1, e2) => {
                format!(
                    "{} - {}",
                    e1.to_sql(args_resolver)?,
                    e2.to_sql(args_resolver)?
                )
            }
            ArithmeticExprWhere::Multiply(e1, e2) => {
                format!(
                    "{} * {}",
                    e1.to_sql(args_resolver)?,
                    e2.to_sql(args_resolver)?
                )
            }
            ArithmeticExprWhere::Divide(e1, e2) => {
                format!(
                    "{} / {}",
                    e1.to_sql(args_resolver)?,
                    e2.to_sql(args_resolver)?
                )
            }
        };
        Ok(sql)
    }
}
impl IntoArithmeticExprWhere for ArithmeticExprWhere {
    fn into_arithmetic_expr_where(self) -> ArithmeticExprWhere {
        self
    }
}

impl IntoArithmeticExprWhere for ValueWhere {
    fn into_arithmetic_expr_where(self) -> ArithmeticExprWhere {
        ArithmeticExprWhere::ValueWhere(Box::new(self))
    }
}

// trait IntoValueInteger {}

impl IntoArithmeticExprWhere for i64 {
    fn into_arithmetic_expr_where(self) -> ArithmeticExprWhere {
        let v = ValueWhere::LiteralValue(Value::from(self).into_nullable_value());
        ArithmeticExprWhere::ValueWhere(Box::new(v))
    }
}

impl IntoArithmeticExprWhere for FieldName {
    fn into_arithmetic_expr_where(self) -> ArithmeticExprWhere {
        let v = ValueWhere::TableField(TableField {
            table: None,
            field_name: self,
        });
        ArithmeticExprWhere::ValueWhere(Box::new(v))
    }
}
impl IntoArithmeticExprWhere for TableField {
    fn into_arithmetic_expr_where(self) -> ArithmeticExprWhere {
        let v = ValueWhere::TableField(self);
        ArithmeticExprWhere::ValueWhere(Box::new(v))
    }
}

#[cfg(test)]
mod tests {
    use super::ArithmeticExprWhere;
    use crate::{
        resolvers::args_resolver_string::args_to_str,
        selections::{
            condition_where::ConditionWhereOperation, logical_expr_where::LogicalExprWhereOps,
            table_field::TableField, IntoArithmeticExprWhere,
        },
    };

    #[test]
    fn test_arithmetic() {
        let id = TableField::new("SYMBOL.id");
        let name = TableField::new("SYMBOL.name");
        let price = TableField::new("SYMBOL.price");
        let discount = TableField::new("SYMBOL.discount");

        let log_expr = id
            .equal(1)
            .or(name.include(vec!["USD", "BRL"]))
            .exp()
            .and(price.equal(1000 + discount));

        let sql = args_to_str(&log_expr).unwrap();
        assert_eq!(
            sql,
            r#"("SYMBOL"."id" = 1 OR "SYMBOL"."name" IN ('USD','BRL')) AND "SYMBOL"."price" = 1000 + "SYMBOL"."discount""#
        );
    }

    #[test]
    fn test_arithmetic_add() {
        let field_a = TableField::new("FIELD_A");
        let field_b = TableField::new("FIELD_B");
        let op = ArithmeticExprWhere::add(field_a.clone(), field_b.clone());
        assert_eq!(
            op,
            ArithmeticExprWhere::Add(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed()
            )
        );
    }

    #[test]
    fn test_arithmetic_subtract() {
        let field_a = TableField::new("FIELD_A");
        let field_b = TableField::new("FIELD_B");
        let op = ArithmeticExprWhere::subtract(field_a.clone(), field_b.clone());
        assert_eq!(
            op,
            ArithmeticExprWhere::Subtract(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed()
            )
        );
    }

    #[test]
    fn test_arithmetic_multiply() {
        let field_a = TableField::new("FIELD_A");
        let field_b = TableField::new("FIELD_B");
        let op = ArithmeticExprWhere::multiply(field_a.clone(), field_b.clone());
        assert_eq!(
            op,
            ArithmeticExprWhere::Multiply(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed()
            )
        );
    }

    #[test]
    fn test_arithmetic_divide() {
        let field_a = TableField::new("FIELD_A");
        let field_b = TableField::new("FIELD_B");
        let op = ArithmeticExprWhere::divide(field_a.clone(), field_b.clone());
        assert_eq!(
            op,
            ArithmeticExprWhere::Divide(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed()
            )
        );
    }
}

#[cfg(test)]
mod test_sql {
    use super::ArithmeticExprWhere;
    use crate::{
        resolvers::args_resolver_string::args_to_str, selections::table_field::TableField,
    };

    #[test]
    fn test_arithmetic_add() {
        let op = ArithmeticExprWhere::add(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
        assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" + "FIELD_B""#)
    }

    #[test]
    fn test_arithmetic_subtract() {
        let op =
            ArithmeticExprWhere::subtract(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
        assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" - "FIELD_B""#)
    }

    #[test]
    fn test_arithmetic_multiply() {
        let op =
            ArithmeticExprWhere::multiply(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
        assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" * "FIELD_B""#)
    }

    #[test]
    fn test_arithmetic_divide() {
        let op =
            ArithmeticExprWhere::divide(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
        assert_eq!(args_to_str(&op).unwrap(), r#""FIELD_A" / "FIELD_B""#)
    }

    #[test]
    fn test_arithmetic_expression() {
        let add = ArithmeticExprWhere::add(TableField::new("FIELD_A"), TableField::new("FIELD_B"));
        let exp = ArithmeticExprWhere::expression(add);
        assert_eq!(
            args_to_str(&exp.clone()).unwrap(),
            r#"("FIELD_A" + "FIELD_B")"#
        );

        let div = ArithmeticExprWhere::divide(exp, TableField::new("FIELD_C"));
        assert_eq!(
            args_to_str(&div).unwrap(),
            r#"("FIELD_A" + "FIELD_B") / "FIELD_C""#
        )
    }
}
