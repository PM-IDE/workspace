using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler.ShadowStacks;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.EventsProcessing.Core;

namespace Procfiler.Core.EventsProcessing.Mutators.MultipleEventsMutators;

public interface IMethodStartEndEventsLogMutator : IMultipleEventsMutator;

public interface IMethodsStartEndProcessor
{
  void Process(IEventsCollection events, IGlobalDataWithStacks context);
}

[EventMutator(MultipleEventMutatorsPasses.MethodStartEndInserter)]
public class MethodStartEndEventsLogMutator(
  IProcfilerEventsFactory factory,
  IProcfilerLogger logger) : IMethodStartEndEventsLogMutator
{
  public IEnumerable<EventLogMutation> Mutations { get; } =
  [
    new AddEventMutation(TraceEventsConstants.ProcfilerMethodStart),
    new AddEventMutation(TraceEventsConstants.ProcfilerMethodEnd)
  ];


  public void Process(IEventsCollection events, IGlobalDataWithStacks context)
  {
    if (events.Count == 0) return;

    IMethodsStartEndProcessor processor = context.Stacks switch
    {
      IFromEventsShadowStacks => new FromEventsMethodsStartEndMutator(factory, logger),
      ICppShadowStacks => new CppStacksMethodsStartEndMutator(factory, logger, true),
      _ => throw new ArgumentOutOfRangeException(context.Stacks.GetType().Name)
    };

    processor.Process(events, context);
  }
}