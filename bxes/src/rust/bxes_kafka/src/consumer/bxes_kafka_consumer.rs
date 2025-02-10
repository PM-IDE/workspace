use bxes::binary_rw::core::{BinaryReader, Endian};
use bxes::binary_rw::cursor_stream::CursorStream;
use bxes::models::domain::bxes_event_log::BxesEvent;
use bxes::models::domain::bxes_value::BxesValue;
use bxes::read::errors::BxesReadError;
use bxes::read::read_context::{ReadContext, ReadMetadata};
use bxes::read::read_utils::{
    try_read_key_values, try_read_system_metadata, try_read_trace_variant_events,
    try_read_trace_variant_metadata, try_read_values,
};
use log::info;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::Message;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;

pub struct BxesKafkaConsumer {
    topic: String,
    consumer: BaseConsumer,
    session_id_to_read_metadata: HashMap<Uuid, ReadMetadata>,
}

unsafe impl Send for BxesKafkaConsumer {}

impl BxesKafkaConsumer {
    pub fn new(topic: String, consumer: BaseConsumer) -> Self {
        Self {
            topic,
            consumer,
            session_id_to_read_metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BxesKafkaTrace {
    metadata: HashMap<String, Rc<Box<BxesValue>>>,
    events: Vec<BxesEvent>,
}

impl BxesKafkaTrace {
    pub fn metadata(&self) -> &HashMap<String, Rc<Box<BxesValue>>> {
        &self.metadata
    }

    pub fn events(&self) -> &Vec<BxesEvent> {
        &self.events
    }
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

impl From<KafkaError> for BxesKafkaError {
    fn from(value: KafkaError) -> Self {
        Self::Kafka(value)
    }
}

impl BxesKafkaConsumer {
    pub fn subscribe(&mut self) -> Result<(), BxesKafkaError> {
        match self.consumer.subscribe(&[self.topic.as_str()]) {
            Ok(_) => Ok(()),
            Err(err) => Err(BxesKafkaError::Kafka(err)),
        }
    }

    pub fn unsubscribe(&mut self) {
        self.consumer.unsubscribe()
    }

    pub fn consume(&mut self) -> Result<Option<BxesKafkaTrace>, BxesKafkaError> {
        match self.consumer.poll(Duration::from_millis(1000)) {
            Some(message) => match message {
                Ok(msg) => {
                    let payload = msg.payload().unwrap();
                    const UUID_LENGTH: usize = 16;

                    let read_metadata_id = Uuid::from_slice(&payload[..UUID_LENGTH]).expect("Should be valid uuid");

                    info!("Read bxes trace with read metadata id {}", read_metadata_id);

                    if !self.session_id_to_read_metadata.contains_key(&read_metadata_id) {
                        info!("Creating new read metadata for id {}", read_metadata_id);
                        self.session_id_to_read_metadata
                            .insert(read_metadata_id.clone(), ReadMetadata::empty());
                    }

                    let mut read_metadata = self
                        .session_id_to_read_metadata
                        .get_mut(&read_metadata_id)
                        .expect("Must be present");

                    let trace = Self::parse_raw_bxes_bytes(&payload[UUID_LENGTH..], &mut read_metadata)?;

                    self.consumer.commit_message(&msg, CommitMode::Sync)?;

                    Ok(Some(trace))
                }
                Err(err) => Err(BxesKafkaError::Kafka(err)),
            },
            None => Ok(None),
        }
    }

    fn parse_raw_bxes_bytes(
        bytes: &[u8],
        read_metadata: &mut ReadMetadata,
    ) -> Result<BxesKafkaTrace, BxesKafkaError> {
        let cursor = Cursor::new(bytes);
        let mut stream = CursorStream::new(cursor);
        let mut reader = BinaryReader::new(&mut stream, Endian::Little);
        let mut read_context = ReadContext::new(&mut reader, read_metadata);

        try_read_system_metadata(&mut read_context)?;
        try_read_values(&mut read_context)?;
        try_read_key_values(&mut read_context)?;

        let metadata = try_read_trace_variant_metadata(&mut read_context)?;
        let metadata = Self::create_trace_metadata(metadata)?;
        let events = try_read_trace_variant_events(&mut read_context)?;

        Ok(BxesKafkaTrace { metadata, events })
    }

    fn create_trace_metadata(
        metadata: Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>,
    ) -> Result<HashMap<String, Rc<Box<BxesValue>>>, BxesReadError> {
        let mut new_metadata = HashMap::new();

        for (key, value) in metadata {
            if let BxesValue::String(key) = key.as_ref().as_ref() {
                new_metadata.insert(key.as_ref().as_ref().to_owned(), value);
            } else {
                return Err(BxesReadError::ExpectedString(
                    key.as_ref().as_ref().to_owned(),
                ));
            }
        }

        Ok(new_metadata)
    }
}
