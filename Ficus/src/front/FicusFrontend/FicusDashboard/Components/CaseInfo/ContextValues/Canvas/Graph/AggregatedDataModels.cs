namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph;

public class AggregatedData
{
  public double TotalAllocatedBytes { get; set; }
  public double TotalExecutionTime { get; set; }
  public double MaxExecutionTime { get; set; }

  public double TotalBufferAllocatedBytes { get; set; }
  public double TotalBufferRentedBytes { get; set; }
  public double TotalBufferReturnedBytes { get; set; }

  public MergedSoftwareData GlobalSoftwareData { get; set; } = new();
}

public class MergedSoftwareData
{
  public Dictionary<string, double> Allocations { get; set; } = new();

  public Dictionary<string, double> InliningFailed { get; set; } = new();
  public Dictionary<string, double> InliningSucceeded { get; set; } = new();
  public Dictionary<string, double> InliningFailedReasons { get; set; } = new();

  public Dictionary<string, double> MethodsLoads {  get; set; } = new();
  public Dictionary<string, double> MethodsUnloads { get; set; } = new();

  public CountAndSum BufferAllocatedBytes { get; set; } = new();
  public CountAndSum BufferRentedBytes { get; set; } = new();
  public CountAndSum BufferReturnedBytes { get; set; } = new();

  public Dictionary<string, double> Exceptions { get; set; } = new();

  public HashSet<double> CreatedThreads { get; set; } = [];
  public HashSet<double> TerminatedThreads { get; set; } = [];

  public Dictionary<string, double> HttpRequests { get; set; } = new();

  public Dictionary<string, ValueWithUnits<Dictionary<string, double>>> Histograms { get; set; } = new();
  public Dictionary<string, ValueWithUnits<double>> Counters { get; set; } = new();
}

public class CountAndSum
{
  public double Count { get; set; }
  public double Sum { get; set; }
}

public class ValueWithUnits<T>
{
  public string Units { get; set; }
  public T Value { get; set; }
}