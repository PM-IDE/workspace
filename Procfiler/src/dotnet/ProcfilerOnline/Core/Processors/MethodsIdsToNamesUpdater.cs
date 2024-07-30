using Core.Container;
using Core.Events.EventRecord;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class MethodsIdsToNamesUpdater(ISharedEventPipeStreamData sharedData) : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.Event.TryGetMethodInfo() is { Id: var methodId, Fqn: var fqn })
    {
      sharedData.UpdateMethodsInfo(methodId, fqn);
    }
  }
}