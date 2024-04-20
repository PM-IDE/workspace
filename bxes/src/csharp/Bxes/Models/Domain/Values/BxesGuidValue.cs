using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesGuidValue(Guid guid) : BxesValue<Guid>(guid)
{
  public override TypeIds TypeId => TypeIds.Guid;


  public override unsafe void WriteTo(BxesWriteContext context)
  {
    base.WriteTo(context);

    Span<byte> guidBytes = stackalloc byte[16];
    Value.TryWriteBytes(guidBytes);

    context.Writer.Write(guidBytes);
  }
}