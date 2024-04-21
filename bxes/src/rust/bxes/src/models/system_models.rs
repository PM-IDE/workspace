use crate::type_ids::TypeIds;

#[derive(Debug, PartialEq, Eq)]
pub struct ValueAttributeDescriptor {
    pub type_id: TypeIds,
    pub name: String,
}

impl ValueAttributeDescriptor {
    pub fn new(type_id: TypeIds, name: String) -> Self {
        Self { type_id, name }
    }
}

pub struct SystemMetadata {
    pub values_attrs: Option<Vec<ValueAttributeDescriptor>>,
}

impl SystemMetadata {
    pub fn new(value_attributes: Option<Vec<ValueAttributeDescriptor>>) -> Self {
        Self {
            values_attrs: value_attributes,
        }
    }
}
