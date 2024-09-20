using Core.Container;
using Core.Events.EventRecord;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Updaters;

[AppComponent]
public class MethodsIdsToNamesUpdater : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.Event.TryGetExtendedMethodInfo() is { } extendedMethodInfo)
    {
      context.SharedData.UpdateMethodsInfo(extendedMethodInfo);
    }
  }
}