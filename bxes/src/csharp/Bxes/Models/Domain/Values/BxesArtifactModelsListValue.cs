using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public record BxesArtifactItem
{
  public required string Model { get; init; }
  public required string Instance { get; init; }
  public required string Transition { get; init; }
}

public class BxesArtifactModelsListValue(List<BxesArtifactItem> items) 
  : BxesValue<List<BxesArtifactItem>>(items), IReadableValue<BxesArtifactModelsListValue>
{
  public static BxesArtifactModelsListValue ReadPureValue(BinaryReader reader, IReadOnlyList<BxesValue> parsedValues)
  {
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
  }


  public override TypeIds TypeId => TypeIds.Artifact;

  public override void WriteTo(BxesWriteContext context)
  {
    foreach (var item in items)
    {
      BxesWriteUtils.WriteValueIfNeeded(new BxesStringValue(item.Model), context);
      BxesWriteUtils.WriteValueIfNeeded(new BxesStringValue(item.Instance), context);
      BxesWriteUtils.WriteValueIfNeeded(new BxesStringValue(item.Transition), context);
    }

    base.WriteTo(context);

    context.Writer.Write((uint)items.Count);

    foreach (var item in items)
    {
      context.Writer.Write(context.ValuesIndices[new BxesStringValue(item.Model)]);
      context.Writer.Write(context.ValuesIndices[new BxesStringValue(item.Instance)]);
      context.Writer.Write(context.ValuesIndices[new BxesStringValue(item.Transition)]);
    }
  }

  public override bool Equals(object? obj)
  {
    return obj is BxesArtifactModelsListValue other &&
           Value.Count == other.Value.Count &&
           other.Value.Zip(Value).All(pair => pair.First.Equals(pair.Second));
  }

  public override int GetHashCode() => Value.CalculateHashCode();
}