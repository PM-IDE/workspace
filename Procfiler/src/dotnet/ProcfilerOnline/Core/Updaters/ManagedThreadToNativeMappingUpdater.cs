using Core.Container;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Updaters;

[AppComponent]
public class ManagedThreadToNativeMappingUpdater : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    switch (context.Event.EventClass)
    {
      case OnlineProcfilerConstants.ManagedThreadToNativeAssignment:
      {
        var managedThreadId = long.Parse(context.Event.Metadata[OnlineProcfilerConstants.ManagedThreadId]);
        var nativeThreadId = long.Parse(context.Event.Metadata[OnlineProcfilerConstants.NativeThreadId]);

        context.SharedData.UpdateManagedToNativeThread(managedThreadId, nativeThreadId);
        break;
      }
    }
  }
}