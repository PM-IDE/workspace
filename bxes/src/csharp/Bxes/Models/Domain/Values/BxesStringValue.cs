using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesStringValue(string value) : BxesValue<string>(value), IReadableValue<BxesStringValue>
{
  public static BxesStringValue ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
    var valuesOffset = reader.BaseStream.Position;

    var length = reader.ReadUInt64();

    var bytes = new byte[length];
    var read = reader.Read(bytes);
    if (read != bytes.Length)
    {
      var message = $"The string has not enough content byte, expected {length} got {read}";
      throw new ParseException(valuesOffset, message);
    }

    return new BxesStringValue(BxesConstants.BxesEncoding.GetString(bytes));
  }


  public override TypeIds TypeId => TypeIds.String;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);

    var bytes = BxesConstants.BxesEncoding.GetBytes(value);
    context.Writer.Write((ulong)bytes.Length);
    context.Writer.Write(bytes);
  }
}