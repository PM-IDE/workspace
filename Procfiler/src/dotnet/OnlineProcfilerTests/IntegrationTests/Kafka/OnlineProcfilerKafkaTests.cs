using System.Text;
using System.Text.RegularExpressions;
using Autofac;
using Core.Events.EventRecord;
using Microsoft.Extensions.Options;
using OnlineProcfilerTests.Core;
using OnlineProcfilerTests.Tests;
using ProcfilerOnline.Core;
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
      var traces = consumer.ConsumeAllEvents()
        .Select(trace =>
          (
            MethodName: trace.Metadata.FirstOrDefault(a => a.Key.Value is "MethodName")?.Value.ToString() ?? "UNRESOLVED",
            Events: trace.Events
              .Select(e => new EventRecordWithMetadata(EventRecordTime.Default, e.Name.IndexOf('_') switch
              {
                -1 => e.Name,
                var index => e.Name[..index]
              }, -1, -1, new EventMetadata(e.Attributes.ToDictionary(a => a.Key.Value, a => a.Value.ToString()!)))
              {
                EventName = e.Name
              })
              .ToList()
            )
        )
        .Where(t => t.Events.Count > 0)
        .ToList();

      foreach (var (executedMethodName, trace) in traces.OrderBy(e => e.MethodName))
      {
        var methodNamesToEvents = new Dictionary<string, List<List<EventRecordWithMetadata>>>
        {
          [executedMethodName] = [trace]
        };

        var filter = new Regex(solution.NamespaceFilterPattern);
        var gold = OnlineProcfilerMethodsUtil.SerializeToGold(globalData, methodNamesToEvents, filter, null);
        sb.Append(gold);
      }
    }

    return sb.ToString();
  }
}