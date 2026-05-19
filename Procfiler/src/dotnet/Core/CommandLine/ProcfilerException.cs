namespace Core.CommandLine;

public abstract class ProcfilerException : Exception
{
  protected ProcfilerException()
  {
  }

  protected ProcfilerException(string? message) : base(message)
  {
  }
}