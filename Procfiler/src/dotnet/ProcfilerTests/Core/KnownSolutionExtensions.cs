using System.CommandLine;
using Core.Collector;
using Core.CppProcfiler;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Utils;
using TestsUtil;

namespace ProcfilerTests.Core;

public static class KnownSolutionExtensions
{
  public static CollectClrEventsFromExeContext CreateContextWithFilter(this KnownSolution solution) =>
    CreateContextInternal(solution, CreateDefaultContextWithFilter(solution));

  public static CollectClrEventsFromExeContext CreateDefaultContext(this KnownSolution solution) =>
    CreateContextInternal(solution, CreateDefaultCommonContext());

  public static CollectClrEventsFromExeContext CreateOnlineSerializationContext(this KnownSolution solution) =>
    CreateContextInternal(solution, CreateOnlineSerializationCommonContext());

  public static CollectClrEventsFromExeContext CreateOnlineSerializationContextWithFilter(this KnownSolution solution) =>
    CreateContextInternal(solution, CreateOnlineSerializationCommonContextWithFilter(solution));

  private static CollectClrEventsFromExeContext CreateContextInternal(
    KnownSolution knownSolution, CollectingClrEventsCommonContext context)
  {
    return new CollectClrEventsFromExeContext(knownSolution.CreateProjectBuildInfo(), context);
  }

  private static CollectingClrEventsCommonContext CreateDefaultCommonContext()
  {
    var serializationContext = new SerializationContext(FileFormat.Csv);
    return new CollectingClrEventsCommonContext(
      string.Empty, serializationContext, new TestParseResultsProvider(), string.Empty, ProvidersCategoryKind.All,
      false, 10_000, 10_000, false, null, 10_000, CppProfilerMode.SingleFileBinStack, false, false, true, false, LogFormat.Xes);
  }

  private static CollectingClrEventsCommonContext CreateOnlineSerializationCommonContext() =>
    CreateDefaultCommonContext() with
    {
      CppProfilerMode = CppProfilerMode.PerThreadBinStacksFilesOnline,
      UseDuringRuntimeFiltering = true
    };

  private static CollectingClrEventsCommonContext CreateDefaultContextWithFilter(KnownSolution solution) =>
    CreateDefaultCommonContext() with
    {
      CppProcfilerMethodsFilterRegex = solution.Name
    };

  private static CollectingClrEventsCommonContext CreateOnlineSerializationCommonContextWithFilter(KnownSolution solution) =>
    CreateOnlineSerializationCommonContext() with
    {
      CppProcfilerMethodsFilterRegex = solution.Name
    };
}

internal class TestParseResultsProvider : IParseResultInfoProvider
{
  public T? TryGetOptionValue<T>(Option<T> option) => default;
}