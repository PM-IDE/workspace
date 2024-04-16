using System.Xml;

namespace ConsoleApp1;

class Program
{
  public static void Main()
  {
    Method1();
  }

  public static void Method1()
  {
    var p = new Program();
    Method2();
    p = new Program();
    GC.Collect(2);
  }

  public static void Method2()
  {
    try
    {
      throw new XmlException("asdasasd");
    }
    catch (Exception ex)
    {
      Console.WriteLine(ex.Message);
    }
  }
}
