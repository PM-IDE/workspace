using Core.Utils;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators;

public static class MutatorsUtil
{
  public static string CreateUnknownEventNamePartAndLog(string name, IProcfilerLogger logger)
  {
    logger.LogError("There is no suitable transformation for {Reason}", name);
    return TraceEventsConstants.Undefined;
  }

  public static void LogAbsenceOfMetadata(this IProcfilerLogger logger, string eventName, string metadataName) =>
    logger.LogError("The {MetadataName} was not present at {EventName}", metadataName, eventName);

  public static void DevastateMetadataValuesAndAppendToName(
    IProcfilerLogger logger,
    EventRecordWithMetadata eventRecord,
    IEnumerable<MetadataKeysWithTransform> keysWithTransformation,
    bool removeFromProperties = true)
  {
    var orderedTransforms = keysWithTransformation.OrderBy(transform => transform.EventClassKind).ToList();
    if (orderedTransforms.Count == 0) return;

    var sb = new StringBuilder();
    sb.Append(eventRecord.EventName);

    void TryAppendMetadata(string metadataKey, Func<string, string> transformation)
    {
      var metadata = eventRecord.Metadata;
      if (!metadata.TryGetValue(metadataKey, out var value))
      {
        logger.LogAbsenceOfMetadata(eventRecord.EventClass, metadataKey);
        return;
      }

      if (removeFromProperties)
      {
        metadata.Remove(metadataKey);
      }

      sb.Append(transformation(value));
    }

    var index = 0;
    while (index < orderedTransforms.Count &&
           orderedTransforms[index] is { EventClassKind: EventClassKind.NotEventClass } transform)
    {
      TryAppendMetadata(transform.MetadataKey, transform.Transformation);
      ++index;
    }

    var lastSeenEventClass = EventClassKind.NotEventClass;

    const char EventClassOpenChar = '{';
    const char EventClassCloseChar = '}';

    while (index < orderedTransforms.Count)
    {
      var (metadataKey, transformation, eventClass) = orderedTransforms[index];
      if (eventClass != lastSeenEventClass)
      {
        lastSeenEventClass = eventClass;
        if (eventClass != EventClassKind.Zero)
        {
          sb.Append(EventClassCloseChar);
        }

        sb.Append(TraceEventsConstants.Underscore)
          .Append(EventClassOpenChar);
      }

      TryAppendMetadata(metadataKey, transformation);
      ++index;
    }

    if (sb[^1] != EventClassCloseChar)
    {
      sb.Append(EventClassCloseChar);
    }

    eventRecord.EventName = string.Intern(sb.ToString());
  }

  public static string TransformMethodLikeNameForEventNameConcatenation(string fullMethodName)
  {
    var sb = new StringBuilder(fullMethodName);

    for (var i = 0; i < sb.Length; i++)
    {
      if (sb[i] == ' ')
      {
        sb[i] = TraceEventsConstants.Dot;
      }

      sb[i] = char.ToUpper(sb[i]);
    }

    return string.Intern(sb.ToString());
  }

  public static string TransformTypeLikeNameForEventNameConcatenation(string typeName) =>
    TransformTypeLikeNameForEventNameConcatenation(new StringBuilder(typeName));

  private static string TransformTypeLikeNameForEventNameConcatenation(StringBuilder sb) => string.Intern(sb.ToString());

  public static string TransformCamelCaseForEventNameConcatenation(string name)
  {
    var sb = new StringBuilder(name);
    for (var i = sb.Length - 1; i >= 0; i--)
    {
      if (char.IsUpper(sb[i]))
      {
        sb.Insert(i - 1, TraceEventsConstants.Dot);
        --i;
      }
    }

    for (var i = 0; i < sb.Length; i++)
    {
      sb[i] = char.ToUpper(sb[i]);
    }

    return string.Intern(sb.ToString());
  }

  public static string TransformAssemblyNameForEventNameConcatenation(string assemblyName)
  {
    var index = assemblyName.IndexOf(',', StringComparison.Ordinal);
    if (index == -1) return assemblyName;

    var sb = new StringBuilder(assemblyName);
    sb.Remove(index, sb.Length - index);

    return TransformTypeLikeNameForEventNameConcatenation(sb);
  }

  public static string TransformModuleFileNameForEventNameConcatenation(string fileName)
  {
    var indexOfExtension = fileName.IndexOf(".dll", StringComparison.Ordinal);
    if (indexOfExtension == -1) return fileName;

    var sb = new StringBuilder(fileName);
    sb.Remove(indexOfExtension, sb.Length - indexOfExtension);

    return TransformTypeLikeNameForEventNameConcatenation(sb);
  }

  public static string TransformDomainModuleFilePathForEventNameConcatenation(string filePath)
  {
    var indexOfLastSeparator = filePath.IndexOf("/", StringComparison.Ordinal);
    if (indexOfLastSeparator == -1) return filePath;

    var sb = new StringBuilder(filePath);
    sb.Remove(0, indexOfLastSeparator + 1);

    return TransformModuleFileNameForEventNameConcatenation(sb.ToString());
  }

  public static string ConcatenateMethodDetails(string methodName, string methodNamespace, string signature) =>
    string.Intern(methodNamespace +
                  (methodNamespace.EndsWith('.') ? "" : ".") +
                  methodName +
                  $"[{signature.Replace(' ', '.')}]");
}