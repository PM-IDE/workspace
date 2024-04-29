using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public record BxesDriver
{
  public required double Amount { get; init; }
  public required string Name { get; init; }
  public required string Type { get; init; }
}

public class BxesDriversListValue(List<BxesDriver> drivers) 
  : BxesValue<List<BxesDriver>>(drivers), IReadableValue<BxesDriversListValue>
{
  public static BxesDriversListValue ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
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
  }
  

  public override TypeIds TypeId => TypeIds.Drivers;


  public override void WriteTo(BxesWriteContext context)
  {
    foreach (var driver in drivers)
    {
      BxesWriteUtils.WriteValueIfNeeded(new BxesStringValue(driver.Name), context);
      BxesWriteUtils.WriteValueIfNeeded(new BxesStringValue(driver.Type), context);
    }

    base.WriteTo(context);

    context.Writer.Write((uint)drivers.Count);

    foreach (var driver in drivers)
    {
      context.Writer.Write(driver.Amount);
      context.Writer.Write(context.ValuesIndices[new BxesStringValue(driver.Name)]);
      context.Writer.Write(context.ValuesIndices[new BxesStringValue(driver.Type)]);
    }
  }

  public override bool Equals(object? obj)
  {
    return obj is BxesDriversListValue other &&
           Value.Count == other.Value.Count &&
           other.Value.Zip(Value).All(pair => pair.First.Equals(pair.Second));
  }

  public override int GetHashCode() => Value.CalculateHashCode();
}