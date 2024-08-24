namespace Core.Constants.TraceEvents;

public static partial class TraceEventsConstants
{
  public const string RequestStart = "Request/Start";
  public const string RequestStop = "Request/Stop";
  public const string RequestFailed = "Request/Failed";
  public const string ConnectionEstablished = "Connection/Established";
  public const string ConnectionClosed = "Connection/Closed";
  public const string RequestLeftQueue = "RequestLeftQueue";
  public const string RequestHeadersStart = "RequestHeaders/Start";
  public const string RequestHeadersStop = "RequestHeaders/Stop";
  public const string RequestContentStart = "RequestContent/Start";
  public const string RequestContentStop = "RequestContent/Stop";
  public const string ResponseContentStart = "ResponseContent/Start";
  public const string ResponseContentStop = "ResponseContent/Stop";
  public const string ResponseHeadersStart = "ResponseHeaders/Start";
  public const string ResponseHeadersStop = "ResponseHeaders/Stop";

  public const string HttpRequestActivityBasePart = "HttpRequest";
}