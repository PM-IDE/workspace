using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesFloat32Value(float value) : BxesValue<float>(value)
{
  public override TypeIds TypeId => TypeIds.F32;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}