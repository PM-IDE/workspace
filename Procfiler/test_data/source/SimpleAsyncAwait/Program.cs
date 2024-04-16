namespace SimpleAsyncAwait;


internal class Program
{
  public static async Task Main(string[] args)
  {
    Task.Factory.StartNew(Console.WriteLine).Wait();
    var x = new Class1();
    await Task.Factory.StartNew(() => 1 + 1);
    var z = new Class2();
    var y = await Foo();
    var result = y + 1;
    var xxx = new Class3();

    async Task<int> Foo()
    {
      return Convert.ToInt32("1");
    }
  }
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