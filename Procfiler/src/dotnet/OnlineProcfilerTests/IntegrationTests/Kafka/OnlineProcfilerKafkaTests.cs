using System.Text;
using System.Text.RegularExpressions;
using Autofac;
using Core.Events.EventRecord;
using Microsoft.Extensions.Options;
using OnlineProcfilerTests.Core;
using OnlineProcfilerTests.Tests;
using ProcfilerOnline.Core.Handlers;
using ProcfilerOnline.Core.Settings;
using TestsUtil;

namespace OnlineProcfilerTests.IntegrationTests.Kafka;

public class OnlineProcfilerKafkaTests : OnlineProcfilerTestWithGold
{
  protected override IEnumerable<IEventPipeStreamEventHandler> HandlersToRegister { get; } = [];


  [Test]
  public void AllSolutionsTest() => Execute(() => DoExecuteTest(KnownSolution.AllSolutions));


  private string DoExecuteTest(IEnumerable<KnownSolution> solutions)
  {
    var settings = Container.Resolve<IOptions<OnlineProcfilerSettings>>().Value;
    var consumer = new MethodExecutionKafkaConsumer(settings);
    var sb = new StringBuilder();

    foreach (var solution in solutions)
    {
      var globalData = ExecuteTest(solution) ?? throw new Exception();
      var events = consumer.ConsumeAllEvents();

      foreach (var @event in events.OrderBy(e => e.MethodFullName))
      {
        var methodNamesToEvents = new Dictionary<string, List<List<EventRecordWithMetadata>>>
        {
          [@event.MethodFullName] =
          [
            @event.Events
              .Select(e => new EventRecordWithMetadata(e.Time, e.EventClass, e.ManagedThreadId, e.StackTraceId, new EventMetadata(e.Attributes)))
              .ToList()
          ]
        };

        var filter = new Regex(solution.NamespaceFilterPattern);
        var gold = OnlineProcfilerMethodsUtil.SerializeToGold(globalData, methodNamesToEvents, filter, null);
        sb.Append(gold);
      }
    }

    return sb.ToString();
  }
}