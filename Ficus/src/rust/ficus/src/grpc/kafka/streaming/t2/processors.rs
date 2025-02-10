use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_events;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::ficus_proto::GrpcPipeline;
use crate::grpc::kafka::models::{KafkaTraceProcessingError, XesFromBxesKafkaTraceCreatingError};
use crate::grpc::kafka::streaming::processors::KafkaTraceProcessingContext;
use crate::grpc::kafka::streaming::t2::dfg_data_structures::DfgDataStructures;
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::context::{PipelineContext, PipelineInfrastructure};
use crate::pipelines::keys::context_keys::EVENT_LOG_KEY;
use crate::pipelines::pipelines::PipelinePart;
use crate::utils::user_data::user_data::UserData;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct T2StreamingProcessor {
    dfg_data_structure: Arc<Mutex<DfgDataStructures>>,
    trace_preprocessing_pipeline: Option<GrpcPipeline>,
}

impl T2StreamingProcessor {
    pub fn new_sliding_window(element_lifetime: Duration, preprocessing_pipeline: Option<GrpcPipeline>) -> Self {
        Self {
            dfg_data_structure: Arc::new(Mutex::new(DfgDataStructures::new_sliding_window(element_lifetime))),
            trace_preprocessing_pipeline: preprocessing_pipeline,
        }
    }

    pub fn new_lossy_count(error: f64, preprocessing_pipeline: Option<GrpcPipeline>) -> Self {
        Self {
            dfg_data_structure: Arc::new(Mutex::new(DfgDataStructures::new_lossy_count(error))),
            trace_preprocessing_pipeline: preprocessing_pipeline,
        }
    }

    pub fn observe(&self, context: &mut KafkaTraceProcessingContext) -> Result<(), KafkaTraceProcessingError> {
        let mut dfg_data_structure = self.dfg_data_structure.lock().expect("Must acquire lock");

        let xes_trace = match read_bxes_events(context.trace.events()) {
            Ok(xes_trace) => xes_trace,
            Err(err) => {
                let err = XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err);
                return Err(KafkaTraceProcessingError::XesFromBxesTraceCreationError(err));
            }
        };

        let xes_trace = if let Some(preprocessing_pipeline) = self.trace_preprocessing_pipeline.as_ref() {
            let mut preprocessing_context = PipelineContext::empty();
            let mut log = XesEventLogImpl::empty();
            log.push(Rc::new(RefCell::new(xes_trace)));

            preprocessing_context.put_concrete(EVENT_LOG_KEY.key(), log);

            let initial_context_values = vec![];
            let preprocessing_pipeline = ServicePipelineExecutionContext::new(
                preprocessing_pipeline,
                &initial_context_values,
                context.execution_dto.pipeline_parts.clone(),
                context.execution_dto.events_handler.clone(),
            );

            match preprocessing_pipeline
                .to_pipeline()
                .execute(&mut preprocessing_context, &PipelineInfrastructure::new(None))
            {
                Ok(_) => preprocessing_context
                    .concrete(EVENT_LOG_KEY.key())
                    .expect("Must be present")
                    .traces()
                    .first()
                    .unwrap()
                    .borrow()
                    .clone(),

                Err(err) => return Err(KafkaTraceProcessingError::FailedToPreprocessTrace(err)),
            }
        } else {
            xes_trace
        };

        dfg_data_structure.invalidate();

        match dfg_data_structure.process_bxes_trace(context.trace.metadata(), &xes_trace, context.context) {
            Ok(()) => Ok(()),
            Err(err) => Err(KafkaTraceProcessingError::XesFromBxesTraceCreationError(err)),
        }
    }
}
