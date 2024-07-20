using Core.Container;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Base;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core;
using Procfiler.Core.Collector;
using Procfiler.Core.EventsCollection;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.EventsProcessing.Mutators;
using Procfiler.Core.Serialization.Core;
using Procfiler.Core.SplitByMethod;
using Procfiler.Utils;

namespace Procfiler.Commands.CollectClrEvents.Split;

public interface ISplitEventsByManagedThreadIdCommand : ICommandWithContext<CollectClrEventsContext>;

[CommandLineCommand]
public class SplitEventsByManagedThreadId(
  IManagedEventsFromUndefinedThreadExtractor managedEventsExtractor,
  ICommandExecutorDependantOnContext commandExecutor,
  IUnitedEventsProcessor processor,
  IUndefinedThreadsEventsMerger eventsMerger,
  IStackTraceSerializer stackTraceSerializer,
  IDelegatingEventsSerializer serializer,
  IProcfilerLogger logger
) : CollectAndSplitCommandBase<long>(logger, commandExecutor, eventsMerger, processor, serializer, stackTraceSerializer),
  ISplitEventsByManagedThreadIdCommand
{
  public override void Execute(CollectClrEventsContext context)
  {
    var parseResult = context.CommonContext.CommandParseResult;
    var collectAndSplitContext = CollectAndSplitContext.DoEverything with
    {
      MergeFromUndefinedThread = parseResult.TryGetOptionValue(MergeFromUndefinedThreadOption)
    };

    ExecuteSimpleSplitCommand(context, SplitEventsHelper.ManagedThreadIdExtractor, collectAndSplitContext);
  }

  protected override KeyValuePair<long, IEventsCollection?>? FindEventsForUndefinedThread(
    Dictionary<long, IEventsCollection> eventsByKey)
  {
    var value = DictionaryExtensions.GetValueOrDefault(eventsByKey, -1);
    return new KeyValuePair<long, IEventsCollection?>(-1, value);
  }

  protected override IEventsCollection? AddNewManagedThreadsFromUndefined(
    IDictionary<long, IEventsCollection> events,
    SessionGlobalData globalData,
    IEventsCollection? undefinedEvents)
  {
    if (undefinedEvents is null) return undefinedEvents;

    UnitedEventsProcessor.ApplyMultipleMutators(undefinedEvents, globalData, EmptyCollections<Type>.EmptySet);
    return managedEventsExtractor.Extract(events, undefinedEvents);
  }

  protected override Command CreateCommandInternal() =>
    new("split-by-threads", "Split the events into different files based on managed thread ID");
}