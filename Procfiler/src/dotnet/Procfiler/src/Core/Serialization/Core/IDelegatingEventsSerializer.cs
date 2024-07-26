using Core.Container;
using Core.Events.EventRecord;
using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Csv;
using Procfiler.Core.Serialization.MethodsCallTree;
using Procfiler.Utils;

namespace Procfiler.Core.Serialization.Core;

public interface IDelegatingEventsSerializer
{
  void SerializeEvents(IEnumerable<EventRecordWithMetadata> events, string path, FileFormat fileFormat);
}

[AppComponent]
public class DelegatingEventsSerializer(
  ICsvEventsSerializer csvEventsSerializer,
  IMethodTreeEventSerializer treeEventSerializer
) : IDelegatingEventsSerializer
{
  public void SerializeEvents(
    IEnumerable<EventRecordWithMetadata> events, string path, FileFormat fileFormat)
  {
    switch (fileFormat)
    {
      case FileFormat.Csv:
        csvEventsSerializer.SerializeEvents(events, path);
        return;
      case FileFormat.MethodCallTree:
        treeEventSerializer.SerializeEvents(events, path);
        return;
      default:
        throw new ArgumentOutOfRangeException(nameof(fileFormat), fileFormat, null);
    }
  }
}