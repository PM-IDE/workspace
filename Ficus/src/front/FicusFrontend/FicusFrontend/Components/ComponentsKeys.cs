using FicusFrontend.Components.SidebarList;
using FicusFrontend.Utils;

namespace FicusFrontend.Components;

public static class ComponentsKeys
{
  public static Key<ItemProcessingState> ProcessingStateKey { get; } = new(nameof(ProcessingStateKey));
}