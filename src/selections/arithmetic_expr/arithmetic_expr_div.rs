use crate::selections::ArithmeticExprWhere;
use crate::selections::TableField;
use crate::FieldName;
use std::ops::Div;

impl Div<FieldName> for i64 {
    type Output = ArithmeticExprWhere;
    fn div(self, rhs: FieldName) -> Self::Output {
        ArithmeticExprWhere::divide(self, rhs)
    }
}

impl Div<i64> for FieldName {
    type Output = ArithmeticExprWhere;
    fn div(self, rhs: i64) -> Self::Output {
        ArithmeticExprWhere::divide(self, rhs)
    }
}

impl Div<FieldName> for FieldName {
    type Output = ArithmeticExprWhere;
    fn div(self, rhs: FieldName) -> Self::Output {
        ArithmeticExprWhere::divide(self, rhs)
    }
}

impl Div<TableField> for i64 {
    type Output = ArithmeticExprWhere;
    fn div(self, rhs: TableField) -> Self::Output {
        ArithmeticExprWhere::divide(self, rhs)
    }
}

impl Div<i64> for TableField {
    type Output = ArithmeticExprWhere;
    fn div(self, rhs: i64) -> Self::Output {
        ArithmeticExprWhere::divide(self, rhs)
    }
}

impl Div<TableField> for TableField {
    type Output = ArithmeticExprWhere;
    fn div(self, rhs: TableField) -> Self::Output {
        ArithmeticExprWhere::divide(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::selections::{ArithmeticExprWhere, IntoArithmeticExprWhere, TableField};
    use crate::FieldName;

    #[test]
    fn test_arithmetic_div_field_name_i64() {
        let field = FieldName::new("price");
        let op = field.clone() / 100i64;
        assert_eq!(
            op,
            ArithmeticExprWhere::Divide(
                field.into_arithmetic_expr_where().boxed(),
                100i64.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_div_i64_field_name() {
        let field = FieldName::new("price");
        let op = 100i64 / field.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Divide(
                100i64.into_arithmetic_expr_where().boxed(),
                field.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_div_field_name() {
        let field_a = FieldName::new("price");
        let field_b = FieldName::new("factor");
        let op = field_a.clone() / field_b.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Divide(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_div_table_field_i64() {
        let field_a = TableField::new("SYMBOL.price");
        let op = field_a.clone() / 100i64;
        assert_eq!(
            op,
            ArithmeticExprWhere::Divide(
                field_a.into_arithmetic_expr_where().boxed(),
                100i64.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_div_i64_table_field() {
        let field_a = TableField::new("SYMBOL.price");
        let op = 100i64 / field_a.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Divide(
                100i64.into_arithmetic_expr_where().boxed(),
                field_a.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_div_table_field() {
        let field_a = TableField::new("SYMBOL.price");
        let field_b = TableField::new("SYMBOL.factor'");
        let op = field_a.clone() / field_b.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Divide(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed(),
            )
        );
    }
}
