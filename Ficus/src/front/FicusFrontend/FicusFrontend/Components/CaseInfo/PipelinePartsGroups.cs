namespace FicusFrontend.Components.CaseInfo;

public static class PipelinePartsNames
{
  public const string AnnotatePetriNetWithCount = nameof(AnnotatePetriNetWithCount);
  public const string AnnotatePetriNetWithFrequency = nameof(AnnotatePetriNetWithFrequency);
  public const string AnnotatePetriNetWithTraceFrequency = nameof(AnnotatePetriNetWithTraceFrequency);
  public const string AnnotateGraphWithTime = nameof(AnnotateGraphWithTime);
}

public static class PipelinePartsGroups
{
  public static IReadOnlySet<string> PetriNetsAnnotationParts { get; } = new HashSet<string>
  {
    PipelinePartsNames.AnnotatePetriNetWithCount,
    PipelinePartsNames.AnnotatePetriNetWithFrequency,
    PipelinePartsNames.AnnotatePetriNetWithTraceFrequency
  };

  public static IReadOnlySet<string> GraphAnnotationParts { get; } = new HashSet<string>
  {
    PipelinePartsNames.AnnotateGraphWithTime
  };
}