using Core.Events.EventRecord;
using Core.Utils;
using ProcfilerOnline.Core;
using ProcfilerOnline.Core.Handlers;
using TestsUtil;

namespace OnlineProcfilerTests.Tests;

public class TestAsyncMethodsHandler : IEventPipeStreamEventHandler
{
  public Dictionary<string, List<List<EventRecordWithMetadata>>> RecordedStateMachineTraces { get; } = [];

  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (eventPipeStreamEvent is not CompletedAsyncMethodEvent completedAsyncMethodEvent) return;

    foreach (var trace in completedAsyncMethodEvent.MethodTraces)
    {
      RecordedStateMachineTraces.GetOrCreate(completedAsyncMethodEvent.StateMachineName, static () => []).Add(trace);
    }
  }
}

[TestFixture]
[NonParallelizable]
public class OnlineAsyncMethodsGroupingTests : OnlineProcfilerMethodsTest
{
  private static readonly IEnumerable<KnownSolution> ourAsyncSolutions = KnownSolution.AsyncSolutions;


  private readonly TestAsyncMethodsHandler myHandler = new();


  protected override IEnumerable<IEventPipeStreamEventHandler> HandlersToRegister =>
  [
    myHandler
  ];

  protected override string? Prefix => null;


  [TestCaseSource(nameof(ourAsyncSolutions))]
  public void DoTest(KnownSolution solution) => Execute(() => DoExecuteTest(solution));

  protected override Dictionary<string, List<List<EventRecordWithMetadata>>> GetLoggedMethods(ISharedEventPipeStreamData data) =>
    myHandler.RecordedStateMachineTraces;
}