using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.SplitByMethod;

namespace Procfiler.Core.Serialization.Core;

public abstract class OnlineMethodsSerializerBase<TState>(
  string outputDirectory,
  Regex? targetMethodsRegex,
  IFullMethodNameBeautifier methodNameBeautifier,
  IProcfilerEventsFactory factory,
  IProcfilerLogger logger,
  bool writeAllEventMetadata) : IOnlineMethodsSerializer where TState : class
{
  protected readonly string OutputDirectory = outputDirectory;
  protected readonly Regex? TargetMethodsRegex = targetMethodsRegex;
  protected readonly IFullMethodNameBeautifier FullMethodNameBeautifier = methodNameBeautifier;
  protected readonly IProcfilerEventsFactory Factory = factory;
  protected readonly IProcfilerLogger Logger = logger;
  protected readonly bool WriteAllEventMetadata = writeAllEventMetadata;

  protected readonly List<string> MethodNames = [];
  protected readonly Dictionary<string, TState> States = new();


  public IReadOnlyList<string> AllMethodNames => MethodNames;


  public void SerializeThreadEvents(IEnumerable<EventRecordWithPointer> events, string filterPattern, InlineMode inlineMode)
  {
    var splitter = new CallbackBasedSplitter<TState>(
      logger, events, filterPattern, inlineMode, TryCreateState, HandleUpdate);

    splitter.Split();
  }

  private TState? TryCreateState(EventRecordWithMetadata contextEvent)
  {
    var methodName = contextEvent.GetMethodStartEndEventInfo().Frame;
    if (TargetMethodsRegex is { } && !TargetMethodsRegex.IsMatch(methodName))
    {
      return null;
    }

    return TryCreateStateInternal(contextEvent);
  }

  protected abstract TState? TryCreateStateInternal(EventRecordWithMetadata contextEvent);
  protected abstract void HandleUpdate(EventUpdateBase<TState> update);

  public abstract void Dispose();
}