use crate::type_ids::TypeIds;

#[derive(Debug, PartialEq, Eq)]
pub struct ValueAttributeDescriptor {
    pub type_id: TypeIds,
    pub name: String,
}

pub struct SystemMetadata {
    pub values_attrs: Option<Vec<ValueAttributeDescriptor>>,
}
