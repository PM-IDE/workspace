using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities;

public interface IIdCreationStrategy
{
  public string CreateIdTemplate();
  public string CreateId(EventRecordWithMetadata eventRecord);
}

public class DefaultIdCreationStrategy(string basePart, IReadOnlySet<string> startEvents) : IIdCreationStrategy
{
  private int myNextId;


  public string CreateIdTemplate() => DoCreateId("{AUTOINCREMENT_ID}");

  private string DoCreateId(string nextId)
  {
    return $"{basePart}_{nextId}";
  }

  public string CreateId(EventRecordWithMetadata eventRecord)
  {
    if (startEvents.Contains(eventRecord.EventClass))
    {
      myNextId++;
    }

    return DoCreateId(myNextId.ToString());
  }
}

public class FromAttributesIdCreationStrategy
  (string basePart, ICollection<string> attributesToUse) : IIdCreationStrategy
{
  public string CreateIdTemplate() => DoCreateId(basePart, attributesToUse);

  private static string DoCreateId(string basePart, IEnumerable<string> attributes)
  {
    var basePartSb = new StringBuilder(basePart);

    foreach (var attribute in attributes)
    {
      basePartSb.Append(TraceEventsConstants.Underscore).Append(attribute);
    }

    return basePartSb.ToString();
  }

  public string CreateId(EventRecordWithMetadata eventRecord)
  {
    var attributeValues = new List<string>();
    foreach (var attributeName in attributesToUse)
    {
      if (!eventRecord.Metadata.TryGetValue(attributeName, out var value))
      {
        throw new KeyNotFoundException(attributeName);
      }

      attributeValues.Add(value);
    }

    return DoCreateId(basePart, attributeValues);
  }
}

public class FromEventActivityIdIdCreationStrategy(string basePart) : IIdCreationStrategy
{
  public string CreateIdTemplate() => $"{basePart}_GUID";

  public string CreateId(EventRecordWithMetadata eventRecord) => eventRecord.ActivityId.ToString();
}