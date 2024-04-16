namespace Procfiler.Core.Constants.TraceEvents;

public static partial class TraceEventsConstants
{
  public const string BufferEventType = "Buffer";

  public const string BufferAllocated = "BufferAllocated";
  public const string BufferAllocationReason = "reason";

  public const string BufferRented = "BufferRented";
  public const string BufferReturned = "BufferReturned";
  public const string BufferTrimmed = "BufferTrimmed";
  public const string BufferTrimPoll = "BufferTrimPoll";
}