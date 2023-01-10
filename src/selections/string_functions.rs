use super::{
    to_sql::ToSQL,
    value_where::{IntoValueWhere, ValueWhere},
    values_where::{IntoValuesWhere, ValuesWhere},
};
use crate::{builder::args_resolver::ArgsResolver, SQLRoxiError};
use serde::{Deserialize, Serialize};

// TODO: add comment
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringFunction {
    Upper(ValueWhere),
    Lower(ValueWhere),
    Substring(ValueWhere, u16, u16),
    Replace(ValueWhere, ValueWhere, ValueWhere),
    Concat(ValuesWhere),
}

impl StringFunction {
    /// Define SQL function for UPPER(<field>).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::StringFunction;
    /// let upper = StringFunction::upper("NAME");
    /// ```
    pub fn upper(value_where: impl IntoValueWhere) -> Self {
        StringFunction::Upper(value_where.into_value_where())
    }

    /// Define SQL function for LOWER(<field>).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::StringFunction;
    /// let lower = StringFunction::lower("NAME");
    /// ```
    pub fn lower(value_where: impl IntoValueWhere) -> Self {
        StringFunction::Lower(value_where.into_value_where())
    }

    /// Define SQL function for SUBSTRING(<field> from <start> for <length>).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::StringFunction;
    /// let substring = StringFunction::substring("NAME", 1, 4);
    /// ```
    pub fn substring(value_where: impl IntoValueWhere, start: u16, length: u16) -> Self {
        StringFunction::Substring(value_where.into_value_where(), start, length)
    }

    /// Define SQL function for SUBSTRING(<field> from <start> for <length>).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::StringFunction;
    /// let substring = StringFunction::substring("NAME", 1, 4);
    /// ```
    pub fn concat(values_where: impl IntoValuesWhere) -> Self {
        StringFunction::Concat(values_where.into_values_where())
    }

    /// Define SQL function for REPLACE(<field>, <old string>, <new string>).
    /// # Example
    /// ```
    /// # use roxi_sql::selections::StringFunction;
    /// let replace = StringFunction::replace("NAME", "OLD", "NEW");
    /// ```
    pub fn replace(
        value_where: impl IntoValueWhere,
        old: impl IntoValueWhere,
        new: impl IntoValueWhere,
    ) -> Self {
        StringFunction::Replace(
            value_where.into_value_where(),
            old.into_value_where(),
            new.into_value_where(),
        )
    }
}

impl ToSQL for StringFunction {
    fn to_sql(
        &self,
        args_resolver: &mut dyn ArgsResolver,
    ) -> error_stack::Result<String, SQLRoxiError> {
        let sql = match &self {
            StringFunction::Upper(value_where) => {
                format!("UPPER({})", value_where.to_sql(args_resolver)?)
            }
            StringFunction::Lower(value_where) => {
                format!("LOWER({})", value_where.to_sql(args_resolver)?)
            }
            StringFunction::Substring(value_where, start, length) => format!(
                "SUBSTRING({} FROM {} FOR {})",
                value_where.to_sql(args_resolver)?,
                start,
                length
            ),
            StringFunction::Replace(value_where, old, new) => {
                format!(
                    "REPLACE({}, {}, {})",
                    value_where.to_sql(args_resolver)?,
                    old.to_sql(args_resolver)?,
                    new.to_sql(args_resolver)?
                )
            }
            StringFunction::Concat(values_where) => {
                format!("CONCAT({})", values_where.to_sql(args_resolver)?)
            }
        };
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builder::args_resolver_string::ArgsResolverString, selections::table_field::IntoTableField,
    };

    #[test]
    fn test_upper() {
        let mut args_resolver_string = ArgsResolverString::new();
        let str_fun = StringFunction::upper("TEXT");
        assert_eq!(str_fun, StringFunction::Upper("TEXT".into_value_where()));
        assert_eq!(
            str_fun.to_sql(&mut args_resolver_string).unwrap(),
            r#"UPPER('TEXT')"#
        );

        let value = "FIELD".into_table_field().into_value_where();
        let str_fun = StringFunction::upper(value);
        assert_eq!(
            str_fun,
            StringFunction::Upper("FIELD".into_table_field().into_value_where())
        );
        assert_eq!(
            str_fun.to_sql(&mut args_resolver_string).unwrap(),
            r#"UPPER("FIELD")"#
        );
    }

    #[test]
    fn test_lower() {
        let mut args_resolver_string = ArgsResolverString::new();
        let str_fun = StringFunction::lower("TEXT");
        assert_eq!(str_fun, StringFunction::Lower("TEXT".into_value_where()));
        assert_eq!(
            str_fun.to_sql(&mut args_resolver_string).unwrap(),
            r#"LOWER('TEXT')"#
        );

        let str_fun = StringFunction::lower("FIELD".into_table_field());
        assert_eq!(
            str_fun,
            StringFunction::Lower("FIELD".into_table_field().into_value_where())
        );
        assert_eq!(
            str_fun.to_sql(&mut args_resolver_string).unwrap(),
            r#"LOWER("FIELD")"#
        );
    }

    #[test]
    fn test_substring() {
        let mut args_resolver_string = ArgsResolverString::new();
        let str_fun = StringFunction::substring("TEXT", 1, 4);
        assert_eq!(
            str_fun,
            StringFunction::Substring("TEXT".into_value_where(), 1, 4)
        );
        assert_eq!(
            str_fun.to_sql(&mut args_resolver_string).unwrap(),
            r#"SUBSTRING('TEXT' FROM 1 FOR 4)"#
        );
    }

    #[test]
    fn test_replace() {
        let mut args_resolver_string = ArgsResolverString::new();
        let str_fun = StringFunction::replace("TEXT", "OLD", "NEW");
        assert_eq!(
            str_fun,
            StringFunction::Replace(
                "TEXT".into_value_where(),
                "OLD".into_value_where(),
                "NEW".into_value_where()
            )
        );
        assert_eq!(
            str_fun.to_sql(&mut args_resolver_string).unwrap(),
            r#"REPLACE('TEXT', 'OLD', 'NEW')"#
        );
    }

    #[test]
    fn test_concat() {
        let mut args_resolver_string = ArgsResolverString::new();
        let str_fun = StringFunction::concat(vec!["OLD", "NEW"]);
        assert_eq!(
            str_fun,
            StringFunction::Concat(
                vec!["OLD".into_value_where(), "NEW".into_value_where()].into_values_where()
            )
        );
        assert_eq!(
            str_fun.to_sql(&mut args_resolver_string).unwrap(),
            r#"CONCAT('OLD','NEW')"#
        );
    }
}
