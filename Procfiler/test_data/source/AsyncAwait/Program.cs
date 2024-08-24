namespace AsyncAwait;

public class Program
{
  public static async Task Main()
  {
    await Method1();
    await Method2();

    await Method3();
    
    await Method2();
    
    await Method3();
  }

  static async Task<int> Method1()
  {
    var x = Allocate1();
    await Task.Delay(1000);
    x = Allocate1();
    return 1;
  }

  static async Task Method2()
  {
    var x = Allocate2();
    var xd = await Method1();
    x = Allocate2();
  }

  static async Task Method3()
  {
    var xd = Allocate3();
  }

  static Method1 Allocate1() => new();
  static Method2 Allocate2() => new();
  static Method3 Allocate3() => new();
}

class Method1;

class Method2;

class Method3;