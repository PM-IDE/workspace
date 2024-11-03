using Core.Events.EventRecord;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core.CppProcfiler.ShadowStacks;

public interface ICppShadowStack : IEnumerable<FrameInfo>
{
  long ManagedThreadId { get; }
  long FramesCount { get; }
}

public static class ExtensionsForICppShadowStack
{
  public static IEnumerable<EventRecordWithMetadata> EnumerateMethods(
    this ICppShadowStack shadowStack,
    EventRecordWithMetadata referenceEvent,
    IProcfilerEventsFactory eventsFactory,
    IGlobalDataWithStacks globalData)
  {
    foreach (var frameInfo in shadowStack)
    {
      var creationContext = new FromFrameInfoCreationContext
      {
        FrameInfo = frameInfo,
        GlobalData = globalData,
        ManagedThreadId = shadowStack.ManagedThreadId,
        NativeThreadId = referenceEvent.NativeThreadId
      };

      yield return eventsFactory.CreateMethodEvent(creationContext);
    }
  }

  public static IEnumerable<EventRecordWithMetadata> EnumerateMethodsAggressiveReuse(
    this ICppShadowStack shadowStack,
    EventRecordWithMetadata referenceEvent,
    IProcfilerEventsFactory eventsFactory,
    IGlobalDataWithStacks globalData)
  {
    var sharedEvent = EventRecordWithMetadata.CreateUninitialized();
    foreach (var frameInfo in shadowStack)
    {
      var creationContext = new FromFrameInfoCreationContext
      {
        FrameInfo = frameInfo,
        GlobalData = globalData,
        ManagedThreadId = shadowStack.ManagedThreadId,
        NativeThreadId = referenceEvent.NativeThreadId
      };

      eventsFactory.FillExistingEventWith(creationContext, sharedEvent);
      yield return sharedEvent;
    }
  }
}