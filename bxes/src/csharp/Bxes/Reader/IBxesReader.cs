using Bxes.Models.Domain;

namespace Bxes.Reader;

public interface IBxesReader
{
  IEventLog Read(string path);
}