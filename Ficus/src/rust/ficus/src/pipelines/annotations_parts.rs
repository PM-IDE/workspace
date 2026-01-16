use std::collections::HashMap;

use crate::{
  event_log::xes::xes_event_log::XesEventLogImpl,
  features::discovery::{
    ocel::graph_annotation::create_ocel_annotation_for_dag,
    petri_net::{
      annotations::{annotate_with_counts, annotate_with_frequencies, annotate_with_time_performance, annotate_with_trace_frequency},
      petri_net::DefaultPetriNet,
    },
  },
  pipeline_part,
  pipelines::{
    context::PipelineContext,
    errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    keys::context_keys::{
      EVENT_LOG_KEY, GRAPH_KEY, GRAPH_TIME_ANNOTATION_KEY, OCEL_ANNOTATION_KEY, PETRI_NET_COUNT_ANNOTATION_KEY,
      PETRI_NET_FREQUENCY_ANNOTATION_KEY, PETRI_NET_KEY, PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY, TERMINATE_ON_UNREPLAYABLE_TRACES_KEY,
      TIME_ANNOTATION_KIND_KEY,
    },
    pipeline_parts::PipelineParts,
    pipelines::PipelinePartFactory,
  },
  utils::{
    context_key::DefaultContextKey,
    user_data::user_data::{UserData, UserDataImpl},
  },
};

impl PipelineParts {
  pipeline_part!(
    annotate_petri_net_count,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      Self::annotate_petri_net(
        &PETRI_NET_COUNT_ANNOTATION_KEY,
        context,
        config,
        |log, net, terminate_on_unreplayable_traces| annotate_with_counts(log, net, terminate_on_unreplayable_traces),
      )
    }
  );

  fn annotate_petri_net<T>(
    annotation_key: &DefaultContextKey<HashMap<u64, T>>,
    context: &mut PipelineContext,
    config: &UserDataImpl,
    annotator: impl Fn(&XesEventLogImpl, &DefaultPetriNet, bool) -> Option<HashMap<u64, T>>,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let petri_net = Self::get_user_data(context, &PETRI_NET_KEY)?;
    let terminate_on_unreplayable_traces = *Self::get_user_data(config, &TERMINATE_ON_UNREPLAYABLE_TRACES_KEY)?;

    let annotation = annotator(log, petri_net, terminate_on_unreplayable_traces);
    if let Some(annotation) = annotation {
      context.put_concrete(annotation_key.key(), annotation);
      Ok(())
    } else {
      let error = RawPartExecutionError::new("Failed to annotate petri net".to_owned());
      Err(PipelinePartExecutionError::Raw(error))
    }
  }

  pipeline_part!(
    annotate_petri_net_frequency,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      Self::annotate_petri_net(
        &PETRI_NET_FREQUENCY_ANNOTATION_KEY,
        context,
        config,
        |log, net, terminate_on_unreplayable_traces| annotate_with_frequencies(log, net, terminate_on_unreplayable_traces),
      )
    }
  );

  pipeline_part!(
    annotate_petri_net_trace_frequency,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      Self::annotate_petri_net(
        &PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY,
        context,
        config,
        |log, net, terminate_on_unreplayable_traces| annotate_with_trace_frequency(log, net, terminate_on_unreplayable_traces),
      )
    }
  );

  pipeline_part!(
    annotate_graph_with_time,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let graph = Self::get_user_data(context, &GRAPH_KEY)?;
      let annotation_kind = *Self::get_user_data(config, &TIME_ANNOTATION_KIND_KEY)?;

      match annotate_with_time_performance(log, graph, annotation_kind) {
        None => {
          let error = RawPartExecutionError::new("Failed to annotate graph".to_owned());
          Err(PipelinePartExecutionError::Raw(error))
        }
        Some(annotation) => {
          context.put_concrete(GRAPH_TIME_ANNOTATION_KEY.key(), annotation);
          Ok(())
        }
      }
    }
  );

  pipeline_part!(create_ocel_annotation_for_dag, |context: &mut PipelineContext, _, _| {
    let graph = Self::get_user_data(context, &GRAPH_KEY)?;

    match create_ocel_annotation_for_dag(graph) {
      Ok(annotation) => {
        context.put_concrete(OCEL_ANNOTATION_KEY.key(), annotation);
        Ok(())
      }
      Err(err) => {
        let message = format!("Failed to create ocel annotation, error: {}", err.to_string());
        Err(PipelinePartExecutionError::new_raw(message))
      }
    }
  });
}
