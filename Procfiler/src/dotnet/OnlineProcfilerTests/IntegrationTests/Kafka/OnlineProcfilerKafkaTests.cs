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
  public void ConsoleApp1() => Execute(() => DoExecuteTest(KnownSolution.AllSolutions));


  private string DoExecuteTest(IEnumerable<KnownSolution> solutions)
  {
    var consumer = new MethodExecutionKafkaConsumer(Container.Resolve<IOptions<OnlineProcfilerSettings>>().Value.KafkaSettings.TopicName);
    var sb = new StringBuilder();

    foreach (var solution in solutions)
    {
      var globalData = ExecuteTest(solution) ?? throw new Exception();
      var events = consumer.ConsumeAllEvents();

      foreach (var @event in events)
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


  protected override void ExecuteBeforeContainerCreation()
  {
    Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__TopicName", "my-topic");
    Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__BootstrapServers", "localhost:9092");
    Environment.SetEnvironmentVariable("ProduceEventsToKafka", "true");
  }
}