// See https://aka.ms/new-console-template for more information

namespace YieldEnumerator;

internal class Program
{
  public static void Main(string[] args)
  {
    foreach (var enumerateSomeNumber in EnumerateSomeNumbers())
    {
      var x = new Program();
      Console.WriteLine(enumerateSomeNumber);
    }

    IEnumerable<int> EnumerateSomeNumbers()
    {
      yield return 1;
      yield return 2;
      yield return 3;
      yield return 13;
      yield return 14;
    }
  }
}
