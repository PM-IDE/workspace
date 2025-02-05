use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_events;
use crate::event_log::core::event::event::Event;
use crate::event_log::core::trace::trace::Trace;
use crate::features::streaming::counters::core::{StreamingCounter, ValueUpdateKind};
use crate::features::streaming::counters::lossy_count::LossyCount;
use crate::features::streaming::counters::sliding_window::SlidingWindow;
use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::grpc::kafka::streaming::processors::{CaseMetadata, ProcessMetadata};
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
use crate::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::keys::context_keys::{EVENT_LOG_INFO, EVENT_LOG_INFO_KEY};
use crate::utils::user_data::user_data::UserData;

#[derive(Clone)]
pub(in crate::grpc::kafka::streaming::t2) struct LossyCountDfgDataStructures {
    error: f64,
    processes_dfg: HashMap<String, LossyCount<(String, String), ()>>,
    traces_last_event_classes: LossyCount<Uuid, String>,
    event_classes_count: HashMap<String, LossyCount<String, ()>>
}

impl LossyCountDfgDataStructures {
    pub fn new(error: f64) -> Self {
        Self {
            error,
            processes_dfg: HashMap::new(),
            traces_last_event_classes: LossyCount::new(error),
            event_classes_count: HashMap::new()
        }
    }

    pub fn observe_dfg_relation(&mut self, process_name: &str, relation: (String, String)) {
        self.processes_dfg
            .entry(process_name.to_owned())
            .or_insert(LossyCount::new(self.error))
            .observe(relation, ValueUpdateKind::DoNothing);
    }

    pub fn observe_event_class(&mut self, process_name: &str, event_class: String) {
        let lc = self.event_classes_count.entry(process_name.to_owned()).or_insert(LossyCount::new(self.error));
        lc.observe(event_class, ValueUpdateKind::DoNothing);
    }

    pub fn observe_last_trace_class(&mut self, case_id: Uuid, last_class: String) {
        self.traces_last_event_classes
            .observe(case_id, ValueUpdateKind::Replace(last_class))
    }

    pub fn last_seen_event_class(&self, case_id: &Uuid) -> Option<String> {
        match self.traces_last_event_classes.get(case_id) {
            None => None,
            Some(value) => Some(value.value().unwrap().to_owned()),
        }
    }

    pub fn to_event_log_info(&self, process_name: &str) -> Option<OfflineEventLogInfo> {
        let event_classes_count = match self.event_classes_count.get(process_name) {
            None => return None,
            Some(classes) => classes.to_count_map().into_iter().map(|(k, v)| (k, v as usize)).collect()
        };

        let relations = match self.processes_dfg.get(process_name) {
            None => return None,
            Some(dfg) => dfg.to_count_map().into_iter().map(|(k, v)| (k, v as u64)).collect()
        };

        Some(OfflineEventLogInfo::create_from_relations(&relations, &event_classes_count))
    }
}

#[derive(Clone)]
pub(in crate::grpc::kafka::streaming::t2) struct SlidingWindowDfgDataStructures {
    element_lifetime: Duration,
    processes_dfg: HashMap<String, SlidingWindow<(String, String), u64>>,
    traces_last_event_classes: SlidingWindow<Uuid, String>,
    event_classes_count: HashMap<String, SlidingWindow<String, u64>>
}

impl SlidingWindowDfgDataStructures {
    pub fn new(element_lifetime: Duration) -> Self {
        Self {
            element_lifetime,
            processes_dfg: HashMap::new(),
            traces_last_event_classes: SlidingWindow::new_time(element_lifetime),
            event_classes_count: HashMap::new()
        }
    }

    pub fn observe_dfg_relation(&mut self, process_name: &str, relation: (String, String)) {
        self.processes_dfg
            .entry(process_name.to_owned())
            .or_insert(SlidingWindow::new_time(self.element_lifetime))
            .increment_current_stamp(relation);
    }

    pub fn observe_event_class(&mut self, process_name: &str, event_class: String) {
        let sw = self.event_classes_count.entry(process_name.to_owned()).or_insert(SlidingWindow::new_time(self.element_lifetime));
        sw.increment_current_stamp(event_class);
    }

    pub fn observe_last_trace_class(&mut self, case_id: Uuid, last_class: String) {
        self.traces_last_event_classes.add_current_stamp(case_id, ValueUpdateKind::Replace(last_class));
    }

    pub fn last_seen_event_class(&self, case_id: &Uuid) -> Option<String> {
        match self.traces_last_event_classes.get(case_id) {
            None => None,
            Some(value) => Some(value.to_owned()),
        }
    }

    pub fn to_event_log_info(&self, process_name: &str) -> Option<OfflineEventLogInfo> {
        let event_classes_count = match self.event_classes_count.get(process_name) {
            None => return None,
            Some(sw) => sw.to_count_map().into_iter().filter(|(k, v)| v.is_some()).map(|(k, v)| (k, v.unwrap())).map(|(k, v)| (k, v as usize)).collect()
        };

        let relations = match self.processes_dfg.get(process_name) {
            None => return None,
            Some(sw) => sw.to_count_map().into_iter().filter(|(k, v)| v.is_some()).map(|(k, v)| (k, v.unwrap())).collect()
        };

        Some(OfflineEventLogInfo::create_from_relations(&relations, &event_classes_count))
    }
}

#[derive(Clone)]
pub(in crate::grpc::kafka::streaming::t2) enum DfgDataStructures {
    LossyCount(LossyCountDfgDataStructures),
    SlidingWindow(SlidingWindowDfgDataStructures),
}

impl DfgDataStructures {
    pub fn process_bxes_trace(&mut self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        if trace.events().is_empty() {
            return Ok(());
        }

        let process_metadata = ProcessMetadata::create_from(trace.metadata())?;
        let case_metadata = CaseMetadata::create_from(trace.metadata())?;

        let xes_trace = match read_bxes_events(trace.events()) {
            Ok(xes_trace) => xes_trace,
            Err(err) => return Err(XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err)),
        };

        for i in 0..(xes_trace.events().len() - 1) {
            let first_name = xes_trace.events().get(i).unwrap().borrow().name().to_owned();
            let second_name = xes_trace.events().get(i + 1).unwrap().borrow().name().to_owned();

            self.observe_dfg_relation(process_metadata.process_name.as_str(), (first_name.clone(), second_name));
            self.observe_event_class(process_metadata.process_name.as_str(), first_name);
        }

        if let Some(last_seen_class) = self.last_seen_event_class(&case_metadata.case_id) {
            let first_class = xes_trace.events().first().unwrap().borrow().name().to_owned();
            self.observe_dfg_relation(process_metadata.process_name.as_str(), (last_seen_class, first_class));
        }

        let new_trace_last_class = xes_trace.events().last().unwrap().borrow().name().to_owned();
        self.observe_event_class(process_metadata.process_name.as_str(), new_trace_last_class.clone());
        self.observe_last_trace_class(case_metadata.case_id.to_owned(), new_trace_last_class);

        match self.to_event_log_info(process_metadata.process_name.as_str()) {
            None => {}
            Some(log_info) => {
                context.put_concrete(EVENT_LOG_INFO_KEY.key(), log_info);
            }
        }

        Ok(())
    }

    fn observe_dfg_relation(&mut self, process_name: &str, relation: (String, String)) {
        match self {
            DfgDataStructures::LossyCount(d) => d.observe_dfg_relation(process_name, relation),
            DfgDataStructures::SlidingWindow(d) => d.observe_dfg_relation(process_name, relation),
        }
    }

    fn observe_event_class(&mut self, process_name: &str, event_class: String) {
        match self {
            DfgDataStructures::LossyCount(lc) => lc.observe_event_class(process_name, event_class),
            DfgDataStructures::SlidingWindow(sw) => sw.observe_event_class(process_name, event_class),
        }
    }

    fn observe_last_trace_class(&mut self, case_id: Uuid, last_class: String) {
        match self {
            DfgDataStructures::LossyCount(d) => d.observe_last_trace_class(case_id, last_class),
            DfgDataStructures::SlidingWindow(d) => d.observe_last_trace_class(case_id, last_class),
        }
    }

    fn last_seen_event_class(&self, case_id: &Uuid) -> Option<String> {
        match self {
            DfgDataStructures::LossyCount(d) => d.last_seen_event_class(case_id),
            DfgDataStructures::SlidingWindow(d) => d.last_seen_event_class(case_id),
        }
    }

    fn to_event_log_info(&self, process_name: &str) -> Option<OfflineEventLogInfo> {
        match self {
            DfgDataStructures::LossyCount(lc) => lc.to_event_log_info(process_name),
            DfgDataStructures::SlidingWindow(sw) => sw.to_event_log_info(process_name)
        }
    }
}
