using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesUint64Value(ulong value) : BxesValue<ulong>(value)
{
  public override TypeIds TypeId => TypeIds.U64;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}