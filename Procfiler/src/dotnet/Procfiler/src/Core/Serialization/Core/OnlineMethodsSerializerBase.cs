using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.SplitByMethod;

namespace Procfiler.Core.Serialization.Core;

public abstract class OnlineMethodsSerializerBase<TState>(
  string outputDirectory,
  Regex? targetMethodsRegex,
  IFullMethodNameBeautifier methodNameBeautifier,
  IProcfilerEventsFactory factory,
  IProcfilerLogger logger,
  bool writeAllEventMetadata) : IOnlineMethodsSerializer
{
  protected readonly string OutputDirectory = outputDirectory;
  protected readonly Regex? TargetMethodsRegex = targetMethodsRegex;
  protected readonly IFullMethodNameBeautifier FullMethodNameBeautifier = methodNameBeautifier;
  protected readonly IProcfilerEventsFactory Factory = factory;
  protected readonly IProcfilerLogger Logger = logger;
  protected readonly bool WriteAllEventMetadata = writeAllEventMetadata;

  protected readonly Dictionary<string, TState> States = new();


  public abstract void HandleUpdate(EventUpdateBase update);

  public object? CreateState(EventRecordWithMetadata eventRecord)
  {
    var methodName = eventRecord.GetMethodStartEndEventInfo().Frame;
    if (TargetMethodsRegex is { } && !TargetMethodsRegex.IsMatch(methodName))
    {
      return null;
    }

    return TryCreateStateInternal(eventRecord);
  }

  protected abstract TState? TryCreateStateInternal(EventRecordWithMetadata contextEvent);

  public abstract void Dispose();
}