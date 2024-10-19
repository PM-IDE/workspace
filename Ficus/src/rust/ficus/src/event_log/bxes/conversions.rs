use std::rc::Rc;

use bxes::models::domain::bxes_artifact::{BxesArtifact, BxesArtifactItem};
use bxes::models::domain::bxes_driver::{BxesDriver, BxesDrivers};
use bxes::models::domain::bxes_log_metadata::BxesGlobalKind;
use bxes::models::domain::bxes_value::BxesValue;
use bxes::{models::domain::software_event_type::SoftwareEventType, read::read_utils::owned_string_or_err};
use chrono::{TimeZone, Utc};

use crate::event_log::core::event::event::{EventPayloadArtifact, EventPayloadArtifactItem, EventPayloadSoftwareEventType};
use crate::event_log::core::event::{
    event::{EventPayloadDriver, EventPayloadDrivers, EventPayloadValue},
    lifecycle::{braf_lifecycle::XesBrafLifecycle, standard_lifecycle::XesStandardLifecycle, xes_lifecycle::Lifecycle},
};

use super::xes_to_bxes_converter::XesToBxesWriterError;

type BxesBrafLifecycle = bxes::models::domain::bxes_lifecycle::BrafLifecycle;
type BxesStandardLifecycle = bxes::models::domain::bxes_lifecycle::StandardLifecycle;

pub(super) fn bxes_value_to_payload_value(value: &BxesValue) -> EventPayloadValue {
    match value {
        BxesValue::Null => EventPayloadValue::Null,
        BxesValue::Int32(value) => EventPayloadValue::Int32(*value),
        BxesValue::Int64(value) => EventPayloadValue::Int64(*value),
        BxesValue::Uint32(value) => EventPayloadValue::Uint32(*value),
        BxesValue::Uint64(value) => EventPayloadValue::Uint64(*value),
        BxesValue::Float32(value) => EventPayloadValue::Float32(*value),
        BxesValue::Float64(value) => EventPayloadValue::Float64(*value),
        BxesValue::String(string) => EventPayloadValue::String(string.clone()),
        BxesValue::Bool(bool) => EventPayloadValue::Boolean(*bool),
        BxesValue::Timestamp(stamp) => EventPayloadValue::Date(Utc.timestamp_nanos(*stamp)),
        BxesValue::BrafLifecycle(lifecycle) => {
            let lifecycle = bxes::models::domain::bxes_lifecycle::Lifecycle::Braf(lifecycle.clone());
            EventPayloadValue::Lifecycle(convert_bxes_to_xes_lifecycle(&lifecycle))
        }
        BxesValue::StandardLifecycle(lifecycle) => {
            let lifecycle = bxes::models::domain::bxes_lifecycle::Lifecycle::Standard(lifecycle.clone());
            EventPayloadValue::Lifecycle(convert_bxes_to_xes_lifecycle(&lifecycle))
        }
        BxesValue::Artifact(artifact) => EventPayloadValue::Artifact(EventPayloadArtifact {
            items: artifact
                .items
                .iter()
                .map(|item| EventPayloadArtifactItem {
                    model: owned_string_or_err(&item.model).ok().unwrap(),
                    instance: owned_string_or_err(&item.instance).ok().unwrap(),
                    transition: owned_string_or_err(&item.transition).ok().unwrap(),
                })
                .collect(),
        }),
        BxesValue::Drivers(drivers) => EventPayloadValue::Drivers(EventPayloadDrivers {
            drivers: drivers
                .drivers
                .iter()
                .map(|d| EventPayloadDriver {
                    amount: if let BxesValue::Float64(value) = d.amount {
                        value
                    } else {
                        panic!("Driver amount should be float64")
                    },
                    driver_type: owned_string_or_err(&d.driver_type).ok().unwrap(),
                    name: owned_string_or_err(&d.name).ok().unwrap(),
                })
                .collect(),
        }),
        BxesValue::Guid(value) => EventPayloadValue::Guid(*value),
        BxesValue::SoftwareEventType(software_event_type) => EventPayloadValue::SoftwareEvent(match software_event_type {
            SoftwareEventType::Unspecified => EventPayloadSoftwareEventType::Unspecified,
            SoftwareEventType::Call => EventPayloadSoftwareEventType::Call,
            SoftwareEventType::Return => EventPayloadSoftwareEventType::Return,
            SoftwareEventType::Throws => EventPayloadSoftwareEventType::Throws,
            SoftwareEventType::Handle => EventPayloadSoftwareEventType::Handle,
            SoftwareEventType::Calling => EventPayloadSoftwareEventType::Calling,
            SoftwareEventType::Returning => EventPayloadSoftwareEventType::Returning,
        }),
    }
}

pub(super) fn payload_value_to_bxes_value(value: &EventPayloadValue) -> BxesValue {
    match value {
        EventPayloadValue::Null => BxesValue::Null,
        EventPayloadValue::Date(value) => BxesValue::Timestamp(value.timestamp_nanos()),
        EventPayloadValue::String(value) => BxesValue::String(value.clone()),
        EventPayloadValue::Boolean(value) => BxesValue::Bool(*value),
        EventPayloadValue::Int32(value) => BxesValue::Int32(*value),
        EventPayloadValue::Int64(value) => BxesValue::Int64(*value),
        EventPayloadValue::Float32(value) => BxesValue::Float32(*value),
        EventPayloadValue::Float64(value) => BxesValue::Float64(*value),
        EventPayloadValue::Uint32(value) => BxesValue::Uint32(*value),
        EventPayloadValue::Uint64(value) => BxesValue::Uint64(*value),
        EventPayloadValue::Guid(value) => BxesValue::Guid(value.clone()),
        EventPayloadValue::Timestamp(value) => BxesValue::Timestamp(*value),
        EventPayloadValue::Lifecycle(lifecycle) => match convert_xes_to_bxes_lifecycle(lifecycle) {
            bxes::models::domain::bxes_lifecycle::Lifecycle::Braf(braf) => BxesValue::BrafLifecycle(braf),
            bxes::models::domain::bxes_lifecycle::Lifecycle::Standard(standard) => BxesValue::StandardLifecycle(standard),
        },
        EventPayloadValue::Artifact(artifact) => BxesValue::Artifact(BxesArtifact {
            items: artifact
                .items
                .iter()
                .map(|a| BxesArtifactItem {
                    instance: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(a.instance.clone()))))),
                    model: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(a.model.clone()))))),
                    transition: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(a.transition.clone()))))),
                })
                .collect(),
        }),
        EventPayloadValue::Drivers(drivers) => BxesValue::Drivers(BxesDrivers {
            drivers: drivers
                .drivers
                .iter()
                .map(|d| BxesDriver {
                    amount: BxesValue::Float64(d.amount),
                    driver_type: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(d.driver_type.clone()))))),
                    name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(d.name.clone()))))),
                })
                .collect(),
        }),
        EventPayloadValue::SoftwareEvent(software_event) => BxesValue::SoftwareEventType(match software_event {
            EventPayloadSoftwareEventType::Unspecified => SoftwareEventType::Unspecified,
            EventPayloadSoftwareEventType::Call => SoftwareEventType::Call,
            EventPayloadSoftwareEventType::Return => SoftwareEventType::Return,
            EventPayloadSoftwareEventType::Throws => SoftwareEventType::Throws,
            EventPayloadSoftwareEventType::Handle => SoftwareEventType::Handle,
            EventPayloadSoftwareEventType::Calling => SoftwareEventType::Calling,
            EventPayloadSoftwareEventType::Returning => SoftwareEventType::Returning,
        }),
    }
}

pub(super) fn convert_bxes_to_xes_lifecycle(bxes_lifecycle: &bxes::models::domain::bxes_lifecycle::Lifecycle) -> Lifecycle {
    match bxes_lifecycle {
        bxes::models::domain::bxes_lifecycle::Lifecycle::Braf(braf_lifecycle) => Lifecycle::BrafLifecycle(match braf_lifecycle {
            BxesBrafLifecycle::Unspecified => XesBrafLifecycle::Unspecified,
            BxesBrafLifecycle::Closed => XesBrafLifecycle::Closed,
            BxesBrafLifecycle::ClosedCancelled => XesBrafLifecycle::ClosedCancelled,
            BxesBrafLifecycle::ClosedCancelledAborted => XesBrafLifecycle::ClosedCancelledAborted,
            BxesBrafLifecycle::ClosedCancelledError => XesBrafLifecycle::ClosedCancelledError,
            BxesBrafLifecycle::ClosedCancelledExited => XesBrafLifecycle::ClosedCancelledExited,
            BxesBrafLifecycle::ClosedCancelledObsolete => XesBrafLifecycle::ClosedCancelledObsolete,
            BxesBrafLifecycle::ClosedCancelledTerminated => XesBrafLifecycle::ClosedCancelledTerminated,
            BxesBrafLifecycle::Completed => XesBrafLifecycle::Completed,
            BxesBrafLifecycle::CompletedFailed => XesBrafLifecycle::CompletedFailed,
            BxesBrafLifecycle::CompletedSuccess => XesBrafLifecycle::CompletedSuccess,
            BxesBrafLifecycle::Open => XesBrafLifecycle::Open,
            BxesBrafLifecycle::OpenNotRunning => XesBrafLifecycle::OpenNotRunning,
            BxesBrafLifecycle::OpenNotRunningAssigned => XesBrafLifecycle::OpenNotRunningAssigned,
            BxesBrafLifecycle::OpenNotRunningReserved => XesBrafLifecycle::OpenNotRunningReserved,
            BxesBrafLifecycle::OpenNotRunningSuspendedAssigned => XesBrafLifecycle::OpenNotRunningSuspendedAssigned,
            BxesBrafLifecycle::OpenNotRunningSuspendedReserved => XesBrafLifecycle::OpenNotRunningSuspendedReserved,
            BxesBrafLifecycle::OpenRunning => XesBrafLifecycle::OpenRunning,
            BxesBrafLifecycle::OpenRunningInProgress => XesBrafLifecycle::OpenRunningInProgress,
            BxesBrafLifecycle::OpenRunningSuspended => XesBrafLifecycle::OpenRunningSuspended,
        }),
        bxes::models::domain::bxes_lifecycle::Lifecycle::Standard(standard_lifecycle) => {
            Lifecycle::XesStandardLifecycle(match standard_lifecycle {
                BxesStandardLifecycle::Unspecified => XesStandardLifecycle::Unspecified,
                BxesStandardLifecycle::Assign => XesStandardLifecycle::Assign,
                BxesStandardLifecycle::AteAbort => XesStandardLifecycle::AteAbort,
                BxesStandardLifecycle::Autoskip => XesStandardLifecycle::Autoskip,
                BxesStandardLifecycle::Complete => XesStandardLifecycle::Complete,
                BxesStandardLifecycle::ManualSkip => XesStandardLifecycle::ManualSkip,
                BxesStandardLifecycle::PiAbort => XesStandardLifecycle::PiAbort,
                BxesStandardLifecycle::ReAssign => XesStandardLifecycle::ReAssign,
                BxesStandardLifecycle::Resume => XesStandardLifecycle::Resume,
                BxesStandardLifecycle::Schedule => XesStandardLifecycle::Schedule,
                BxesStandardLifecycle::Start => XesStandardLifecycle::Start,
                BxesStandardLifecycle::Suspend => XesStandardLifecycle::Suspend,
                BxesStandardLifecycle::Unknown => XesStandardLifecycle::Unknown,
                BxesStandardLifecycle::Withdraw => XesStandardLifecycle::Withdraw,
            })
        }
    }
}

pub(super) fn convert_xes_to_bxes_lifecycle(ficus_lifecycle: &Lifecycle) -> bxes::models::domain::bxes_lifecycle::Lifecycle {
    match ficus_lifecycle {
        Lifecycle::BrafLifecycle(braf_lifecycle) => bxes::models::domain::bxes_lifecycle::Lifecycle::Braf(match braf_lifecycle {
            XesBrafLifecycle::Unspecified => BxesBrafLifecycle::Unspecified,
            XesBrafLifecycle::Closed => BxesBrafLifecycle::Closed,
            XesBrafLifecycle::ClosedCancelled => BxesBrafLifecycle::ClosedCancelled,
            XesBrafLifecycle::ClosedCancelledAborted => BxesBrafLifecycle::ClosedCancelledAborted,
            XesBrafLifecycle::ClosedCancelledError => BxesBrafLifecycle::ClosedCancelledError,
            XesBrafLifecycle::ClosedCancelledExited => BxesBrafLifecycle::ClosedCancelledExited,
            XesBrafLifecycle::ClosedCancelledObsolete => BxesBrafLifecycle::ClosedCancelledObsolete,
            XesBrafLifecycle::ClosedCancelledTerminated => BxesBrafLifecycle::ClosedCancelledTerminated,
            XesBrafLifecycle::Completed => BxesBrafLifecycle::Completed,
            XesBrafLifecycle::CompletedFailed => BxesBrafLifecycle::CompletedFailed,
            XesBrafLifecycle::CompletedSuccess => BxesBrafLifecycle::CompletedSuccess,
            XesBrafLifecycle::Open => BxesBrafLifecycle::Open,
            XesBrafLifecycle::OpenNotRunning => BxesBrafLifecycle::OpenNotRunning,
            XesBrafLifecycle::OpenNotRunningAssigned => BxesBrafLifecycle::OpenNotRunningAssigned,
            XesBrafLifecycle::OpenNotRunningReserved => BxesBrafLifecycle::OpenNotRunningReserved,
            XesBrafLifecycle::OpenNotRunningSuspendedAssigned => BxesBrafLifecycle::OpenNotRunningSuspendedAssigned,
            XesBrafLifecycle::OpenNotRunningSuspendedReserved => BxesBrafLifecycle::OpenNotRunningSuspendedReserved,
            XesBrafLifecycle::OpenRunning => BxesBrafLifecycle::OpenRunning,
            XesBrafLifecycle::OpenRunningInProgress => BxesBrafLifecycle::OpenRunningInProgress,
            XesBrafLifecycle::OpenRunningSuspended => BxesBrafLifecycle::OpenRunningSuspended,
        }),
        Lifecycle::XesStandardLifecycle(standard_lifecycle) => {
            bxes::models::domain::bxes_lifecycle::Lifecycle::Standard(match standard_lifecycle {
                XesStandardLifecycle::Unspecified => BxesStandardLifecycle::Unspecified,
                XesStandardLifecycle::Assign => BxesStandardLifecycle::Assign,
                XesStandardLifecycle::AteAbort => BxesStandardLifecycle::AteAbort,
                XesStandardLifecycle::Autoskip => BxesStandardLifecycle::Autoskip,
                XesStandardLifecycle::Complete => BxesStandardLifecycle::Complete,
                XesStandardLifecycle::ManualSkip => BxesStandardLifecycle::ManualSkip,
                XesStandardLifecycle::PiAbort => BxesStandardLifecycle::PiAbort,
                XesStandardLifecycle::ReAssign => BxesStandardLifecycle::ReAssign,
                XesStandardLifecycle::Resume => BxesStandardLifecycle::Resume,
                XesStandardLifecycle::Schedule => BxesStandardLifecycle::Schedule,
                XesStandardLifecycle::Start => BxesStandardLifecycle::Start,
                XesStandardLifecycle::Suspend => BxesStandardLifecycle::Suspend,
                XesStandardLifecycle::Unknown => BxesStandardLifecycle::Unknown,
                XesStandardLifecycle::Withdraw => BxesStandardLifecycle::Withdraw,
            })
        }
    }
}

const EVENT_GLOBAL_ENTITY_TYPE: &str = "event";
const TRACE_GLOBAL_ENTITY_TYPE: &str = "trace";
const LOG_GLOBAL_ENTITY_TYPE: &str = "log";

pub(super) fn parse_entity_kind(string: &str) -> Result<BxesGlobalKind, XesToBxesWriterError> {
    match string {
        EVENT_GLOBAL_ENTITY_TYPE => Ok(BxesGlobalKind::Event),
        TRACE_GLOBAL_ENTITY_TYPE => Ok(BxesGlobalKind::Trace),
        LOG_GLOBAL_ENTITY_TYPE => Ok(BxesGlobalKind::Log),
        _ => Err(XesToBxesWriterError::ConversionError(format!(
            "Not supported global entity type: {}",
            string
        ))),
    }
}

pub(super) fn global_type_to_string(entity_type: &BxesGlobalKind) -> String {
    match entity_type {
        BxesGlobalKind::Event => EVENT_GLOBAL_ENTITY_TYPE.to_string(),
        BxesGlobalKind::Trace => TRACE_GLOBAL_ENTITY_TYPE.to_string(),
        BxesGlobalKind::Log => LOG_GLOBAL_ENTITY_TYPE.to_string(),
    }
}
