using FicusDashboard.Services.Cases;
using FicusDashboard.Utils;

namespace FicusDashboard.Components.TreeView;

public class TreeViewItem : FrontModelBase
{
  public required TreeViewItem? Parent { get; init; }
  public required string DisplayName { get; set; }
  public required string Id { get; set; }
  public required Dictionary<string, TreeViewItem> InnerItems { get; set; }

  public bool IsExpanded { get; set; } = true;
}