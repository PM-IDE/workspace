use super::{context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError, pipelines::PipelinePartFactory};
use crate::{
  event_log::{
    core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
  },
  features::analysis::patterns::activity_instances::{SubTraceKind, UNDEF_ACTIVITY_NAME},
  pipeline_part,
  pipelines::{
    keys::context_keys::{
      ATTRIBUTE_KEY, COLORS_EVENT_LOG_KEY, COLORS_HOLDER_KEY, EVENT_LOG_KEY, EVENT_NAME_KEY, REGEX_KEY, TRACE_ACTIVITIES_KEY,
    },
    pipeline_parts::PipelineParts,
  },
  utils::{
    colors::{Color, ColoredRectangle, ColorsEventLog, ColorsHolder},
    user_data::user_data::{UserData, UserDataImpl},
  },
};
use fancy_regex::Regex;
use std::{collections::HashMap, rc::Rc};
use std::sync::Arc;

impl PipelineParts {
  pipeline_part!(traces_diversity_diagram, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let colors_holder = context.concrete_mut(COLORS_HOLDER_KEY.key()).expect("Should be initialized");
    let colors_log = Self::create_traces_diversity_colors_log(log, colors_holder, |e| e.name_pointer().clone());

    context.put_concrete(COLORS_EVENT_LOG_KEY.key(), colors_log);

    Ok(())
  });

  fn create_traces_diversity_colors_log(
    log: &XesEventLogImpl,
    colors_holder: &mut ColorsHolder,
    color_key_selector: impl Fn(&XesEventImpl) -> Arc<str>,
  ) -> ColorsEventLog {
    let mut mapping = HashMap::new();
    let mut traces = vec![];
    for trace in log.traces() {
      let mut vec = vec![];

      for (index, event) in trace.borrow().events().iter().enumerate() {
        let event = event.borrow();

        let colors_key = color_key_selector(&event);

        let color = colors_holder.get_or_create(&colors_key);
        if !mapping.contains_key(&colors_key) {
          mapping.insert(colors_key.to_owned(), color);
        }

        let name = colors_key.to_owned();
        vec.push(ColoredRectangle::square(name, index as f64));
      }

      traces.push(vec);
    }

    ColorsEventLog { mapping, traces }
  }

  pipeline_part!(
    draw_placement_of_event_by_name,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let event_name = Self::get_user_data(config, &EVENT_NAME_KEY)?;
      Self::draw_events_placement(context, &|event| event.name() == event_name.as_ref())
    }
  );

  pub(super) fn draw_events_placement(
    context: &mut PipelineContext,
    selector: &impl Fn(&XesEventImpl) -> bool,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let colors_holder = Self::get_user_data_mut(context, &COLORS_HOLDER_KEY).expect("Default value should be initialized");

    let mut traces = vec![];
    let mut mapping = HashMap::new();
    mapping.insert(Arc::from(UNDEF_ACTIVITY_NAME.to_owned()), Color::black());

    for trace in log.traces() {
      let mut colors_trace = vec![];

      for (index, event) in trace.borrow().events().iter().enumerate() {
        let event = event.borrow();
        let name = event.name_pointer();
        if selector(&event) {
          let color = colors_holder.get_or_create(name);
          if !mapping.contains_key(name) {
            mapping.insert(name.clone(), color);
          }

          let name = event.name_pointer().clone();
          colors_trace.push(ColoredRectangle::square(name, index as f64));
        } else {
          let name = Arc::from(UNDEF_ACTIVITY_NAME.to_owned());
          colors_trace.push(ColoredRectangle::square(name, index as f64));
        }
      }

      traces.push(colors_trace);
    }

    context.put_concrete(COLORS_EVENT_LOG_KEY.key(), ColorsEventLog { mapping, traces });

    Ok(())
  }

  pipeline_part!(
    draw_placement_of_event_by_regex,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let regex = Self::get_user_data(config, &REGEX_KEY)?;
      let regex = Regex::new(regex).ok().unwrap();
      Self::draw_events_placement(context, &|event| regex.is_match(event.name()).ok().unwrap())
    }
  );

  pipeline_part!(draw_full_activities_diagram, |context: &mut PipelineContext, _, _| {
    let traces_activities = Self::get_user_data(context, &TRACE_ACTIVITIES_KEY)?;
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let colors_holder = Self::get_user_data_mut(context, &COLORS_HOLDER_KEY)?;

    let mut traces = vec![];
    let mut mapping = HashMap::new();
    mapping.insert(Arc::from(UNDEF_ACTIVITY_NAME), Color::black());

    for (activities, trace) in traces_activities.iter().zip(log.traces().iter()) {
      let mut colors_trace = vec![];
      let trace_length = trace.borrow().events().len();

      Self::execute_with_activities_instances(activities, trace_length, &mut |sub_trace| match sub_trace {
        SubTraceKind::Attached(activity) => {
          let color = colors_holder.get_or_create(activity.node().borrow().name());
          let name = activity.node().borrow().name().clone();
          if !mapping.contains_key(name.as_ref()) {
            mapping.insert(name.clone(), color);
          }

          colors_trace.push(ColoredRectangle::new(name, *activity.start_pos() as f64, *activity.length() as f64));
        }
        SubTraceKind::Unattached(start_pos, length) => {
          colors_trace.push(ColoredRectangle::new(
            Arc::from(UNDEF_ACTIVITY_NAME.to_string()),
            start_pos as f64,
            length as f64,
          ));
        }
      })?;

      traces.push(colors_trace);
    }

    context.put_concrete(COLORS_EVENT_LOG_KEY.key(), ColorsEventLog { mapping, traces });

    Ok(())
  });

  pipeline_part!(draw_short_activities_diagram, |context: &mut PipelineContext, _, _| {
    let traces_activities = Self::get_user_data(context, &TRACE_ACTIVITIES_KEY)?;
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let colors_holder = Self::get_user_data_mut(context, &COLORS_HOLDER_KEY)?;

    let mut traces = vec![];
    let mut mapping = HashMap::new();
    mapping.insert(Arc::<str>::from(UNDEF_ACTIVITY_NAME), Color::black());

    for (activities, trace) in traces_activities.iter().zip(log.traces().iter()) {
      let mut colors_trace = vec![];
      let mut index = 0;
      let trace_length = trace.borrow().events().len();
      Self::execute_with_activities_instances(activities, trace_length, &mut |sub_trace| {
        match sub_trace {
          SubTraceKind::Attached(activity) => {
            let color = colors_holder.get_or_create(activity.node().borrow().name());
            let node = activity.node().borrow();
            let name = node.name();

            if !mapping.contains_key(name) {
              mapping.insert(name.clone(), color);
            }

            colors_trace.push(ColoredRectangle::new(name.clone(), index as f64, 1.));
          }
          SubTraceKind::Unattached(_, _) => {
            let ptr = Arc::from(UNDEF_ACTIVITY_NAME.to_owned());
            colors_trace.push(ColoredRectangle::new(ptr, index as f64, 1.));
          }
        }

        index += 1;
      })?;

      traces.push(colors_trace);
    }

    context.put_concrete(COLORS_EVENT_LOG_KEY.key(), ColorsEventLog { mapping, traces });

    Ok(())
  });

  pipeline_part!(
    traces_diversity_diagram_by_attribute,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let colors_holder = context.concrete_mut(COLORS_HOLDER_KEY.key()).expect("Should be initialized");
      let attribute = Self::get_user_data(config, &ATTRIBUTE_KEY)?;

      let colors_log = Self::create_traces_diversity_colors_log(log, colors_holder, |e| {
        if let Some(attributes) = e.payload_map()
          && let Some(value) = attributes.get(attribute)
        {
          return value.to_string_repr();
        }

        Arc::from("UNDEF_ATTRIBUTE".to_string())
      });

      context.put_concrete(COLORS_EVENT_LOG_KEY.key(), colors_log);

      Ok(())
    }
  );
}
