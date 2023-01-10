use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer, Serializer};

pub const FORMAT_DATE: &str = "%Y-%m-%d";
pub const FORMAT_DATE_TIME: &str = "%Y-%m-%dT%H:%M:%S";

pub mod iso_date_format {
    use super::*;

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT_DATE));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT_DATE).map_err(serde::de::Error::custom)
    }
}

pub mod iso_date_opt_format {
    use super::*;

    pub fn serialize<S>(date: &Option<NaiveDate>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ref d) = *date {
            return s.serialize_str(&d.format(FORMAT_DATE).to_string());
        }
        s.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(Some(
                NaiveDate::parse_from_str(&s, FORMAT_DATE).map_err(serde::de::Error::custom)?,
            ));
        }

        Ok(None)
    }
}

pub mod iso_date_time_format {
    use super::*;
    use chrono::NaiveDateTime;

    fn date_time_to_json(date_time: &NaiveDateTime) -> String {
        let s = format!("{}", date_time.format(FORMAT_DATE));
        s
    }

    pub fn serialize<S>(date_time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date_time_to_json(date_time);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT_DATE_TIME).map_err(serde::de::Error::custom)
    }
}
