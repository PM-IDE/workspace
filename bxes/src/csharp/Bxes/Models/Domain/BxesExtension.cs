using Bxes.Models.Domain.Values;

namespace Bxes.Models.Domain;

public record BxesExtension
{
    public required BxesStringValue Prefix { get; init; }
    public required BxesStringValue Uri { get; init; }
    public required BxesStringValue Name { get; init; }


    public virtual bool Equals(BxesExtension? other) =>
        other is { } &&
        Prefix.Equals(other.Prefix) &&
        Uri.Equals(other.Uri) &&
        Name.Equals(other.Name);

    public override int GetHashCode() => HashCode.Combine(Prefix, Uri, Name);
}