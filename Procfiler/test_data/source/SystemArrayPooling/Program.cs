using System.Buffers;

namespace SystemArrayPooling;

internal class Program
{
  public static void Main(string[] args)
  {
    var pool = ArrayPool<string>.Shared;

    var arrays = new List<string[]>();

    for (var j = 0; j < 10; ++j)
    {
      for (var i = 0; i < 100; ++i)
      {
        arrays.Add(pool.Rent(Random.Shared.Next(0, 1000 * j)));
      }

      while (arrays.Count > 0)
      {
        var index = Random.Shared.Next(arrays.Count);
        var buffer = arrays[index];
        arrays.RemoveAt(index);
        pool.Return(buffer);
      } 
    }
  }
}