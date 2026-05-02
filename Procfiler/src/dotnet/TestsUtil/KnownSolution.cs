namespace TestsUtil;

public record KnownSolution
{
  private const string Net8 = "net8.0";
  private const string Net9 = "net9.0";
  private const string Net10 = "net10.0";

  private const string TargetFramework = Net10;

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
  public static KnownSolution ProcfilerEventPipeLogger { get; } = new("ProcfilerEventPipeLogger");
  public static KnownSolution Ocel { get; } = new("Ocel");
  public static KnownSolution Ocel2 { get; } = new("Ocel2");
  public static KnownSolution OcelWithIfs { get; } = new("OcelWithIfs");


  private static KnownSolution[] ourSolutions =
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


  private static KnownSolution[] ourAsyncSolutions =
  [
    SimpleAsyncAwait,
    NotSimpleAsyncAwait,
    AsyncAwait,
    AsyncDisposable,
    AwaitForeach,
    AsyncAwaitTaskFactoryNew
  ];


  public static IEnumerable<KnownSolution> AllSolutionsLatestFramework => ourSolutions;
  public static IEnumerable<KnownSolution> AllSolutions { get; } = AdjustTargetFrameworks(ourSolutions);
  public static IEnumerable<KnownSolution> AsyncSolutions { get; } = AdjustTargetFrameworks(ourAsyncSolutions);

  private static IEnumerable<KnownSolution> AdjustTargetFrameworks(KnownSolution[] solutions) =>
    solutions.Concat(solutions.Select(s => s with { Tfm = Net9 }).Concat(solutions.Select(s => s with { Tfm = Net8 })));


  public string Name { get; }
  public string Tfm { get; private init; }
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