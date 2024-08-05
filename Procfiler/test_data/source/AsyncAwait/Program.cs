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
    var x = new Method1();
    await Task.Delay(1000);
    x = new Method1();
    return 1;
  }

  static async Task Method2()
  {
    var x = new Method2();
    var xd = await Method1();
    x = new Method2();
  }

  static async Task Method3()
  {
    var xd = new Method3();
  }
}

class Method1;

class Method2;

class Method3;