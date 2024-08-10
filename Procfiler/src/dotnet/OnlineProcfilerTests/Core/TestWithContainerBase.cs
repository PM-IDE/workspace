using Autofac;
using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Handlers;
using ProcfilerOnline.Core.Processors;
using TestsUtil;

namespace OnlineProcfilerTests.Core;

public abstract class TestWithContainerBase
{
  protected abstract IEnumerable<IEventPipeStreamEventHandler> HandlersToRegister { get; }
  protected readonly IContainer Container;


  protected TestWithContainerBase()
  {
    var assembly = typeof(SingleThreadMethodsProcessor).Assembly;
    var builder = ProcfilerContainerBuilder.BuildFromAssembly(LogLevel.Trace, [assembly, typeof(ProgramEntryPoint).Assembly]);
    builder.RegisterInstance(TestLogger.CreateInstance()).As<IProcfilerLogger>();

    foreach (var handler in HandlersToRegister)
    {
      builder.RegisterInstance(handler).As<IEventPipeStreamEventHandler>();
    }

    Container = builder.Build();
  }
}