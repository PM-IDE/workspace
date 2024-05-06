using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesBoolValue(bool value) : BxesValue<bool>(value), IReadableValue<BxesBoolValue>
{
  public static BxesBoolValue ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
    var position = reader.BaseStream.Position;
    var value = reader.ReadByte() switch
    {
      0 => false,
      1 => true,
      var other => throw new ParseException(position, $"Failed to parse bool, expected 1 or 0, got {other}")
    };

    return new BxesBoolValue(value);
  }

  public override TypeIds TypeId => TypeIds.Bool;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write(Value);
  }
}