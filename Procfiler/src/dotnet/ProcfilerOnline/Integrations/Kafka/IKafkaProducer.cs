namespace ProcfilerOnline.Integrations.Kafka;

public interface IKafkaProducer<TKey, TValue>
{
  void Produce(string topicName, TKey key, TValue value);
}