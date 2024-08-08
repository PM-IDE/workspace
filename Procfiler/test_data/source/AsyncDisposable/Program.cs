
namespace AsyncDisposable;

public static class Program
{
  public static async Task Main()
  {
    await Method1();
    await using var xd = new Nested1.Nested2.Nested3.AsyncDisposable();
    await Task.Delay(1000);
  }

  public static async Task<int> Method1()
  {
    await Task.Delay(100);
    return 1;
  }

  private class Nested1
  {
    public class Nested2
    {
      public class Nested3
      {
        public class AsyncDisposable : IAsyncDisposable
        {
          public async ValueTask DisposeAsync()
          {
            await Method1();
          }
        }     
      }
    }
  }
}