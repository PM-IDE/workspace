// See https://aka.ms/new-console-template for more information

namespace TaskTestProject1;

internal class Program
{
  public static void Main(string[] args)
  {
    Console.WriteLine("Hello, World!");

    var x = 0;
    var task1 = new Task<int>(() => ++x);
    var task2 = task1.ContinueWith(result =>
    {
      x += result.Result;
    });

    task1.Start();

    task2.Wait();
    Console.WriteLine(x);
  }
}