use crate::{FieldName, FieldNameType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalForeignField {
    pub local: FieldNameType,
    pub foreign: FieldName,
}
