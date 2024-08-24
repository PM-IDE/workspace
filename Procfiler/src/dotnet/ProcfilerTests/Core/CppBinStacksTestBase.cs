using Autofac;
using Core.Builder;
using Core.CppProcfiler;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler;
using Procfiler.Core.Processes;

namespace ProcfilerTests.Core;

public abstract class CppBinStacksTestBase : ProcessTestBase
{
  protected abstract bool UseMethodsFilter { get; }

  protected void DoTestWithPath(CollectClrEventsFromExeContext context, Action<string, CppProfilerMode> testAction)
  {
    var result = Container.Resolve<IDotnetProjectBuilder>().TryBuildDotnetProject(context.ProjectBuildInfo);
    Assert.That(result, Is.Not.Null);

    var locator = Container.Resolve<ICppProcfilerLocator>();
    var binStacksSavePathCreator = Container.Resolve<IBinaryStackSavePathCreator>();

    var dto = DotnetProcessLauncherDto.CreateFrom(
      context.CommonContext, result!.Value, locator, binStacksSavePathCreator);

    var launcher = Container.Resolve<IDotnetProcessLauncher>();
    var process = launcher.TryStartDotnetProcess(dto with { DefaultDiagnosticPortSuspend = false });
    Assert.That(process, Is.Not.Null);
    process!.WaitForExit();

    var binStacksPath = binStacksSavePathCreator.CreateSavePath(result.Value, context.CommonContext.CppProfilerMode);
    testAction(binStacksPath, context.CommonContext.CppProfilerMode);
  }

  protected void DoTestWithCollectedEvents(CollectClrEventsFromExeContext context, Action<CollectedEvents> testAction) =>
    StartProcessAndDoTest(context, testAction);
}