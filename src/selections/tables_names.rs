use super::{
    condition_where::ConditionWhere, logical_expr_where::LogicalExprWhere, table_name::TableName,
    value_where::ValueWhere, ArithmeticExprWhere,
};
use std::collections::HashSet;

// TODO: add comment
pub trait TablesNames {
    fn tables_names(&self) -> HashSet<&TableName>;
}

impl TablesNames for ArithmeticExprWhere {
    fn tables_names(&self) -> HashSet<&TableName> {
        match self {
            ArithmeticExprWhere::ValueWhere(c) => c.tables_names().into_iter().collect(),
            ArithmeticExprWhere::Expression(e) => e.tables_names(),
            ArithmeticExprWhere::Subtract(e1, e2)
            | ArithmeticExprWhere::Multiply(e1, e2)
            | ArithmeticExprWhere::Divide(e1, e2)
            | ArithmeticExprWhere::Add(e1, e2) => e1
                .tables_names()
                .into_iter()
                .chain(e2.tables_names().into_iter())
                .collect(),
        }
    }
}

impl TablesNames for ConditionWhere {
    fn tables_names(&self) -> HashSet<&TableName> {
        match self {
            ConditionWhere::ConditionEq(t, _)
            | ConditionWhere::ConditionNull(t)
            | ConditionWhere::ConditionDf(t, _)
            | ConditionWhere::ConditionGt(t, _)
            | ConditionWhere::ConditionLs(t, _)
            | ConditionWhere::ConditionGe(t, _)
            | ConditionWhere::ConditionLe(t, _)
            | ConditionWhere::ConditionIn(t, _) => t.tables_names().into_iter().collect(),
            ConditionWhere::ConditionBetween(t, _, _) => t.tables_names().into_iter().collect(),
            ConditionWhere::Expression(e) => e.tables_names(),
            ConditionWhere::Exists(_) => todo!(),
            ConditionWhere::ConditionLk(_, _) => todo!(),
        }
    }
}

impl TablesNames for LogicalExprWhere {
    fn tables_names(&self) -> HashSet<&TableName> {
        match self {
            LogicalExprWhere::Condition(c) => c.tables_names().into_iter().collect(),
            LogicalExprWhere::Not(e) | LogicalExprWhere::Expression(e) => e.tables_names(),
            LogicalExprWhere::And(e1, e2) | LogicalExprWhere::Or(e1, e2) => {
                let e: HashSet<&TableName> = e1
                    .tables_names()
                    .into_iter()
                    .chain(e2.tables_names().into_iter())
                    .collect();
                e
            }
        }
    }
}

impl TablesNames for ValueWhere {
    /// FIXME: probably tables_names could be removed
    /// Get a reference to the value condition's table name.
    fn tables_names(&self) -> HashSet<&TableName> {
        match self {
            ValueWhere::LiteralValue(_) => HashSet::default(),
            ValueWhere::FieldName(t) => t.table.iter().map(|f| f.name()).collect(),
            // TODO: implement!
            ValueWhere::Expression(_) => HashSet::default(),
            ValueWhere::BindParameter(_) => HashSet::default(),
            ValueWhere::SingleQuery(s) => s.tables_names(),
            ValueWhere::CaseCondition(_) => todo!(),
            ValueWhere::CaseValue(_) => todo!(),
            ValueWhere::AggFunction(_) => todo!(),
            ValueWhere::StringFunction(_) => todo!(),
        }
    }
}

impl TablesNames for Vec<ValueWhere> {
    fn tables_names(&self) -> HashSet<&TableName> {
        self.iter()
            .flat_map(|v| v.tables_names())
            .collect::<HashSet<_>>()
    }
}
