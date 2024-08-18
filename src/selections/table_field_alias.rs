use super::{
    Alias, FieldAttribs, IntoTableField, IntoValueSelect, TableField, ToSQL, ValueSelect,
    ValueWhere,
};
use crate::{
    resolvers::{args_resolver::ArgsResolver, args_resolver_string::ArgsResolverString},
    FieldName, FieldNameType, IntoFieldName,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableFieldAlias {
    #[serde(flatten)]
    pub table_field: TableField,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(default)]
    // pub table: Option<Table>,
    // pub field_name: FieldName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alias: Option<Alias>,
}

pub trait IntoTableFieldAlias {
    fn into_table_field_alias(self) -> TableFieldAlias;
}

impl IntoTableFieldAlias for &str {
    fn into_table_field_alias(self) -> TableFieldAlias {
        self.into_field_name().into_table_field_alias()
    }
}

impl IntoTableFieldAlias for String {
    fn into_table_field_alias(self) -> TableFieldAlias {
        self.into_field_name().into_table_field_alias()
    }
}

impl IntoTableFieldAlias for FieldNameType {
    fn into_table_field_alias(self) -> TableFieldAlias {
        self.name.into_table_field_alias()
    }
}

impl IntoTableFieldAlias for TableFieldAlias {
    fn into_table_field_alias(self) -> TableFieldAlias {
        self
    }
}

impl IntoTableFieldAlias for TableField {
    fn into_table_field_alias(self) -> TableFieldAlias {
        TableFieldAlias {
            table_field: self,
            alias: None,
        }
    }
}

impl IntoTableFieldAlias for FieldName {
    fn into_table_field_alias(self) -> TableFieldAlias {
        TableFieldAlias {
            table_field: TableField {
                table: None,
                field_name: self,
            },
            alias: None,
        }
    }
}

impl IntoTableFieldAlias for FieldAttribs {
    fn into_table_field_alias(self) -> TableFieldAlias {
        // let alias_opt = self
        //     .value_select_name
        //     .value_select
        //     .as_ref()
        //     .map(|value_select| &value_select.alias)
        //     .cloned()
        //     .flatten();
        // let alias = Some(Alias::new(self.value_select_name.name.0.clone()));
        let alias = Some(Alias::new(self.value_select_name.name.0.clone()));
        let tf = self.value_select_name.into_table_field();
        TableFieldAlias {
            table_field: tf,
            alias,
        }
    }
}

impl IntoValueSelect for TableFieldAlias {
    fn into_value_select(self) -> ValueSelect {
        ValueSelect {
            value_where: ValueWhere::TableField(self.table_field),
            alias: self.alias,
        }
    }
}

impl IntoTableField for TableFieldAlias {
    fn into_table_field(self) -> TableField {
        self.table_field
    }
}

impl IntoFieldName for TableFieldAlias {
    fn into_field_name(self) -> FieldName {
        match self.alias {
            Some(alias) => FieldName(alias.alias),
            None => self.table_field.field_name,
        }
    }
}

impl ToSQL for TableFieldAlias {
    fn to_sql(&self, args_resolver: &mut dyn ArgsResolver) -> Result<String, crate::SQLError> {
        self.clone().into_value_select().to_sql(args_resolver)
    }
}

impl TableFieldAlias {
    pub fn with_out_alias(mut self) -> Self {
        self.alias = None;
        self
    }
}

impl fmt::Display for TableFieldAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.to_sql(&mut ArgsResolverString::new()).unwrap(),
            self.alias
                .as_ref()
                .map(|a| a.to_string())
                .unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TableFieldAlias;
    use crate::{
        selections::{Alias, Table, TableField},
        FieldName, IntoFieldName,
    };

    #[test]
    fn test() {
        let field_alias = TableFieldAlias {
            table_field: TableField {
                table: Some(Table::new("table")),
                field_name: FieldName("field_name".into()),
            },
            alias: Some(Alias::new("alias".into())),
        };
        assert_eq!(field_alias.into_field_name(), FieldName("alias".into()));
        let field_alias = TableFieldAlias {
            table_field: TableField {
                table: Some(Table::new("table")),
                field_name: FieldName("field_name".into()),
            },
            alias: None,
        };
        assert_eq!(
            field_alias.into_field_name(),
            FieldName("field_name".into())
        );
    }
}
