use bxes::models::{BxesGlobalKind, BxesValue};
use chrono::{TimeZone, Utc};

use crate::event_log::core::event::{
    event::EventPayloadValue,
    lifecycle::{braf_lifecycle::XesBrafLifecycle, standard_lifecycle::XesStandardLifecycle, xes_lifecycle::Lifecycle},
};

use super::xes_to_bxes_converter::XesToBxesWriterError;

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
            let lifecycle = bxes::models::Lifecycle::Braf(lifecycle.clone());
            EventPayloadValue::Lifecycle(convert_bxes_to_xes_lifecycle(&lifecycle))
        }
        BxesValue::StandardLifecycle(lifecycle) => {
            let lifecycle = bxes::models::Lifecycle::Standard(lifecycle.clone());
            EventPayloadValue::Lifecycle(convert_bxes_to_xes_lifecycle(&lifecycle))
        }
        BxesValue::Artifact(_) => todo!(),
        BxesValue::Drivers(_) => todo!(),
        BxesValue::Guid(value) => EventPayloadValue::Guid(*value),
        BxesValue::SoftwareEventType(_) => todo!(),
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
            bxes::models::Lifecycle::Braf(braf) => BxesValue::BrafLifecycle(braf),
            bxes::models::Lifecycle::Standard(standard) => BxesValue::StandardLifecycle(standard),
        },
    }
}

pub(super) fn convert_bxes_to_xes_lifecycle(bxes_lifecycle: &bxes::models::Lifecycle) -> Lifecycle {
    match bxes_lifecycle {
        bxes::models::Lifecycle::Braf(braf_lifecycle) => Lifecycle::BrafLifecycle(match braf_lifecycle {
            bxes::models::BrafLifecycle::Unspecified => XesBrafLifecycle::Unspecified,
            bxes::models::BrafLifecycle::Closed => XesBrafLifecycle::Closed,
            bxes::models::BrafLifecycle::ClosedCancelled => XesBrafLifecycle::ClosedCancelled,
            bxes::models::BrafLifecycle::ClosedCancelledAborted => XesBrafLifecycle::ClosedCancelledAborted,
            bxes::models::BrafLifecycle::ClosedCancelledError => XesBrafLifecycle::ClosedCancelledError,
            bxes::models::BrafLifecycle::ClosedCancelledExited => XesBrafLifecycle::ClosedCancelledExited,
            bxes::models::BrafLifecycle::ClosedCancelledObsolete => XesBrafLifecycle::ClosedCancelledObsolete,
            bxes::models::BrafLifecycle::ClosedCancelledTerminated => XesBrafLifecycle::ClosedCancelledTerminated,
            bxes::models::BrafLifecycle::Completed => XesBrafLifecycle::Completed,
            bxes::models::BrafLifecycle::CompletedFailed => XesBrafLifecycle::CompletedFailed,
            bxes::models::BrafLifecycle::CompletedSuccess => XesBrafLifecycle::CompletedSuccess,
            bxes::models::BrafLifecycle::Open => XesBrafLifecycle::Open,
            bxes::models::BrafLifecycle::OpenNotRunning => XesBrafLifecycle::OpenNotRunning,
            bxes::models::BrafLifecycle::OpenNotRunningAssigned => XesBrafLifecycle::OpenNotRunningAssigned,
            bxes::models::BrafLifecycle::OpenNotRunningReserved => XesBrafLifecycle::OpenNotRunningReserved,
            bxes::models::BrafLifecycle::OpenNotRunningSuspendedAssigned => XesBrafLifecycle::OpenNotRunningSuspendedAssigned,
            bxes::models::BrafLifecycle::OpenNotRunningSuspendedReserved => XesBrafLifecycle::OpenNotRunningSuspendedReserved,
            bxes::models::BrafLifecycle::OpenRunning => XesBrafLifecycle::OpenRunning,
            bxes::models::BrafLifecycle::OpenRunningInProgress => XesBrafLifecycle::OpenRunningInProgress,
            bxes::models::BrafLifecycle::OpenRunningSuspended => XesBrafLifecycle::OpenRunningSuspended,
        }),
        bxes::models::Lifecycle::Standard(standard_lifecycle) => Lifecycle::XesStandardLifecycle(match standard_lifecycle {
            bxes::models::StandardLifecycle::Unspecified => XesStandardLifecycle::Unspecified,
            bxes::models::StandardLifecycle::Assign => XesStandardLifecycle::Assign,
            bxes::models::StandardLifecycle::AteAbort => XesStandardLifecycle::AteAbort,
            bxes::models::StandardLifecycle::Autoskip => XesStandardLifecycle::Autoskip,
            bxes::models::StandardLifecycle::Complete => XesStandardLifecycle::Complete,
            bxes::models::StandardLifecycle::ManualSkip => XesStandardLifecycle::ManualSkip,
            bxes::models::StandardLifecycle::PiAbort => XesStandardLifecycle::PiAbort,
            bxes::models::StandardLifecycle::ReAssign => XesStandardLifecycle::ReAssign,
            bxes::models::StandardLifecycle::Resume => XesStandardLifecycle::Resume,
            bxes::models::StandardLifecycle::Schedule => XesStandardLifecycle::Schedule,
            bxes::models::StandardLifecycle::Start => XesStandardLifecycle::Start,
            bxes::models::StandardLifecycle::Suspend => XesStandardLifecycle::Suspend,
            bxes::models::StandardLifecycle::Unknown => XesStandardLifecycle::Unknown,
            bxes::models::StandardLifecycle::Withdraw => XesStandardLifecycle::Withdraw,
        }),
    }
}

pub(super) fn convert_xes_to_bxes_lifecycle(ficus_lifecycle: &Lifecycle) -> bxes::models::Lifecycle {
    match ficus_lifecycle {
        Lifecycle::BrafLifecycle(braf_lifecycle) => bxes::models::Lifecycle::Braf(match braf_lifecycle {
            XesBrafLifecycle::Unspecified => bxes::models::BrafLifecycle::Unspecified,
            XesBrafLifecycle::Closed => bxes::models::BrafLifecycle::Closed,
            XesBrafLifecycle::ClosedCancelled => bxes::models::BrafLifecycle::ClosedCancelled,
            XesBrafLifecycle::ClosedCancelledAborted => bxes::models::BrafLifecycle::ClosedCancelledAborted,
            XesBrafLifecycle::ClosedCancelledError => bxes::models::BrafLifecycle::ClosedCancelledError,
            XesBrafLifecycle::ClosedCancelledExited => bxes::models::BrafLifecycle::ClosedCancelledExited,
            XesBrafLifecycle::ClosedCancelledObsolete => bxes::models::BrafLifecycle::ClosedCancelledObsolete,
            XesBrafLifecycle::ClosedCancelledTerminated => bxes::models::BrafLifecycle::ClosedCancelledTerminated,
            XesBrafLifecycle::Completed => bxes::models::BrafLifecycle::Completed,
            XesBrafLifecycle::CompletedFailed => bxes::models::BrafLifecycle::CompletedFailed,
            XesBrafLifecycle::CompletedSuccess => bxes::models::BrafLifecycle::CompletedSuccess,
            XesBrafLifecycle::Open => bxes::models::BrafLifecycle::Open,
            XesBrafLifecycle::OpenNotRunning => bxes::models::BrafLifecycle::OpenNotRunning,
            XesBrafLifecycle::OpenNotRunningAssigned => bxes::models::BrafLifecycle::OpenNotRunningAssigned,
            XesBrafLifecycle::OpenNotRunningReserved => bxes::models::BrafLifecycle::OpenNotRunningReserved,
            XesBrafLifecycle::OpenNotRunningSuspendedAssigned => bxes::models::BrafLifecycle::OpenNotRunningSuspendedAssigned,
            XesBrafLifecycle::OpenNotRunningSuspendedReserved => bxes::models::BrafLifecycle::OpenNotRunningSuspendedReserved,
            XesBrafLifecycle::OpenRunning => bxes::models::BrafLifecycle::OpenRunning,
            XesBrafLifecycle::OpenRunningInProgress => bxes::models::BrafLifecycle::OpenRunningInProgress,
            XesBrafLifecycle::OpenRunningSuspended => bxes::models::BrafLifecycle::OpenRunningSuspended,
        }),
        Lifecycle::XesStandardLifecycle(standard_lifecycle) => bxes::models::Lifecycle::Standard(match standard_lifecycle {
            XesStandardLifecycle::Unspecified => bxes::models::StandardLifecycle::Unspecified,
            XesStandardLifecycle::Assign => bxes::models::StandardLifecycle::Assign,
            XesStandardLifecycle::AteAbort => bxes::models::StandardLifecycle::AteAbort,
            XesStandardLifecycle::Autoskip => bxes::models::StandardLifecycle::Autoskip,
            XesStandardLifecycle::Complete => bxes::models::StandardLifecycle::Complete,
            XesStandardLifecycle::ManualSkip => bxes::models::StandardLifecycle::ManualSkip,
            XesStandardLifecycle::PiAbort => bxes::models::StandardLifecycle::PiAbort,
            XesStandardLifecycle::ReAssign => bxes::models::StandardLifecycle::ReAssign,
            XesStandardLifecycle::Resume => bxes::models::StandardLifecycle::Resume,
            XesStandardLifecycle::Schedule => bxes::models::StandardLifecycle::Schedule,
            XesStandardLifecycle::Start => bxes::models::StandardLifecycle::Start,
            XesStandardLifecycle::Suspend => bxes::models::StandardLifecycle::Suspend,
            XesStandardLifecycle::Unknown => bxes::models::StandardLifecycle::Unknown,
            XesStandardLifecycle::Withdraw => bxes::models::StandardLifecycle::Withdraw,
        }),
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
