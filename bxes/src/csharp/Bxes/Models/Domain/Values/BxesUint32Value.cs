using Bxes.Writer;

namespace Bxes.Models.Values;

public class BxesUint32Value(uint value) : BxesValue<uint>(value)
{
  public override TypeIds TypeId => TypeIds.U32;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}