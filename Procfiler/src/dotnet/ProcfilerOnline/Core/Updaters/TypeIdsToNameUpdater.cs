using Core.Container;
using Core.Events.EventRecord;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Updaters;

[AppComponent]
file class TypeIdsToNameUpdater : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.Event.TryExtractTypeIdToName() is { Id: var typeId, Name: var name })
    {
      context.SharedData.UpdateTypeIdsToNames(typeId, name);
    }
  }
}