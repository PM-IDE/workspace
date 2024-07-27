using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.Serialization.Core;

public interface IOnlineMethodsSerializer : IDisposable
{
  IReadOnlyList<string> AllMethodNames { get; }

  void SerializeThreadEvents(
    IEnumerable<EventRecordWithPointer> events,
    string filterPattern,
    InlineMode inlineMode);
}