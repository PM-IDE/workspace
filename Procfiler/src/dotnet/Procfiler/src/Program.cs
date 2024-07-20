using Autofac;
using Core.Container;
using Core.Utils;
using Procfiler.Core;
using Procfiler.Utils;

var builder = ProcfilerContainerBuilder.BuildFromAssembly(LogLevel.Information);
builder.RegisterType(typeof(ProcfilerLogger)).As<IProcfilerLogger>();

var container = builder.Build();
var rootCommand = new Command("procfiler");
var cmdBuilder = new CommandLineBuilder(rootCommand);

foreach (var command in container.Resolve<IEnumerable<IVisibleToUserCommand>>())
{
  rootCommand.AddCommand(command.CreateCommand());
}

cmdBuilder.UseDefaults();

var parser = cmdBuilder.Build();

using var cookie = new PerformanceCookie("Program", container.Resolve<IProcfilerLogger>());
parser.Invoke(args);