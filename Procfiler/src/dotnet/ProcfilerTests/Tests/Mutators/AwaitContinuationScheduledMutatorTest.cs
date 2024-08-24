using Core.Constants.TraceEvents;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Tasks;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.Mutators;

[TestFixture]
public class AwaitContinuationScheduledMutatorTest : SingleMutatorTestBase
{
  protected override string EventClass => TraceEventsConstants.AwaitTaskContinuationScheduledSend;

  protected override ISingleEventMutator CreateMutator() =>
    new AwaitContinuationScheduledMutator(TestLogger.CreateInstance());

  [Test]
  public void TestMutation()
  {
    const string ContinueWithId = "1231312";
    var metadata = new EventMetadata
    {
      [TraceEventsConstants.OriginatingTaskSchedulerId] = "1",
      [TraceEventsConstants.OriginatingTaskId] = "123",
      [TraceEventsConstants.ContinuationId] = ContinueWithId
    };

    ExecuteWithRandomEvent(metadata, eventRecord =>
    {
      Assert.Multiple(() =>
      {
        Assert.That(eventRecord.Metadata.ContainsKey(TraceEventsConstants.ContinuationId), Is.False);
        Assert.That(eventRecord.Metadata.ContainsKey(TraceEventsConstants.TaskId), Is.True);
        Assert.That(eventRecord.Metadata[TraceEventsConstants.TaskId], Is.EqualTo(ContinueWithId));
      });
    });
  }
}