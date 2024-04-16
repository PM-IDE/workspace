// See https://aka.ms/new-console-template for more information

using System.Reflection;
using System.Runtime.Loader;

namespace NotExistingAssemblyLoading;

internal class Program
{
  public static void Main(string[] args)
  {
    AppDomain.CurrentDomain.AssemblyResolve += (_, _) =>
    {
      Console.WriteLine("Hello from App Domain assembly resolve");
      return null;
    };

    try
    {
      var loadContext = new AssemblyLoadContext(null);
      loadContext.Resolving += (_, _) =>
      {
        Console.WriteLine("Hello from Load Context handler");
        return null;
      };

      loadContext.LoadFromAssemblyName(new AssemblyName("asdasdasdasd"));
    }
    catch (Exception ex)
    {
      Console.WriteLine(ex.Message);
    }
  }
}