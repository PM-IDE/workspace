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
    if (!context.Event.EventClass.StartsWith(TraceEventsConstants.GcPrefix)) return;

    if (myGcCount > 0)
    {
      myCurrentGcTrace.Add(context.Event);
    }

    switch (context.Event.EventClass)
    {
      case TraceEventsConstants.GcStart:
      {
        myCurrentGcTrace.Add(context.Event);
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
            ApplicationName = context.CommandContext.ApplicationName,
            GcTrace = gcTrace
          });
        }

        break;
      }
    }
  }
}