// See https://aka.ms/new-console-template for more information

namespace LOHAllocations;

internal class Program
{
  public static void Main(string[] args)
  {
    var arrays = new List<byte[]>();
    for (var i = 0; i < 100000; ++i)
    {
      var length = (i % 123 == 0) switch
      {
        true => 1_123_123,
        false => 10_000,
      };
  
      var array = new byte[length];
      arrays.Add(array);

      if (i > 1000 && i % 100 == 0)
      {
        for (var j = 0; j < 100; ++j)
        {
          var count = arrays.Count;
          var index = Random.Shared.Next(count);
          arrays.RemoveAt(index);
        }
      }
    }

    Console.WriteLine(arrays.Count);
  }
}