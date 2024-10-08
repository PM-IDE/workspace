using System.Runtime.Serialization;

namespace Core.CommandLine;

public abstract class ProcfilerException : Exception
{
  protected ProcfilerException()
  {
  }

  protected ProcfilerException(SerializationInfo info, StreamingContext context) : base(info, context)
  {
  }

  protected ProcfilerException(string? message) : base(message)
  {
  }

  protected ProcfilerException(string? message, Exception? innerException) : base(message, innerException)
  {
  }
}