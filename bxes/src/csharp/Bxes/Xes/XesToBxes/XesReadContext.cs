using Bxes.Logging;
using Bxes.Models;
using Bxes.Models.Domain;
using Bxes.Writer.Stream;

namespace Bxes.Xes.XesToBxes;

public readonly struct XesReadContext(SingleFileBxesStreamWriterImpl<FromXesBxesEvent> writer, ILogger logger)
{
  public ILogger Logger { get; } = logger;
  public SingleFileBxesStreamWriterImpl<FromXesBxesEvent> Writer { get; } = writer;
  public Dictionary<string, BxesValue> EventDefaults { get; } = new();
}