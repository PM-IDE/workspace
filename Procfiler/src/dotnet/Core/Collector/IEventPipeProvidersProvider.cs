using System.Diagnostics.Tracing;
using Core.Container;
using Microsoft.Diagnostics.NETCore.Client;
using Microsoft.Diagnostics.Tracing.EventPipe;
using Microsoft.Diagnostics.Tracing.Parsers;
using ProcfilerEventSources;

namespace Core.Collector;

public enum ProvidersCategoryKind
{
  All,
  Gc,
  GcAllocHigh,
  GcAllocLow,
  CppProcfilerMethodsAndTasks
}

public interface IEventPipeProvidersProvider
{
  IReadOnlyList<EventPipeProvider> GetProvidersFor(ProvidersCategoryKind category);
}

public static class EventPipeProvidersNames
{
  public const string FrameworkEventSource = "System.Diagnostics.Eventing.FrameworkEventSource";
  public const string NetHttp = "System.Net.Http";
  public const string NetSockets = "System.Net.Sockets";
  public const string Runtime = "System.Runtime";
  public const string ArrayPoolSource = "System.Buffers.ArrayPoolEventSource";
  public const string ProcfilerCppProvider = "ProcfilerCppEventPipeProvider";
}

[AppComponent]
public class EventPipeProvidersProviderImpl : IEventPipeProvidersProvider
{
  private static readonly IReadOnlyDictionary<ProvidersCategoryKind, EventPipeProvider[]> ourProvidersForCategories =
    new Dictionary<ProvidersCategoryKind, EventPipeProvider[]>
    {
      [ProvidersCategoryKind.All] =
      [
        new EventPipeProvider(ClrTraceEventParser.ProviderName, EventLevel.Verbose, (long)ClrTraceEventParser.Keywords.All),
        new EventPipeProvider(SampleProfilerTraceEventParser.ProviderName, EventLevel.Verbose),
        new EventPipeProvider(TplEtwProviderTraceEventParser.ProviderName, EventLevel.Verbose,
          (long)TplEtwProviderTraceEventParser.Keywords.Default),
        new EventPipeProvider(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, ClrPrivateTraceEventParserKeywords),
        new EventPipeProvider(EventPipeProvidersNames.FrameworkEventSource, EventLevel.Verbose, FrameworkTraceEventParserKeywords),
        new EventPipeProvider(EventPipeProvidersNames.NetHttp, EventLevel.Verbose),
        new EventPipeProvider(EventPipeProvidersNames.NetSockets, EventLevel.Verbose),
        new EventPipeProvider(EventPipeProvidersNames.Runtime, EventLevel.Verbose),
        new EventPipeProvider(EventPipeProvidersNames.ArrayPoolSource, EventLevel.Verbose),
        new EventPipeProvider(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new EventPipeProvider(EventPipeProvidersNames.ProcfilerCppProvider, EventLevel.LogAlways),
        new EventPipeProvider(nameof(ProcfilerBusinessEventsSource), EventLevel.LogAlways),
        new EventPipeProvider(nameof(OcelEventsSource), EventLevel.LogAlways)
      ],
      [ProvidersCategoryKind.Gc] =
      [
        new EventPipeProvider(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new EventPipeProvider(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, GcPrivateKeywords),
        new EventPipeProvider(ClrTraceEventParser.ProviderName, EventLevel.Verbose, GcKeywords),
        new EventPipeProvider(nameof(ProcfilerBusinessEventsSource), EventLevel.LogAlways)
      ],
      [ProvidersCategoryKind.GcAllocHigh] =
      [
        new EventPipeProvider(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new EventPipeProvider(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, GcPrivateKeywords),
        new EventPipeProvider(ClrTraceEventParser.ProviderName, EventLevel.Verbose, GcAllocHighKeywords),
        new EventPipeProvider(nameof(ProcfilerBusinessEventsSource), EventLevel.LogAlways)
      ],
      [ProvidersCategoryKind.GcAllocLow] =
      [
        new EventPipeProvider(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new EventPipeProvider(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, GcPrivateKeywords),
        new EventPipeProvider(ClrTraceEventParser.ProviderName, EventLevel.Verbose, GcAllocLowKeywords),
        new EventPipeProvider(nameof(ProcfilerBusinessEventsSource), EventLevel.LogAlways)
      ],
      [ProvidersCategoryKind.CppProcfilerMethodsAndTasks] =
      [
        new EventPipeProvider(EventPipeProvidersNames.ProcfilerCppProvider, EventLevel.LogAlways),
        new EventPipeProvider(ClrTraceEventParser.ProviderName, EventLevel.Verbose, (long)ClrTraceEventParser.Keywords.Jit),
        new EventPipeProvider(TplEtwProviderTraceEventParser.ProviderName, EventLevel.Verbose,
          (long)TplEtwProviderTraceEventParser.Keywords.Tasks)
      ]
    };


  private static long ClrPrivateTraceEventParserKeywords => (long)
  (
    ClrPrivateTraceEventParser.Keywords.GC |
    ClrPrivateTraceEventParser.Keywords.Binding |
    ClrPrivateTraceEventParser.Keywords.NGenForceRestore |
    ClrPrivateTraceEventParser.Keywords.Fusion |
    ClrPrivateTraceEventParser.Keywords.LoaderHeap |
    ClrPrivateTraceEventParser.Keywords.Security |
    ClrPrivateTraceEventParser.Keywords.Threading |
    ClrPrivateTraceEventParser.Keywords.MulticoreJit |
    ClrPrivateTraceEventParser.Keywords.PerfTrack |
    ClrPrivateTraceEventParser.Keywords.Stack |
    ClrPrivateTraceEventParser.Keywords.Startup
  );

  private static long FrameworkTraceEventParserKeywords => (long)
  (
    FrameworkEventSourceTraceEventParser.Keywords.Loader |
    FrameworkEventSourceTraceEventParser.Keywords.NetClient |
    FrameworkEventSourceTraceEventParser.Keywords.ThreadPool |
    FrameworkEventSourceTraceEventParser.Keywords.ThreadTransfer |
    FrameworkEventSourceTraceEventParser.Keywords.DynamicTypeUsage
  );

  private static long GcKeywords => (long)ClrTraceEventParser.Keywords.GCHeapSnapshot;

  private static long GcAllocHighKeywords => (long)
  (
    ClrTraceEventParser.Keywords.GCHeapSnapshot |
    ClrTraceEventParser.Keywords.GCSampledObjectAllocationHigh
  );

  private static long GcAllocLowKeywords => (long)
  (
    ClrTraceEventParser.Keywords.GCHeapSnapshot |
    ClrTraceEventParser.Keywords.GCSampledObjectAllocationLow
  );

  private static long GcPrivateKeywords => (long)ClrPrivateTraceEventParser.Keywords.GC;


  public IReadOnlyList<EventPipeProvider> GetProvidersFor(ProvidersCategoryKind category) =>
    ourProvidersForCategories[category];
}