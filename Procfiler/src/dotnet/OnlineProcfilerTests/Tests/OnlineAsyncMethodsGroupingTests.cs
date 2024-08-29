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
  private readonly TestAsyncMethodsHandler myHandler = new();

  protected override IEnumerable<IEventPipeStreamEventHandler> HandlersToRegister =>
  [
    myHandler
  ];

  protected override string? Prefix => "ASYNC_";


  [Test]
  public void SimpleAsyncAwait() => Execute(() => DoExecuteTest(KnownSolution.SimpleAsyncAwait));

  [Test]
  public void NotSimpleAsyncAwait() => Execute(() => DoExecuteTest(KnownSolution.NotSimpleAsyncAwait));

  [Test]
  public void AsyncAwait() => Execute(() => DoExecuteTest(KnownSolution.AsyncAwait));

  [Test]
  public void AsyncDisposable() => Execute(() => DoExecuteTest(KnownSolution.AsyncDisposable));

  [Test]
  public void AwaitForeach() => Execute(() => DoExecuteTest(KnownSolution.AwaitForeach));

  [Test]
  public void AsyncAwaitTaskFactoryNew() => Execute(() => DoExecuteTest(KnownSolution.AsyncAwaitTaskFactoryNew));


  protected override Dictionary<string, List<List<EventRecordWithMetadata>>> GetLoggedMethods(ISharedEventPipeStreamData data) =>
    myHandler.RecordedStateMachineTraces;
}