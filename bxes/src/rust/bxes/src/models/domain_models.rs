use num_derive::FromPrimitive;
use num_traits::ToBytes;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use variant_count::VariantCount;

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

#[derive(FromPrimitive, ToPrimitive, VariantCount, Clone, Debug, Hash, PartialEq, Eq)]
pub enum SoftwareEventType {
    Unspecified = 0,
    Call = 1,
    Return = 2,
    Throws = 3,
    Handle = 4,
    Calling = 5,
    Returning = 6,
}

#[derive(Debug, Clone)]
pub struct BxesArtifact {
    pub items: Vec<BxesArtifactItem>,
}

impl Hash for BxesArtifact {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.items {
            item.hash(state);
        }
    }
}

impl PartialEq for BxesArtifact {
    fn eq(&self, other: &Self) -> bool {
        if self.items.len() != other.items.len() {
            return false;
        }

        for (self_item, other_item) in self.items.iter().zip(&other.items) {
            if !self_item.eq(&other_item) {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
pub struct BxesArtifactItem {
    pub model: Rc<Box<BxesValue>>,
    pub instance: Rc<Box<BxesValue>>,
    pub transition: Rc<Box<BxesValue>>,
}

impl Hash for BxesArtifactItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.instance.hash(state);
        self.transition.hash(state);
    }
}

impl PartialEq for BxesArtifactItem {
    fn eq(&self, other: &Self) -> bool {
        self.instance == other.instance && self.transition == other.transition
    }
}

#[derive(Debug, Clone)]
pub struct BxesDrivers {
    pub drivers: Vec<BxesDriver>,
}

impl Hash for BxesDrivers {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for driver in &self.drivers {
            driver.hash(state);
        }
    }
}

impl PartialEq for BxesDrivers {
    fn eq(&self, other: &Self) -> bool {
        if self.drivers.len() != other.drivers.len() {
            return false;
        }

        for (self_driver, other_driver) in self.drivers.iter().zip(&other.drivers) {
            if !self_driver.eq(other_driver) {
                return false;
            }
        }

        return true;
    }
}

#[derive(Clone, Debug)]
pub struct BxesDriver {
    pub amount: BxesValue,
    pub name: Rc<Box<BxesValue>>,
    pub driver_type: Rc<Box<BxesValue>>,
}

impl BxesDriver {
    pub fn amount(&self) -> f64 {
        if let BxesValue::Float64(amount) = self.amount {
            return amount;
        }

        panic!("Expected f64 BxesValue, got {:?}", self.amount)
    }
}

impl Hash for BxesDriver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.amount.hash(state);
        self.name.hash(state);
        self.driver_type.hash(state);
    }
}

impl PartialEq for BxesDriver {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
            && self.name == other.name
            && self.driver_type == other.driver_type
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Lifecycle {
    Braf(BrafLifecycle),
    Standard(StandardLifecycle),
}

#[derive(FromPrimitive, ToPrimitive, Clone, Debug, PartialEq, Eq, Hash, VariantCount)]
pub enum BrafLifecycle {
    Unspecified = 0,
    Closed = 1,
    ClosedCancelled = 2,
    ClosedCancelledAborted = 3,
    ClosedCancelledError = 4,
    ClosedCancelledExited = 5,
    ClosedCancelledObsolete = 6,
    ClosedCancelledTerminated = 7,
    Completed = 8,
    CompletedFailed = 9,
    CompletedSuccess = 10,
    Open = 11,
    OpenNotRunning = 12,
    OpenNotRunningAssigned = 13,
    OpenNotRunningReserved = 14,
    OpenNotRunningSuspendedAssigned = 15,
    OpenNotRunningSuspendedReserved = 16,
    OpenRunning = 17,
    OpenRunningInProgress = 18,
    OpenRunningSuspended = 19,
}

#[derive(FromPrimitive, ToPrimitive, Clone, Debug, PartialEq, Eq, Hash, VariantCount)]
pub enum StandardLifecycle {
    Unspecified = 0,
    Assign = 1,
    AteAbort = 2,
    Autoskip = 3,
    Complete = 4,
    ManualSkip = 5,
    PiAbort = 6,
    ReAssign = 7,
    Resume = 8,
    Schedule = 9,
    Start = 10,
    Suspend = 11,
    Unknown = 12,
    Withdraw = 13,
}

#[derive(Debug)]
pub struct BxesEventLog {
    pub version: u32,
    pub metadata: BxesEventLogMetadata,
    pub variants: Vec<BxesTraceVariant>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesEventLogMetadata {
    pub extensions: Option<Vec<BxesExtension>>,
    pub classifiers: Option<Vec<BxesClassifier>>,
    pub properties: Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
    pub globals: Option<Vec<BxesGlobal>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesExtension {
    pub name: Rc<Box<BxesValue>>,
    pub prefix: Rc<Box<BxesValue>>,
    pub uri: Rc<Box<BxesValue>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesClassifier {
    pub name: Rc<Box<BxesValue>>,
    pub keys: Vec<Rc<Box<BxesValue>>>,
}

#[derive(Debug, FromPrimitive, ToPrimitive, VariantCount, PartialEq, Eq)]
pub enum BxesGlobalKind {
    Event = 0,
    Trace = 1,
    Log = 2,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesGlobal {
    pub entity_kind: BxesGlobalKind,
    pub globals: Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>,
}

#[derive(Debug)]
pub struct BxesTraceVariant {
    pub traces_count: u32,
    pub metadata: Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>,
    pub events: Vec<BxesEvent>,
}

#[derive(Debug)]
pub struct BxesEvent {
    pub name: Rc<Box<BxesValue>>,
    pub timestamp: i64,
    pub attributes: Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
}

impl PartialEq for BxesEvent {
    fn eq(&self, other: &Self) -> bool {
        if !self.compare_events_by_properties(other) {
            return false;
        }

        compare_list_of_attributes(&self.attributes, &other.attributes)
    }
}

impl BxesEvent {
    fn compare_events_by_properties(&self, other: &Self) -> bool {
        self.name == other.name && self.timestamp == other.timestamp
    }
}

fn compare_list_of_attributes(
    first_attributes: &Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
    second_attributes: &Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
) -> bool {
    if first_attributes.is_none() && second_attributes.is_none() {
        return true;
    }

    if let Some(self_attributes) = first_attributes.as_ref() {
        if let Some(other_attributes) = second_attributes.as_ref() {
            if self_attributes.len() != other_attributes.len() {
                return false;
            }

            for (self_attribute, other_attribute) in self_attributes.iter().zip(other_attributes) {
                if !(attributes_equals(self_attribute, other_attribute)) {
                    return false;
                }
            }

            return true;
        }
    }

    return false;
}

fn attributes_equals(
    first_attribute: &(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>),
    second_attribute: &(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>),
) -> bool {
    first_attribute.0.eq(&second_attribute.0) && first_attribute.1.eq(&second_attribute.1)
}

impl PartialEq for BxesTraceVariant {
    fn eq(&self, other: &Self) -> bool {
        if self.traces_count != other.traces_count {
            return false;
        }

        if self.events.len() != other.events.len() {
            return false;
        }

        for (self_event, other_event) in self.events.iter().zip(&other.events) {
            if !self_event.eq(&other_event) {
                return false;
            }
        }

        return true;
    }
}

impl PartialEq for BxesEventLog {
    fn eq(&self, other: &Self) -> bool {
        if self.version != other.version {
            return false;
        }

        if !self.metadata.eq(&other.metadata) {
            return false;
        }

        for (self_variant, other_variant) in self.variants.iter().zip(&other.variants) {
            if !self_variant.eq(&other_variant) {
                return false;
            }
        }

        return true;
    }
}
