namespace Bxes.Xes;

public static class XesConstants
{
  public const string DefaultName = "name";

  public const string LogTagName = "log";
  public const string TraceTagName = "trace";
  public const string EventTagName = "event";
  public const string ExtensionTagName = "extension";
  public const string ClassifierTagName = "classifier";
  public const string GlobalTagName = "global";
  public const string ListTagName = "list";
  public const string ValuesTagName = "values";

  public const string ClassifierNameAttribute = DefaultName;
  public const string ClassifierKeysAttribute = "keys";

  public const string ExtensionNameAttribute = DefaultName;
  public const string ExtensionPrefixAttribute = "prefix";
  public const string ExtensionUriAttribute = "uri";

  public const string GlobalScopeAttribute = "scope";

  public const string StringTagName = "string";
  public const string DateTagName = "date";
  public const string IntTagName = "int";
  public const string FloatTagName = "float";
  public const string BoolTagName = "boolean";
  public const string IdTagName = "id";

  public const string KeyAttributeName = "key";
  public const string ValueAttributeName = "value";

  public const string ConceptName = "concept:name";
  public const string TimeTimestamp = "time:timestamp";
  public const string LifecycleTransition = "lifecycle:transition";

  public const string ArtifactMoves = "artifactlifecycle:moves";
  public const string ArtifactItemModel = "artifactlifecycle:model";
  public const string ArtifactItemInstance = "artifactlifecycle:instance";
  public const string ArtifactItemTransition = "artifactlifecycle:transition";

  public const string CostTotal = "cost:total";
  public const string CostCurrency = "cost:currency";
  public const string CostDrivers = "cost:drivers";
  public const string CostDriver = "cost:driver";
  public const string CostAmount = "cost:amount";
}