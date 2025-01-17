use crate::{
    event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    features::{analysis::event_log_info::OfflineEventLogInfo, discovery::alpha::providers::relations_cache::RelationsCaches},
};
use crate::features::analysis::event_log_info::EventLogInfo;
use super::fuzzy_miner::FuzzyGraph;

const PROXIMITY_CORRELATION: &'static str = "ProximityCorrelation";

pub struct FuzzyMetricsProvider<'a, TLog>
where
    TLog: EventLog,
{
    log: &'a TLog,
    log_info: &'a dyn EventLogInfo,
    caches: RelationsCaches<f64>,
}

impl<'a, TLog> FuzzyMetricsProvider<'a, TLog>
where
    TLog: EventLog,
{
    pub fn new(log: &'a TLog, log_info: &'a OfflineEventLogInfo) -> Self {
        Self {
            log,
            log_info,
            caches: RelationsCaches::new(&[PROXIMITY_CORRELATION]),
        }
    }

    pub fn log_info(&self) -> &dyn EventLogInfo {
        self.log_info
    }

    pub fn unary_frequency_significance(&self, event_class: &String) -> f64 {
        self.log_info.event_count(event_class) as f64
    }

    pub fn binary_frequency_significance(&self, first_class: &String, second_class: &String) -> f64 {
        self.log_info.dfg_info().get_directly_follows_count(first_class, second_class) as f64
    }

    pub fn proximity_correlation(&mut self, first_class: &String, second_class: &String) -> f64 {
        if let Some(value) = self.caches.cache(PROXIMITY_CORRELATION).try_get(first_class, second_class) {
            return *value;
        }

        let mut count = 0;
        let mut result = 0.0;
        for trace in self.log.traces() {
            let trace = trace.borrow();
            let events = trace.events();
            let mut last_seen_first = None;

            for i in 0..events.len() {
                let event = events[i].borrow();
                let name = event.name();

                if name == first_class {
                    last_seen_first = Some(i.clone());
                    continue;
                }

                if name == second_class {
                    if let Some(first_index) = last_seen_first {
                        let second_stamp = event.timestamp();
                        let first_event = events.get(first_index).unwrap();
                        let first_event = first_event.borrow();
                        let first_stamp = first_event.timestamp();

                        result += second_stamp.signed_duration_since(*first_stamp).num_milliseconds() as f64;
                        count += 1;
                        last_seen_first = None;
                    }
                }
            }
        }

        result = if count != 0 { result / (count as f64) } else { 0.0 };

        self.caches
            .cache_mut(PROXIMITY_CORRELATION)
            .put(first_class, second_class, result.clone());

        result
    }

    pub fn relative_significance(&self, a: &String, b: &String, graph: &FuzzyGraph) -> f64 {
        let a_b_sig = self.binary_frequency_significance(a, b);

        let mut first_sig = 0.5 * a_b_sig;
        let mut second_sig = 0.5 * a_b_sig;
        let mut first_sum = 0.0;
        let mut second_sum = 0.0;

        for node in graph.all_nodes() {
            let name = node.data().unwrap();
            first_sum += self.binary_frequency_significance(a, name);
            second_sum += self.binary_frequency_significance(name, b);
        }

        first_sig /= first_sum;
        second_sig /= second_sum;

        first_sig + second_sig
    }

    pub fn utility_measure(&mut self, first: &String, second: &String, utility_rate: f64) -> f64 {
        utility_rate * self.binary_frequency_significance(first, second) + (1.0 - utility_rate) * self.proximity_correlation(first, second)
    }
}
