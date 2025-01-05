use crate::FieldNameType;

pub trait FieldTypeDescriptor {
    fn fields_type(&self) -> Vec<FieldNameType>;
}
