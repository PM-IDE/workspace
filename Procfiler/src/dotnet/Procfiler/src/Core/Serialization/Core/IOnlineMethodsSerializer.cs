using Core.Events.EventsCollection;
using Procfiler.Commands.CollectClrEvents.Split;

namespace Procfiler.Core.Serialization.Core;

public interface IOnlineMethodsSerializer : IDisposable
{
  IReadOnlyList<string> AllMethodNames { get; }

  void SerializeThreadEvents(
    IEnumerable<EventRecordWithPointer> events,
    string filterPattern,
    InlineMode inlineMode);
}