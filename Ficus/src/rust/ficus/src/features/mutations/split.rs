use std::{
  cell::RefCell,
  collections::{hash_map::DefaultHasher, HashMap},
  hash::{Hash, Hasher},
  rc::Rc,
};

use crate::event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace};

struct TracePointer<TTrace> {
  pub trace: Rc<RefCell<TTrace>>,
  pub index: usize,
}

impl<TTrace> TracePointer<TTrace> {
  pub fn new(trace: Rc<RefCell<TTrace>>, index: usize) -> TracePointer<TTrace> {
    TracePointer { trace, index }
  }
}

impl<TTrace> Clone for TracePointer<TTrace> {
  fn clone(&self) -> Self {
    Self {
      trace: Rc::clone(&self.trace),
      index: self.index,
    }
  }
}

pub fn get_traces_groups_indices<TLog>(log: &TLog) -> Vec<Vec<usize>>
where
  TLog: EventLog,
{
  create_trace_pointers_groups(log)
    .into_iter()
    .map(|group| group.into_iter().map(|ptr| ptr.index).collect())
    .collect()
}

pub fn split_by_traces<TLog>(log: &TLog) -> Vec<Vec<Rc<RefCell<TLog::TTrace>>>>
where
  TLog: EventLog,
{
  create_trace_pointers_groups(log)
    .into_iter()
    .map(|group| group.into_iter().map(|ptr| ptr.trace).collect())
    .collect()
}

fn create_trace_pointers_groups<TLog>(log: &TLog) -> Vec<Vec<TracePointer<TLog::TTrace>>>
where
  TLog: EventLog,
{
  let len_to_traces = create_len_to_traces_map(log);

  let mut result = Vec::new();
  for (_, traces) in len_to_traces {
    process_traces_group(traces, &mut result);
  }

  result.sort_by(|first, second| first[0].index.cmp(&second[0].index));
  result
}

fn create_len_to_traces_map<TLog>(log: &TLog) -> HashMap<usize, Vec<TracePointer<TLog::TTrace>>>
where
  TLog: EventLog,
{
  let mut len_to_traces: HashMap<usize, Vec<TracePointer<TLog::TTrace>>> = HashMap::new();
  let traces = log.traces();

  for index in 0..traces.len() {
    let trace = Rc::clone(&traces[index]);
    let len = trace.borrow().events().len();
    if len_to_traces.contains_key(&len) {
      (*len_to_traces.get_mut(&len).unwrap()).push(TracePointer::new(trace, index));
    } else {
      len_to_traces.insert(len, vec![TracePointer::new(trace, index)]);
    }
  }

  len_to_traces
}

fn process_traces_group<TTrace>(traces: Vec<TracePointer<TTrace>>, result: &mut Vec<Vec<TracePointer<TTrace>>>)
where
  TTrace: Trace,
{
  if traces.len() == 1 {
    result.push(traces);
    return;
  }

  let mut groups = create_initial_groups(&traces);
  let mut index = 0;

  loop {
    if index >= traces[0].trace.borrow().events().len() {
      break;
    }

    let (new_groups, all_groups_have_one_trace) = update_groups(&groups, index);

    if all_groups_have_one_trace {
      break;
    }

    index += 1;
    groups = new_groups;
  }

  for group in groups {
    result.push(group);
  }
}

fn create_initial_groups<TTrace>(traces: &Vec<TracePointer<TTrace>>) -> Vec<Vec<TracePointer<TTrace>>> {
  let mut groups = Vec::new();
  for trace in traces {
    groups.push((*trace).clone());
  }

  vec![groups]
}

fn update_groups<TTrace>(groups: &Vec<Vec<TracePointer<TTrace>>>, index: usize) -> (Vec<Vec<TracePointer<TTrace>>>, bool)
where
  TTrace: Trace,
{
  let mut new_groups = Vec::new();
  let mut all_groups_have_one_trace = true;
  for group in groups {
    if group.len() == 1 {
      new_groups.push(group.to_vec());
      continue;
    }

    all_groups_have_one_trace = false;
    let mut hashes_to_traces: HashMap<u64, Vec<TracePointer<TTrace>>> = HashMap::new();
    for trace in group {
      let mut hasher = DefaultHasher::new();
      let actual_trace = trace.trace.borrow();
      let event = actual_trace.events()[index].borrow();
      event.name().hash(&mut hasher);

      let hash_code = hasher.finish();
      if hashes_to_traces.contains_key(&hash_code) {
        (*hashes_to_traces.get_mut(&hash_code).unwrap()).push((*trace).clone());
      } else {
        hashes_to_traces.insert(hash_code, vec![(*trace).clone()]);
      }
    }

    for (_, new_group) in hashes_to_traces {
      new_groups.push(new_group);
    }
  }

  (new_groups, all_groups_have_one_trace)
}
