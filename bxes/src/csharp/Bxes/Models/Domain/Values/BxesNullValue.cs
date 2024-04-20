using Bxes.Writer;

namespace Bxes.Models.Domain.Values;

public class BxesNullValue : BxesValue
{
    public static BxesNullValue Instance { get; } = new();


    public override TypeIds TypeId => TypeIds.Null;


    private BxesNullValue()
    {
    }
    

    public override void WriteTo(BxesWriteContext context)
    {
        context.Writer.Write((byte)0);
    }
}