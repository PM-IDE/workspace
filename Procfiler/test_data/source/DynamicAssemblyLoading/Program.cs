// See https://aka.ms/new-console-template for more information

using System.Reflection;

namespace DynamicAssemblyLoading;

internal class Program
{
  public static void Main(string[] args)
  {
    Console.WriteLine("Hello, World!");
    var path = Path.Combine(Directory.GetCurrentDirectory(), "SomeAssembly", "SomeAssembly.dll");
    Console.WriteLine(File.Exists(path));
    var asm = Assembly.LoadFrom(path);
    var classInstance = asm.CreateInstance("SomeAssembly.Class1");
    Console.WriteLine(classInstance.GetType().Name);
  }
}