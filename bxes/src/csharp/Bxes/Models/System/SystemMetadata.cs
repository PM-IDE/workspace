using Bxes.Utils;

namespace Bxes.Models.System;

public interface ISystemMetadata : IEquatable<ISystemMetadata>
{
    List<ValueAttributeDescriptor> ValueAttributeDescriptors { get; }
}

public class SystemMetadata : ISystemMetadata
{
    public static SystemMetadata Default { get; } = new SystemMetadata
    {
        ValueAttributeDescriptors = []
    };


    public required List<ValueAttributeDescriptor> ValueAttributeDescriptors { get; init; }


    public override int GetHashCode()
    {
        return HashCode.Combine(
            ValueAttributeDescriptors.CalculateHashCode()
        );
    }

    public bool Equals(ISystemMetadata? other)
    {
        return other is { } &&
               other is SystemMetadata &&
               EventLogUtil.Equals(other.ValueAttributeDescriptors, ValueAttributeDescriptors);
    }

    public override bool Equals(object? obj)
    {
        if (obj is not SystemMetadata other) return false;

        return Equals(other);
    }
}