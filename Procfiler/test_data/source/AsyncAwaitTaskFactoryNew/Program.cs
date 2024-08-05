namespace AsyncAwaitTaskFactoryNew;

public static class Program
{
  public static async Task Main()
  {
    await Task.Factory.StartNew(() =>
    {
      var xd = new XdClass1();
    });

    await Task.Factory.StartNew(async () =>
    {
      var xd = new XdClass2();
      await Task.Delay(100);
    });
    
    await Task.Factory.StartNew(() =>
    {
      var xd = new XdClass3();
    });
    
    Thread.Sleep(1000);
  }
}

class XdClass1;

class XdClass2;

class XdClass3;