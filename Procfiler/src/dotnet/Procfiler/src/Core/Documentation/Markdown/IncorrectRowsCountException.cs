namespace Procfiler.Core.Documentation.Markdown;

public class IncorrectRowsCountException(int actualRowsCount, int expectedRowsCount)
  : Exception($"Expected {expectedRowsCount}, but got {actualRowsCount}");