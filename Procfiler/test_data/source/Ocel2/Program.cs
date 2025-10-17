// See https://aka.ms/new-console-template for more information

using System.Runtime.CompilerServices;
using ProcfilerLoggerProvider;

namespace Ocel2;

public static class Program
{
  public static void Main()
  {
  }


  private static int ourNextId;

  private static int NextId()
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
    OcelLogger.LogObjectAllocated(id1, Type1);
    OcelLogger.LogObjectAllocated(id2, Type1);
    OcelLogger.LogObjectAllocated(id3, Type1);

    var id4 = NextId();
    var id5 = NextId();
    var id6 = NextId();

    OcelLogger.LogObjectAllocated(id4, Type2);
    OcelLogger.LogObjectAllocated(id5, Type2);
    OcelLogger.LogObjectAllocated(id6, Type2);

    Method1();

    var id7 = NextId();
    OcelLogger.LogMergeAllocateRelation(id7, id1, id3, id5);

    var id8 = NextId();
    OcelLogger.LogMergeAllocateRelation(id8, id2, id4, id6);

    Method1();

    OcelLogger.LogConsumeProduceRelation(id7);
    OcelLogger.LogConsumeProduceRelation(id8);

    Method1();
  }

  [MethodImpl(MethodImplOptions.NoInlining)]
  private static void Method1()
  {
  }
}