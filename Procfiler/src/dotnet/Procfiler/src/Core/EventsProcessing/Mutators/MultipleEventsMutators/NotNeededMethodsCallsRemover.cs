﻿using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.EventsProcessing.Core;

namespace Procfiler.Core.EventsProcessing.Mutators.MultipleEventsMutators;

[EventMutator(MultipleEventMutatorsPasses.NotNeededMethodsRemove)]
public class NotNeededMethodsCallsRemover : IMultipleEventsMutator
{
  private static Regex[] PatternsToRemove { get; } =
  [
    new(@"System\.Diagnostics\.Tracing\..*")
  ];


  public IEnumerable<EventLogMutation> Mutations => EmptyCollections<EventLogMutation>.EmptyList;


  public void Process(IEventsCollection events, IGlobalDataWithStacks context)
  {
    events.AddFilter(eventRecord => eventRecord.TryGetMethodStartEndEventInfo() is var (frameName, _) && ShouldSkipFrame(frameName));
  }

  private static bool ShouldSkipFrame(string frameName)
  {
    foreach (var regex in PatternsToRemove)
    {
      if (regex.IsMatch(frameName)) return true;
    }

    return false;
  }
}