using System.Text.RegularExpressions;
using Core.Events.EventRecord;
using Core.Utils;
using OnlineProcfilerTests.Core;
using ProcfilerOnline.Core;
using ProcfilerOnline.Core.Handlers;
using ProcfilerTests.Core;
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
public class OnlineAsyncMethodsGroupingTests : OnlineProcfilerTestWithGold
{
  private readonly TestAsyncMethodsHandler myHandler = new();

  protected override IEnumerable<IEventPipeStreamEventHandler> HandlersToRegister =>
  [
    myHandler
  ];


  [Test] public void SimpleAsyncAwait() => Execute(() => DoExecuteTest(KnownSolution.SimpleAsyncAwait));
  [Test] public void NotSimpleAsyncAwait() => Execute(() => DoExecuteTest(KnownSolution.NotSimpleAsyncAwait));
  [Test] public void AsyncAwait() => Execute(() => DoExecuteTest(KnownSolution.AsyncAwait));
  [Test] public void AsyncDisposable() => Execute(() => DoExecuteTest(KnownSolution.AsyncDisposable));
  [Test] public void AwaitForeach() => Execute(() => DoExecuteTest(KnownSolution.AwaitForeach));
  [Test] public void AsyncAwaitTaskFactoryNew() => Execute(() => DoExecuteTest(KnownSolution.AsyncAwaitTaskFactoryNew));


  private string DoExecuteTest(KnownSolution solution)
  {
    var sharedData = ExecuteTest(solution) ?? throw new Exception();

    var filter = new Regex(solution.NamespaceFilterPattern);

    return AsyncMethodsTestsUtil.SerializeToGold(myHandler.RecordedStateMachineTraces, filter, "ASYNC_", e =>
    {
      if (e.TryGetMethodDetails() is var (_, methodId))
      {
        return sharedData.MethodIdToFqn.GetValueOrDefault(methodId);
      }

      return null;
    }, trace => ProgramMethodCallTreeDumper.CreateDump(trace, filter.ToString(), e => e.TryGetMethodDetails() switch
    {
      var (_, id) => (sharedData.MethodIdToFqn[id], e.GetMethodEventKind() == MethodKind.Begin),
      _ => null
    }));
  }
}