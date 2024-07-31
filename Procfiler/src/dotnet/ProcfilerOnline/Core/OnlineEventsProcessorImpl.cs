using System.Text.RegularExpressions;
using Core.Events.EventRecord;
using Core.GlobalData;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Diagnostics.Tracing.Parsers;
using ProcfilerOnline.Commands;
using ProcfilerOnline.Core.Processors;
using ProcfilerOnline.Core.Updaters;

namespace ProcfilerOnline.Core;

public interface ISharedEventPipeStreamData : IGlobalData
{
  void UpdateMethodsInfo(long methodId, string fqn);
  void UpdateTypeIdsToNames(long typeId, string typeName);
  void UpdateSyncTimes(long qpcSyncTime, long qpcFreq, DateTime utcSyncTime);
}

public class SharedEventPipeStreamData : ISharedEventPipeStreamData
{
  private readonly Dictionary<long, string> myMethodIdsToFqns = new();
  private readonly Dictionary<long, string> myTypeIdsToNames = new();


  public long QpcSyncTime { get; private set; }
  public long QpcFreq { get; private set; }
  public DateTime UtcSyncTime { get; private set; }

  public IReadOnlyDictionary<long, string> TypeIdToNames => myTypeIdsToNames;
  public IReadOnlyDictionary<long, string> MethodIdToFqn => myMethodIdsToFqns;


  public void UpdateSyncTimes(long qpcSyncTime, long qpcFreq, DateTime utcSyncTime)
  {
    QpcFreq = qpcFreq;
    QpcSyncTime = qpcSyncTime;
    UtcSyncTime = utcSyncTime;
  }

  public void UpdateMethodsInfo(long methodId, string fqn) => myMethodIdsToFqns[methodId] = fqn;
  public void UpdateTypeIdsToNames(long typeId, string typeName) => myTypeIdsToNames[typeId] = typeName;
}

public class OnlineEventsProcessorImpl(
  IEnumerable<ITraceEventProcessor> processors,
  CollectEventsOnlineContext commandContext)
{
  public void Process(Stream eventPipeStream)
  {
    var source = new EventPipeEventSource(eventPipeStream);

    var globalData = new SharedEventPipeStreamData();

    new TplEtwProviderTraceEventParser(source).All += e => ProcessEvent(e, globalData);
    source.Clr.All += e => ProcessEvent(e, globalData);
    source.Dynamic.All += e => ProcessEvent(e, globalData);

    source.Process();
  }

  private void ProcessEvent(TraceEvent traceEvent, ISharedEventPipeStreamData globalData)
  {
    var eventRecord = new EventRecordWithMetadata(traceEvent, traceEvent.ThreadID, -1);

    var context = new EventProcessingContext
    {
      TraceEvent = traceEvent,
      SharedData = globalData,
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