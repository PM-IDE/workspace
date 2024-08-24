namespace SimpleAsyncAwait;


internal class Program
{
  public static async Task Main()
  {
    Task.Factory.StartNew(Console.WriteLine).Wait();
    var x = Allocate1();
    await Task.Factory.StartNew(() => 1 + 1);
    var z = Allocate2();
    var y = await Foo();
    var result = y + 1;
    var xxx = Allocate3();

    async Task<int> Foo()
    {
      return Convert.ToInt32("1");
    }
  }

  static Class1 Allocate1() => new();
  static Class2 Allocate2() => new();
  static Class3 Allocate3() => new();
}

class Class1
{
}

class Class2
{
  
}

class Class3
{
}