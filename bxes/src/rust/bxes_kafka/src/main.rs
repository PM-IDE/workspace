use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaConsumer;
use log::debug;
use rdkafka::ClientConfig;

pub fn main() {
    colog::init();

    let consumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "xd")
        .create()
        .expect("invalid consumer config");

    let mut consumer = BxesKafkaConsumer::new("my-topic".to_string(), consumer);
    consumer.subscribe().expect("Must subscribe");

    loop {
        let trace = consumer.consume().ok().unwrap();
        debug!("{:?}", trace);
    }
}
