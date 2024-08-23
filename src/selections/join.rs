use super::from::{FromSelect, IntoFrom};
use crate::{
    resolvers::args_resolver::ArgsResolver,
    selections::{
        condition_where::{ConditionWhere, IntoConditionWhere},
        to_sql::ToSQL,
    },
    SQLError,
};
use serde::{Deserialize, Serialize};

/// Definition for SQL join relation between other tables/queries.
/// ```
/// # use voxi_core::selections::Join;
/// # use voxi_core::selections::IntoTableField;
/// # use voxi_core::selections::ToSQL;
/// # use voxi_core::selections::ConditionWhereOperation;
/// # use voxi_core::resolvers::args_resolver_string::args_to_str;
/// let field_detail = "DET.MASTER".into_table_field();
/// let field_master = "MAS.ID".into_table_field();
/// let join = Join::inner("DETAIL DET", field_detail.equal(field_master));
/// assert_eq!(
///     args_to_str(join).unwrap(),
///     r#"INNER JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Join {
    from: FromSelect,
    condition: ConditionWhere,
    join_type: JoinType,
}

impl Join {
    /// Create a Join from a IntoJoin implementation.
    /// ```
    /// # use voxi_core::selections::Join;
    /// # use voxi_core::selections::IntoTableField;
    /// # use voxi_core::selections::ToSQL;
    /// # use voxi_core::selections::ConditionWhereOperation;
    /// let field_detail = "DET.MASTER".into_table_field();
    /// let field_master = "MAS.ID".into_table_field();
    /// let join = Join::full("DETAIL DET", field_detail.equal(field_master));
    /// let new_join = Join::new(join.clone());
    /// assert_eq!(join, new_join);
    /// ```
    pub fn new(into_join: impl IntoJoin) -> Self {
        into_join.into_join()
    }

    /// Define SQL join for INNER JOIN.
    /// ```
    /// # use voxi_core::selections::Join;
    /// # use voxi_core::selections::IntoTableField;
    /// # use voxi_core::selections::ToSQL;
    /// # use voxi_core::selections::ConditionWhereOperation;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let field_detail = "DET.MASTER".into_table_field();
    /// let field_master = "MAS.ID".into_table_field();
    /// let join = Join::inner("DETAIL DET", field_detail.equal(field_master));
    /// assert_eq!(
    ///     args_to_str(join).unwrap(),
    ///     r#"INNER JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
    /// );
    /// ```
    pub fn inner(from: impl IntoFrom, condition: impl IntoConditionWhere) -> Self {
        Self {
            from: from.into_from(),
            condition: condition.into_condition_where(),
            join_type: JoinType::Inner,
        }
    }

    /// Define SQL join for FULL JOIN.
    /// ```
    /// # use voxi_core::selections::Join;
    /// # use voxi_core::selections::IntoTableField;
    /// # use voxi_core::selections::ToSQL;
    /// # use voxi_core::selections::ConditionWhereOperation;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let field_detail = "DET.MASTER".into_table_field();
    /// let field_master = "MAS.ID".into_table_field();
    /// let join = Join::full("DETAIL DET", field_detail.equal(field_master));
    /// assert_eq!(
    ///     args_to_str(join).unwrap(),
    ///     r#"FULL JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
    /// );
    /// ```
    pub fn full(from: impl IntoFrom, condition: impl IntoConditionWhere) -> Self {
        Self {
            from: from.into_from(),
            condition: condition.into_condition_where(),
            join_type: JoinType::Full,
        }
    }

    /// Define SQL join for LEFT JOIN.
    /// ```
    /// # use voxi_core::selections::Join;
    /// # use voxi_core::selections::IntoTableField;
    /// # use voxi_core::selections::ToSQL;
    /// # use voxi_core::selections::ConditionWhereOperation;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let field_detail = "DET.MASTER".into_table_field();
    /// let field_master = "MAS.ID".into_table_field();
    /// let join = Join::left("DETAIL DET", field_detail.equal(field_master));
    /// assert_eq!(
    ///     args_to_str(join).unwrap(),
    ///     r#"LEFT JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
    /// );
    /// ```
    pub fn left(from: impl IntoFrom, condition: impl IntoConditionWhere) -> Self {
        Self {
            from: from.into_from(),
            condition: condition.into_condition_where(),
            join_type: JoinType::Left,
        }
    }

    /// Define SQL join for RIGHT JOIN.
    /// ```
    /// # use crate::voxi_core::selections::ToSQL;
    /// # use voxi_core::selections::IntoTableField;
    /// # use voxi_core::selections::Join;
    /// # use voxi_core::selections::ConditionWhereOperation;
    /// # use voxi_core::resolvers::args_resolver_string::args_to_str;
    /// let field_detail = "DET.MASTER".into_table_field();
    /// let field_master = "MAS.ID".into_table_field();
    /// let join = Join::right("DETAIL DET", field_detail.equal(field_master));
    /// assert_eq!(
    ///     args_to_str(join).unwrap(),
    ///     r#"RIGHT JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
    /// );
    /// ```
    pub fn right(from: impl IntoFrom, condition: impl IntoConditionWhere) -> Self {
        Self {
            from: from.into_from(),
            condition: condition.into_condition_where(),
            join_type: JoinType::Right,
        }
    }

    /// Get a reference to the condition's table name.
    pub fn condition(&self) -> &ConditionWhere {
        &self.condition
    }

    /// Get a reference to the join's table name.
    pub fn from(&self) -> &FromSelect {
        &self.from
    }

    /// Get a reference to the join's join type.
    pub fn join_type(&self) -> &JoinType {
        &self.join_type
    }
}

pub trait IntoJoin {
    fn into_join(self) -> Join;
}

impl IntoJoin for Join {
    fn into_join(self) -> Join {
        self
    }
}

pub trait IntoJoins {
    fn into_joins(self) -> Vec<Join>;
}

impl IntoJoins for Join {
    fn into_joins(self) -> Vec<Join> {
        vec![self]
    }
}

impl IntoJoins for Vec<Join> {
    fn into_joins(self) -> Vec<Join> {
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum JoinType {
    Full,
    Inner,
    Left,
    Right,
}

impl ToSQL for Join {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLError> {
        let join = match self.join_type {
            JoinType::Full => "FULL JOIN",
            JoinType::Inner => "INNER JOIN",
            JoinType::Left => "LEFT JOIN",
            JoinType::Right => "RIGHT JOIN",
        };
        let sql = format!(
            "{} {} ON {}",
            join,
            self.from.to_sql(args_resolver)?,
            self.condition.to_sql(args_resolver)?
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_condition() {
        let mut args_resolver_string = ArgsResolverString::new();
        let field_detail = "DET.MASTER".into_table_field();
        let field_master = "MAS.ID".into_table_field();
        let join = Join::inner("DETAIL DET", field_detail.equal(field_master));
        assert_eq!(
            join.condition().to_sql(&mut args_resolver_string).unwrap(),
            r#""DET"."MASTER" = "MAS"."ID""#
        );
    }

    #[test]
    fn test_join_inner() {
        let mut args_resolver_string = ArgsResolverString::new();
        let field_detail = "DET.MASTER".into_table_field();
        let field_master = "MAS.ID".into_table_field();
        let join = Join::inner("DETAIL DET", field_detail.equal(field_master));
        assert_eq!(
            join.to_sql(&mut args_resolver_string).unwrap(),
            r#"INNER JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
        );
    }

    #[test]
    fn test_join_left() {
        let mut args_resolver_string = ArgsResolverString::new();
        let field_detail = "DET.MASTER".into_table_field();
        let field_master = "MAS.ID".into_table_field();
        let join = Join::left("DETAIL DET", field_detail.equal(field_master));
        assert_eq!(
            join.to_sql(&mut args_resolver_string).unwrap(),
            r#"LEFT JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
        );
    }

    #[test]
    fn test_join_right() {
        let mut args_resolver_string = ArgsResolverString::new();
        let field_detail = "DET.MASTER".into_table_field();
        let field_master = "MAS.ID".into_table_field();
        let join = Join::right("DETAIL DET", field_detail.equal(field_master));
        assert_eq!(
            join.to_sql(&mut args_resolver_string).unwrap(),
            r#"RIGHT JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
        );
    }

    #[test]
    fn test_join_full() {
        let mut args_resolver_string = ArgsResolverString::new();
        let field_detail = "DET.MASTER".into_table_field();
        let field_master = "MAS.ID".into_table_field();
        let join = Join::full("DETAIL DET", field_detail.equal(field_master));
        assert_eq!(
            join.to_sql(&mut args_resolver_string).unwrap(),
            r#"FULL JOIN "DETAIL" "DET" ON "DET"."MASTER" = "MAS"."ID""#
        );
    }

    #[test]
    fn test_join_new() {
        let field_detail = "DET.MASTER".into_table_field();
        let field_master = "MAS.ID".into_table_field();
        let join = Join::full("DETAIL DET", field_detail.equal(field_master));
        let new_join = Join::new(join.clone());
        assert_eq!(join, new_join);
    }
}
