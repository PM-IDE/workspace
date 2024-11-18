namespace TestsUtil;

public class KnownSolution
{
  private const string TargetFramework = "net9.0";

  public static KnownSolution ConsoleApp1 { get; } = new("ConsoleApp1");
  public static KnownSolution TaskTestProject1 { get; } = new("TaskTestProject1");
  public static KnownSolution ExceptionTryCatchFinally { get; } = new("ExceptionTryCatchFinally");
  public static KnownSolution Sockets { get; } = new("Sockets");
  public static KnownSolution FileWriteProject { get; } = new("FileWriteProject");
  public static KnownSolution DynamicAssemblyLoading { get; } = new("DynamicAssemblyLoading");
  public static KnownSolution DynamicAssemblyCreation { get; } = new("DynamicAssemblyCreation");
  public static KnownSolution ExceptionTryCatchFinallyWhen { get; } = new("ExceptionTryCatchFinallyWhen");
  public static KnownSolution FinalizableObject { get; } = new("FinalizableObject");
  public static KnownSolution IntensiveThreadPoolUse { get; } = new("IntensiveThreadPoolUse");
  public static KnownSolution UnsafeFixed { get; } = new("UnsafeFixed");
  public static KnownSolution SystemArrayPooling { get; } = new("SystemArrayPooling");
  public static KnownSolution NotExistingAssemblyLoading { get; } = new("NotExistingAssemblyLoading");
  public static KnownSolution LohAllocations { get; } = new("LOHAllocations");
  public static KnownSolution HttpRequests { get; } = new("HttpRequests");
  public static KnownSolution SimpleAsyncAwait { get; } = new("SimpleAsyncAwait");
  public static KnownSolution NotSimpleAsyncAwait { get; } = new("NotSimpleAsyncAwait");
  public static KnownSolution AsyncAwait { get; } = new("AsyncAwait");
  public static KnownSolution AsyncAwaitTaskFactoryNew { get; } = new("AsyncAwaitTaskFactoryNew");
  public static KnownSolution AwaitForeach { get; } = new("AwaitForeach");
  public static KnownSolution AsyncDisposable { get; } = new("AsyncDisposable");


  public static IEnumerable<KnownSolution> AllSolutions { get; } =
  [
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
  ];

  public string Name { get; }
  public string Tfm { get; }
  public int ExpectedEventsCount { get; }
  public string NamespaceFilterPattern { get; }


  private KnownSolution(string name, string tfm = TargetFramework, int expectedEventsCount = 15_000)
  {
    Name = name;
    Tfm = tfm;
    ExpectedEventsCount = expectedEventsCount;
    NamespaceFilterPattern = name;
  }


  public override string ToString() => Name;
}