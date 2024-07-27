using System.Text.RegularExpressions;
using Core.Container;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;
using ProcfilerOnline.Commands;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core;

public interface ISharedEventPipeStreamData
{
  string? FindMethodFqn(ulong methodId);
  void UpdateMethodsInfo(ulong methodId, string fqn);
}

[AppComponent]
public class SharedEventPipeStreamData : ISharedEventPipeStreamData
{
  private readonly Dictionary<ulong, string> myIdsToMethodsFqns = new();


  public string? FindMethodFqn(ulong methodId) => ((IDictionary<ulong, string>)myIdsToMethodsFqns).GetValueOrDefault(methodId);

  public void UpdateMethodsInfo(ulong methodId, string fqn) => myIdsToMethodsFqns[methodId] = fqn;
}

public class OnlineEventsProcessorImpl(
  IEnumerable<ITraceEventProcessor> processors,
  CollectEventsOnlineContext commandContext)
{
  public void Process(Stream eventPipeStream)
  {
    var source = new EventPipeEventSource(eventPipeStream);

    source.Clr.All += ProcessEvent;
    source.Dynamic.All += ProcessEvent;

    source.Process();
  }

  private void ProcessEvent(TraceEvent traceEvent)
  {
    var context = new EventProcessingContext
    {
      Event = traceEvent,
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