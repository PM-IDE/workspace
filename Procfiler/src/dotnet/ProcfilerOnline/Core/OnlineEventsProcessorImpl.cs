﻿using Core.Collector;
using Core.Container;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;
using ProcfilerOnline.Core.Handlers;

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
  IProcfilerLogger logger,
  ICompositeEventPipeStreamEventHandler handler,
  ISharedEventPipeStreamData sharedData,
  string? targetMethodsRegex)
{
  private readonly Dictionary<int, ThreadEventsProcessor> myThreadsProcessors = new();


  public void Process(Stream eventPipeStream)
  {
    var source = new EventPipeEventSource(eventPipeStream);
    source.Clr.MethodLoadVerbose += traceEvent =>
    {
      sharedData.UpdateMethodsInfo((ulong)traceEvent.MethodID, traceEvent.MethodName);
    };

    source.Dynamic.All += traceEvent =>
    {
      if (traceEvent.ProviderName != EventPipeProvidersNames.ProcfilerCppProvider) return;

      myThreadsProcessors
        .GetOrCreate(traceEvent.ThreadID, () => new ThreadEventsProcessor(logger, handler, sharedData, traceEvent.ThreadID, targetMethodsRegex))
        .Process(traceEvent);
    };

    source.Process();
  }
}