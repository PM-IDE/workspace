using Bxes.Models;
using Bxes.Writer;
using Bxes.Writer.Stream;

namespace Bxes.Xes.XesToBxes;

public class XesToBxesHandler(IBxesStreamWriter writer) : XesElementHandlerBase
{
  public override void HandleProperty(AttributeKeyValue property) => 
    writer.HandleEvent(new BxesLogMetadataPropertyEvent(property));

  public override void HandleTraceStart() => 
    writer.HandleEvent(new BxesTraceVariantStartEvent(1, new List<AttributeKeyValue>()));

  public override void HandleEvent(FromXesBxesEvent fromXesBxesEvent) => 
    writer.HandleEvent(new BxesEventEvent<FromXesBxesEvent>(fromXesBxesEvent));

  public override void HandleClassifier(BxesClassifier classifier) => 
    writer.HandleEvent(new BxesLogMetadataClassifierEvent(classifier));

  public override void HandleExtension(BxesExtension extension) => 
    writer.HandleEvent(new BxesLogMetadataExtensionEvent(extension));

  public override void HandleGlobal(BxesGlobal global) => 
    writer.HandleEvent(new BxesLogMetadataGlobalEvent(global));
}