using System.Text;
using System.Text.RegularExpressions;
using Autofac;
using Bxes.Models.Domain;
using Core.Events.EventRecord;
using Microsoft.Extensions.Options;
using OnlineProcfilerTests.Core;
using OnlineProcfilerTests.Tests;
using ProcfilerOnline.Core;
using ProcfilerOnline.Core.Handlers;
using ProcfilerOnline.Core.Settings;
using TestsUtil;

namespace OnlineProcfilerTests.IntegrationTests.Kafka;

using Traces = List<(string MethodName, List<EventRecordWithMetadata> Events)>;

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
      var traces = ConsumeAllEvents(consumer);

      AddSerializedTracesToGold(solution, globalData, traces, sb);
    }

    return sb.ToString();
  }

  private static Traces ConsumeAllEvents(MethodExecutionKafkaConsumer consumer) =>
    consumer.ConsumeAllEvents()
      .Select(trace =>
        (
          MethodName: trace.Metadata.FirstOrDefault(a => a.Key.Value is "MethodName")?.Value.ToString() ?? "UNRESOLVED",
          Events: trace.Events.Select(CreateFromBxesEvent).ToList()
        )
      )
      .Where(t => t.Events.Count > 0)
      .ToList();

  private static EventRecordWithMetadata CreateFromBxesEvent(IEvent bxesEvent) =>
    new(EventRecordTime.Default, CreateEventClass(bxesEvent), -1, -1, CreateEventMetadata(bxesEvent))
    {
      EventName = bxesEvent.Name
    };

  private static string CreateEventClass(IEvent bxesEvent) => bxesEvent.Name.IndexOf('_') switch
  {
    -1 => bxesEvent.Name,
    var index => bxesEvent.Name[..index]
  };

  private static IEventMetadata CreateEventMetadata(IEvent bxesEvent) =>
    new EventMetadata(bxesEvent.Attributes.ToDictionary(a => a.Key.Value, a => a.Value.ToString()!));

  private static void AddSerializedTracesToGold(
    KnownSolution solution, ISharedEventPipeStreamData globalData, Traces traces, StringBuilder sb)
  {
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

  protected override void ExecuteBeforeContainerCreation()
  {
    Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__TopicName", "my-topic");
    Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__BootstrapServers", "localhost:9092");
    Environment.SetEnvironmentVariable("ProduceEventsToKafka", "true");
  }
}