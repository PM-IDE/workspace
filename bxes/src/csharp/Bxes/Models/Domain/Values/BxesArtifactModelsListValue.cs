using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.Models.Values;

public record BxesArtifactItem
{
  public required string Model { get; init; }
  public required string Instance { get; init; }
  public required string Transition { get; init; }
}

public class BxesArtifactModelsListValue(List<BxesArtifactItem> items) : BxesValue<List<BxesArtifactItem>>(items)
{
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