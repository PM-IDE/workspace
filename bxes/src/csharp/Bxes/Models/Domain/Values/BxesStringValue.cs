using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesStringValue(string value) : BxesValue<string>(value)
{
  public override TypeIds TypeId => TypeIds.String;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);

    var bytes = BxesConstants.BxesEncoding.GetBytes(value);
    context.Writer.Write((ulong)bytes.Length);
    context.Writer.Write(bytes);
  }
}