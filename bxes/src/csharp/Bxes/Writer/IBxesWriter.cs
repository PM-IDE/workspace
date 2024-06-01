using Bxes.Models.Domain;

namespace Bxes.Writer;

public interface IBxesWriter
{
  void Write(IEventLog log, string savePath);
}