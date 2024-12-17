using Bxes.Models.Domain.Values;
using Bxes.Writer;
using Core.Events.EventRecord;
using ProcfilerOnline.Integrations.Kafka.Bxes;

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

  public static BxesKafkaCaseName ToBxesKafkaCaseName(this ExtendedMethodInfo methodInfo) => new()
  {
    DisplayName = methodInfo.Name,
    NameParts = [methodInfo.Namespace, methodInfo.Name, methodInfo.Signature]
  };
}

