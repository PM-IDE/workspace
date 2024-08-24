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
  }
  
  static object Allocate1() => new Class1();
  static object Allocate2() => new Class2();
  static object Allocate3() => new Class3();
  static object Allocate4() => new Class4();

  static object Allocate(int index) => index switch
  {
    0 => Allocate1(),
    1 => Allocate2(),
    2 => Allocate3(),
    3 => Allocate4(),
    _ => throw new ArgumentOutOfRangeException(nameof(index), index, null)
  };
}

class Class1 {}
class Class2 {}
class Class3 {}
class Class4 {}