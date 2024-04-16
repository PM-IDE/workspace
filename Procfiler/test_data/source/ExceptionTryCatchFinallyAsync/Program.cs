namespace ExceptionTryCatchFinallyAsync;

internal class Program
{
  public static async Task Main(string[] args)
  {
    await Task.WhenAll(Enumerable.Range(0, 10).Select(i => Task.Factory.StartNew(() =>
    {
      X(i).GetAwaiter().GetResult();
    })).ToList());

    async Task X(int index)
    {
      try
      {
        throw new Exception();
      }
      catch (Exception ex)
      {
        Console.WriteLine($"Catch start {index}: {Environment.CurrentManagedThreadId}");
        await Print().ConfigureAwait(false);
        Console.WriteLine($"Catch end {index}: {Environment.CurrentManagedThreadId}");
      }
      finally
      {
        Console.WriteLine($"Finally start {index}: {Environment.CurrentManagedThreadId}");
        await Print().ConfigureAwait(false);
        Console.WriteLine($"Finally end {index}: {Environment.CurrentManagedThreadId}");
      }
    }

    async Task Print()
    {
      await Task.Delay(100);
      Console.WriteLine("Hello");
    }
  }
}