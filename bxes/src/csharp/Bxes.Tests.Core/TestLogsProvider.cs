using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.Domain.Values.Lifecycle;
using Bxes.Models.System;
using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.Tests.Core;

public static class TestLogsProvider
{
  public static IEventLog CreateSimpleTestLog()
  {
    var variants = new List<ITraceVariant>();
    var variantsCount = Random.Shared.Next(5, 10);
    for (var i = 0; i < variantsCount; ++i)
    {
      variants.Add(CreateRandomVariant());
    }

    return new InMemoryEventLog(1, GenerateRandomMetadata(), variants);
  }

  private static ITraceVariant CreateRandomVariant()
  {
    var eventsCount = Random.Shared.Next(1, 100);
    var events = new List<IEvent>();

    for (var i = 0; i < eventsCount; ++i)
    {
      events.Add(CreateRandomEvent());
    }

    return new TraceVariantImpl((uint)Random.Shared.Next(10000), events, GenerateRandomAttributes().ToList());
  }

  private static InMemoryEventImpl CreateRandomEvent() =>
    new(
      Random.Shared.Next(10123123),
      new BxesStringValue(GenerateRandomString()),
      GenerateRandomAttributes()
    );

  private static List<AttributeKeyValue> GenerateRandomAttributes()
  {
    var attributes = new List<AttributeKeyValue>();
    var attributesCount = Random.Shared.Next(1, 10);

    for (var i = 0; i < attributesCount; ++i)
    {
      attributes.Add(new AttributeKeyValue(new BxesStringValue(GenerateRandomString()), GenerateRandomBxesValue()));
    }

    return attributes;
  }

  private static IEventLogMetadata GenerateRandomMetadata()
  {
    var metadata = new EventLogMetadata();

    metadata.Properties.AddRange(GenerateRandomAttributes());
    metadata.Classifiers.AddRange(GenerateRandomClassifiers());
    metadata.Extensions.AddRange(GenerateRandomExtensions());
    metadata.Globals.AddRange(GenerateRandomGlobals());

    return metadata;
  }

  private static List<BxesClassifier> GenerateRandomClassifiers()
  {
    var classifiers = new List<BxesClassifier>();
    var classifiersCount = Random.Shared.Next(10);

    for (var i = 0; i > classifiersCount; ++i)
    {
      var keysCount = Random.Shared.Next(5);
      var keys = Enumerable.Range(0, keysCount).Select(_ => GenerateRandomBxesStringValue()).ToList();
      var classifier = new BxesClassifier
      {
        Name = GenerateRandomBxesStringValue(),
        Keys = keys
      };

      classifiers.Add(classifier);
    }

    return classifiers;
  }

  private static List<BxesExtension> GenerateRandomExtensions()
  {
    var extensions = new List<BxesExtension>();
    var extensionsCount = Random.Shared.Next(10);

    for (var i = 0; i < extensionsCount; ++i)
    {
      extensions.Add(new BxesExtension
      {
        Name = GenerateRandomBxesStringValue(),
        Prefix = GenerateRandomBxesStringValue(),
        Uri = GenerateRandomBxesStringValue()
      });
    }

    return extensions;
  }

  private static List<BxesGlobal> GenerateRandomGlobals()
  {
    var globals = new List<BxesGlobal>();
    var globalsCount = Random.Shared.Next(10);
    var kindValues = Enum.GetValues<GlobalsEntityKind>();
    var usedKinds = new HashSet<GlobalsEntityKind>();

    for (var i = 0; i < globalsCount; ++i)
    {
      var kind = kindValues[Random.Shared.Next(kindValues.Length)];
      if (!usedKinds.Add(kind)) continue;

      globals.Add(new BxesGlobal
      {
        Kind = kind,
        Globals = GenerateRandomAttributes().ToList()
      });
    }

    return globals;
  }

  private static BxesValue GenerateRandomBxesValue()
  {
    return GenerateRandomTypeId() switch
    {
      TypeIds.Null => new BxesStringValue(GenerateRandomString()),
      TypeIds.I32 => new BxesInt32Value(Random.Shared.Next(10000)),
      TypeIds.I64 => new BxesInt64Value(Random.Shared.Next(10000)),
      TypeIds.U32 => new BxesUint32Value((uint)Random.Shared.Next(10000)),
      TypeIds.U64 => new BxesUint64Value((ulong)Random.Shared.Next(10000)),
      TypeIds.F32 => new BxesFloat32Value((float)(Random.Shared.Next(10000) + Random.Shared.NextDouble())),
      TypeIds.F64 => new BxesFloat64Value(Random.Shared.Next(10000) + Random.Shared.NextDouble()),
      TypeIds.String => new BxesStringValue(GenerateRandomString()),
      TypeIds.Bool => new BxesBoolValue(GenerateRandomBool()),
      TypeIds.Timestamp => new BxesInt64Value(Random.Shared.Next(10000)),
      TypeIds.BrafLifecycle => new BrafLifecycle(GenerateRandomEnum<BrafLifecycleValues>()),
      TypeIds.StandardLifecycle => new StandardXesLifecycle(GenerateRandomEnum<StandardLifecycleValues>()),
      TypeIds.Artifact => GenerateRandomArtifact(),
      TypeIds.Drivers => GenerateRandomDrivers(),
      TypeIds.Guid => GenerateGuidValue(),
      TypeIds.SoftwareEventType => new BxesSoftwareEventTypeValue(GenerateRandomEnum<SoftwareEventTypeValues>()),
      _ => throw new ArgumentOutOfRangeException()
    };
  }

  private static TypeIds GenerateRandomTypeId() => (TypeIds)Random.Shared.Next(Enum.GetValues<TypeIds>().Length);

  private static BxesGuidValue GenerateGuidValue() => new(Guid.NewGuid());

  private static BxesDriversListValue GenerateRandomDrivers()
  {
    var artifactCount = Random.Shared.Next(100);
    var drivers = new List<BxesDriver>();

    for (var i = 0; i < artifactCount; ++i)
    {
      drivers.Add(GenerateRandomDriver());
    }

    return new BxesDriversListValue(drivers);
  }

  private static BxesDriver GenerateRandomDriver() =>
    new()
    {
      Amount = Random.Shared.NextDouble(),
      Name = GenerateRandomString(),
      Type = GenerateRandomString()
    };

  private static BxesArtifactModelsListValue GenerateRandomArtifact()
  {
    var artifactsCount = Random.Shared.Next(100);
    var models = new List<BxesArtifactItem>();

    for (var i = 0; i < artifactsCount; ++i)
    {
      models.Add(GenerateRandomArtifactItem());
    }

    return new BxesArtifactModelsListValue(models);
  }

  private static BxesArtifactItem GenerateRandomArtifactItem() =>
    new()
    {
      Model = GenerateRandomString(),
      Instance = GenerateRandomString(),
      Transition = GenerateRandomString()
    };

  private static bool GenerateRandomBool() => Random.Shared.Next(2) == 1;

  private static BxesStringValue GenerateRandomBxesStringValue() => new(GenerateRandomString());

  private static string GenerateRandomString()
  {
    var length = Random.Shared.Next(100);
    return new string(Enumerable.Range(0, length).Select(_ => GenerateRandomChar()).ToArray());
  }

  public static ISystemMetadata GenerateRandomSystemMetadata(IEventLog log)
  {
    var metadata = new SystemMetadata();
    var descriptors = new Dictionary<string, TypeIds>();
    var count = Random.Shared.Next(1, 10);
    for (var i = 0; i < count;)
    {
      var randomVariant = log.Traces[Random.Shared.Next(log.Traces.Count)];
      var randomEvent = randomVariant.Events[Random.Shared.Next(randomVariant.Events.Count)];
      if (randomEvent.Attributes.Count == 0) continue;

      var randomAttribute = randomEvent.Attributes[Random.Shared.Next(randomEvent.Attributes.Count)];
      if (descriptors.ContainsKey(randomAttribute.Key.Value)) continue;

      descriptors[randomAttribute.Key.Value] = randomAttribute.Value.TypeId;
      ++i;
    }

    metadata.ValueAttributeDescriptors.AddRange(
      descriptors.Select(d => new ValueAttributeDescriptor(d.Value, d.Key)).ToList());

    return metadata;
  }

  private static char GenerateRandomChar() => (char)('a' + Random.Shared.Next('z' - 'a' + 1));

  private static T GenerateRandomEnum<T>() where T : struct, Enum =>
    Enum.GetValues<T>()[Random.Shared.Next(Enum.GetValues<T>().Length)];
}