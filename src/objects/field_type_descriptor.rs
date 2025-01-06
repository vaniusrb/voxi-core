use crate::{FieldName, ValueType};
use indexmap::IndexMap;

pub trait FieldTypeDescriptor {
    // fn fields_type(&self) -> Vec<FieldNameType>;
    fn fields_type(&self) -> IndexMap<FieldName, ValueType>;
}
