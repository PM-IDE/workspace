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
  CppProcfiler
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
        new(ClrTraceEventParser.ProviderName, EventLevel.Verbose, (long)ClrTraceEventParser.Keywords.All),
        new(SampleProfilerTraceEventParser.ProviderName, EventLevel.Verbose),
        new(TplEtwProviderTraceEventParser.ProviderName, EventLevel.Verbose, (long)TplEtwProviderTraceEventParser.Keywords.Default),
        new(ClrPrivateTraceEventParser.ProviderName, EventLevel.Verbose, ClrPrivateTraceEventParserKeywords),
        new(EventPipeProvidersNames.FrameworkEventSource, EventLevel.Verbose, FrameworkTraceEventParserKeywords),
        new(EventPipeProvidersNames.NetHttp, EventLevel.Verbose),
        new(EventPipeProvidersNames.NetSockets, EventLevel.Verbose),
        new(EventPipeProvidersNames.Runtime, EventLevel.Verbose),
        new(EventPipeProvidersNames.ArrayPoolSource, EventLevel.Verbose),
        new(nameof(MethodStartEndEventSource), EventLevel.LogAlways),
        new(EventPipeProvidersNames.ProcfilerCppProvider, EventLevel.LogAlways)
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
      ],
      [ProvidersCategoryKind.CppProcfiler] =
      [
        new(EventPipeProvidersNames.ProcfilerCppProvider, EventLevel.LogAlways),
        new(ClrTraceEventParser.ProviderName, EventLevel.Verbose, (long)ClrTraceEventParser.Keywords.All),
        new(TplEtwProviderTraceEventParser.ProviderName, EventLevel.Verbose, (long)TplEtwProviderTraceEventParser.Keywords.Default),
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