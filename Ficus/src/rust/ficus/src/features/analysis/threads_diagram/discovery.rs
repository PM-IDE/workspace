use std::collections::HashMap;
use std::ops::Sub;
use chrono::{DateTime, Utc};
use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;

pub struct LogThreadsDiagram {
    traces: Vec<TraceThreadsDiagram>
}

pub struct TraceThreadsDiagram {
    threads: Vec<TraceThread>
}

pub struct TraceThread {
    events: Vec<TraceThreadEvent>
}

pub struct TraceThreadEvent {
    name: String,
    timestamp: DateTime<Utc>,
    relative_edge_len: f64
}

pub fn discover_threads_diagram(log: &XesEventLogImpl, thread_attribute: &str) -> LogThreadsDiagram {
    let mut max_time_delta_ms: Option<f64> = None;
    let mut traces = vec![];

    for trace in log.traces() {
        let trace = trace.borrow();

        let mut threads: HashMap<Option<String>, TraceThread> = HashMap::new();
        
        for i in 0..trace.events().len() {
            let event = trace.events().get(i).expect("Must be in range");
            let event = event.borrow();

            let thread_id = if let Some(map) = event.payload_map() {
                if let Some(value) = map.get(thread_attribute) {
                    Some(value.to_string_repr().as_str().to_owned())
                } else {
                    None
                }
            } else {
                None
            };
            
            let edge_len = if i + 1 < trace.events().len() {
                let this_stamp = event.timestamp();
                let next_stamp = trace.events().get(i + 1).expect("Must be in range").borrow().timestamp().clone();
                next_stamp.sub(this_stamp).num_milliseconds() as f64
            } else {
                0.
            };
            
            if let Some(prev_max) = max_time_delta_ms {
                max_time_delta_ms = Some(prev_max.max(edge_len));
            } else {
                max_time_delta_ms = Some(edge_len);
            }

            let thread_event = TraceThreadEvent {
                timestamp: event.timestamp().clone(),
                name: event.name().to_owned(),
                relative_edge_len: edge_len
            };
            
            if let Some(thread) = threads.get_mut(&thread_id) {
                thread.events.push(thread_event);
            } else {
                threads.insert(thread_id, TraceThread {
                    events: vec![thread_event]
                });
            }
        }
        
        traces.push(TraceThreadsDiagram {
            threads: threads.into_iter().map(|(_, v)| v).collect()
        })
    }

    if let Some(max_edge_len) = max_time_delta_ms {
        for trace in traces.iter_mut() {
            for thread in trace.threads.iter_mut() {
                for event in thread.events.iter_mut() {
                    event.relative_edge_len = event.relative_edge_len / max_edge_len;
                }
            }
        }
    }

    LogThreadsDiagram {
        traces
    }
}