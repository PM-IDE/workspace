using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesUint32Value(uint value) : BxesValue<uint>(value), IReadableValue<BxesUint32Value>
{
  public static BxesUint32Value ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
    return new BxesUint32Value(reader.ReadUInt32());
  }


  public override TypeIds TypeId => TypeIds.U32;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}