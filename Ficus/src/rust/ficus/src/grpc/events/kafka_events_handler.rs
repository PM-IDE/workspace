use super::events_handler::{PipelineEvent, PipelineEventsHandler};
use crate::ficus_proto::GrpcKafkaUpdate;
use crate::grpc::events::utils::create_grpc_context_values;
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::utils::stream_queue::AsyncStreamQueuePusher;

pub struct KafkaEventsHandler {
    stream_pusher: AsyncStreamQueuePusher<GrpcKafkaUpdate>,
    console_logs_handler: ConsoleLogMessageHandler,
}

impl KafkaEventsHandler {
    pub fn new(stream_pusher: AsyncStreamQueuePusher<GrpcKafkaUpdate>) -> Self {
        Self {
            stream_pusher,
            console_logs_handler: ConsoleLogMessageHandler::new(),
        }
    }
}

impl PipelineEventsHandler for KafkaEventsHandler {
    fn handle(&self, event: PipelineEvent) {
        let update = match event {
            PipelineEvent::GetContextValuesEvent(event) => GrpcKafkaUpdate {
                case_name: event.case_name,
                context_values: create_grpc_context_values(&event.key_values),
            },
            PipelineEvent::LogMessage(_) => {
                todo!()
            }
            PipelineEvent::FinalResult(_) => {
                todo!()
            }
        };
    }

    fn is_alive(&self) -> bool {
        true
    }
}
