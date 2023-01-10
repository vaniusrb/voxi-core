use crate::NullableValue;

use super::args_resolver::ArgsResolver;
use crate::selections::bind_name::BindName;
use std::collections::HashMap;

// This struct is used by `BindingQuery` and works like a decorator.
pub struct ArgsResolverBindsDecorator<'a> {
    binds_values: &'a HashMap<BindName, NullableValue>,
    super_args_resolver: &'a mut dyn ArgsResolver,
}

impl<'a> ArgsResolverBindsDecorator<'a> {
    pub fn new(
        super_args_resolver: &'a mut dyn ArgsResolver,
        binds_values: &'a HashMap<BindName, NullableValue>,
    ) -> Self {
        Self {
            binds_values,
            super_args_resolver,
        }
    }
}

impl<'a> ArgsResolver for ArgsResolverBindsDecorator<'a> {
    fn add_arg(&mut self, value: NullableValue) -> String {
        self.super_args_resolver.add_arg(value)
    }

    fn add_bind(&mut self, bind_name: BindName) -> Option<NullableValue> {
        self.binds_values.get(&bind_name).cloned()
    }
}
