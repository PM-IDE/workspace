using Procfiler.Core.Exceptions;

namespace Procfiler.Core.EventsCollection;

public readonly record struct EventPointer(
  bool IsInFirstEvents,
  int IndexInArray,
  int IndexInInsertionMap,
  IEventsOwner Owner)
{
  public static EventPointer ForFirstEvent(int indexInFirstEvents, IEventsOwner owner) =>
    new(true, -1, indexInFirstEvents, owner);

  public static EventPointer ForInitialArray(int indexInInitialArray, IEventsOwner owner) =>
    new(false, indexInInitialArray, -1, owner);

  public static EventPointer ForInsertionMap(int indexInInitialArray, int indexInInsertionMap, IEventsOwner owner) =>
    new(false, indexInInitialArray, indexInInsertionMap, owner);


  public bool IsInInitialArray => IndexInInsertionMap == -1 && IndexInArray >= 0;

  public bool IsInInsertedMap => IndexInInsertionMap >= 0 &&
                                 (IndexInArray >= 0 && !IsInFirstEvents || IsInFirstEvents && IndexInArray == -1);


  public void AssertStateOrThrow()
  {
    if (!((IsInInitialArray && !IsInInsertedMap) || (!IsInInitialArray && IsInInsertedMap)))
    {
      throw new InvalidStateException($"array: {IsInInitialArray} , map: {IsInInsertedMap}");
    }
  }

  public override int GetHashCode() => HashCode.Combine(IsInFirstEvents, IndexInArray, IndexInInsertionMap);
}