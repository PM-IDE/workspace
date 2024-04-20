using Bxes.Writer;

namespace Bxes.Models.Values;

public class BxesInt32Value(int value) : BxesValue<int>(value)
{
  public override TypeIds TypeId => TypeIds.I32;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}