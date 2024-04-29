using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesInt64Value(long value) : BxesValue<long>(value), IReadableValue<BxesInt64Value>
{
  public static BxesInt64Value ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
    return new BxesInt64Value(reader.ReadInt64());
  }


  public override TypeIds TypeId => TypeIds.I64;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}