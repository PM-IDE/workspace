using System.Text.RegularExpressions;
using Autofac;
using Core.Builder;
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
    var context = new CollectEventsOnlineContext(dllPath, targetMethodsRegex, targetMethodsRegex);

    return Container.Resolve<IClrOnlineEventsProcessor>().StartProfiling(context);
  }
}