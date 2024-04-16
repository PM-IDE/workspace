namespace FileAsyncOperations;

internal class Program
{
  public static async Task Main(string[] args)
  {
    const string Text = "Some text";
    const string FilePath = "file.txt";

    var list = new List<object>();
    for (var i = 0; i < 3; i++)
    {
      list.Add(new Class1());
      list.Add(new Class1());
      list.Add(new Class1());
      list.Add(new Class1());
      list.Add(new Class1());
      await using var fs = File.OpenWrite(FilePath);
      {
        await using var sw = new StreamWriter(fs);
        {
          await sw.WriteAsync(Text);
        }
      }

      list.Add(new Class2());

      await File.WriteAllTextAsync(Text, FilePath);

      list.Add(new Class3());

      var bytes = await File.ReadAllBytesAsync(FilePath);
    }

    Console.WriteLine(list.Count);
    await Task.Delay(1000);
  }
}

class Class1
{
  public int x;
}

class Class2
{
  public long x;
}

class Class3
{
  public string x;
}

class Class4
{
  public double x;
}

class Class5
{
  public bool x;
}

class Class6
{
  public IntPtr X;
}