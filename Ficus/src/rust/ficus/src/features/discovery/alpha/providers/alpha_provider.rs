use crate::features::analysis::event_log_info::{EventLogInfo, OfflineEventLogInfo};

pub trait AlphaRelationsProvider {
    fn causal_relation(&self, first: &str, second: &str) -> bool;
    fn parallel_relation(&self, first: &str, second: &str) -> bool;
    fn direct_relation(&self, first: &str, second: &str) -> bool;
    fn unrelated_relation(&self, first: &str, second: &str) -> bool;

    fn log_info(&self) -> &dyn EventLogInfo;
}

pub struct DefaultAlphaRelationsProvider<'a> {
    log_info: &'a OfflineEventLogInfo,
}

impl<'a> DefaultAlphaRelationsProvider<'a> {
    pub fn new(log_info: &'a OfflineEventLogInfo) -> Self {
        Self { log_info }
    }
}

impl<'a> AlphaRelationsProvider for DefaultAlphaRelationsProvider<'a> {
    fn causal_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && !self.direct_relation(second, first)
    }

    fn parallel_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && self.direct_relation(second, first)
    }

    fn direct_relation(&self, first: &str, second: &str) -> bool {
        self.log_info.dfg_info().is_in_directly_follows_relation(first, second)
    }

    fn unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.direct_relation(first, second) && !self.direct_relation(second, first)
    }

    fn log_info(&self) -> &dyn EventLogInfo {
        self.log_info
    }
}
