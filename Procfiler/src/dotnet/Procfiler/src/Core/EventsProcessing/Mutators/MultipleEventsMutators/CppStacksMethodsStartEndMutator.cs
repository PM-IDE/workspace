using Core.Events.EventsCollection;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler.ShadowStacks;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core.EventsProcessing.Mutators.MultipleEventsMutators;

public class CppStacksMethodsStartEndMutator(
  IProcfilerEventsFactory factory,
  IProcfilerLogger logger,
  bool aggressiveReuse) : IMethodsStartEndProcessor
{
  public void Process(IEventsCollection events, SessionGlobalData context)
  {
    if (context.Stacks is not ICppShadowStacks cppShadowStacks)
    {
      var name = context.Stacks.GetType().Name;
      logger.LogError("Not compatible shadow stacks, got {Type}, expected {Type}", name, nameof(ICppShadowStacks));

      return;
    }

    using var collectionEnumerator = events.GetEnumerator();
    if (!collectionEnumerator.MoveNext())
    {
      return;
    }

    var managedThreadId = collectionEnumerator.Current.Event.ManagedThreadId;
    if (cppShadowStacks.FindShadowStack(managedThreadId) is not { } foundShadowStack)
    {
      logger.LogWarning("Managed thread {Id} was not in shadow stacks", managedThreadId);
      return;
    }

    if (foundShadowStack.FramesCount == 0)
    {
      logger.LogWarning("Skipping shadow stack for {Id} because it does not contain frames", managedThreadId);
      return;
    }

    var modificationSource = new MethodStartEndModificationSource(logger, factory, context, foundShadowStack, aggressiveReuse);
    events.InjectModificationSource(modificationSource);
  }
}