namespace ProcfilerOnline.Integrations.Kafka;

public interface IKafkaProducer<in TKey, in TValue>
{
  void Produce(TKey key, TValue value);
}