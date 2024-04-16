using System.Diagnostics;
using System.Xml;
using Bxes.Models;
using Bxes.Models.Values;
using Bxes.Models.Values.Lifecycle;
using Bxes.Utils;

namespace Bxes.Xes.XesToBxes;

public readonly struct AttributeValueParseResult
{
  public required string Value { get; init; }
  public required BxesValue BxesValue { get; init; }


  public static AttributeValueParseResult Create(string value, BxesValue parsedValue) => new()
  {
    Value = value,
    BxesValue = parsedValue
  };
}

public readonly struct AttributeParseResult
{
  public required string? Key { get; init; }
  public required AttributeValueParseResult? Value { get; init; }

  public bool IsEmpty => Key is null && Value is null;


  public static AttributeParseResult Empty() => new()
  {
    Key = null,
    Value = null
  };

  public static AttributeParseResult KeyValue(string key, AttributeValueParseResult value) => new()
  {
    Key = key,
    Value = value
  };

  public static AttributeParseResult OnlyValue(AttributeValueParseResult value) => new()
  {
    Key = null,
    Value = value
  };
}

public static class XesReadUtil
{
  public static AttributeParseResult ParseAttribute(XmlReader reader, XesReadContext context)
  {
    var key = reader.GetAttribute(XesConstants.KeyAttributeName);
    var value = reader.GetAttribute(XesConstants.ValueAttributeName);

    if (key is null && value is null) return AttributeParseResult.Empty();

    if (key is { } && value is null) throw new XesReadException(reader, "Attribute contains key and no value");

    Debug.Assert(value is { });

    if (reader.Name is XesConstants.ListTagName)
    {
      switch (key)
      {
        case XesConstants.ArtifactMoves:
          var parseResult = AttributeValueParseResult.Create(value, ReadArtifact(reader, context));
          return AttributeParseResult.KeyValue(key, parseResult);
        case XesConstants.CostDrivers:
          throw new NotImplementedException();
        default:
          throw new XesReadException(reader, $"Failed to parse list {key}");
      }
    }

    if (key is XesConstants.LifecycleTransition)
    {
      var bxesLifecycle = (BxesValue)IEventLifecycle.Parse(value);
      var lifecycleParseResult = AttributeValueParseResult.Create(value, bxesLifecycle);
      return AttributeParseResult.KeyValue(key, lifecycleParseResult);
    }

    BxesValue bxesValue = reader.Name switch
    {
      XesConstants.StringTagName => new BxesStringValue(value),
      XesConstants.DateTagName => new BxesTimeStampValue((DateTimeOffset.Parse(value) - DateTimeOffset.UnixEpoch).Ticks * 100),
      XesConstants.IntTagName => new BxesInt64Value(long.Parse(value)),
      XesConstants.FloatTagName => new BxesFloat64Value(double.Parse(value)),
      XesConstants.BoolTagName => new BxesBoolValue(bool.Parse(value)),
      XesConstants.IdTagName => new BxesGuidValue(Guid.Parse(value)),
      _ => throw new XesReadException(reader, $"Failed to create value for type {reader.Name}")
    };

    var valueParseResult = AttributeValueParseResult.Create(value, bxesValue);
    return key switch
    {
      null => AttributeParseResult.OnlyValue(valueParseResult),
      { } => AttributeParseResult.KeyValue(key, valueParseResult)
    };
  }

  private static BxesArtifactModelsListValue ReadArtifact(XmlReader reader, XesReadContext context)
  {
    var items = new List<BxesArtifactItem>();
    while (reader.Read())
    {
      if (reader.NodeType is not XmlNodeType.Element || reader.Name != XesConstants.ValuesTagName) continue;

      while (reader.Read())
      {
        if (reader.NodeType is not XmlNodeType.Element || reader.Name != XesConstants.StringTagName) continue;

        if (reader.GetAttribute(XesConstants.ArtifactItemModel) is not { } model)
        {
          throw new XesReadException(reader, $"{XesConstants.ArtifactItemModel} was not specified");
        }

        string? instance = null;
        string? transition = null!;

        var subtreeReader = reader.ReadSubtree();

        while (subtreeReader.Read())
        {
          if (reader.NodeType is XmlNodeType.Element && reader.Name == XesConstants.StringTagName)
          {
            var parsedAttribute = ParseAttribute(reader, context);
            if (parsedAttribute is { Key: { }, Value: { } value })
            {
              switch (parsedAttribute.Key)
              {
                case XesConstants.ArtifactItemInstance:
                  instance = value.Value;
                  break;
                case XesConstants.ArtifactItemTransition:
                  transition = value.Value;
                  break;
              }
            }
            else
            {
              context.Logger.LogWarning(subtreeReader, "Failed to read artifact attribute");
            }
          }
        }

        if (instance is null || transition is null)
        {
          throw new XesReadException(reader,
            $"Expected not null instance and transition, got {instance}, {transition}");
        }

        items.Add(new BxesArtifactItem
        {
          Model = model,
          Instance = instance,
          Transition = transition
        });
      }

      break;
    }

    return new BxesArtifactModelsListValue(items);
  }
}