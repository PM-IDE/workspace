using Bxes.Models;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Writer;
using Bxes.Writer.Stream;

namespace Bxes.Xes.XesToBxes;

public class XesValuesPreprocessor(IBxesStreamWriter writer) : XesElementHandlerBase
{
  public override void HandleProperty(AttributeKeyValue property) => 
    writer.HandleEvent(new BxesKeyValueEvent(property));

  public override void HandleTraceStart()
  {
  }

  public override void HandleEvent(FromXesBxesEvent fromXesBxesEvent)
  {
    writer.HandleEvent(new BxesValueEvent(new BxesStringValue(fromXesBxesEvent.Name)));

    foreach (var attribute in fromXesBxesEvent.Attributes)
    {
      writer.HandleEvent(new BxesKeyValueEvent(attribute));
    }
  }

  public override void HandleClassifier(BxesClassifier classifier)
  {
    writer.HandleEvent(new BxesValueEvent(classifier.Name));
    foreach (var value in classifier.Keys)
    {
      writer.HandleEvent(new BxesValueEvent(value));
    }
  }

  public override void HandleExtension(BxesExtension extension)
  {
    writer.HandleEvent(new BxesValueEvent(extension.Name));
    writer.HandleEvent(new BxesValueEvent(extension.Prefix));
    writer.HandleEvent(new BxesValueEvent(extension.Uri));
  }

  public override void HandleGlobal(BxesGlobal global)
  {
    foreach (var attribute in global.Globals)
    {
      writer.HandleEvent(new BxesKeyValueEvent(attribute));
    }
  }
}