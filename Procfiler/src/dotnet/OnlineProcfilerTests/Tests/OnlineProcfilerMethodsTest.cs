using System.Text.RegularExpressions;
using Core.Events.EventRecord;
using OnlineProcfilerTests.Core;
using ProcfilerOnline.Core;
using ProcfilerTests.Core;
using TestsUtil;

namespace OnlineProcfilerTests.Tests;

public abstract class OnlineProcfilerMethodsTest : OnlineProcfilerTestWithGold
{
  protected abstract string? Prefix { get; }

  protected abstract Dictionary<string, List<List<EventRecordWithMetadata>>> GetLoggedMethods(ISharedEventPipeStreamData sharedData);

  protected string DoExecuteTest(KnownSolution solution)
  {
    var sharedData = ExecuteTest(solution) ?? throw new Exception();

    var filter = new Regex(solution.NamespaceFilterPattern);

    return MethodsTestsUtil.SerializeToGold(GetLoggedMethods(sharedData), filter, Prefix, e =>
    {
      if (e.TryGetMethodDetails() is var (_, methodId))
      {
        return sharedData.MethodIdToFqn.GetValueOrDefault(methodId);
      }

      return null;
    }, trace => ProgramMethodCallTreeDumper.CreateDump(trace, filter.ToString(), e => e.TryGetMethodDetails() switch
    {
      var (_, id) => (sharedData.MethodIdToFqn[id], e.GetMethodEventKind() == MethodKind.Begin),
      _ => null
    }));
  }
}