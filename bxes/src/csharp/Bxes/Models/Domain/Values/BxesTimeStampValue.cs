using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesTimeStampValue(long nanoseconds) : BxesValue<long>(nanoseconds)
{
  public override TypeIds TypeId => TypeIds.Timestamp;

  public DateTime Timestamp { get; } = DateTime.UnixEpoch + TimeSpan.FromTicks(nanoseconds / 100);


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}