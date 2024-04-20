using Bxes.Writer;

namespace Bxes.Models.Values.Lifecycle;

public class StandardXesLifecycle(StandardLifecycleValues value) : EventLifecycle<StandardLifecycleValues>(value)
{
  public static StandardXesLifecycle Parse(byte value) => Enum.IsDefined(typeof(StandardLifecycleValues), value) switch
  {
    true => new StandardXesLifecycle((StandardLifecycleValues)value),
    false => throw new IndexOutOfRangeException()
  };

  public override TypeIds TypeId => TypeIds.StandardLifecycle;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write((byte)Value);
  }

  public override bool IsDefined() => value != StandardLifecycleValues.Unspecified;
  public override string ToStringValue() => value.ToString().ToLower();
}