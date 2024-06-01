use crate::models::domain::bxes_artifact::BxesArtifact;
use crate::models::domain::bxes_driver::BxesDrivers;
use crate::models::domain::bxes_lifecycle::{BrafLifecycle, StandardLifecycle};
use crate::models::domain::software_event_type::SoftwareEventType;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum BxesValue {
    Null,
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
    String(Rc<Box<String>>),
    Bool(bool),
    Timestamp(i64),
    BrafLifecycle(BrafLifecycle),
    StandardLifecycle(StandardLifecycle),
    Artifact(BxesArtifact),
    Drivers(BxesDrivers),
    Guid(uuid::Uuid),
    SoftwareEventType(SoftwareEventType),
}

impl Hash for BxesValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            BxesValue::Null => state.write_u8(0),
            BxesValue::Int32(value) => state.write_i32(*value),
            BxesValue::Int64(value) => state.write_i64(*value),
            BxesValue::Uint32(value) => state.write_u32(*value),
            BxesValue::Uint64(value) => state.write_u64(*value),
            BxesValue::Float32(value) => state.write(value.to_le_bytes().as_slice()),
            BxesValue::Float64(value) => state.write(value.to_le_bytes().as_slice()),
            BxesValue::String(value) => state.write(value.as_bytes()),
            BxesValue::Bool(value) => state.write(if *value { &[1] } else { &[0] }),
            BxesValue::Timestamp(value) => state.write_i64(*value),
            BxesValue::BrafLifecycle(value) => value.hash(state),
            BxesValue::StandardLifecycle(value) => value.hash(state),
            BxesValue::Artifact(artifacts) => artifacts.hash(state),
            BxesValue::Drivers(drivers) => drivers.hash(state),
            BxesValue::Guid(guid) => guid.hash(state),
            BxesValue::SoftwareEventType(event_type) => event_type.hash(state),
        }
    }
}

impl PartialEq for BxesValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Int32(left), Self::Int32(right)) => left == right,
            (Self::Int64(left), Self::Int64(right)) => left == right,
            (Self::Uint32(left), Self::Uint32(right)) => left == right,
            (Self::Uint64(left), Self::Uint64(right)) => left == right,
            (Self::Float32(left), Self::Float32(right)) => left == right,
            (Self::Float64(left), Self::Float64(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Timestamp(left), Self::Timestamp(right)) => left == right,
            (Self::BrafLifecycle(left), Self::BrafLifecycle(right)) => left == right,
            (Self::StandardLifecycle(left), Self::StandardLifecycle(right)) => left == right,
            (Self::Artifact(left), Self::Artifact(right)) => left == right,
            (Self::Drivers(left), Self::Drivers(right)) => left == right,
            (Self::Guid(left), Self::Guid(right)) => left == right,
            (Self::SoftwareEventType(left), Self::SoftwareEventType(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for BxesValue {}
