using Bxes.Models;

namespace Bxes.Writer.Stream;

public interface IBxesStreamWriter : IDisposable
{
  void HandleEvent(BxesStreamEvent @event);
}

public interface IXesToBxesStatisticsCollector
{
  XesToBxesConversionStatistics ObtainStatistics();
}

public readonly struct XesToBxesConversionStatistics
{
  public required IReadOnlyDictionary<BxesValue, int> Values { get; init; }
  public required IReadOnlyDictionary<AttributeKeyValue, int> Attributes { get; init; }
}