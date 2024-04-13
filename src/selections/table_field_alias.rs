use std::fmt;

use super::{
    Alias, IntoTableField, IntoValueSelect, Table, TableField, ToSQL, ValueSelect, ValueWhere,
};
use crate::{
    resolvers::{args_resolver::ArgsResolver, args_resolver_string::ArgsResolverString},
    FieldName, IntoFieldName,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableFieldAlias {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(flatten)]
    pub table: Option<Table>,
    pub field_name: FieldName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alias: Option<Alias>,
}

impl IntoValueSelect for TableFieldAlias {
    fn into_value_select(self) -> ValueSelect {
        ValueSelect {
            value_where: ValueWhere::FieldName(TableField {
                table: self.table,
                field_name: self.field_name,
            }),
            alias: self.alias,
        }
    }
}

impl IntoTableField for TableFieldAlias {
    fn into_table_field(self) -> TableField {
        TableField {
            table: self.table,
            field_name: self.field_name,
        }
    }
}

impl IntoFieldName for TableFieldAlias {
    fn into_field_name(self) -> FieldName {
        self.field_name
    }
}

impl ToSQL for TableFieldAlias {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, crate::SQLError> {
        self.clone().into_value_select().to_sql(args_resolver)
    }
}

impl fmt::Display for TableFieldAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_sql(&mut ArgsResolverString::new()).unwrap()
        )
    }
}
