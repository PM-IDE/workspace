use crate::event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace};
use std::{cell::RefCell, rc::Rc};

pub fn rename_events<TLog, TFilter>(log: &mut TLog, new_name: &str, filter: TFilter)
where
  TLog: EventLog,
  TFilter: Fn(&TLog::TEvent) -> bool,
{
  log.mutate_events(|event| {
    if filter(event) {
      event.set_name(new_name.to_owned())
    }
  })
}

pub const ARTIFICIAL_START_EVENT_NAME: &str = "ARTIFICIAL_START";
pub const ARTIFICIAL_END_EVENT_NAME: &str = "ARTIFICIAL_END";

enum StartOrEnd {
  Start,
  End,
}

pub fn add_artificial_start_end_activities<TLog: EventLog>(
  log: &mut TLog,
  add_start_events: bool,
  add_end_events: bool,
  attributes_to_copy: Option<&Vec<String>>,
) {
  for trace in log.traces() {
    let mut trace = trace.borrow_mut();
    let events = trace.events_mut();

    let mut add_artificial_event = |start_or_end: StartOrEnd| {
      let name = match start_or_end {
        StartOrEnd::Start => ARTIFICIAL_START_EVENT_NAME,
        StartOrEnd::End => ARTIFICIAL_END_EVENT_NAME,
      }
      .to_string();

      let artificial_start_event = if events.is_empty() {
        match start_or_end {
          StartOrEnd::Start => TLog::TEvent::new_with_min_date(name),
          StartOrEnd::End => TLog::TEvent::new_with_max_date(name),
        }
      } else {
        let reference_event = match start_or_end {
          StartOrEnd::Start => events.first(),
          StartOrEnd::End => events.last(),
        }
        .expect("!events.is_empty()");

        let mut start_event = TLog::TEvent::new(name, *reference_event.borrow().timestamp());
        copy_payload::<TLog>(&reference_event.borrow(), &mut start_event, attributes_to_copy);

        start_event
      };

      let insertion_index = match start_or_end {
        StartOrEnd::Start => 0,
        StartOrEnd::End => events.len(),
      };

      events.insert(insertion_index, Rc::new(RefCell::new(artificial_start_event)));
    };

    if add_start_events {
      add_artificial_event(StartOrEnd::Start);
    }

    if add_end_events {
      add_artificial_event(StartOrEnd::End);
    }
  }
}

fn copy_payload<TLog: EventLog>(from: &TLog::TEvent, to: &mut TLog::TEvent, attributes_to_copy: Option<&Vec<String>>) {
  let Some(attributes_to_copy) = attributes_to_copy else { return };
  let Some(payload_map) = from.payload_map() else { return };

  for attr in attributes_to_copy {
    let Some(value) = payload_map.get(attr) else { continue };
    to.add_or_update_payload(attr.clone(), value.clone());
  }
}

pub fn append_attributes_to_name<TLog: EventLog>(log: &mut TLog, attributes: &Vec<String>) {
  log.mutate_events(|event| {
    let mut new_name = event.name().to_owned();
    let payload = event.payload_map();

    for attribute in attributes {
      let value = match payload {
        None => None,
        Some(payload) => payload.get(attribute).map(|value| value.to_string_repr()),
      };

      let attribute_value_string = match value {
        None => "None".to_string(),
        Some(value) => value.as_str().to_owned(),
      };

      new_name += format!("_{}", attribute_value_string).as_str();
    }

    event.set_name(new_name);
  })
}
