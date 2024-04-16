using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Core;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.Serialization.Csv;

public interface ICsvEventsSerializer : IEventsSerializer;

[AppComponent]
public class CsvEventsSerializer : ICsvEventsSerializer
{
  private const char Delimiter = '\t';


  public void SerializeEvents(IEnumerable<EventRecordWithMetadata> events, string path)
  {
    using var fs = File.OpenWrite(path);
    using var sw = new StreamWriter(fs);
    using var enumerator = events.GetEnumerator();

    if (!enumerator.MoveNext()) return;

    var firstEvent = enumerator.Current;
    sw.Write(SerializeCsvHeader(firstEvent));
    sw.Write(SerializeEventToCsvBytes(firstEvent));

    while (enumerator.MoveNext())
    {
      sw.Write(SerializeEventToCsvBytes(enumerator.Current));
    }
  }

  private static string SerializeCsvHeader(EventRecordWithMetadata firstEvent)
  {
    var sb = new StringBuilder();

    sb.Append(nameof(firstEvent.Stamp)).Append(Delimiter)
      .Append(nameof(firstEvent.EventName)).Append(Delimiter)
      .Append(nameof(firstEvent.ManagedThreadId)).Append(Delimiter)
      .Append(nameof(firstEvent.ActivityId)).Append(Delimiter);

    foreach (var (name, _) in firstEvent.Metadata)
    {
      sb.Append(name).Append(Delimiter);
    }

    sb.AppendNewLine();
    return sb.ToString();
  }

  private static string SerializeEventToCsvBytes(EventRecordWithMetadata @event)
  {
    var sb = new StringBuilder();

    sb.Append(@event.Stamp).Append(Delimiter)
      .Append(@event.EventName).Append(Delimiter)
      .Append(@event.ManagedThreadId).Append(Delimiter)
      .Append(@event.ActivityId).Append(Delimiter);

    foreach (var (_, value) in @event.Metadata)
    {
      sb.Append(StringBuilderExtensions.SerializeValue(value)).Append(Delimiter);
    }

    sb.AppendNewLine();
    return sb.ToString();
  }
}