using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesInt64Value(long value) : BxesValue<long>(value)
{
  public override TypeIds TypeId => TypeIds.I64;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}