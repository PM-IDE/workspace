using Bxes.Models.Domain;
using Bxes.Models.System;

namespace Bxes.Reader;

public record struct EventLogReadResult(IEventLog EventLog, ISystemMetadata SystemMetadata);

public interface IBxesReader
{
  EventLogReadResult Read(string path);
}