using System.Runtime.CompilerServices;
using ProcfilerLoggerProvider;

namespace OcelWithIfs;

public static class Program
{
  public static void Main()
  {
    for (var i = 0; i < 10; ++i)
    {
      Method(i);
    }
  }

  private static long ourNextId;

  private static long NextId()
  {
    ourNextId++;
    return ourNextId;
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

    var id7 = NextId();
    OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(id7, Type1), objects[0], objects[1], objects[2]);

    Barrier();

    OcelLogger.LogConsumeProduceRaw(id7, new OcelObjectDto(NextId(), Type3), new OcelObjectDto(NextId(), Type2));
  }

  [MethodImpl(MethodImplOptions.NoInlining)]
  private static void Barrier()
  {
  }

  private static List<long> AllocateObjects(int x)
  {
    var id1 = NextId();
    var id2 = NextId();
    var id3 = NextId();

    var type = x % 2 == 0 ? Type1 : Type2;

    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id1, type));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id2, type));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id3, type));

    return [id1, id2, id3];
  }

  private static List<long> AllocateObjects2(int x)
  {
    var id4 = NextId();
    var id5 = NextId();
    var id6 = NextId();

    var type = x % 2 == 0 ? Type3 : Type2;

    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id4, type));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id5, type));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id6, type));

    return [id4, id5, id6];
  }
}