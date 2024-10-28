using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.Domain.Values.Lifecycle;
using Bxes.Writer;
using Core.Events.EventRecord;

namespace Core.Bxes;

public class BxesEvent : IEvent
{
  public long Timestamp { get; }
  public string Name { get; }
  public IEventLifecycle Lifecycle { get; }
  public IList<AttributeKeyValue> Attributes { get; }


  public BxesEvent(EventRecordWithMetadata eventRecord, bool writeAllEventMetadata)
  {
    Timestamp = (eventRecord.Time.LoggedAt.Ticks - DateTime.UnixEpoch.Ticks) * 100;
    Name = eventRecord.EventName;
    Lifecycle = new BrafLifecycle(BrafLifecycleValues.Unspecified);

    Attributes = writeAllEventMetadata switch
    {
      false => [],
      true => eventRecord.Metadata.Select(kv =>
        new AttributeKeyValue(new BxesStringValue(kv.Key), new BxesStringValue(kv.Value))).ToList()
    };

    Attributes.Add(new AttributeKeyValue(new BxesStringValue("ThreadId"), new BxesInt64Value(eventRecord.ManagedThreadId)));
    Attributes.Add(new AttributeKeyValue(new BxesStringValue("QpcStamp"), new BxesInt64Value(eventRecord.Time.QpcStamp)));
  }

  public bool Equals(IEvent? other) => other is { } && EventUtil.Equals(this, other);
}