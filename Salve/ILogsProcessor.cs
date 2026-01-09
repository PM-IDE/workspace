namespace Salve;

internal interface ILogsProcessor : IDisposable
{
  void Initialize();
  void Process(string? line);
}