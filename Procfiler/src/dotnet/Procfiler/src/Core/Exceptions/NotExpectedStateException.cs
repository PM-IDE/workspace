namespace Procfiler.Core.Exceptions;

public class NotExpectedStateException(
  Type expectedType, Type actualType) : ProcfilerException($"Expected {expectedType.Name} but found {actualType.Name}");