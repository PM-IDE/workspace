using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesGuidValue(Guid guid) : BxesValue<Guid>(guid), IReadableValue<BxesGuidValue>
{
  public static unsafe BxesGuidValue ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
    var valuesOffset = reader.BaseStream.Position;

    Span<byte> guidSpan = stackalloc byte[16];

    var readBytes = reader.Read(guidSpan);
    if (readBytes != guidSpan.Length)
    {
      throw new ParseException(valuesOffset, $"Failed to read guid, read {readBytes}, expected {guidSpan.Length}");
    }

    return new BxesGuidValue(new Guid(guidSpan));
  }


  public override TypeIds TypeId => TypeIds.Guid;


  public override unsafe void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);

    Span<byte> guidBytes = stackalloc byte[16];
    Value.TryWriteBytes(guidBytes);

    context.Writer.Write(guidBytes);
  }
}