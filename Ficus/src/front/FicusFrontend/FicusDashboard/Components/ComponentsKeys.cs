using FicusDashboard.Components.SidebarList;
using FicusDashboard.Utils;

namespace FicusDashboard.Components;

public static class ComponentsKeys
{
  public static Key<ItemProcessingState> ProcessingStateKey { get; } = new(nameof(ProcessingStateKey));
}