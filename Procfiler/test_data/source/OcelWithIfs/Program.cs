using System.Runtime.CompilerServices;
using ProcfilerLoggerProvider;

namespace OcelWithIfs
{
  public static class Program
  {
    public static void Main()
    {
      for (var i = 0; i < 10; ++i)
      {
        Method(i);
      }
    }

    private const string Type1 = nameof(Type1);
    private const string Type2 = nameof(Type2);
    private const string Type3 = nameof(Type3);

    private static void Method(int x)
    {
      var objects = (x % 2 == 0) switch
      {
        true => AllocateObjects(x),
        false => AllocateObjects2(x)
      };

      var id7 = Utils.Utils.NextId();
      OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(id7, Type1), objects[0], objects[1], objects[2]);

      Barrier();

      OcelLogger.LogConsumeProduceRaw(
        id7,
        new OcelObjectDto(Utils.Utils.NextId(), Type3),
        new OcelObjectDto(Utils.Utils.NextId(), Type2)
      );
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void Barrier()
    {
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void Barrier1()
    {
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void Barrier2()
    {
    }

    private static List<long> AllocateObjects(int x)
    {
      var type = x % 2 == 0 ? Type1 : Type2;

      var ids = Utils.Utils.Allocate(type);

      Barrier1();

      return
      [
        OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(Utils.Utils.NextId(), type), ids[..2].ToArray())!.Value,
        OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(Utils.Utils.NextId(), type), ids[2..4].ToArray())!.Value,
        OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(Utils.Utils.NextId(), type), ids[4..].ToArray())!.Value
      ];
    }

    private static List<long> AllocateObjects2(int x)
    {
      var type = x % 2 == 0 ? Type3 : Type2;

      var ids = Utils.Utils.Allocate(type);

      Barrier2();

      return
      [
        OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(Utils.Utils.NextId(), type), ids[..2].ToArray())!.Value,
        OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(Utils.Utils.NextId(), type), ids[2..4].ToArray())!.Value,
        OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(Utils.Utils.NextId(), type), ids[4..].ToArray())!.Value
      ];
    }
  }
}

namespace Utils
{
  public static class Utils
  {
    private static long ourNextId;

    public static long NextId()
    {
      ourNextId++;
      return ourNextId;
    }

    public static List<long> Allocate(string type)
    {
      return Enumerable.Range(0, 6).Select(_ => OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(NextId(), type))!.Value).ToList();
    }
  }
}