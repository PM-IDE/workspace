use bxes::binary_rw::core::{BinaryReader, Endian};
use bxes::binary_rw::cursor_stream::CursorStream;
use bxes::models::domain::bxes_event_log::BxesEvent;
use bxes::models::domain::bxes_value::BxesValue;
use bxes::read::errors::BxesReadError;
use bxes::read::read_context::ReadContext;
use bxes::read::read_utils::{
    try_read_key_values, try_read_system_metadata,
    try_read_trace_variant_events, try_read_trace_variant_metadata,
    try_read_values,
};
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::Message;
use std::io::Cursor;
use std::rc::Rc;
use std::time::Duration;

pub struct BxesKafkaConsumer {
    consumer: BaseConsumer,
}

impl BxesKafkaConsumer {
    pub fn new(consumer: BaseConsumer) -> Self {
        Self { consumer }
    }
}

#[derive(Debug)]
pub struct BxesKafkaTrace {
    metadata: Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>,
    events: Vec<BxesEvent>,
}

#[derive(Debug)]
pub enum BxesKafkaError {
    Kafka(KafkaError),
    Bxes(BxesReadError),
}

impl From<BxesReadError> for BxesKafkaError {
    fn from(value: BxesReadError) -> Self {
        Self::Bxes(value)
    }
}

impl BxesKafkaConsumer {
    pub fn consume(&mut self, action: impl Fn(BxesKafkaTrace) -> ()) -> Result<(), BxesKafkaError> {
        self.consumer.subscribe(&["my-topic"]).expect("Subscribe to topic");
        let mut read_context = ReadContext::new_without_reader();

        loop {
            match self.consumer.poll(Duration::from_secs(5)) {
                Some(message) => {
                    match message {
                        Ok(msg) => {
                            let payload = msg.payload().unwrap();

                            action(Self::parse_raw_bxes_bytes(payload, &mut read_context)?);

                            match self.consumer.commit_message(&msg, CommitMode::Async) {
                                Ok(_) => {}
                                Err(err) => return Err(BxesKafkaError::Kafka(err))
                            }
                        },
                        Err(err) => return Err(BxesKafkaError::Kafka(err)),
                    }
                },
                None => {}
            }
        }
    }

    fn parse_raw_bxes_bytes<'a, 'b>(bytes: &'b [u8], read_context: &mut ReadContext<'a>) -> Result<BxesKafkaTrace, BxesKafkaError> where 'b: 'a {
        let cursor = Cursor::new(bytes);
        let mut stream = CursorStream::new(cursor);
        let mut reader = BinaryReader::new(&mut stream, Endian::Little);

        read_context.set_reader(&mut reader);

        try_read_system_metadata(read_context)?;
        try_read_values(read_context)?;
        try_read_key_values(read_context)?;

        let metadata = try_read_trace_variant_metadata(read_context)?;
        let events = try_read_trace_variant_events(read_context)?;

        Ok(BxesKafkaTrace { metadata, events })
    }
}
