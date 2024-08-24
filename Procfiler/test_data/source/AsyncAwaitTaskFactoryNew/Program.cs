namespace AsyncAwaitTaskFactoryNew;

public static class Program
{
  public static async Task Main()
  {
    await Task.Factory.StartNew(() =>
    {
      var xd = Allocate1();
    });

    await await Task.Factory.StartNew(async () =>
    {
      var xd = Allocate2();
      await Task.Delay(100);
    });
    
    await Task.Factory.StartNew(() =>
    {
      var xd = Allocate3();
    });
  }

  static object Allocate1() => new XdClass1();
  static object Allocate2() => new XdClass2();
  static object Allocate3() => new XdClass3();
}

class XdClass1;

class XdClass2;

class XdClass3;