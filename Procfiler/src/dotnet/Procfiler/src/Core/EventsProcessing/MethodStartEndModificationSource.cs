using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler.ShadowStacks;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection.ModificationSources;

namespace Procfiler.Core.EventsProcessing;

public class MethodStartEndModificationSource : ModificationSourceBase
{
  private readonly IProcfilerEventsFactory myEventsFactory;
  private readonly IGlobalDataWithStacks myGlobalData;
  private readonly ICppShadowStack myShadowStack;
  private readonly EventRecordWithMetadata myReferenceEvent;
  private readonly bool myAggressiveReuse;


  public override long Count => PointersManager.Count;


  public MethodStartEndModificationSource(
    IProcfilerLogger logger,
    IProcfilerEventsFactory eventsFactory,
    IGlobalDataWithStacks globalData,
    ICppShadowStack shadowStack,
    EventRecordWithMetadata referenceEvent,
    bool aggressiveReuse) : base(logger, shadowStack.FramesCount)
  {
    Debug.Assert(shadowStack.FramesCount > 0);

    myAggressiveReuse = aggressiveReuse;
    myGlobalData = globalData;
    myShadowStack = shadowStack;
    myReferenceEvent = referenceEvent;
    myEventsFactory = eventsFactory;
  }


  protected override IEnumerable<EventRecordWithMetadata> EnumerateInitialEvents() =>
    myAggressiveReuse switch
    {
      true => myShadowStack.EnumerateMethodsAggressiveReuse(myReferenceEvent, myEventsFactory, myGlobalData),
      false => myShadowStack.EnumerateMethods(myReferenceEvent, myEventsFactory, myGlobalData)
    };
}