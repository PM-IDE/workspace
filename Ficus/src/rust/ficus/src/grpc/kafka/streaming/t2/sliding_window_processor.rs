use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use uuid::Uuid;
use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_events;
use crate::event_log::core::event::event::Event;
use crate::event_log::core::trace::trace::Trace;
use crate::features::streaming::counters::sliding_window::SlidingWindow;
use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::grpc::kafka::streaming::processors::{CaseMetadata, ProcessMetadata};
use crate::pipelines::context::PipelineContext;

struct DfgDataStructures {
    processes_dfg: HashMap<String, SlidingWindow<(String, String), u64>>,
    traces_last_event_classes: SlidingWindow<Uuid, String>,
}

#[derive(Clone)]
pub struct T2SlidingWindowProcessor {
    element_lifespan: Duration,
    maps: Arc<Mutex<DfgDataStructures>>
}

impl T2SlidingWindowProcessor {
    pub fn new(element_lifespan: Duration) -> Self {
        Self {
            element_lifespan,
            maps: Arc::new(Mutex::new(DfgDataStructures {
                processes_dfg: HashMap::new(),
                traces_last_event_classes: SlidingWindow::new_time(element_lifespan),
            }))
        }
    }

    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        if trace.events().is_empty() {
            return Ok(());
        }

        let process_metadata = ProcessMetadata::create_from(trace.metadata())?;
        let case_metadata = CaseMetadata::create_from(trace.metadata())?;

        let xes_trace = match read_bxes_events(trace.events()) {
            Ok(xes_trace) => xes_trace,
            Err(err) => return Err(XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err)),
        };

        let mut maps = self.maps.lock().expect("Must acquire lock");

        let last_seen_class = match maps.traces_last_event_classes.get(&case_metadata.case_id) {
            None => None,
            Some(value) => Some(value.to_owned())
        };

        let dfgs_map = &mut maps.processes_dfg;

        if !dfgs_map.contains_key(&process_metadata.process_name) {
            dfgs_map.insert(process_metadata.process_name.to_owned(), SlidingWindow::new_time(self.element_lifespan));
        }

        let dfg = dfgs_map.get_mut(&process_metadata.process_name).expect("Must be present");

        for i in 0..(xes_trace.events().len() - 1) {
            let first_name = xes_trace.events().get(i).unwrap().borrow().name().to_owned();
            let second_name = xes_trace.events().get(i + 1).unwrap().borrow().name().to_owned();

            dfg.increment_current_stamp((first_name, second_name));
        }

        if let Some(last_seen_class) = last_seen_class {
            let first_class = xes_trace.events().first().unwrap().borrow().name().to_owned();
            dfg.increment_current_stamp((last_seen_class, first_class));
        }

        let new_trace_last_class = xes_trace.events().last().unwrap().borrow().name().to_owned();
        maps.traces_last_event_classes.add_current_stamp(case_metadata.case_id.to_owned(), new_trace_last_class);

        Ok(())
    }
}
