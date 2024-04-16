using Bxes.Writer;

namespace Bxes.Models.Values;

public class BxesBoolValue(bool value) : BxesValue<bool>(value)
{
  public override TypeIds TypeId => TypeIds.Bool;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}