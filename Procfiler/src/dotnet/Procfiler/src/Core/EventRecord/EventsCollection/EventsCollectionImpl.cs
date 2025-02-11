﻿using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord.EventsCollection.ModificationSources;

namespace Procfiler.Core.EventRecord.EventsCollection;

public class EventsCollectionImpl(EventRecordWithMetadata[] initialEvents, IProcfilerLogger logger)
  : EventsOwnerBase(logger, initialEvents.Length), IEventsCollection
{
  private readonly IProcfilerLogger myLogger = logger;
  private readonly List<IModificationSource> myModificationSources = [];


  public override long Count => PointersManager.Count + myModificationSources.Select(source => source.Count).Sum();


  public void InjectModificationSource(IModificationSource modificationSource)
  {
    myModificationSources.Add(modificationSource);
  }

  public void ApplyNotPureActionForAllEvents(Func<EventRecordWithPointer, bool> action)
  {
    foreach (var eventWithPtr in this)
    {
      var shouldStop = action(eventWithPtr);
      if (shouldStop) return;
    }
  }

  public override bool Remove(EventPointer pointer)
  {
    AssertNotFrozen();
    if (!ReferenceEquals(pointer.Owner, this))
    {
      if (TryFindModificationSourceForOwner(pointer) is { } modificationSource)
      {
        return modificationSource.Remove(pointer);
      }

      return false;
    }

    return PointersManager.Remove(pointer);
  }

  private IModificationSource? TryFindModificationSourceForOwner(EventPointer pointer)
  {
    foreach (var modificationSource in myModificationSources)
    {
      if (ReferenceEquals(modificationSource, pointer.Owner))
      {
        return modificationSource;
      }
    }

    myLogger.LogError("Failed to find modification source for {Owner}, skipping remove", pointer.Owner.GetType().Name);
    return null;
  }

  public override EventPointer InsertAfter(EventPointer pointer, EventRecordWithMetadata eventToInsert)
  {
    AssertNotFrozen();
    if (!ReferenceEquals(pointer.Owner, this))
    {
      if (TryFindModificationSourceForOwner(pointer) is { } modificationSource)
      {
        return modificationSource.InsertAfter(pointer, eventToInsert);
      }

      throw new ArgumentOutOfRangeException();
    }

    return base.InsertAfter(pointer, eventToInsert);
  }

  public override EventPointer InsertBefore(EventPointer pointer, EventRecordWithMetadata eventToInsert)
  {
    AssertNotFrozen();
    if (!ReferenceEquals(pointer.Owner, this))
    {
      if (TryFindModificationSourceForOwner(pointer) is { } modificationSource)
      {
        return modificationSource.InsertBefore(pointer, eventToInsert);
      }

      throw new ArgumentOutOfRangeException();
    }

    return base.InsertBefore(pointer, eventToInsert);
  }

  protected override IEnumerable<EventRecordWithMetadata> EnumerateInitialEvents() => initialEvents;

  public override IEnumerator<EventRecordWithPointer> GetEnumerator()
  {
    var enumerators = new List<IEnumerable<EventRecordWithPointer>>
    {
      EnumerateInternal()
    };

    foreach (var modificationSource in myModificationSources)
    {
      enumerators.Add(modificationSource);
    }

    return new OrderedEventsEnumerator(enumerators);
  }
}