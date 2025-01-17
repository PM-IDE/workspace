use std::collections::HashMap;
use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;

pub trait TriangleRelation {
    fn get(&self, first: &str, second: &str) -> Option<usize>;
}

pub struct OfflineTriangleRelation {
    relations: HashMap<(String, String), usize>
}

impl OfflineTriangleRelation {
    pub fn new(log: &impl EventLog) -> Self {
        let mut relations = HashMap::new();
        for trace in log.traces() {
            let trace = trace.borrow();
            let events = trace.events();

            if events.len() < 3 {
                continue;
            }

            for index in 0..(events.len() - 2) {
                if events[index].borrow().name() == events[index + 2].borrow().name() {
                    let pair = (
                        events[index].borrow().name().to_owned(),
                        events[index + 1].borrow().name().to_owned(),
                    );

                    if let Some(value) = relations.get_mut(&pair) {
                        *value += 1;
                    } else {
                        relations.insert(pair, 1);
                    }
                }
            }
        }

        Self {
            relations
        }
    }
}

impl TriangleRelation for OfflineTriangleRelation {
    fn get(&self, first: &str, second: &str) -> Option<usize> {
        if let Some(measure) = self.relations.get(&(first.to_owned(), second.to_owned())) {
            Some(*measure)
        } else {
            None
        }
    }
}