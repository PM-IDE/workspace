using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;

namespace Bxes.Writer;

public class LogValuesEnumerator(HashSet<string> valuesAttributes)
{
    public static LogValuesEnumerator Default { get; } = new([]);


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
    
    IEnumerable<BxesValue> EnumerateEventValues(IEvent @event)
    {
        yield return new BxesStringValue(@event.Name);

        foreach (var (key, value) in @event.Attributes)
        {
            yield return key;
            yield return value;
        }
    }

    IEnumerable<AttributeKeyValue> EnumerateEventKeyValuePairs(IEvent @event) => @event.Attributes;
}