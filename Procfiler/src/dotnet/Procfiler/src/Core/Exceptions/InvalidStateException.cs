using Core.CommandLine;

namespace Procfiler.Core.Exceptions;

public class InvalidStateException(string message) : ProcfilerException(message);