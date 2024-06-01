using Bxes.Models.Domain;
using Bxes.Writer;

namespace Bxes.Xes.XesToBxes;

public abstract class XesElementHandlerBase
{
  public abstract void HandleProperty(AttributeKeyValue property);
  public abstract void HandleTraceStart();
  public abstract void HandleEvent(FromXesBxesEvent fromXesBxesEvent);
  public abstract void HandleClassifier(BxesClassifier classifier);
  public abstract void HandleExtension(BxesExtension extension);
  public abstract void HandleGlobal(BxesGlobal global);
}