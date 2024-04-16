namespace Bxes.Logging;

public interface ILogger
{
  void LogWarning(string message);
}

public readonly struct ConsoleForegroundCookie : IDisposable
{
  private readonly ConsoleColor myPreviousColor;


  public ConsoleForegroundCookie(ConsoleColor color)
  {
    myPreviousColor = Console.ForegroundColor;
    Console.ForegroundColor = color;
  }
  

  public void Dispose()
  {
    Console.ForegroundColor = myPreviousColor;
  }
}

public class BxesLogger : ILogger
{
  public void LogWarning(string message)
  {
    using var _ = new ConsoleForegroundCookie(ConsoleColor.DarkYellow);
    Console.WriteLine(message);
  }
}