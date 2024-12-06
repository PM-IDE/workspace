namespace FicusFrontend.Components.SidebarList;

public class CollectionItemInfo<TItem, TInnerItem, TId> where TId : notnull
{
  public required TId Id { get; init; }
  public required string Name { get; init; }
  public required TItem Item { get; init; }
  public required List<InnerCollectionItemInfo<TInnerItem, TId>> InnerItems { get; init; }
}

public class InnerCollectionItemInfo<TCollectionInnerItem, TId> where TId : notnull
{
  public required TId Id { get; init; }
  public required TCollectionInnerItem InnerItem { get; init; }
  public required ListItemInfo ListItemInfo { get; init; }
}

public class ListItemInfo
{
  public required DateTime UpdatedAt { get; init; }
  public required string Name { get; init; }
  public required ItemProcessingState ProcessingState { get; set; }
}

public enum ItemProcessingState
{
  New,
  Updated,
  Seen
}
