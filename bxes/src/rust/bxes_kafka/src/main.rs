use rdkafka::ClientConfig;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaConsumer;

pub fn main() {
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "xd")
        .create()
        .expect("invalid consumer config");

    let mut consumer = BxesKafkaConsumer::new(consumer);

    consumer.consume(|trace| {
        println!("{:?}", trace);
    }).unwrap()
}