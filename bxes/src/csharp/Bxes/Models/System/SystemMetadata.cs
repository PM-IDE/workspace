using Bxes.Models.Domain;
using Bxes.Utils;

namespace Bxes.Models.System;

public interface ISystemMetadata : IEquatable<ISystemMetadata>
{
    List<ValueAttributeDescriptor> ValueAttributeDescriptors { get; }
}

public class SystemMetadata : ISystemMetadata
{
    public static SystemMetadata Default { get; } = new();


    public List<ValueAttributeDescriptor> ValueAttributeDescriptors { get; } = [];


    public override int GetHashCode() => HashCode.Combine(ValueAttributeDescriptors.CalculateHashCode());

    public bool Equals(ISystemMetadata? other) =>
        other is SystemMetadata &&
        EventLogUtil.EqualsRegardingOrder(other.ValueAttributeDescriptors, ValueAttributeDescriptors);

    public override bool Equals(object? obj)
    {
        if (obj is not SystemMetadata other) return false;

        return Equals(other);
    }
}