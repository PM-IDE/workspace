
namespace AsyncDisposable;

public static class Program
{
  public static async Task Main()
  {
    await Method1();
    await using var xd = new AsyncDisposable();
  }

  public static async Task<int> Method1()
  {
    await Task.Delay(100);
    return 1;
  }

  private class AsyncDisposable : IAsyncDisposable
  {
    public async ValueTask DisposeAsync()
    {
      await Method1();
    }
  }
}