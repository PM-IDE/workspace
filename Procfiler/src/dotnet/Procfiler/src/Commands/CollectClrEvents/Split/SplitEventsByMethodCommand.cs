using Core.CommandLine;
using Core.Container;
using Core.Events.EventRecord;
using Core.Events.EventsCollection;
using Core.Methods;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Base;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.EventsProcessing.Mutators;
using Procfiler.Core.Serialization.Bxes;
using Procfiler.Core.Serialization.Core;
using Procfiler.Core.Serialization.Xes;
using Procfiler.Core.SplitByMethod;
using Procfiler.Utils;

namespace Procfiler.Commands.CollectClrEvents.Split;

public interface ISplitEventsByMethodCommand : ICommandWithContext<CollectClrEventsContext>;

public enum InlineMode
{
  NotInline,

  OnlyEvents,
  EventsAndMethodsEvents,
  EventsAndMethodsEventsWithFilter
}

[CommandLineCommand]
public class SplitEventsByMethodCommand(
  ICommandExecutorDependantOnContext commandExecutor,
  IUndefinedThreadsEventsMerger undefinedThreadsEventsMerger,
  IUnitedEventsProcessor unitedEventsProcessor,
  IXesEventsSessionSerializer xesEventsSessionSerializer,
  IByMethodsSplitter splitter,
  IFullMethodNameBeautifier methodNameBeautifier,
  IProcfilerLogger logger,
  IManagedEventsFromUndefinedThreadExtractor managedEventsExtractor,
  IAsyncMethodsGrouper asyncMethodsGrouper,
  IProcfilerEventsFactory eventsFactory
) : CollectCommandBase(logger, commandExecutor), ISplitEventsByMethodCommand
{
  private Option<bool> GroupAsyncMethods { get; } =
    new("--group-async-methods", static () => true, "Group events from async methods");

  private Option<InlineMode> InlineInnerMethodsCalls { get; } =
    new("--inline", static () => InlineMode.NotInline, "Should we inline inner methods calls to all previous traces");

  private Option<string?> TargetMethodsRegex { get; } =
    new("--target-methods-regex", static () => null, "Target methods regex ");


  public override void Execute(CollectClrEventsContext context)
  {
    using var _ = new PerformanceCookie("SplittingEventsByMethods", Logger);

    var parseResult = context.CommonContext.CommandParseResult;
    var mergeUndefinedThreadEvents = parseResult.TryGetOptionValue(MergeFromUndefinedThreadOption);
    var directory = context.CommonContext.OutputPath;

    using var onlineSerializer = CreateOnlineSerializer(context);
    using var notStoringSerializer = CreateNotStoringSerializer(context);

    ExecuteCommand(context, events =>
    {
      var (allEvents, globalData) = events;
      var processingContext = EventsProcessingContext.DoEverything(allEvents, globalData);
      unitedEventsProcessor.ProcessFullEventLog(processingContext);

      var filterPattern = GetFilterPattern(context.CommonContext);
      var inlineInnerCalls = parseResult.TryGetOptionValue(InlineInnerMethodsCalls);
      var addAsyncMethods = parseResult.TryGetOptionValue(GroupAsyncMethods);

      var splitContext = new SplitContext(events, filterPattern, inlineInnerCalls, mergeUndefinedThreadEvents, addAsyncMethods);
      // ReSharper disable once AccessToDisposedClosure
      var asyncMethods = splitter.SplitNonAlloc(onlineSerializer, splitContext);

      if (asyncMethods is { })
      {
        foreach (var (methodName, traces) in asyncMethods)
        {
          var eventsByMethodsInvocation = PrepareEventSessionInfo(traces, globalData);
          var filePath = GetFileNameForMethod(directory, methodName);

          foreach (var (_, sessionInfo) in eventsByMethodsInvocation)
          {
            // ReSharper disable once AccessToDisposedClosure
            notStoringSerializer.WriteTrace(filePath, sessionInfo);
          }
        }
      }
    });
  }

  private INotStoringMergingTraceSerializer CreateNotStoringSerializer(CollectClrEventsContext context)
  {
    var writeAllMetadata = context.CommonContext.WriteAllEventMetadata;

    return context.CommonContext.LogSerializationFormat switch
    {
      LogFormat.Xes => new NotStoringMergingTraceXesSerializer(xesEventsSessionSerializer, logger, writeAllMetadata),
      LogFormat.Bxes => new NotStoringMergingTraceBxesSerializer(logger, writeAllMetadata),
      _ => throw new ArgumentOutOfRangeException()
    };
  }

  private IOnlineMethodsSerializer CreateOnlineSerializer(CollectClrEventsContext context)
  {
    var writeAllEventData = context.CommonContext.WriteAllEventMetadata;
    var directory = context.CommonContext.OutputPath;
    var targetMethodsRegexString = context.CommonContext.CommandParseResult.TryGetOptionValue(TargetMethodsRegex);
    var targetMethodsRegex = targetMethodsRegexString switch
    {
      { } => new Regex(targetMethodsRegexString),
      _ => null
    };

    return context.CommonContext.LogSerializationFormat switch
    {
      LogFormat.Bxes => new OnlineBxesMethodsSerializer(
        directory, targetMethodsRegex, methodNameBeautifier, eventsFactory, logger, writeAllEventData),
      LogFormat.Xes => new OnlineMethodsXesSerializer(
        directory, targetMethodsRegex, xesEventsSessionSerializer, methodNameBeautifier, eventsFactory, logger, writeAllEventData),
      _ => throw new ArgumentOutOfRangeException()
    };
  }

  private string GetFilterPattern(CollectingClrEventsCommonContext context)
  {
    if (context.CommandParseResult.TryGetOptionValue(FilterOption) is { } pattern) return pattern;

    return string.Empty;
  }

  private string GetFileNameForMethod(string directory, string methodName)
  {
    var fileName = methodNameBeautifier.Beautify(methodName);
    return Path.Combine(directory, $"{fileName}.xes");
  }

  private Dictionary<int, EventSessionInfo> PrepareEventSessionInfo(
    IEnumerable<IReadOnlyList<EventRecordWithMetadata>> traces,
    SessionGlobalData mergedGlobalData)
  {
    var index = 0;
    return traces.ToDictionary(
      _ => index++,
      values =>
      {
        var collection = new EventsCollectionImpl(values.ToArray(), Logger);
        return new EventSessionInfo(new[] { collection }, mergedGlobalData);
      });
  }

  protected override Command CreateCommandInternal()
  {
    const string CommandName = "split-by-methods";
    const string CommandDescription = "Splits the events by methods, in which they occured, and serializes to XES";

    var splitByMethodsCommand = new Command(CommandName, CommandDescription);

    splitByMethodsCommand.AddOption(RepeatOption);
    splitByMethodsCommand.AddOption(InlineInnerMethodsCalls);
    splitByMethodsCommand.AddOption(GroupAsyncMethods);
    splitByMethodsCommand.AddOption(TargetMethodsRegex);

    return splitByMethodsCommand;
  }
}