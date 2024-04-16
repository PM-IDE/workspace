var map = new Dictionary<int, List<SomeRecord>>();
const int ThreadsCount = 10;
for (int i = 0; i < ThreadsCount; i++)
{
  map[i] = new List<SomeRecord>();
}

for (var i = 0; i < 5; ++i)
{
  Console.WriteLine(i);
  var threads = Enumerable.Range(0, 10).Select(index => new Thread(() =>
  {
    var list = map[index];
    for (var j = 0; j < 10_000; j++)
    {
      list.Add(new SomeRecord(j));

      if (Random.Shared.Next(20) < 5 && list.Count > 0)
      {
        list.RemoveAt(0);
      }

      if (Random.Shared.Next(10) < 2)
      {
        GC.Collect(2, GCCollectionMode.Default, false, false);
      }
    }
  })).ToList();

  foreach (var thread in threads)
  {
    thread.Start();
  }
  
  foreach (var thread in threads)
  {
    thread.Join();
  }
  
  foreach (var (_, value) in map)
  {
    value.Clear();
  }
}

foreach (var (key, value) in map)
{
  Console.WriteLine($"{key} {value}");
}

internal record SomeRecord(int X)
{
  public string? Name { get; set; } = new('c', Random.Shared.Next(1, 5));
}