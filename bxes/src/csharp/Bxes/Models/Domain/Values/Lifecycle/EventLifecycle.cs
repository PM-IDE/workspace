namespace Bxes.Models.Values.Lifecycle;

public abstract class EventLifecycle<TLifecycleValue>(TLifecycleValue value)
  : BxesValue<TLifecycleValue>(value), IEventLifecycle where TLifecycleValue : notnull
{
  public abstract bool IsDefined();
  public abstract string ToStringValue();
}