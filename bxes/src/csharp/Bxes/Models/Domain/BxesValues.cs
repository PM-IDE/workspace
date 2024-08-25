using Bxes.Models.Domain.Values;
using Bxes.Models.Domain.Values.Lifecycle;
using Bxes.Writer;

namespace Bxes.Models.Domain;

public abstract class BxesValue
{
  public abstract TypeIds TypeId { get; }
  public abstract void WriteTo(BxesWriteContext context);

  public static BxesValue Parse(BinaryReader reader, List<BxesValue> parsedValues)
  {
    var valuesOffset = reader.BaseStream.Position;

    var typeId = (TypeIds)reader.ReadByte();

    return typeId switch
    {
      TypeIds.Null => BxesNullValue.Instance,
      TypeIds.Bool => BxesBoolValue.ReadPureValue(reader, parsedValues),
      TypeIds.I32 => BxesInt32Value.ReadPureValue(reader, parsedValues),
      TypeIds.I64 => BxesInt64Value.ReadPureValue(reader, parsedValues),
      TypeIds.U32 => BxesUint32Value.ReadPureValue(reader, parsedValues),
      TypeIds.F32 => BxesFloat32Value.ReadPureValue(reader, parsedValues),
      TypeIds.U64 => BxesUint64Value.ReadPureValue(reader, parsedValues),
      TypeIds.F64 => BxesFloat64Value.ReadPureValue(reader, parsedValues),
      TypeIds.Timestamp => BxesTimeStampValue.ReadPureValue(reader, parsedValues),
      TypeIds.String => BxesStringValue.ReadPureValue(reader, parsedValues),
      TypeIds.BrafLifecycle => BrafLifecycle.ReadPureValue(reader, parsedValues),
      TypeIds.StandardLifecycle => StandardXesLifecycle.ReadPureValue(reader, parsedValues),
      TypeIds.Artifact => BxesArtifactModelsListValue.ReadPureValue(reader, parsedValues),
      TypeIds.Drivers => BxesDriversListValue.ReadPureValue(reader, parsedValues),
      TypeIds.Guid => BxesGuidValue.ReadPureValue(reader, parsedValues),
      TypeIds.SoftwareEventType => BxesSoftwareEventTypeValue.ReadPureValue(reader, parsedValues),
      _ => throw new ParseException(valuesOffset, $"Failed to find type for type id {typeId}")
    };
  }
}

public class ParseException(long offset, string message) : BxesException
{
  public override string Message { get; } = $"Failed to parse file at {offset}, {message}";
}

public interface IReadableValue<out TSelf> where TSelf : IReadableValue<TSelf>
{
  static abstract TSelf ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues);
}

public abstract class BxesValue<TValue>(TValue value) : BxesValue where TValue : notnull
{
  public TValue Value { get; } = value;


  public override void WriteTo(BxesWriteContext context) => context.Writer.Write((byte)TypeId);

  public override bool Equals(object? obj) =>
    obj is BxesValue<TValue> otherValue && EqualityComparer<TValue>.Default.Equals(Value, otherValue.Value);

  public override int GetHashCode() => Value.GetHashCode();

  public override string ToString() => Value.ToString();
}