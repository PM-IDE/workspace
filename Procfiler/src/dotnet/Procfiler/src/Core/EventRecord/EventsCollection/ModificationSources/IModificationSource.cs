using Core.Utils;

namespace Procfiler.Core.EventRecord.EventsCollection.ModificationSources;

public interface IModificationSource : IEventsOwner;

public abstract class ModificationSourceBase(IProcfilerLogger logger, long initialEventsCount)
  : EventsOwnerBase(logger, initialEventsCount), IModificationSource
{
  public override bool Remove(EventPointer pointer)
  {
    AssertNotFrozen();
    return PointersManager.Remove(pointer);
  }
}