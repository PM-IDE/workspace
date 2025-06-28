using FicusDashboard.Utils;

namespace FicusDashboard.Components.SidebarList;

public class CollectionItemInfo<TItem, TInnerItem, TId>
  where TId : notnull where TItem : FrontModelBase where TInnerItem : FrontModelBase
{
  public required TId Id { get; init; }
  public required string Name { get; init; }
  public required TItem Item { get; init; }
  public required Dictionary<TId, InnerCollectionItemInfo<TInnerItem, TId>> InnerItems { get; init; }
}

public class InnerCollectionItemInfo<TInnerItem, TId> where TId : notnull where TInnerItem : FrontModelBase
{
  public required TId Id { get; init; }
  public required TInnerItem InnerItem { get; init; }
  public required ListItemInfo ListItemInfo { get; init; }
}

public class ListItemInfo
{
  public required DateTime UpdatedAt { get; init; }
  public required string Name { get; init; }
  public required List<string> NameParts { get; init; }
}

public enum ItemProcessingState
{
  New,
  Updated,
  Seen
}

public static class ItemProcessingStateExtensions
{
  public static string GetNotificationClass(this ItemProcessingState state) => state switch
  {
    ItemProcessingState.New => "new-notification",
    ItemProcessingState.Updated => "update-notification",
    ItemProcessingState.Seen => "seen-notification",
    _ => throw new ArgumentOutOfRangeException()
  };
}