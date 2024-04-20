using Bxes.Writer;

namespace Bxes.Models;

public enum GlobalsEntityKind : byte
{
    Event = 0,
    Trace = 1,
    Log = 2
}

public record BxesGlobal
{
    public required GlobalsEntityKind Kind { get; init; }
    public required List<AttributeKeyValue> Globals { get; init; }

    public virtual bool Equals(BxesGlobal? other) =>
        other is { } &&
        other.Kind == Kind &&
        EventLogUtil.EqualsRegardingOrder(Globals, other.Globals);

    public override int GetHashCode()
    {
        var kindHash = Kind.GetHashCode();
        foreach (var kv in Globals.OrderBy(key => key.Key.Value))
        {
            kindHash = HashCode.Combine(kv.GetHashCode(), kindHash);
        }

        return kindHash;
    }
}