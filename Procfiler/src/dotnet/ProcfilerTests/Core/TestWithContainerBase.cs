using System.Collections;
using Autofac;
using Microsoft.Extensions.Logging;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.Collector;
using Procfiler.Utils;
using Procfiler.Utils.Container;
using TestsUtil;

namespace ProcfilerTests.Core;

public record ContextWithSolution(KnownSolution Solution, CollectClrEventsFromExeContext Context);

public abstract class TestWithContainerBase
{
  protected static IEnumerable<ContextWithSolution> DefaultContexts() => 
    KnownSolution.AllSolutions.Select(s => new ContextWithSolution(s, s.CreateDefaultContext()));

  protected static IEnumerable<ContextWithSolution> OnlineSerializationContexts() => 
    KnownSolution.AllSolutions.Select(s => new ContextWithSolution(s, s.CreateOnlineSerializationContext()));

  protected static IEnumerable<ContextWithSolution> DefaultContextsWithFilter() => 
    KnownSolution.AllSolutions.Select(s => new ContextWithSolution(s, s.CreateContextWithFilter()));

  protected static IEnumerable<ContextWithSolution> OnlineSerializationContextsWithFilter() =>
    KnownSolution.AllSolutions.Select(s => new ContextWithSolution(s, s.CreateOnlineSerializationContextWithFilter()));


  protected readonly IContainer Container;


  protected TestWithContainerBase()
  {
    var assembly = typeof(IClrEventsCollector).Assembly;
    var builder = ProcfilerContainerBuilder.BuildFromAssembly(LogLevel.Trace, assembly);
    builder.RegisterInstance(TestLogger.CreateInstance()).As<IProcfilerLogger>();
    Container = builder.Build();
  }
}