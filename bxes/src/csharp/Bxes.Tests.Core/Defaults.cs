using Bxes.Utils;

namespace Bxes.Tests.Core;

public static class Defaults
{
  public static RandomLogGenerationParameters DefaultRandomLogGenerationParameters { get; } = new()
  {
    EventsCount = new LowerUpperBound(1, 10),
    VariantsCount = new LowerUpperBound(5, 10),
    EventAttributesCount = new LowerUpperBound(1, 10)
  };
}