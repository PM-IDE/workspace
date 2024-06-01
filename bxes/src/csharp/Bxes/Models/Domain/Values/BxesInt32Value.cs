using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesInt32Value(int value) : BxesValue<int>(value), IReadableValue<BxesInt32Value>
{
  public static BxesInt32Value ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues) => new(reader.ReadInt32());


  public override TypeIds TypeId => TypeIds.I32;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}