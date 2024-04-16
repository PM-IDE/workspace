// See https://aka.ms/new-console-template for more information

namespace LOHAllocations;

internal class Program
{
  public static void Main(string[] args)
  {
    var largeArrays = new List<byte[]>();
    for (var i = 0; i < 100000; ++i)
    {
      var length = (i % 123 == 0) switch
      {
        true => 1_123_123,
        false => 86_000,
      };
  
      var array = new byte[length];
      largeArrays.Add(array);

      if (i > 1000 && i % 100 == 0)
      {
        for (var j = 0; j < 100; ++j)
        {
          largeArrays.RemoveAt(Random.Shared.Next(largeArrays.Count));
        }
      }
    }

    Console.WriteLine(largeArrays.Count);
  }
}