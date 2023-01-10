use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindName {
    name: String,
}

impl BindName {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
pub trait IntoBindName {
    fn into_bind_name(self) -> BindName;
}

impl IntoBindName for BindName {
    fn into_bind_name(self) -> BindName {
        self
    }
}

impl IntoBindName for &str {
    fn into_bind_name(self) -> BindName {
        BindName {
            name: self.to_string(),
        }
    }
}

impl IntoBindName for String {
    fn into_bind_name(self) -> BindName {
        BindName { name: self }
    }
}
