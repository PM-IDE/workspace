namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph;

public class AggregatedData
{
  public double TotalExecutionTimeNs { get; set; }
  public double MaxExecutionTime { get; set; }

  public MergedSoftwareData GlobalSoftwareData { get; set; } = new();


  public bool IsEmpty => GlobalSoftwareData.IsEmpty;
}

public class MergedSoftwareData
{
  public Dictionary<string, ValueWithUnits<Dictionary<string, double>>> Histograms { get; set; } = new();
  public Dictionary<string, ValueWithUnits<double>> Counters { get; set; } = new();
  public Dictionary<string, ValueWithUnits<Duration>> ActivitiesDurations { get; set; } = new();


  public bool IsEmpty =>
    Histograms.Count == 0 &&
    Counters.Count == 0 &&
    ActivitiesDurations.Count == 0;
}

public class ValueWithUnits<T>
{
  public string? Group { get; set; }
  public string? Units { get; set; }
  public T Value { get; set; }
}

public class Duration
{
  public double Value { get; set; }
  public DurationKind Kind { get; set; }
}

public enum DurationKind
{
  Unspecified,

  Nanos,
  Micros,
  Millis,
  Seconds,
  Minutes,
  Hours,
  Days
}