using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.EventRecord.EventsCollection.ModificationSources;

namespace ProcfilerTests.Tests.EventsCollection;

public class TestModificationSource(IProcfilerLogger logger, EventRecordWithMetadata[] initialEvents)
  : ModificationSourceBase(logger, initialEvents.Length), IModificationSource
{
  public override long Count => PointersManager.Count;


  protected override IEnumerable<EventRecordWithMetadata> EnumerateInitialEvents()
  {
    for (var i = 0; i < initialEvents.Length; i++)
    {
      if (PointersManager.IsRemoved(EventPointer.ForInitialArray(i, this))) continue;

      yield return initialEvents[i];
    }
  }
}