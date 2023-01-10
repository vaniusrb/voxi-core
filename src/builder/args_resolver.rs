use crate::selections::bind_name::BindName;
use crate::NullableValue;

/// Trait for a callback to receive definition of argument (bind).
pub trait ArgsResolver {
    fn add_arg(&mut self, value: NullableValue) -> String;

    fn add_bind(&mut self, _bind_name: BindName) -> Option<NullableValue> {
        None
    }
}

#[cfg(test)]
mod test {}
