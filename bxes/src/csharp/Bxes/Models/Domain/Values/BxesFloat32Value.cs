using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesFloat32Value(float value) : BxesValue<float>(value), IReadableValue<BxesFloat32Value>
{
  public static BxesFloat32Value ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues) => new(reader.ReadSingle());


  public override TypeIds TypeId => TypeIds.F32;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}