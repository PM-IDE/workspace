using System.Text.Json;
using NUnit.Framework;

namespace Bxes.Tests.Core;

public static class AssertUtil
{
  public static void CompareAndSerializeIfFail<T>(T originalValue, T testValue) where T : IEquatable<T>
  {
    if (originalValue.Equals(testValue)) return;

    var tempPath = Path.GetTempFileName();
    File.WriteAllText(tempPath, JsonSerializer.Serialize(originalValue));
    Assert.Fail($"{typeof(T)}s are not equal, serialized original {typeof(T)} to {tempPath}");
  }
}