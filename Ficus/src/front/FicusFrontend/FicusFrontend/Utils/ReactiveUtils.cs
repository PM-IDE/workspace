using JetBrains.Collections.Viewable;
using JetBrains.Lifetimes;

namespace FicusFrontend.Utils;

public static class ReactiveUtils
{
  public static void FlowIntoWithCallback<TKey, TValue>(
    this IViewableMap<TKey, TValue> map,
    Lifetime lifetime,
    IDictionary<TKey, TValue> target,
    Action callback
  ) where TKey : notnull
  {
    map.Advise(lifetime, update =>
    {
      switch (update.Kind)
      {
        case AddUpdateRemove.Add or AddUpdateRemove.Update:
        {
          if (update.NewValue is null) return;

          target[update.Key] = update.NewValue;
          break;
        }
        case AddUpdateRemove.Remove:
        {
          target.Remove(update.Key);
          break;
        }
      }

      callback();
    });
  }
}