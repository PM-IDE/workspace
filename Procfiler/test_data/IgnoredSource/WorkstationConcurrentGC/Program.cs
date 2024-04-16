// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

const int ThreadsCount = 1;
Dictionary<int, List<SomeRecord>> map = new();
for (var i = 0; i < ThreadsCount; i++)
{
  map[i] = new List<SomeRecord>();
}

for (var i = 0; i < 100; i++)
{
  Console.WriteLine(i);

  List<SomeRecord> list = map[0];
  for (int j = 0; j < 1_000; j++)
  {
    list.Add(new SomeRecord(0));

    if (Random.Shared.Next(0, 100) < 15)
    {
      if (list.Count > 0)
      {
        list.RemoveAt(0);
      }
      
      GC.Collect(2, GCCollectionMode.Default, false, false);
    }
  }
}

Console.WriteLine(map.Count);

internal record SomeRecord(int X)
{
  public string Name { get; set; } = new string('c', Random.Shared.Next(100));
}
