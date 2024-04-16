namespace Procfiler.Core.Exceptions;

public class MissingOptionException(Option option) : ProcfilerException($"Missing option {option.Name}");

public class OneOfFollowingOptionsMustBeSpecifiedException(params Option[] options)
  : Exception($"One of following options must be specified: {string.Join(", ", options.Select(o => o.Name))}");