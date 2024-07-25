// See https://aka.ms/new-console-template for more information

namespace NotSimpleAsyncAwait;

internal class Program
{
  public static async Task Main(string[] args)
  {
    Console.WriteLine("Hello, World!");

    for (var i = 0; i < 4; i++)
    {
      Allocate(i);
      var index = i;
      var list = new List<object>();
      await await Task.Factory.StartNew(async () =>
      {
        list.Add(Allocate(index));
        list.Add(Allocate(index));
        list.Add(Allocate(index));
        list.Add(Allocate(index));
        list.Add(Allocate(index));
        list.Add(Allocate(index));
        await Task.Delay(100);
        list.Add(Allocate(index));
        await Task.Delay(100);
        list.Add(Allocate(index));
      });

      Console.WriteLine(list.Count);
    }


    object Allocate(int index) => index switch
    {
      0 => new Class1(),
      1 => new Class2(),
      2 => new Class3(),
      3 => new Class4(),
      _ => throw new ArgumentOutOfRangeException(nameof(index), index, null)
    };
  }
}

class Class1 {}
class Class2 {}
class Class3 {}
class Class4 {}