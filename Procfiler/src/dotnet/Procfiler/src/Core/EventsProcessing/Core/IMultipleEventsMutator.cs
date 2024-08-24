using Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.EventsProcessing.Core;

public interface IMultipleEventsMutator : IEventsLogMutator
{
  void Process(IEventsCollection events, IGlobalDataWithStacks context);
}