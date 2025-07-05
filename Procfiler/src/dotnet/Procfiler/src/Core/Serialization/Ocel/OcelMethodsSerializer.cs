using Core.Events.EventRecord;
using Core.Ocel;
using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Core;
using Procfiler.Core.SplitByMethod;

namespace Procfiler.Core.Serialization.Ocel;

public class OcelMethodsSerializer(IProcfilerLogger logger, string outputDirectory, IFullMethodNameBeautifier beautifier) : IOnlineMethodsSerializer
{
  private class OcelMethodWriteState(MethodOcelLogWriter writer)
  {
    public MethodOcelLogWriter Writer { get; } = writer;
  }


  private readonly List<OcelMethodWriteState> myStates = [];
  private readonly Dictionary<string, int> mySeenMethods = [];
  private int myCounter;
  private OcelMethodWriteState? myCurrentState;


  public object? CreateState(EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.TryGetMethodStartEndEventInfo() is not { Frame: var fqn })
    {
      logger.LogDebug("Will not process ");
      return null;
    }

    if (myCounter != 0)
    {
      logger.LogDebug("Will not create new writer for method {Fqn} as counter is {Counter}", fqn, myCounter);
      return myCurrentState;
    }

    logger.LogInformation("Created new state for {Fqn}", fqn);
    fqn = beautifier.Beautify(fqn);

    var index = mySeenMethods.GetOrCreate(fqn, static () => 0);
    mySeenMethods[fqn] += 1;

    var outputFileName = Path.Combine(outputDirectory, $"{index}_{fqn}.ocel.csv");

    var state = new OcelMethodWriteState(new MethodOcelLogWriter(outputFileName, logger));

    myCurrentState = state;
    myStates.Add(state);

    return state;
  }

  public void HandleUpdate(EventUpdateBase update)
  {
    if (update.FrameInfo.State is not OcelMethodWriteState state) return;

    switch (update)
    {
      case MethodStartedUpdate:
        myCounter++;
        break;
      case MethodFinishedUpdate:
        myCounter--;
        break;
      case NormalEventUpdate { Event: var eventRecord }:
        state.Writer.Process(eventRecord);
        break;
    }
  }

  public void Dispose()
  {
    foreach (var state in myStates)
    {
      try
      {
        state.Writer.Flush();
      }
      catch (Exception ex)
      {
        logger.LogError(ex, "Failed to flush ocel writer");
      }
    }
  }
}