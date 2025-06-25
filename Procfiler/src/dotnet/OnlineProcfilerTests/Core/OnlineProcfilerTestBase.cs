using System.Text.RegularExpressions;
using Autofac;
using Core.Builder;
using Core.Collector;
using ProcfilerOnline.Commands;
using ProcfilerOnline.Core;
using TestsUtil;

namespace OnlineProcfilerTests.Core;

public abstract class OnlineProcfilerTestBase : TestWithContainerBase
{
  protected ISharedEventPipeStreamData? ExecuteTest(KnownSolution solution)
  {
    var buildResult = Container.Resolve<IDotnetProjectBuilder>().TryBuildDotnetProject(solution.CreateProjectBuildInfo());
    if (buildResult is null)
    {
      Assert.Fail($"Failed to build {solution}");
      return null;
    }

    var dllPath = buildResult.Value.BuiltDllPath;
    var targetMethodsRegex = new Regex(solution.NamespaceFilterPattern);
    var baseContext = new BaseContext(
      targetMethodsRegex, targetMethodsRegex, ProvidersCategoryKind.CppProcfilerMethodsAndTasks, ulong.MaxValue, false);

    var context = new CollectEventsOnlineFromDllContext(dllPath, baseContext);

    return Container.Resolve<IClrOnlineEventsProcessor>().StartProfiling(context);
  }
}