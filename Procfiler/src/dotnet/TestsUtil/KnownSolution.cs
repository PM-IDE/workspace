namespace TestsUtil;

public class KnownSolution
{
  private const string TargetFramework = "net6.0";

  public static KnownSolution ConsoleApp1 { get; } = new("ConsoleApp1", TargetFramework, 15_000);
  public static KnownSolution TaskTestProject1 { get; } = new("TaskTestProject1", TargetFramework, 15_000);
  public static KnownSolution ExceptionTryCatchFinally { get; } = new("ExceptionTryCatchFinally", TargetFramework, 15_000);
  public static KnownSolution Sockets { get; } = new("Sockets", TargetFramework, 15_000);
  public static KnownSolution FileWriteProject { get; } = new("FileWriteProject", TargetFramework, 15_000);
  public static KnownSolution DynamicAssemblyLoading { get; } = new("DynamicAssemblyLoading", TargetFramework, 15_000);
  public static KnownSolution DynamicAssemblyCreation { get; } = new("DynamicAssemblyCreation", TargetFramework, 15_000);
  public static KnownSolution ExceptionTryCatchFinallyWhen { get; } = new("ExceptionTryCatchFinallyWhen", TargetFramework, 15_000);
  public static KnownSolution FinalizableObject { get; } = new("FinalizableObject", TargetFramework, 15_000);
  public static KnownSolution IntensiveThreadPoolUse { get; } = new("IntensiveThreadPoolUse", TargetFramework, 15_000);
  public static KnownSolution UnsafeFixed { get; } = new("UnsafeFixed", TargetFramework, 15_000);
  public static KnownSolution SystemArrayPooling { get; } = new("SystemArrayPooling", TargetFramework, 15_000);
  public static KnownSolution NotExistingAssemblyLoading { get; } = new("NotExistingAssemblyLoading", TargetFramework, 15_000);
  public static KnownSolution LohAllocations { get; } = new("LOHAllocations", TargetFramework, 15_000);
  public static KnownSolution HttpRequests { get; } = new("HttpRequests", TargetFramework, 15_000);
  public static KnownSolution SimpleAsyncAwait { get; } = new("SimpleAsyncAwait", TargetFramework, 15_000);
  public static KnownSolution NotSimpleAsyncAwait { get; } = new("NotSimpleAsyncAwait", TargetFramework, 15_000);


  public static IEnumerable<KnownSolution> AllSolutions { get; } = new[]
  {
    ConsoleApp1,
    TaskTestProject1,
    ExceptionTryCatchFinally,

    //todo: for some reasons this test is not stable on mac
    //Sockets,
    //HttpRequests

    FileWriteProject,
    DynamicAssemblyLoading,
    DynamicAssemblyCreation,
    ExceptionTryCatchFinallyWhen,
    FinalizableObject,
    IntensiveThreadPoolUse,
    UnsafeFixed,
    SystemArrayPooling,
    NotExistingAssemblyLoading,
    LohAllocations
  };

  public string Name { get; }
  public string Tfm { get; }
  public int ExpectedEventsCount { get; }
  public string NamespaceFilterPattern { get; }


  private KnownSolution(string name, string tfm, int expectedEventsCount)
  {
    Name = name;
    Tfm = tfm;
    ExpectedEventsCount = expectedEventsCount;
    NamespaceFilterPattern = name;
  }


  public override string ToString() => Name;
}