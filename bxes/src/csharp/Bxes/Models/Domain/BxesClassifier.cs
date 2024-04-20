using Bxes.Models.Values;

namespace Bxes.Models;

public record BxesClassifier
{
    public required List<BxesStringValue> Keys { get; init; }
    public required BxesStringValue Name { get; init; }


    public virtual bool Equals(BxesClassifier? other) =>
        other is { } &&
        Name.Equals(other.Name) &&
        EventLogUtil.EqualsRegardingOrder(Keys, other.Keys);

    public override int GetHashCode()
    {
        var nameHash = Name.GetHashCode();
        foreach (var key in Keys.OrderBy(key => key.Value))
        {
            nameHash = HashCode.Combine(nameHash, key.GetHashCode());
        }

        return nameHash;
    }
}