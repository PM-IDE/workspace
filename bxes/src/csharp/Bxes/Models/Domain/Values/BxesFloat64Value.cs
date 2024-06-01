using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesFloat64Value(double value) : BxesValue<double>(value), IReadableValue<BxesFloat64Value>
{
  public static BxesFloat64Value ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues) => new(reader.ReadDouble());


  public override TypeIds TypeId => TypeIds.F64;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}