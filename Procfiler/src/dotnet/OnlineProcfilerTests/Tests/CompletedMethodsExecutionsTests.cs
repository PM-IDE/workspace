using Core.Events.EventRecord;
using Core.Utils;
using ProcfilerOnline.Core;
using ProcfilerOnline.Core.Handlers;
using TestsUtil;

namespace OnlineProcfilerTests.Tests;

public class TestCompletedMethodExecutionHandler : IEventPipeStreamEventHandler
{
  public Dictionary<long, List<List<EventRecordWithMetadata>>> SeenMethods { get; } = [];

  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (eventPipeStreamEvent is not MethodExecutionEvent executionEvent) return;

    SeenMethods.GetOrCreate(executionEvent.Frame.MethodId, static () => []).Add(executionEvent.Frame.InnerEvents);
  }
}

[TestFixture]
[NonParallelizable]
public class CompletedMethodsExecutionsTests : OnlineProcfilerMethodsTest
{
  private readonly TestCompletedMethodExecutionHandler myHandler = new();

  protected override IEnumerable<IEventPipeStreamEventHandler> HandlersToRegister =>
  [
    myHandler
  ];

  protected override string? Prefix => null;


  [TestCaseSource(nameof(AllSolutionsSource))]
  public void DoTest(KnownSolution solution) => Execute(() => DoExecuteTest(solution));


  protected override Dictionary<string, List<List<EventRecordWithMetadata>>> GetLoggedMethods(ISharedEventPipeStreamData data) =>
    myHandler.SeenMethods
      .Select(pair => (data.FindMethodName(pair.Key)!, pair.Value))
      .ToDictionary();
}