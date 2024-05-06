using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public enum SoftwareEventTypeValues : byte
{
  Unspecified = 0,
  Call = 1,
  Return = 2,
  Throws = 3,
  Handle = 4,
  Calling = 5,
  Returning = 6
}

public class BxesSoftwareEventTypeValue(SoftwareEventTypeValues values)
  : BxesValue<SoftwareEventTypeValues>(values), IReadableValue<BxesSoftwareEventTypeValue>
{
  public static BxesSoftwareEventTypeValue Parse(byte value) =>
    Enum.IsDefined(typeof(SoftwareEventTypeValues), value) switch
    {
      true => new BxesSoftwareEventTypeValue((SoftwareEventTypeValues)value),
      false => throw new IndexOutOfRangeException()
    };

  public static BxesSoftwareEventTypeValue ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues) =>
    Parse(reader.ReadByte());


  public override TypeIds TypeId => TypeIds.SoftwareEventType;

  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write((byte)Value);
  }

  public string ToStringValue() => values.ToString().ToLower();
}