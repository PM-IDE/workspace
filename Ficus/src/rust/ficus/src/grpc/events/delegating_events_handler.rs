use crate::grpc::events::events_handler::{PipelineEvent, PipelineEventsHandler};

pub struct DelegatingEventsHandler {
    handlers: Vec<Box<dyn PipelineEventsHandler>>,
}

impl DelegatingEventsHandler {
    pub fn new(handlers: Vec<Box<dyn PipelineEventsHandler>>) -> Self {
        Self { handlers }
    }
}

impl PipelineEventsHandler for DelegatingEventsHandler {
    fn handle(&self, event: &PipelineEvent) {
        for handler in &self.handlers {
            handler.handle(event);
        }
    }

    fn is_alive(&self) -> bool {
        self.handlers.iter().map(|h| h.is_alive()).fold(true, |a, b| a && b)
    }
}
