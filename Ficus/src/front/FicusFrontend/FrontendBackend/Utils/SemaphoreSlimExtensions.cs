namespace FrontendBackend.Utils;

public static class SemaphoreSlimExtensions
{
  public static async Task Execute(this SemaphoreSlim semaphoreSlim, Func<Task> action)
  {
    try
    {
      await semaphoreSlim.WaitAsync();
      await action();
    }
    finally
    {
      semaphoreSlim.Release();
    }
  }

  public static async Task<T> Execute<T>(this SemaphoreSlim semaphoreSlim, Func<Task<T>> action)
  {
    try
    {
      await semaphoreSlim.WaitAsync();
      return await action();
    }
    finally
    {
      semaphoreSlim.Release();
    }
  }
}