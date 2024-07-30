using System.Text.RegularExpressions;
using Core.Container;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Diagnostics.Tracing.Parsers;
using Procfiler.Core.EventRecord.EventRecord;
using ProcfilerOnline.Commands;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core;

public interface ISharedEventPipeStreamData
{
  string? FindMethodFqn(long methodId);
  void UpdateMethodsInfo(long methodId, string fqn);
}

[AppComponent]
public class SharedEventPipeStreamData : ISharedEventPipeStreamData
{
  private readonly Dictionary<long, string> myIdsToMethodsFqns = new();


  public string? FindMethodFqn(long methodId) => ((IDictionary<long, string>)myIdsToMethodsFqns).GetValueOrDefault(methodId);

  public void UpdateMethodsInfo(long methodId, string fqn) => myIdsToMethodsFqns[methodId] = fqn;
}

public class OnlineEventsProcessorImpl(
  IEnumerable<ITraceEventProcessor> processors,
  CollectEventsOnlineContext commandContext)
{
  public void Process(Stream eventPipeStream)
  {
    var source = new EventPipeEventSource(eventPipeStream);

    new TplEtwProviderTraceEventParser(source).All += ProcessEvent;
    source.Clr.All += ProcessEvent;
    source.Dynamic.All += ProcessEvent;

    source.Process();
  }

  private void ProcessEvent(TraceEvent traceEvent)
  {
    var eventRecord = new EventRecordWithMetadata(traceEvent, traceEvent.ThreadID, -1);
    var context = new EventProcessingContext
    {
      Event = eventRecord,
      CommandContext = new CommandContext
      {
        TargetMethodsRegex = commandContext.TargetMethodsRegex is { } ? new Regex(commandContext.TargetMethodsRegex) : null
      }
    };

    foreach (var sharedDataUpdater in processors.OfType<ISharedDataUpdater>())
    {
      sharedDataUpdater.Process(context);
    }

    foreach (var processor in processors.Where(p => p is not ISharedDataUpdater))
    {
      processor.Process(context);
    }
  }
}