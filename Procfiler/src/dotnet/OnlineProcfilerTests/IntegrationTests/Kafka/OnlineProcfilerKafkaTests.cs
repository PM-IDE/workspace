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
      var events = consumer.ConsumeAllEvents()
        .Select(trace =>
          trace
            .Select(e => new EventRecordWithMetadata(EventRecordTime.Default, e.Name.IndexOf('_') switch
            {
              -1 => e.Name,
              var index => e.Name[..index]
            }, -1, -1, new EventMetadata(e.Attributes.ToDictionary(a => a.Key.Value, a => a.Value.ToString())))
            {
              EventName = e.Name
            })
            .ToList()
        )
        .Where(t => t.Count > 0)
        .ToList();

      foreach (var trace in events.OrderBy(e => e.First().EventName))
      {
        var methodNamesToEvents = new Dictionary<string, List<List<EventRecordWithMetadata>>>
        {
          [trace.First().EventName] = [trace]
        };

        var filter = new Regex(solution.NamespaceFilterPattern);
        var gold = OnlineProcfilerMethodsUtil.SerializeToGold(globalData, methodNamesToEvents, filter, null);
        sb.Append(gold);
      }
    }

    return sb.ToString();
  }

  protected override void ExecuteBeforeContainerCreation()
  {
    Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__TopicName", "my-topic");
    Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__BootstrapServers", "localhost:9092");
    Environment.SetEnvironmentVariable("ProduceEventsToKafka", "true");
  }
}