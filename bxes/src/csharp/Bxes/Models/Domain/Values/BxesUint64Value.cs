using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesUint64Value(ulong value) : BxesValue<ulong>(value), IReadableValue<BxesUint64Value>
{
  public static BxesUint64Value ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
    return new BxesUint64Value(reader.ReadUInt64());
  }


  public override TypeIds TypeId => TypeIds.U64;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}