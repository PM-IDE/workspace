using Bxes.Writer;

namespace Bxes.Models.Domain.Values.Lifecycle;

public interface IEventLifecycle
{
  public static IEventLifecycle Parse(string value)
  {
    if (StandardLifecycleValuesUtil.TryParse(value) is { } standardLifecycle) 
      return new StandardXesLifecycle(standardLifecycle);

    if (BrafLifecycleValuesUtil.TryParse(value) is { } brafLifecycleValues)
      return new BrafLifecycle(brafLifecycleValues);

    return new StandardXesLifecycle(StandardLifecycleValues.Unspecified);
  }

  void WriteTo(BxesWriteContext context);
  bool IsDefined();
  string ToStringValue();
}