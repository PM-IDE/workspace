use std::cell::RefCell;
use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_events;
use crate::event_log::core::event::event::Event;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use crate::features::streaming::counters::core::{StreamingCounter, ValueUpdateKind};
use crate::features::streaming::counters::lossy_count::LossyCount;
use crate::features::streaming::counters::sliding_window::SlidingWindow;
use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::grpc::kafka::streaming::processors::{CaseMetadata, ProcessMetadata};
use crate::pipelines::context::PipelineContext;
use crate::pipelines::keys::context_keys::EVENT_LOG_INFO_KEY;
use crate::utils::user_data::user_data::UserData;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use log::warn;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
enum StreamingCounterFactory {
    LossyCount(f64),
    SlidingWindow(Duration)
}

impl StreamingCounterFactory {
    pub fn create<TKey: Hash + Eq + Clone + 'static, TValue: Clone + 'static>(&self) -> Rc<RefCell<dyn StreamingCounter<TKey, TValue>>> {
        match self {
            StreamingCounterFactory::LossyCount(error) => Rc::new(RefCell::new(LossyCount::new(*error))),
            StreamingCounterFactory::SlidingWindow(lifetime) => Rc::new(RefCell::new(SlidingWindow::new_time(*lifetime)))
        }
    }
}

#[derive(Clone)]
struct DfgDataStructureBase {
    factory: StreamingCounterFactory,
    processes_dfg: HashMap<String, Rc<RefCell<dyn StreamingCounter<(String, String), ()>>>>,
    traces_last_event_classes: Rc<RefCell<dyn StreamingCounter<Uuid, String>>>,
    event_classes_count: HashMap<String, Rc<RefCell<dyn StreamingCounter<String, ()>>>>,
}

unsafe impl Send for DfgDataStructureBase {}
unsafe impl Sync for DfgDataStructureBase {}

impl DfgDataStructureBase {
    pub fn observe_dfg_relation(&mut self, process_name: &str, relation: (String, String)) {
        self.processes_dfg
            .entry(process_name.to_owned())
            .or_insert(self.factory.create())
            .borrow_mut()
            .observe(relation, ValueUpdateKind::DoNothing);
    }

    pub fn observe_event_class(&mut self, process_name: &str, event_class: String) {
        self.event_classes_count
            .entry(process_name.to_owned())
            .or_insert(self.factory.create())
            .borrow_mut()
            .observe(event_class, ValueUpdateKind::DoNothing);
    }

    pub fn observe_last_trace_class(&self, case_id: Uuid, last_class: String) {
        self.traces_last_event_classes
            .borrow_mut()
            .observe(case_id, ValueUpdateKind::Replace(last_class))
    }

    pub fn last_seen_event_class(&self, case_id: &Uuid) -> Option<String> {
        match self.traces_last_event_classes.borrow().get(case_id) {
            None => None,
            Some(value) => Some(value.value().unwrap().to_owned()),
        }
    }

    pub fn to_event_log_info(&self, process_name: &str) -> Option<OfflineEventLogInfo> {
        let event_classes_count = match self.event_classes_count.get(process_name) {
            None => return None,
            Some(classes) => classes.borrow().to_count_map().into_iter().map(|(k, v)| (k, v as usize)).collect(),
        };

        let relations = match self.processes_dfg.get(process_name) {
            None => return None,
            Some(dfg) => dfg.borrow().to_count_map().into_iter().map(|(k, v)| (k, v as u64)).collect(),
        };

        Some(OfflineEventLogInfo::create_from_relations(&relations, &event_classes_count))
    }
    
    pub fn invalidate(&self) {
        for (_, sw) in self.processes_dfg.iter() {
            sw.borrow_mut().invalidate();
        }

        for (_, sw) in self.event_classes_count.iter() {
            sw.borrow_mut().invalidate();
        }

        self.traces_last_event_classes.borrow_mut().invalidate();
    }
}

#[derive(Clone)]
pub(in crate::grpc::kafka::streaming::t2) struct LossyCountDfgDataStructures {
    error: f64,
    dfg_data_structure: DfgDataStructureBase
}

impl LossyCountDfgDataStructures {
    pub fn new(error: f64) -> Self {
        Self {
            error,
            dfg_data_structure: DfgDataStructureBase {
                factory: StreamingCounterFactory::LossyCount(error),
                traces_last_event_classes: Rc::new(RefCell::new(LossyCount::new(error))),
                processes_dfg: HashMap::new(),
                event_classes_count: HashMap::new()
            }
        }
    }
}

#[derive(Clone)]
pub(in crate::grpc::kafka::streaming::t2) struct SlidingWindowDfgDataStructures {
    element_lifetime: Duration,
    dfg_data_structure : DfgDataStructureBase
}

impl SlidingWindowDfgDataStructures {
    pub fn new(element_lifetime: Duration) -> Self {
        Self {
            element_lifetime,
            dfg_data_structure: DfgDataStructureBase {
                factory: StreamingCounterFactory::SlidingWindow(element_lifetime),
                traces_last_event_classes: Rc::new(RefCell::new(SlidingWindow::new_time(element_lifetime))),
                processes_dfg: HashMap::new(),
                event_classes_count: HashMap::new()
            }
        }
    }
}

#[derive(Clone)]
pub(in crate::grpc::kafka::streaming::t2) enum DfgDataStructures {
    LossyCount(LossyCountDfgDataStructures),
    SlidingWindow(SlidingWindowDfgDataStructures),
}

impl DfgDataStructures {
    pub fn process_bxes_trace(
        &mut self,
        trace: BxesKafkaTrace,
        context: &mut PipelineContext,
    ) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
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
            None => {
                warn!("Failed to create offline event log info")
            }
            Some(log_info) => {
                context.put_concrete(EVENT_LOG_INFO_KEY.key(), log_info);
            }
        }

        Ok(())
    }

    pub fn invalidate(&mut self) {
        match self {
            DfgDataStructures::LossyCount(_) => {}
            DfgDataStructures::SlidingWindow(sw) => {

            }
        }
    }

    fn observe_dfg_relation(&mut self, process_name: &str, relation: (String, String)) {
        match self {
            DfgDataStructures::LossyCount(d) => d.dfg_data_structure.observe_dfg_relation(process_name, relation),
            DfgDataStructures::SlidingWindow(d) => d.dfg_data_structure.observe_dfg_relation(process_name, relation),
        }
    }

    fn observe_event_class(&mut self, process_name: &str, event_class: String) {
        match self {
            DfgDataStructures::LossyCount(lc) => lc.dfg_data_structure.observe_event_class(process_name, event_class),
            DfgDataStructures::SlidingWindow(sw) => sw.dfg_data_structure.observe_event_class(process_name, event_class),
        }
    }

    fn observe_last_trace_class(&mut self, case_id: Uuid, last_class: String) {
        match self {
            DfgDataStructures::LossyCount(d) => d.dfg_data_structure.observe_last_trace_class(case_id, last_class),
            DfgDataStructures::SlidingWindow(d) => d.dfg_data_structure.observe_last_trace_class(case_id, last_class),
        }
    }

    fn last_seen_event_class(&self, case_id: &Uuid) -> Option<String> {
        match self {
            DfgDataStructures::LossyCount(d) => d.dfg_data_structure.last_seen_event_class(case_id),
            DfgDataStructures::SlidingWindow(d) => d.dfg_data_structure.last_seen_event_class(case_id),
        }
    }

    fn to_event_log_info(&self, process_name: &str) -> Option<OfflineEventLogInfo> {
        match self {
            DfgDataStructures::LossyCount(lc) => lc.dfg_data_structure.to_event_log_info(process_name),
            DfgDataStructures::SlidingWindow(sw) => sw.dfg_data_structure.to_event_log_info(process_name),
        }
    }
}
