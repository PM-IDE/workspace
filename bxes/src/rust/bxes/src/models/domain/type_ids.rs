use variant_count::VariantCount;
use crate::models::domain::bxes_value::BxesValue;

#[derive(FromPrimitive, ToPrimitive, VariantCount, Debug, PartialEq, Eq, Hash)]
pub enum TypeIds {
    Null = 0,
    I32 = 1,
    I64 = 2,
    U32 = 3,
    U64 = 4,
    F32 = 5,
    F64 = 6,
    String = 7,
    Bool = 8,
    Timestamp = 9,
    BrafLifecycle = 10,
    StandardLifecycle = 11,
    Artifact = 12,
    Drivers = 13,
    Guid = 14,
    SoftwareEventType = 15,
}

pub fn get_type_id(value: &BxesValue) -> TypeIds {
    match value {
        BxesValue::Null => TypeIds::Null,
        BxesValue::Int32(_) => TypeIds::I32,
        BxesValue::Int64(_) => TypeIds::I64,
        BxesValue::Uint32(_) => TypeIds::U32,
        BxesValue::Uint64(_) => TypeIds::U64,
        BxesValue::Float32(_) => TypeIds::F32,
        BxesValue::Float64(_) => TypeIds::F64,
        BxesValue::String(_) => TypeIds::String,
        BxesValue::Bool(_) => TypeIds::Bool,
        BxesValue::Timestamp(_) => TypeIds::Timestamp,
        BxesValue::BrafLifecycle(_) => TypeIds::BrafLifecycle,
        BxesValue::StandardLifecycle(_) => TypeIds::StandardLifecycle,
        BxesValue::Artifact(_) => TypeIds::Artifact,
        BxesValue::Drivers(_) => TypeIds::Drivers,
        BxesValue::Guid(_) => TypeIds::Guid,
        BxesValue::SoftwareEventType(_) => TypeIds::SoftwareEventType,
    }
}