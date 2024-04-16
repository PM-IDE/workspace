// See https://aka.ms/new-console-template for more information

namespace UnsafeFixed;

internal class Program
{
  public static void Main(string[] args)
  {
    unsafe
    {
      var array = new string[10];
      const string Content = "Content";
      for (var i = 0; i < array.Length; i++)
      {
        array[i] = Content;
      }

      Console.WriteLine(GC.TryStartNoGCRegion(10000000));
      foreach (var str in array)
      {
        var charArray = str.ToCharArray();
        fixed (char* first = &charArray[0], end = &charArray[^1])
        {
          Console.WriteLine((int)first);
          Console.WriteLine((int)(end + 1));
          var current = first;
          while (current != end + 1)
          {
            Console.Write(*current);
            ++current;
          }

          Console.WriteLine();
        }
      }
  
      GC.EndNoGCRegion();
    }
  }
}