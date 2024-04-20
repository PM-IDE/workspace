using System.Xml;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Writer;

namespace Bxes.Xes.XesToBxes;

public static class FromXesBxesEventFactory
{
  public static FromXesBxesEvent? CreateFrom(XmlReader reader, XesReadContext context)
  {
    var attributes = new Lazy<List<AttributeKeyValue>>(static () => new List<AttributeKeyValue>());
    var initializedName = false;
    var initializedTimestamp = false;

    string name = null!;
    long timestamp = 0;

    while (reader.Read())
    {
      if (reader.NodeType == XmlNodeType.EndElement) break;

      if (reader.NodeType == XmlNodeType.Element)
      {
        var parsedAttribute = XesReadUtil.ParseAttribute(reader, context);

        if (parsedAttribute is { Key: { } key, Value: { Value: { } value, BxesValue: { } bxesValue } })
        {
          switch (key)
          {
            case XesConstants.ConceptName:
              name = value;
              initializedName = true;
              break;
            case XesConstants.TimeTimestamp:
              timestamp = ((BxesTimeStampValue)bxesValue).Value;
              initializedTimestamp = true;
              break;
            default:
              attributes.Value.Add(new AttributeKeyValue(new BxesStringValue(key), bxesValue));
              break;
          }
        }
      }
    }

    if (!initializedName)
      TryInitializeFromDefaults(XesConstants.ConceptName, ref name, ref initializedName, context);

    if (!initializedTimestamp)
      TryInitializeFromDefaults(XesConstants.TimeTimestamp, ref timestamp, ref initializedTimestamp, context);

    return new FromXesBxesEvent
    {
      Timestamp = timestamp,
      Name = name,
      Attributes = attributes.IsValueCreated ? attributes.Value : ArraySegment<AttributeKeyValue>.Empty
    };
  }

  private static void TryInitializeFromDefaults<TValue>(
    string key, ref TValue value, ref bool initialized, XesReadContext context) where TValue : notnull
  {
    if (context.EventDefaults.TryGetValue(key, out var defaultValue) &&
        defaultValue is BxesValue<TValue> { Value: { } existingValue })
    {
      value = existingValue;
      initialized = true;
    }
  }
}