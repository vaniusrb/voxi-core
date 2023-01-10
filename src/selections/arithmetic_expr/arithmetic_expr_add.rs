use crate::selections::ArithmeticExprWhere;
use crate::selections::TableField;
use crate::FieldName;
use std::ops::Add;

impl Add<FieldName> for i64 {
    type Output = ArithmeticExprWhere;
    fn add(self, rhs: FieldName) -> Self::Output {
        ArithmeticExprWhere::add(self, rhs)
    }
}

impl Add<i64> for FieldName {
    type Output = ArithmeticExprWhere;
    fn add(self, rhs: i64) -> Self::Output {
        ArithmeticExprWhere::add(self, rhs)
    }
}

impl Add<FieldName> for FieldName {
    type Output = ArithmeticExprWhere;
    fn add(self, rhs: FieldName) -> Self::Output {
        ArithmeticExprWhere::add(self, rhs)
    }
}

impl Add<TableField> for i64 {
    type Output = ArithmeticExprWhere;
    fn add(self, rhs: TableField) -> Self::Output {
        ArithmeticExprWhere::add(self, rhs)
    }
}

impl Add<i64> for TableField {
    type Output = ArithmeticExprWhere;
    fn add(self, rhs: i64) -> Self::Output {
        ArithmeticExprWhere::add(self, rhs)
    }
}

impl Add<TableField> for TableField {
    type Output = ArithmeticExprWhere;
    fn add(self, rhs: TableField) -> Self::Output {
        ArithmeticExprWhere::add(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::FieldName;

    use crate::selections::{ArithmeticExprWhere, IntoArithmeticExprWhere, TableField};

    #[test]
    fn test_arithmetic_add_field_name_i64() {
        let field = FieldName::new("price");
        let op = field.clone() + 100i64;
        assert_eq!(
            op,
            ArithmeticExprWhere::Add(
                field.into_arithmetic_expr_where().boxed(),
                100i64.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_add_i64_field_name() {
        let field = FieldName::new("price");
        let op = 100i64 + field.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Add(
                100i64.into_arithmetic_expr_where().boxed(),
                field.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_add_field_name() {
        let field_a = FieldName::new("price");
        let field_b = FieldName::new("factor");
        let op = field_a.clone() + field_b.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Add(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_add_table_field_i64() {
        let field_a = TableField::new("SYMBOL.price");
        let op = field_a.clone() + 100i64;
        assert_eq!(
            op,
            ArithmeticExprWhere::Add(
                field_a.into_arithmetic_expr_where().boxed(),
                100i64.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_add_i64_table_field() {
        let field_a = TableField::new("SYMBOL.price");
        let op = 100i64 + field_a.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Add(
                100i64.into_arithmetic_expr_where().boxed(),
                field_a.into_arithmetic_expr_where().boxed(),
            )
        );
    }

    #[test]
    fn test_arithmetic_add_table_field() {
        let field_a = TableField::new("SYMBOL.price");
        let field_b = TableField::new("SYMBOL.factor'");
        let op = field_a.clone() + field_b.clone();
        assert_eq!(
            op,
            ArithmeticExprWhere::Add(
                field_a.into_arithmetic_expr_where().boxed(),
                field_b.into_arithmetic_expr_where().boxed(),
            )
        );
    }
}
