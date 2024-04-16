namespace Procfiler.Utils;

public static class EmptyCollections<T>
{
  public static List<T> EmptyList { get; } = new();
  public static HashSet<T> EmptySet { get; } = new();
  public static SortedSet<T> EmptySortedSet { get; } = new();
  public static LinkedList<T> EmptyLinkedList { get; } = new();
}