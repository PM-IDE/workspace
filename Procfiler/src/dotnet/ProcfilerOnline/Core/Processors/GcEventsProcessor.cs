using Core.Constants.TraceEvents;
using Core.Container;
using Core.Events.EventRecord;
using ProcfilerOnline.Core.Handlers;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class GcEventsProcessor(ICompositeEventPipeStreamEventHandler handler) : ITraceEventProcessor
{
  private int myGcCount;
  private readonly List<EventRecordWithMetadata> myCurrentGcTrace = [];

  public void Process(EventProcessingContext context)
  {
    myCurrentGcTrace.Add(context.Event);

    switch (context.Event.EventClass)
    {
      case TraceEventsConstants.GcStart:
      {
        myGcCount++;
        break;
      }
      case TraceEventsConstants.GcStop:
      {
        myGcCount--;
        if (myGcCount == 0)
        {
          var gcTrace = myCurrentGcTrace.ToList();
          myCurrentGcTrace.Clear();

          handler.Handle(new GcEvent
          {
            GcTrace = gcTrace
          });
        }

        break;
      }
    }
  }
}