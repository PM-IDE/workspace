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
use log::{debug, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
enum StreamingCounterFactory {
    LossyCount(f64),
    SlidingWindow(Duration),
}

impl StreamingCounterFactory {
    pub fn create<TKey: Hash + Eq + Clone + 'static, TValue: Clone + 'static>(&self) -> Rc<RefCell<dyn StreamingCounter<TKey, TValue>>> {
        match self {
            StreamingCounterFactory::LossyCount(error) => Rc::new(RefCell::new(LossyCount::new(*error))),
            StreamingCounterFactory::SlidingWindow(lifetime) => Rc::new(RefCell::new(SlidingWindow::new_time(*lifetime))),
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
        debug!("Observing relation, process: {}, relation: ({}, {})", process_name, &relation.0, &relation.1);

        self.processes_dfg
            .entry(process_name.to_owned())
            .or_insert(self.factory.create())
            .borrow_mut()
            .observe(relation, ValueUpdateKind::DoNothing);
    }

    pub fn observe_event_class(&mut self, process_name: &str, event_class: String) {
        debug!("Observing event class, process: {}, event class: {}", process_name, event_class);

        self.event_classes_count
            .entry(process_name.to_owned())
            .or_insert(self.factory.create())
            .borrow_mut()
            .observe(event_class, ValueUpdateKind::DoNothing);
    }

    pub fn observe_last_trace_class(&self, case_id: Uuid, last_class: String) {
        debug!("Observing last trace class, case id: {}, last class: {}", &case_id, last_class.as_str());

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

        debug!("Creating offline dfg info from relations: {:?}, event classes count: {:?}", &relations, &event_classes_count);

        Some(OfflineEventLogInfo::create_from_relations(&relations, &event_classes_count))
    }

    pub fn invalidate(&self) {
        debug!("Started invalidating DfgDataStructureBase");

        for (_, sw) in self.processes_dfg.iter() {
            sw.borrow_mut().invalidate();
        }

        for (_, sw) in self.event_classes_count.iter() {
            sw.borrow_mut().invalidate();
        }

        self.traces_last_event_classes.borrow_mut().invalidate();

        debug!("Finished invalidating DfgDataStructureBase");
    }
}

#[derive(Clone)]
pub(in crate::grpc::kafka::streaming::t2) struct DfgDataStructures {
    base: DfgDataStructureBase,
}

impl DfgDataStructures {
    pub fn new_lossy_count(error: f64) -> Self {
        Self {
            base: DfgDataStructureBase {
                factory: StreamingCounterFactory::LossyCount(error),
                traces_last_event_classes: Rc::new(RefCell::new(LossyCount::new(error))),
                processes_dfg: HashMap::new(),
                event_classes_count: HashMap::new(),
            },
        }
    }

    pub fn new_sliding_window(lifetime: Duration) -> Self {
        Self {
            base: DfgDataStructureBase {
                factory: StreamingCounterFactory::SlidingWindow(lifetime),
                traces_last_event_classes: Rc::new(RefCell::new(SlidingWindow::new_time(lifetime))),
                processes_dfg: HashMap::new(),
                event_classes_count: HashMap::new(),
            },
        }
    }
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

        let xes_trace = match read_bxes_events(trace.events()) {
            Ok(xes_trace) => xes_trace,
            Err(err) => return Err(XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err)),
        };

        let process_metadata = ProcessMetadata::create_from(trace.metadata())?;
        let case_metadata = CaseMetadata::create_from(trace.metadata())?;
        let process_name = process_metadata.process_name.as_str();

        for i in 0..(xes_trace.events().len() - 1) {
            let first_name = xes_trace.events().get(i).unwrap().borrow().name().to_owned();
            let second_name = xes_trace.events().get(i + 1).unwrap().borrow().name().to_owned();

            self.base.observe_dfg_relation(process_name, (first_name.clone(), second_name));
            self.base.observe_event_class(process_name, first_name);
        }

        if let Some(last_seen_class) = self.base.last_seen_event_class(&case_metadata.case_id) {
            let first_class = xes_trace.events().first().unwrap().borrow().name().to_owned();
            self.base.observe_dfg_relation(process_name, (last_seen_class, first_class));
        }

        let new_trace_last_class = xes_trace.events().last().unwrap().borrow().name().to_owned();

        self.base.observe_event_class(process_name, new_trace_last_class.clone());
        self.base.observe_last_trace_class(case_metadata.case_id.to_owned(), new_trace_last_class);

        match self.base.to_event_log_info(process_name) {
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
        self.base.invalidate();
    }
}
