using Bxes.Models;

namespace Bxes.Reader;

public interface IBxesReader
{
  IEventLog Read(string path);
}