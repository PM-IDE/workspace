using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesFloat64Value(double value) : BxesValue<double>(value)
{
  public override TypeIds TypeId => TypeIds.F64;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}