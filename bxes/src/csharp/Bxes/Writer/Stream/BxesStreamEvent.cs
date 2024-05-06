using Bxes.Models.Domain;

namespace Bxes.Writer.Stream;

public abstract class BxesStreamEvent;

public sealed class BxesTraceVariantStartEvent(uint tracesCount, IList<AttributeKeyValue> metadata) : BxesStreamEvent
{
  public IList<AttributeKeyValue> Metadata { get; } = metadata;
  public uint TracesCount { get; } = tracesCount;
}

public sealed class BxesEventEvent<TEvent>(TEvent @event) : BxesStreamEvent
  where TEvent : IEvent
{
  public TEvent Event { get; set; } = @event;
}

public sealed class BxesKeyValueEvent(AttributeKeyValue metadataKeyValue)
  : BxesStreamEvent
{
  public AttributeKeyValue MetadataKeyValue { get; } = metadataKeyValue;
}

public sealed class BxesValueEvent(BxesValue value) : BxesStreamEvent
{
  public BxesValue Value { get; } = value;
}

public sealed class BxesLogMetadataClassifierEvent(BxesClassifier classifier) : BxesStreamEvent
{
  public BxesClassifier Classifier { get; } = classifier;
}

public sealed class BxesLogMetadataExtensionEvent(BxesExtension extension) : BxesStreamEvent
{
  public BxesExtension Extensions { get; } = extension;
}

public sealed class BxesLogMetadataPropertyEvent(AttributeKeyValue attribute) : BxesStreamEvent
{
  public AttributeKeyValue Attribute { get; } = attribute;
}

public sealed class BxesLogMetadataGlobalEvent(BxesGlobal global) : BxesStreamEvent
{
  public BxesGlobal Global { get; } = global;
}

public sealed class BxesRecalculateIndicesEvent : BxesStreamEvent;