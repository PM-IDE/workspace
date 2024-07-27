using Core.Container;
using Microsoft.Diagnostics.Tracing.Parsers.Clr;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class MethodsIdsToNamesUpdater(ISharedEventPipeStreamData sharedData) : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.Event is MethodLoadUnloadVerboseTraceData traceData)
    {
      sharedData.UpdateMethodsInfo((ulong)traceData.MethodID, traceData.MethodName);
    }
  }
}