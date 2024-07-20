using System.CommandLine;
using System.CommandLine.Invocation;

namespace Core.CommandLine;

public interface IVisibleToUserCommand : ICommandHandler
{
  Command CreateCommand();
}

public interface ICommandWithContext<in TContext> : IVisibleToUserCommand
{
  void Execute(TContext context);
}