namespace Procfiler.Utils;

public static class EmptyCollections<T>
{
  public static List<T> EmptyList { get; } = [];
  public static HashSet<T> EmptySet { get; } = [];
  public static SortedSet<T> EmptySortedSet { get; } = [];
  public static LinkedList<T> EmptyLinkedList { get; } = [];
}