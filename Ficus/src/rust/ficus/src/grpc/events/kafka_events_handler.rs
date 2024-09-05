use super::events_handler::{PipelineEvent, PipelineEventsHandler};

pub struct KafkaEventsHandler {}

impl KafkaEventsHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineEventsHandler for KafkaEventsHandler {
    fn handle(&self, event: PipelineEvent) {}

    fn is_alive(&self) -> bool {
        true
    }
}
