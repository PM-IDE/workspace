use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::discovery::root_sequence::context_keys::{EDGE_SOFTWARE_DATA_KEY, EDGE_START_END_ACTIVITIES_TIMES_KEY};
use crate::features::discovery::root_sequence::models::ActivityStartEndTimeData;
use crate::features::discovery::timeline::abstraction::extract_edge_software_data;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::utils::user_data::user_data::{UserData, UserDataOwner};
use std::cell::RefCell;
use std::rc::Rc;
use crate::features::discovery::timeline::utils::get_stamp;

pub fn prepare_software_log(
  log: &XesEventLogImpl,
  config: &SoftwareDataExtractionConfig,
  time_attribute: Option<&String>,
) -> Result<XesEventLogImpl, String> {
  let control_flow_regexes = config.control_flow_regexes()?;
  if control_flow_regexes.is_none() {
    return Ok(log.clone())
  }

  let is_control_flow = |event: &Rc<RefCell<XesEventImpl>>| {
    control_flow_regexes.as_ref().unwrap().iter().any(|r| r.is_match(event.borrow().name().as_str()).unwrap_or(false))
  };

  let mut new_log = XesEventLogImpl::empty();
  for trace in log.traces() {
    let trace = trace.borrow();

    let mut index = 0;
    let mut new_trace = XesTraceImpl::empty();

    loop {
      if index >= trace.events().len() {
        break;
      }

      let event = trace.events().get(index).unwrap();
      if is_control_flow(event) {
        let mut next_control_flow_event_index = index + 1;
        let start_stamp = get_stamp(&event.borrow(), time_attribute).map_err(|_| "Failed to get stamp of first control flow event")?;

        while next_control_flow_event_index < trace.events().len() && !is_control_flow(trace.events().get(next_control_flow_event_index).unwrap()) {
          next_control_flow_event_index += 1;
        }

        let mut new_event = event.borrow().clone();

        if next_control_flow_event_index > index + 1 {
          let last_stamp_index = if next_control_flow_event_index >= trace.events().len() {
            trace.events().len() - 1
          } else {
            next_control_flow_event_index
          };

          let end_event = &trace.events()[last_stamp_index];
          let end_stamp = get_stamp(&end_event.borrow(), time_attribute).map_err(|_| "Failed to get stamp of first control flow event")?;

          let edge_data = extract_edge_software_data(config, &trace.events()[index + 1..next_control_flow_event_index]).map_err(|e| e.to_string())?;

          if let Some(edge_data) = edge_data {
            new_event.user_data_mut().put_concrete(EDGE_SOFTWARE_DATA_KEY.key(), vec![edge_data]);
            new_event.user_data_mut().put_concrete(EDGE_START_END_ACTIVITIES_TIMES_KEY.key(), vec![
              ActivityStartEndTimeData::new(start_stamp, end_stamp)
            ]);
          }
        }

        new_trace.push(Rc::new(RefCell::new(new_event)));
        index = next_control_flow_event_index;
      } else { 
        index += 1;
      }
    }
    
    new_log.push(Rc::new(RefCell::new(new_trace)));
  }

  Ok(new_log)
}