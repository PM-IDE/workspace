namespace FicusFrontend.Components.CaseInfo;

public static class PipelinePartsGroups
{
  public static IReadOnlySet<string> AnnotationParts { get; } = new HashSet<string>
  {
    "AnnotatePetriNetWithCount",
    "AnnotatePetriNetWithFrequency",
    "AnnotatePetriNetWithTraceFrequency"
  };
}