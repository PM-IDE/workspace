using Core.Events.EventRecord;
using Core.Events.EventsCollection;
using Core.Events.EventsCollection.ModificationSources;
using Core.Utils;
using Procfiler.Core.EventRecord;

namespace ProcfilerTests.Tests.EventsCollection;

public class TestModificationSource : ModificationSourceBase, IModificationSource
{
  private readonly EventRecordWithMetadata[] myInitialEvents;


  public override long Count => PointersManager.Count;


  public TestModificationSource(IProcfilerLogger logger, EventRecordWithMetadata[] initialEvents) : base(logger, initialEvents.Length)
  {
    myInitialEvents = initialEvents;
  }


  protected override IEnumerable<EventRecordWithMetadata> EnumerateInitialEvents()
  {
    for (var i = 0; i < myInitialEvents.Length; i++)
    {
      if (PointersManager.IsRemoved(EventPointer.ForInitialArray(i, this))) continue;

      yield return myInitialEvents[i];
    }
  }
}