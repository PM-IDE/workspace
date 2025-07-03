// See https://aka.ms/new-console-template for more information

using Microsoft.Extensions.Options;
using ProcfilerLoggerProvider;

namespace Ocel;

public static class Program
{
  public static async Task Main()
  {
    await Task.Delay(1);
    var objects = Enumerable.Range(0, 1000).Select(_ => (Animal)(Random.Shared.Next(3) switch
    {
      0 => new Dog(),
      1 => new Cat(),
      2 => new Sheep(),
      _ => throw new Exception()
    })).ToList();

    using (OcelLogger.StartOcelActivity("Initializing"))
    {
      foreach (var myObject in objects)
      {
        OcelLogger.LogObject(myObject, myObject.GetType().Name);
        myObject.Name = "xd";
      }
    }

    Cleaning(objects.Where((_, i) => i % 2 == 0).ToList());
    VetClinic(objects.Where((_, i) => i % 3 == 0).ToList());
    Playground(objects.Where((_, i) => i % 4 == 0).ToList());
    Cleaning(objects.Where((_, i) => i % 5 == 0).ToList());
    Farm(objects.Where((_, i) => i % 6 == 0).ToList());

    void Cleaning(List<Animal> objects)
    {
      using var _ = OcelLogger.StartOcelActivity(nameof(Cleaning));
      foreach (var obj in objects)
      {
        OcelLogger.LogObject(obj, obj.GetType().Name);
      }
    }

    void VetClinic(List<Animal> objects)
    {
      using var _ = OcelLogger.StartOcelActivity(nameof(VetClinic));
      foreach (var obj in objects)
      {
        OcelLogger.LogObject(obj, obj.GetType().Name);
      }
    }

    void Playground(List<Animal> objects)
    {
      using var _ = OcelLogger.StartOcelActivity(nameof(Playground));
      foreach (var obj in objects)
      {
        OcelLogger.LogObject(obj, obj.GetType().Name);
      }
    }

    void Farm(List<Animal> objects)
    {
      using var _ = OcelLogger.StartOcelActivity(nameof(Farm));
      foreach (var obj in objects)
      {
        OcelLogger.LogObject(obj, obj.GetType().Name);
      }
    }
  }
}

class Animal
{
  public string Name { get; set; }
}

class Dog : Animal;

class Cat : Animal;

class Sheep : Animal;


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