use crate::models::domain::bxes_log_metadata::BxesEventLogMetadata;
use crate::models::domain::bxes_value::BxesValue;
use crate::models::domain::utils::compare_list_of_attributes;
use std::rc::Rc;

#[derive(Debug)]
pub struct BxesEventLog {
    pub version: u32,
    pub metadata: BxesEventLogMetadata,
    pub variants: Vec<BxesTraceVariant>,
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

#[derive(Debug)]
pub struct BxesTraceVariant {
    pub traces_count: u32,
    pub metadata: Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>,
    pub events: Vec<BxesEvent>,
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

#[derive(Debug, Clone)]
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
