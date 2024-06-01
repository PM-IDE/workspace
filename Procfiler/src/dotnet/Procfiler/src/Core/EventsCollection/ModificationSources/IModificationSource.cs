using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler.ShadowStacks;
using Procfiler.Core.EventRecord;
using Procfiler.Utils;

namespace Procfiler.Core.EventsCollection.ModificationSources;

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

public class MethodStartEndModificationSource : ModificationSourceBase
{
  private readonly IProcfilerEventsFactory myEventsFactory;
  private readonly SessionGlobalData myGlobalData;
  private readonly ICppShadowStack myShadowStack;
  private readonly bool myAggressiveReuse;


  public override long Count => PointersManager.Count;


  public MethodStartEndModificationSource(
    IProcfilerLogger logger,
    IProcfilerEventsFactory eventsFactory,
    SessionGlobalData globalData,
    ICppShadowStack shadowStack,
    bool aggressiveReuse) : base(logger, shadowStack.FramesCount)
  {
    Debug.Assert(shadowStack.FramesCount > 0);

    myAggressiveReuse = aggressiveReuse;
    myGlobalData = globalData;
    myShadowStack = shadowStack;
    myEventsFactory = eventsFactory;
  }


  protected override IEnumerable<EventRecordWithMetadata> EnumerateInitialEvents() =>
    myAggressiveReuse switch
    {
      true => myShadowStack.EnumerateMethodsAggressiveReuse(myEventsFactory, myGlobalData),
      false => myShadowStack.EnumerateMethods(myEventsFactory, myGlobalData)
    };
}