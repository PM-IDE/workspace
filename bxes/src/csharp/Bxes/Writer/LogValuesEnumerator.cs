using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.System;

namespace Bxes.Writer;

public readonly ref struct EventAttributes(int allAttributesCount)
{
    public required IReadOnlyList<AttributeKeyValue> ValueAttributes { get; init; }
    public required IEnumerable<AttributeKeyValue> DefaultAttributes { get; init; }


    public int DefaultAttributesCount => allAttributesCount - ValueAttributes.Count;


    public void Deconstruct(
        out IReadOnlyList<AttributeKeyValue> valueAttributes,
        out IEnumerable<AttributeKeyValue> defaultAttributes,
        out int defaultAttributesCount)
    {
        valueAttributes = ValueAttributes;
        defaultAttributes = DefaultAttributes;
        defaultAttributesCount = DefaultAttributesCount;
    }
}

public class AttributeNotFoundForDescriptorException(ValueAttributeDescriptor descriptor) : BxesException
{
    public override string Message { get; } = $"Failed to find attribute for descriptor {descriptor}";
}

public class LogValuesEnumerator(IReadOnlyList<ValueAttributeDescriptor> valuesAttributes)
{
    public static LogValuesEnumerator Default { get; } = new([]);


    private readonly HashSet<string> myValueAttributesNames = valuesAttributes.Select(d => d.Name).ToHashSet();


    public IReadOnlyList<ValueAttributeDescriptor> OrderedValueAttributes { get; } = 
        valuesAttributes.OrderBy(d => d.Name).ToList();

    
    public EventAttributes SplitEventAttributesOrThrow(IEvent @event)
    {
        if (OrderedValueAttributes.Count == 0)
        {
            return new EventAttributes(@event.Attributes.Count)
            {
                ValueAttributes = [],
                DefaultAttributes = @event.Attributes
            };
        }

        return new EventAttributes(@event.Attributes.Count)
        {
            ValueAttributes = ExtractValueAttributesOrThrow(@event),
            DefaultAttributes = @event.Attributes.Where(attr => !myValueAttributesNames.Contains(attr.Key.Value))
        };
    }

    //todo: allocations
    private List<AttributeKeyValue> ExtractValueAttributesOrThrow(IEvent @event)
    {
        if (OrderedValueAttributes.Count == 0) return [];

        var valuesAttributes = new List<AttributeKeyValue>();
        foreach (var descriptor in OrderedValueAttributes)
        {
            var foundAttribute = false;
            foreach (var attribute in @event.Attributes)
            {
                if (attribute.Key.Value == descriptor.Name && attribute.Value.TypeId == descriptor.TypeId)
                {
                    foundAttribute = true;
                    valuesAttributes.Add(attribute);
                    break;
                }
            }

            if (!foundAttribute)
            {
                var nullAttr = new AttributeKeyValue(new BxesStringValue(descriptor.Name), BxesNullValue.Instance);
                valuesAttributes.Add(nullAttr);
            }
        }

        return valuesAttributes;
    }
    
    public IEnumerable<BxesValue> EnumerateValues(IEventLog log)
    {
        foreach (var variant in log.Traces)
        {
            foreach (var value in EnumerateVariantValues(variant))
            {
                yield return value;
            }
        }
        
        foreach (var metadataValue in EnumerateMetadataValues(log.Metadata))
        {
            yield return metadataValue;
        }
    }

    public IEnumerable<AttributeKeyValue> EnumerateKeyValues(IEventLog log)
    {
        foreach (var variant in log.Traces)
        {
            foreach (var value in EnumerateVariantKeyValuePairs(variant))
            {
                yield return value;
            }
        }
        
        foreach (var metadataValue in EnumerateMetadataKeyValuePairs(log.Metadata))
        {
            yield return metadataValue;
        }
    }
    

    public IEnumerable<BxesValue> EnumerateMetadataValues(IEventLogMetadata metadata)
    {
        foreach (var (key, value) in metadata.Properties)
        {
            yield return key;
            yield return value;
        }

        foreach (var extension in metadata.Extensions)
        {
            yield return extension.Name;
            yield return extension.Prefix;
            yield return extension.Uri;
        }

        foreach (var classifier in metadata.Classifiers)
        {
            yield return classifier.Name;

            foreach (var key in classifier.Keys)
            {
                yield return key;
            }
        }

        foreach (var global in metadata.Globals)
        {
            foreach (var attribute in global.Globals)
            {
                yield return attribute.Key;
                yield return attribute.Value;
            }
        }
    }

    public IEnumerable<AttributeKeyValue> EnumerateMetadataKeyValuePairs(IEventLogMetadata metadata)
    {
        foreach (var pair in metadata.Properties)
        {
            yield return pair;
        }

        foreach (var global in metadata.Globals)
        {
            foreach (var attribute in global.Globals)
            {
                yield return attribute;
            }
        }
    }
    
    private IEnumerable<BxesValue> EnumerateVariantValues(ITraceVariant variant)
    {
        foreach (var pair in variant.Metadata)
        {
            yield return pair.Key;
            yield return pair.Value;
        }

        foreach (var @event in variant.Events)
        {
            foreach (var value in EnumerateEventValues(@event))
            {
                yield return value;
            }
        }
    }

    private IEnumerable<AttributeKeyValue> EnumerateVariantKeyValuePairs(ITraceVariant variant)
    {
        foreach (var pair in variant.Metadata)
        {
            yield return pair;
        }

        foreach (var @event in variant.Events)
        {
            foreach (var pair in EnumerateEventKeyValuePairs(@event))
            {
                yield return pair;
            }
        }
    }
    
    private IEnumerable<BxesValue> EnumerateEventValues(IEvent @event)
    {
        yield return new BxesStringValue(@event.Name);

        foreach (var (key, value) in EnumerateEventKeyValuePairs(@event))
        {
            yield return key;
            yield return value;
        }
    }

    private IEnumerable<AttributeKeyValue> EnumerateEventKeyValuePairs(IEvent @event) => 
        @event.Attributes.Where(attr => !myValueAttributesNames.Contains(attr.Key.Value));
}