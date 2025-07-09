using Core.Events.EventRecord;
using Procfiler.Core.SplitByMethod;

namespace Procfiler.Core.Serialization.Core;

public interface IOnlineMethodsSerializer : IDisposable
{
  object? CreateState(EventRecordWithMetadata eventRecord);
  void HandleUpdate(EventUpdateBase update);
}