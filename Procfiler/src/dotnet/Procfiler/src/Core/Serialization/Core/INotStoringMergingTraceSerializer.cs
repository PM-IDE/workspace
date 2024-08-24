using Core.Utils;
using Procfiler.Core.Collector;

namespace Procfiler.Core.Serialization.Core;

public interface INotStoringMergingTraceSerializer : IDisposable
{
  void WriteTrace(string path, EventSessionInfo sessionInfo);
}

public abstract class NotStoringMergingTraceSerializerBase<TState>(IProcfilerLogger logger, bool writeAllEventData)
  : INotStoringMergingTraceSerializer
{
  protected readonly IProcfilerLogger Logger = logger;
  protected readonly bool WriteAllEventData = writeAllEventData;
  protected readonly Dictionary<string, TState> States = new();

  public abstract void WriteTrace(string path, EventSessionInfo sessionInfo);
  public abstract void Dispose();
}