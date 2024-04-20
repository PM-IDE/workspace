using Bxes.Models.Domain.Values;
using Bxes.Models.Domain.Values.Lifecycle;
using Bxes.Writer;

namespace Bxes.Models.Domain;

public abstract class BxesValue
{
  public abstract TypeIds TypeId { get; }
  public abstract void WriteTo(BxesWriteContext context);

  public static unsafe BxesValue Parse(BinaryReader reader, List<BxesValue> parsedValues)
  {
    var valuesOffset = reader.BaseStream.Position;

    var typeId = (TypeIds)reader.ReadByte();

    switch (typeId)
    {
      case TypeIds.Null:
        return BxesNullValue.Instance;
      case TypeIds.Bool:
        var value = reader.ReadByte() switch
        {
          0 => false,
          1 => true,
          var other => throw new ParseException(valuesOffset, $"Failed to parse bool, expected 1 or 0, got {other}")
        };

        return new BxesBoolValue(value);
      case TypeIds.I32:
        return new BxesInt32Value(reader.ReadInt32());
      case TypeIds.I64:
        return new BxesInt64Value(reader.ReadInt64());
      case TypeIds.U32:
        return new BxesUint32Value(reader.ReadUInt32());
      case TypeIds.F32:
        return new BxesFloat32Value(reader.ReadSingle());
      case TypeIds.U64:
        return new BxesUint64Value(reader.ReadUInt64());
      case TypeIds.F64:
        return new BxesFloat64Value(reader.ReadDouble());
      case TypeIds.Timestamp:
        return new BxesTimeStampValue(reader.ReadInt64());
      case TypeIds.String:
        var length = reader.ReadUInt64();
        var bytes = new byte[length];
        var read = reader.Read(bytes);
        if (read != bytes.Length)
        {
          var message = $"The string has not enough content byte, expected {length} got {read}";
          throw new ParseException(valuesOffset, message);
        }

        return new BxesStringValue(BxesConstants.BxesEncoding.GetString(bytes));
      case TypeIds.BrafLifecycle:
        return BrafLifecycle.Parse(reader.ReadByte());
      case TypeIds.StandardLifecycle:
        return StandardXesLifecycle.Parse(reader.ReadByte());
      case TypeIds.Artifact:
        var modelsCount = reader.ReadUInt32();
        var models = new List<BxesArtifactItem>();
        for (var i = 0; i < modelsCount; ++i)
        {
          var model = (BxesStringValue)parsedValues[(int)reader.ReadUInt32()];
          var instance = (BxesStringValue)parsedValues[(int)reader.ReadUInt32()];
          var transition = (BxesStringValue)parsedValues[(int)reader.ReadUInt32()];

          models.Add(new BxesArtifactItem
          {
            Model = model.Value,
            Instance = instance.Value,
            Transition = transition.Value
          });
        }

        return new BxesArtifactModelsListValue(models);
      case TypeIds.Drivers:
        var driversCount = reader.ReadUInt32();
        var drivers = new List<BxesDriver>();
        for (var i = 0; i < driversCount; ++i)
        {
          var amount = reader.ReadDouble();
          var nameIndex = reader.ReadUInt32();
          var typeIndex = reader.ReadUInt32();

          drivers.Add(new BxesDriver
          {
            Amount = amount,
            Name = ((BxesStringValue)parsedValues[(int)nameIndex]).Value,
            Type = ((BxesStringValue)parsedValues[(int)typeIndex]).Value
          });
        }

        return new BxesDriversListValue(drivers);
      case TypeIds.Guid:
        Span<byte> guidSpan = stackalloc byte[16];
        var readBytes = reader.Read(guidSpan);
        if (readBytes != guidSpan.Length)
        {
          throw new ParseException(valuesOffset, $"Failed to read guid, read {readBytes}, expected {guidSpan.Length}");
        }

        return new BxesGuidValue(new Guid(guidSpan));
      case TypeIds.SoftwareEventType:
        return BxesSoftwareEventTypeValue.Parse(reader.ReadByte());
    }

    throw new ParseException(valuesOffset, $"Failed to find type for type id {typeId}");
  }
}

public class ParseException(long offset, string message) : BxesException
{
  public override string Message { get; } = $"Failed to parse file at {offset}, {message}";
}

public abstract class BxesValue<TValue>(TValue value) : BxesValue
  where TValue : notnull
{
  public TValue Value { get; } = value;


  public override void WriteTo(BxesWriteContext context)
  {
    context.Writer.Write((byte)TypeId);
  }

  public override bool Equals(object? obj) =>
    obj is BxesValue<TValue> otherValue && EqualityComparer<TValue>.Default.Equals(Value, otherValue.Value);

  public override int GetHashCode() => Value.GetHashCode();

  public override string ToString() => Value.ToString();
}