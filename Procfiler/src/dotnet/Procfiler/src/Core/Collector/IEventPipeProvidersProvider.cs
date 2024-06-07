using Procfiler.Utils.Container;
using ProcfilerEventSources;

namespace Procfiler.Core.Collector;

public enum ProvidersCategoryKind
{
  All,
  Gc,
  GcAllocHigh,
  GcAllocLow
}

public interface IEventPipeProvidersProvider
{
  IReadOnlyList<EventPipeProvider> GetProvidersFor(ProvidersCategoryKind category);
}

[AppComponent]
public class EventPipeProvidersProviderImpl : IEventPipeProvidersProvider
{
  private const string FrameworkEventSource = "System.Diagnostics.Eventing.FrameworkEventSource";
  private const string NetHttp = "System.Net.Http";
  private const string NetSockets = "System.Net.Sockets";
  private const string Runtime = "System.Runtime";
  private const string ArrayPoolSource = "System.Buffers.ArrayPoolEventSource";
  private const string ProcfilerCppProvider = "ProcfilerCppEventPipeProvider";


  private static readonly IReadOnlyDictionary<ProvidersCategoryKind, EventPipeProvider[]> ourProvidersForCategories =
    new Dictionary<ProvidersCategoryKind, EventPipeProvider[]>
    {
      [ProvidersCategoryKind.All] =
      [
        new(ClrTraceEventParser.ProviderName, EventLevel.Verbose, (long)ClrTraceEventParser.Keywords.All),
        new(SampleProfilerTraceEventParser.ProviderName, EventLevel.Verbose),
        new(TplEtwProviderTraceEventParser.ProviderName, EventLevel.Verbose, (long)TplEtwProviderTraceEventParser.Keywords.Default),
        new(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, ClrPrivateTraceEventParserKeywords),
        new(FrameworkEventSource, EventLevel.Verbose, FrameworkTraceEventParserKeywords),
        new(NetHttp, EventLevel.Verbose),
        new(NetSockets, EventLevel.Verbose),
        new(Runtime, EventLevel.Verbose),
        new(ArrayPoolSource, EventLevel.Verbose),
        new(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new(ProcfilerCppProvider, EventLevel.LogAlways)
      ],
      [ProvidersCategoryKind.Gc] =
      [
        new(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, GcPrivateKeywords),
        new(ClrTraceEventParser.ProviderName, EventLevel.Verbose, GcKeywords)
      ],
      [ProvidersCategoryKind.GcAllocHigh] =
      [
        new(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, GcPrivateKeywords),
        new(ClrTraceEventParser.ProviderName, EventLevel.Verbose, GcAllocHighKeywords)
      ],
      [ProvidersCategoryKind.GcAllocLow] =
      [
        new(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, GcPrivateKeywords),
        new(ClrTraceEventParser.ProviderName, EventLevel.Verbose, GcAllocLowKeywords)
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