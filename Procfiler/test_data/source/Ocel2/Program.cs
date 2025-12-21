// See https://aka.ms/new-console-template for more information

using System.Runtime.CompilerServices;
using ProcfilerLoggerProvider;

namespace Ocel2;

public static class Program
{
  public static void Main()
  {
    Method();
  }


  private static long ourNextId;

  private static long NextId()
  {
    ourNextId++;
    return ourNextId;
  }

  private static void Method()
  {
    const string Type1 = nameof(Type1);
    const string Type2 = nameof(Type2);
    const string Type3 = nameof(Type3);

    var id1 = NextId();
    var id2 = NextId();
    var id3 = NextId();
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id1, Type1));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id2, Type1));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id3, Type1));

    var id4 = NextId();
    var id5 = NextId();
    var id6 = NextId();

    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id4, Type2));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id5, Type2));
    OcelLogger.LogObjectAllocatedRaw(new OcelObjectDto(id6, Type2));

    Method1();

    var id7 = NextId();
    OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(id7, Type1), id1, id3, id5);

    var id8 = NextId();
    OcelLogger.LogMergeAllocateRaw(new OcelObjectDto(id8, Type1), id2, id4, id6);

    Method1();

    OcelLogger.LogConsumeProduceRaw(id7, new OcelObjectDto(NextId(), Type3), new OcelObjectDto(NextId(), Type2));
    OcelLogger.LogConsumeProduceRaw(id8, new OcelObjectDto(NextId(), Type3));

    Method1();
  }

  [MethodImpl(MethodImplOptions.NoInlining)]
  private static void Method1()
  {
  }
}