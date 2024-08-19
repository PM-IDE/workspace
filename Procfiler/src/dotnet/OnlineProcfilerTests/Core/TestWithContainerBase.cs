using Autofac;
using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core;
using ProcfilerOnline.Core.Container;
using ProcfilerOnline.Core.Handlers;
using TestsUtil;

namespace OnlineProcfilerTests.Core;

[FixtureLifeCycle(LifeCycle.InstancePerTestCase)]
public abstract class TestWithContainerBase
{
  public static IEnumerable<KnownSolution> AllSolutionsSource => KnownSolution.AllSolutions;


  protected abstract IEnumerable<IEventPipeStreamEventHandler> HandlersToRegister { get; }
  protected IContainer Container;


  [SetUp]
  public void SetUpContainer()
  {
    ExecuteBeforeContainerCreation();

    var assembly = typeof(ThreadsMethodsProcessor).Assembly;
    var builder = ProcfilerContainerBuilder.BuildFromAssembly(LogLevel.Trace, [assembly, typeof(ProgramEntryPoint).Assembly]);
    builder.RegisterInstance(TestLogger.CreateInstance()).As<IProcfilerLogger>();

    ConfigurationUtil.AddConfiguration(builder);

    foreach (var handler in HandlersToRegister)
    {
      builder.RegisterInstance(handler).As<IEventPipeStreamEventHandler>();
    }

    Container = builder.Build();
  }

  protected virtual void ExecuteBeforeContainerCreation()
  {
  }
}