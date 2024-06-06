using Autofac;
using Procfiler.Core.EventsProcessing;
using Procfiler.Utils;
using ProcfilerTests.Core;

namespace ProcfilerTests.Tests;

[TestFixture]
public class EventTimeStampsConsistencyTest : ProcessTestBase
{
  [TestCaseSource(nameof(DefaultContexts))]
  [TestCaseSource(nameof(OnlineSerializationContexts))]
  public void Test(ContextWithSolution dto) => DoTest(dto);


  private void DoTest(ContextWithSolution dto)
  {
    StartProcessSplitEventsByThreadsAndDoTest(dto.Context, (eventsByThreads, globalData) =>
    {
      foreach (var (_, events) in eventsByThreads)
      {
        var processor = Container.Resolve<IUnitedEventsProcessor>();
        processor.ApplyMultipleMutators(events, globalData, EmptyCollections<Type>.EmptySet);

        long? prevStamp = null;
        long? prevThreadId = null;

        foreach (var (_, currentEvent) in events)
        {
          if (prevStamp is null)
          {
            prevStamp = currentEvent.Time.QpcStamp;
            prevThreadId = currentEvent.ManagedThreadId;
          }
          else
          {
            if (prevThreadId != currentEvent.ManagedThreadId)
            {
              Assert.Fail("Managed thread ids were not equal");
            }

            if (prevStamp > currentEvent.Time.QpcStamp)
            {
              Assert.Fail("first.Value.Stamp > second.Value.Stamp");
            }

            prevStamp = currentEvent.Time.QpcStamp;
            prevThreadId = currentEvent.ManagedThreadId;
          }
        }
      }
    });
  }
}