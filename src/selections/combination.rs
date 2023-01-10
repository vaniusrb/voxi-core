use crate::{builder::args_resolver::ArgsResolver, SQLRoxiError};

use super::{
    select::{IntoSelect, Select},
    to_sql::ToSQL,
};
use serde::{Deserialize, Serialize};

/// Define combination with other query, like `UNION`, `UNION ALL`, `INTERCEPT` and `EXCEPT`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Combination {
    query: Select,
    combination_type: CombinationType,
}

impl Combination {
    // TODO: add comment
    pub fn new(query: impl IntoSelect, combination_type: CombinationType) -> Self {
        Self {
            query: query.into_select(),
            combination_type,
        }
    }

    pub fn union(query: impl IntoSelect) -> Self {
        Self::new(query, CombinationType::Union)
    }

    pub fn union_all(query: impl IntoSelect) -> Self {
        Self::new(query, CombinationType::UnionAll)
    }

    pub fn intersect(query: impl IntoSelect) -> Self {
        Self::new(query, CombinationType::Intersect)
    }

    pub fn except(query: impl IntoSelect) -> Self {
        Self::new(query, CombinationType::Except)
    }

    /// Get a reference to the combination's combination type.
    pub fn combination_type(&self) -> &CombinationType {
        &self.combination_type
    }

    /// Get a reference to the combination's query.
    pub fn query(&self) -> &Select {
        &self.query
    }
}

impl ToSQL for Combination {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, SQLRoxiError> {
        let lit = match self.combination_type {
            CombinationType::Union => "UNION",
            CombinationType::UnionAll => "UNION ALL",
            CombinationType::Intersect => "INTERSECT",
            CombinationType::Except => "EXCEPT",
        };
        Ok(format!("{} {}", lit, self.query.to_sql(args_resolver)?))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombinationType {
    Union,
    UnionAll,
    Intersect,
    Except,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builder::args_resolver_string::ArgsResolverString, selections::select::QueryBuilder,
    };

    #[test]
    fn test_query_union() {
        let query = QueryBuilder::new()
            .field("ID")
            .from("TABLE")
            .build()
            .unwrap();
        let combination = Combination::union(query.clone());
        assert_eq!(combination.query(), &query);
    }

    #[test]
    fn test_type() {
        let query = QueryBuilder::new()
            .field("ID")
            .from("TABLE")
            .build()
            .unwrap();
        let combination = Combination::union(query.clone());
        assert_eq!(combination.combination_type(), &CombinationType::Union);

        let combination = Combination::union_all(query.clone());
        assert_eq!(combination.combination_type(), &CombinationType::UnionAll);

        let combination = Combination::except(query.clone());
        assert_eq!(combination.combination_type(), &CombinationType::Except);

        let combination = Combination::intersect(query);
        assert_eq!(combination.combination_type(), &CombinationType::Intersect);
    }

    #[test]
    fn test_to_sql() {
        let mut args_resolver_string = ArgsResolverString::new();
        let query = QueryBuilder::new()
            .field("ID")
            .from("TABLE")
            .build()
            .unwrap();
        let combination = Combination::union(query.clone());
        assert_eq!(
            combination.to_sql(&mut args_resolver_string).unwrap(),
            r#"UNION SELECT "ID" FROM "TABLE""#
        );

        let combination = Combination::union_all(query.clone());
        assert_eq!(
            combination.to_sql(&mut args_resolver_string).unwrap(),
            r#"UNION ALL SELECT "ID" FROM "TABLE""#
        );

        let combination = Combination::except(query.clone());
        assert_eq!(
            combination.to_sql(&mut args_resolver_string).unwrap(),
            r#"EXCEPT SELECT "ID" FROM "TABLE""#
        );

        let combination = Combination::intersect(query);
        assert_eq!(
            combination.to_sql(&mut args_resolver_string).unwrap(),
            r#"INTERSECT SELECT "ID" FROM "TABLE""#
        );
    }
}
