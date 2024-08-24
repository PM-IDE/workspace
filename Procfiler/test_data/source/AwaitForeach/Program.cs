
namespace AwaitForeach;

public static class Program
{
  public static async Task Main()
  {
    await foreach (var xd in EnumerateAsync())
    {
      Console.WriteLine(xd);
    }
  }

  static async IAsyncEnumerable<object> EnumerateAsync()
  {
    for (var i = 0; i < 10; ++i)
    {
      yield return await Allocate();
    }
  }

  static async Task<object> Allocate() => new();
}