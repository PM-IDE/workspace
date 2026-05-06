use crate::{
  ficus_proto::GrpcPipeline,
  grpc::{
    kafka::{
      models::{KafkaTraceProcessingError, XesFromBxesKafkaTraceCreatingError},
      streaming::{processors::KafkaTraceProcessingContext, t2::dfg_data_structures::DfgDataStructures},
    },
    pipeline_executor::ServicePipelineExecutionContext,
  },
};
use ficus::{
  event_log::{bxes::bxes_to_xes_converter::read_bxes_events, core::event_log::EventLog, xes::xes_event_log::XesEventLogImpl},
  pipelines::{
    context::{PipelineContext, PipelineInfrastructure},
    keys::context_keys::EVENT_LOG_KEY,
    pipelines::PipelinePart,
  },
  utils::user_data::user_data::UserData,
};
use std::{
  cell::RefCell,
  rc::Rc,
  sync::{Arc, Mutex},
  time::Duration,
};

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
    let xes_trace = match read_bxes_events(context.trace.events()) {
      Ok(xes_trace) => xes_trace,
      Err(err) => {
        let err = XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err);
        return Err(KafkaTraceProcessingError::XesFromBxesTraceCreationError(err));
      }
    };

    let xes_trace = if let Some(preprocessing_pipeline) = self.trace_preprocessing_pipeline.as_ref() {
      let mut preprocessing_context = PipelineContext::default();
      let mut log = XesEventLogImpl::default();
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

    let mut dfg_data_structure = self.dfg_data_structure.lock().expect("Must acquire lock");
    dfg_data_structure.invalidate();

    match dfg_data_structure.process_bxes_trace(context.trace.metadata(), &xes_trace) {
      Ok(()) => Ok(()),
      Err(err) => Err(KafkaTraceProcessingError::XesFromBxesTraceCreationError(err)),
    }
  }

  pub fn fill_pipeline_context(&self, context: &mut PipelineContext, case_name: &str) {
    self.dfg_data_structure.lock().unwrap().fill_pipeline_context(context, case_name);
  }
}
