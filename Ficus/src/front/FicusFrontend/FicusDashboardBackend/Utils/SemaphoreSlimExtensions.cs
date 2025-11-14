namespace FicusDashboardBackend.Utils;

public static class SemaphoreSlimExtensions
{
  extension(SemaphoreSlim semaphoreSlim)
  {
    public async Task Execute(Func<Task> action)
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

    public async Task<T> Execute<T>(Func<Task<T>> action)
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
}