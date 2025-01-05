use std::collections::HashMap;

use crate::{FieldName, FieldNameType, ValueType};

pub trait FieldTypeDescriptor {
    // fn fields_type(&self) -> Vec<FieldNameType>;
    fn fields_type(&self) -> IndexMap<FieldName, ValueType>;
}
