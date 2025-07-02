// See https://aka.ms/new-console-template for more information

using Microsoft.Extensions.Options;
using ProcfilerLoggerProvider;


var objects = Enumerable.Range(0, 1000).Select(_ => new MyObject()).ToList();
var categoryName = typeof(MyObject).GetType().Name;

using (OcelLogger.StartOcelActivity("Initializing"))
{
  foreach (var myObject in objects)
  {
    OcelLogger.LogObject(myObject, categoryName);
    myObject.Name = "xd";
  }
}

Method1(objects.Where((_, i) => i % 2 == 0).ToList());
Method2(objects.Where((_, i) => i % 3 == 0).ToList());
Method3(objects.Where((_, i) => i % 4 == 0).ToList());
Method1(objects.Where((_, i) => i % 5 == 0).ToList());
Method3(objects.Where((_, i) => i % 6 == 0).ToList());

void Method1(List<MyObject> objects)
{
  using var _ = OcelLogger.StartOcelActivity(nameof(Method1));
  foreach (var obj in objects)
  {
    OcelLogger.LogObject(obj, categoryName);
  }
}

void Method2(List<MyObject> objects)
{
  using var _ = OcelLogger.StartOcelActivity(nameof(Method2));
  foreach (var obj in objects)
  {
    OcelLogger.LogObject(obj, categoryName);
  }
}

void Method3(List<MyObject> objects)
{
  using var _ = OcelLogger.StartOcelActivity(nameof(Method3));
  foreach (var obj in objects)
  {
    OcelLogger.LogObject(obj, categoryName);
  }
}


class MyObject
{
  public string Name { get; set; }
}


class MyOptionsMonitor(ProcfilerLoggerConfiguration configuration) : IOptionsMonitor<ProcfilerLoggerConfiguration>
{
  public ProcfilerLoggerConfiguration CurrentValue { get; } = configuration;


  public ProcfilerLoggerConfiguration Get(string? name)
  {
    return configuration;
  }

  public IDisposable? OnChange(Action<ProcfilerLoggerConfiguration, string?> listener)
  {
    return default;
  }
}