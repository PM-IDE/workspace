using Bxes.Models.Domain.Values;
using Bxes.Writer;
using Core.Events.EventRecord;

namespace ProcfilerOnline.Core.Handlers;

public static class HandlerUtil
{
  public static void AddToMetadata(this ExtendedMethodInfo? methodInfo, List<AttributeKeyValue> metadata)
  {
    if (methodInfo is null) return;

    metadata.AddRange([
      new AttributeKeyValue(new BxesStringValue("method_name"), new BxesStringValue(methodInfo.Name)),
      new AttributeKeyValue(new BxesStringValue("method_signature"), new BxesStringValue(methodInfo.Signature))
    ]);
  }
}

