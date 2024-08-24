using Core.CommandLine;

namespace Core.Exceptions;

public class InvalidStateException(string message) : ProcfilerException(message);