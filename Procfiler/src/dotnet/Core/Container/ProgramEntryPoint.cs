using System.CommandLine;
using System.CommandLine.Builder;
using System.CommandLine.Parsing;
using System.Reflection;
using Autofac;
using Core.CommandLine;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.Container;

public static class ProgramEntryPoint
{
  public static void SetupContainerAndRun(string toplevelCommand, string[] args)
  {
    List<Assembly> assemblies = [Assembly.GetEntryAssembly()!, typeof(ProgramEntryPoint).Assembly];
    var builder = ProcfilerContainerBuilder.BuildFromAssembly(LogLevel.Information, assemblies);
    builder.RegisterType(typeof(ProcfilerLogger)).As<IProcfilerLogger>();

    var container = builder.Build();
    var rootCommand = new Command(toplevelCommand);
    var cmdBuilder = new CommandLineBuilder(rootCommand);

    foreach (var command in container.Resolve<IEnumerable<IVisibleToUserCommand>>())
    {
      rootCommand.AddCommand(command.CreateCommand());
    }

    cmdBuilder.UseDefaults();

    var parser = cmdBuilder.Build();

    using var cookie = new PerformanceCookie($"Program::{toplevelCommand}", container.Resolve<IProcfilerLogger>());
    parser.Invoke(args);
  }
}