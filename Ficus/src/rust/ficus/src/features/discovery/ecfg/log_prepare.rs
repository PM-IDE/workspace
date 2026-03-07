use crate::{
  event_log::{
    core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
  },
  features::discovery::timeline::{
    abstraction::abstract_event_groups, events_groups::EventGroup, software_data::extraction_config::SoftwareDataExtractionConfig,
  },
  pipelines::errors::pipeline_errors::PipelinePartExecutionError,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn prepare_software_log(
  log: &XesEventLogImpl,
  config: &SoftwareDataExtractionConfig,
  time_attribute: Option<&Rc<str>>,
) -> Result<XesEventLogImpl, PipelinePartExecutionError> {
  let control_flow_regexes = config.control_flow_regexes().map_err(PipelinePartExecutionError::new_raw)?;
  if control_flow_regexes.is_none() {
    return Ok(log.clone());
  }

  let is_control_flow = |event: &Rc<RefCell<XesEventImpl>>| {
    control_flow_regexes
      .as_ref()
      .unwrap()
      .iter()
      .any(|r| r.is_match(event.borrow().name()).unwrap_or(false))
  };

  let mut event_groups = vec![];
  for trace in log.traces() {
    let trace = trace.borrow();

    let mut index = 0;
    let mut trace_groups = vec![];

    loop {
      if index >= trace.events().len() {
        break;
      }

      let event = trace.events().get(index).unwrap();
      if is_control_flow(event) {
        let mut group = EventGroup::default();

        group.control_flow_events_mut().push(event.clone());

        let mut next_control_flow_event_index = index + 1;

        while next_control_flow_event_index < trace.events().len()
          && !is_control_flow(trace.events().get(next_control_flow_event_index).unwrap())
        {
          next_control_flow_event_index += 1;
        }

        if next_control_flow_event_index > index + 1 {
          let last_stamp_index = if next_control_flow_event_index >= trace.events().len() {
            trace.events().len() - 1
          } else {
            next_control_flow_event_index
          };

          if index + 1 < last_stamp_index {
            group.set_after_group_events(Some(trace.events()[index + 1..last_stamp_index].to_vec()));
          }
        }

        trace_groups.push(group);
        index = next_control_flow_event_index;
      } else {
        index += 1;
      }
    }

    event_groups.push(trace_groups);
  }

  let mut labels = vec![];
  let mut names_labels = HashMap::new();
  let mut next_label_index = 1;

  for trace_groups in &event_groups {
    for group in trace_groups {
      let first_event = group.control_flow_events().first().unwrap().borrow();
      let group_name = first_event.name();
      let label = if let Some(label) = names_labels.get(group_name) {
        *label
      } else {
        let label = next_label_index;
        names_labels.insert(group_name.to_string(), next_label_index);
        next_label_index += 1;

        label
      };

      labels.push(label);
    }
  }

  abstract_event_groups(event_groups, &labels, None, time_attribute, config)
}
