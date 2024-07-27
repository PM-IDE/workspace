using Procfiler.Core.EventRecord.EventRecord;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.EventMetadataTests;

[TestFixture]
public class EventMetadataTestWithRealEventsTest : ProcessTestBase
{
  [Test]
  public void TestWithConsoleApp1()
  {
    StartProcessAndDoTestWithDefaultContext(KnownSolution.ConsoleApp1.CreateDefaultContext(), collectedEvents =>
    {
      foreach (var (_, eventRecord) in collectedEvents.Events)
      {
        DoTestWithMetadata(eventRecord.Metadata);
      }
    });
  }

  private static void DoTestWithMetadata(IEventMetadata metadata)
  {
    var initialCount = metadata.Count;
    if (initialCount == 0) return;

    var indexToRemove = Random.Shared.Next(initialCount);
    var initialKeys = metadata.Keys.ToList();
    var initialValues = metadata.Values.ToList();

    Assert.That(initialKeys, Has.Count.EqualTo(initialCount));
    Assert.That(initialValues, Has.Count.EqualTo(initialCount));

    var keyToRemove = initialKeys[indexToRemove];
    Assert.That(metadata.Remove(keyToRemove), Is.True);
    initialKeys.RemoveAt(indexToRemove);
    initialValues.RemoveAt(indexToRemove);

    Assert.That(metadata, Has.Count.EqualTo(initialCount - 1));

    TestUtil.AssertCollectionsAreSame(initialKeys, metadata.Keys.ToList());
    TestUtil.AssertCollectionsAreSame(initialValues, metadata.Values.ToList());
  }
}