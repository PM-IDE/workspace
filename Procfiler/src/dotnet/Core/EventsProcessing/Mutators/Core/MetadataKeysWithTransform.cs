namespace Core.EventsProcessing.Mutators.Core;

public enum EventClassKind
{
  NotEventClass = -1,
  Zero = 0
}

public readonly record struct MetadataKeysWithTransform(
  string MetadataKey,
  Func<string, string> Transformation,
  EventClassKind EventClassKind)
{
  public static MetadataKeysWithTransform CreateForTypeLikeName(string metadataKey, EventClassKind eventClass) =>
    new(metadataKey, MutatorsUtil.TransformTypeLikeNameForEventNameConcatenation, eventClass);

  public static MetadataKeysWithTransform CreateForCamelCaseName(string metadataKey, EventClassKind eventClass) =>
    new(metadataKey, MutatorsUtil.TransformCamelCaseForEventNameConcatenation, eventClass);

  public static MetadataKeysWithTransform CreateForAssemblyName(string metadataKey, EventClassKind eventClass) =>
    new(metadataKey, MutatorsUtil.TransformAssemblyNameForEventNameConcatenation, eventClass);

  public static MetadataKeysWithTransform CreateForModuleILFileName(string metadataKey, EventClassKind eventClass) =>
    new(metadataKey, MutatorsUtil.TransformModuleFileNameForEventNameConcatenation, eventClass);

  public static MetadataKeysWithTransform CreateForModuleILPath(string metadataKey, EventClassKind eventClass) =>
    new(metadataKey, MutatorsUtil.TransformDomainModuleFilePathForEventNameConcatenation, eventClass);

  public static MetadataKeysWithTransform CreateForMethodLikeName(string metadataKey, EventClassKind eventClass) =>
    new(metadataKey, MutatorsUtil.TransformMethodLikeNameForEventNameConcatenation, eventClass);

  public static MetadataKeysWithTransform CreateIdenticalTransform(string metadataKey, EventClassKind eventClass) =>
    new(metadataKey, value => value, eventClass);
}