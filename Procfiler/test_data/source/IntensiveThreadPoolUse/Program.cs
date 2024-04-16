// See https://aka.ms/new-console-template for more information

namespace IntensiveThreadPoolUse;

internal class Program
{
  public static void Main(string[] args)
  {
    for (int i = 0; i < 10_000; ++i)
    {
      ThreadPool.QueueUserWorkItem(Console.WriteLine, i, false);
    }

    Thread.Sleep(2000);
  }
}