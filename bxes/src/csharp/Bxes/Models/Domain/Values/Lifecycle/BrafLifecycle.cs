using Bxes.Writer;

namespace Bxes.Models.Domain.Values.Lifecycle;

public class BrafLifecycle(BrafLifecycleValues value) : EventLifecycle<BrafLifecycleValues>(value)
{
  public static BrafLifecycle Parse(byte value) => Enum.IsDefined(typeof(BrafLifecycleValues), value) switch
  {
    true => new BrafLifecycle((BrafLifecycleValues)value),
    false => throw new IndexOutOfRangeException()
  };

  public override TypeIds TypeId => TypeIds.BrafLifecycle;


  public override void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);
    context.Writer.Write((byte)Value);
  }

  public override bool IsDefined() => value != BrafLifecycleValues.Unspecified;

  public override string ToStringValue() => value.ToString().ToLower();
}