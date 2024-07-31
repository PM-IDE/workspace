using Core.Container;
using Core.Events.EventRecord;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Updaters;

[AppComponent]
public class MethodsIdsToNamesUpdater : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.Event.TryGetMethodInfo() is { Id: var methodId, Fqn: var fqn })
    {
      context.SharedData.UpdateMethodsInfo(methodId, fqn);
    }
  }
}