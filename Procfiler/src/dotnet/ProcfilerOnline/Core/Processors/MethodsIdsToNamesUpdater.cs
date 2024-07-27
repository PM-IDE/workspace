using Core.Container;
using Core.Utils;
using Microsoft.Diagnostics.Tracing.Parsers.Clr;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class MethodsIdsToNamesUpdater(ISharedEventPipeStreamData sharedData) : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.Event is MethodLoadUnloadVerboseTraceData traceData)
    {
      var fqn = MethodsUtil.ConcatenateMethodDetails(traceData.MethodName, traceData.MethodNamespace, traceData.MethodSignature);
      sharedData.UpdateMethodsInfo((ulong)traceData.MethodID, fqn);
    }
  }
}