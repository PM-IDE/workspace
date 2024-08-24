using System.Diagnostics;
using Microsoft.Extensions.Logging;

namespace Core.Utils;

public readonly struct PerformanceCookie : IDisposable
{
  private readonly string myActivityName;
  private readonly IProcfilerLogger myLogger;
  private readonly LogLevel myLogLevel;
  private readonly Stopwatch myStopwatch;
  private readonly ProcfilerLoggerIndentCookie myProcfilerLoggerIndentCookie;


  public PerformanceCookie(string activityName, IProcfilerLogger logger, LogLevel logLevel = LogLevel.Information)
  {
    myActivityName = activityName;
    myLogger = logger;
    myLogLevel = logLevel;
    myStopwatch = Stopwatch.StartNew();
    myLogger.Log(logLevel, "Started activity {Name}", activityName);
    myProcfilerLoggerIndentCookie = myLogger.CreateIndentCookie();
  }


  public void Dispose()
  {
    myStopwatch.Stop();
    myProcfilerLoggerIndentCookie.Dispose();
    var elapsed = myStopwatch.Elapsed.TotalMilliseconds;

    myLogger.Log(myLogLevel, "Activity {Name} finished in {Time}ms", myActivityName, elapsed);
  }
}