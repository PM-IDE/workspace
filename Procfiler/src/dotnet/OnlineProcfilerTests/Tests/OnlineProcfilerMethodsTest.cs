﻿using System.Text.RegularExpressions;
using Core.Events.EventRecord;
using Core.Utils;
using OnlineProcfilerTests.Core;
using ProcfilerOnline.Core;
using TestsUtil;

namespace OnlineProcfilerTests.Tests;

public abstract class OnlineProcfilerMethodsTest : OnlineProcfilerTestWithGold
{
  protected abstract string? Prefix { get; }

  protected abstract Dictionary<string, List<List<EventRecordWithMetadata>>> GetLoggedMethods(ISharedEventPipeStreamData sharedData);

  protected string DoExecuteTest(KnownSolution solution)
  {
    var sharedData = ExecuteTest(solution);
    if (sharedData is null)
    {
      Assert.Fail("Shared data was null");
      return null!;
    }

    var filter = new Regex(solution.NamespaceFilterPattern);
    return OnlineProcfilerMethodsUtil.SerializeToGold(sharedData, GetLoggedMethods(sharedData), filter, Prefix);
  }
}

public static class OnlineProcfilerMethodsUtil
{
  public static string SerializeToGold(
    ISharedEventPipeStreamData sharedData,
    Dictionary<string, List<List<EventRecordWithMetadata>>> loggedEvents,
    Regex filter,
    string? prefix)
  {
    return MethodsTestsUtil.SerializeToGold(loggedEvents, filter, prefix, e =>
    {
      if (e.TryGetMethodDetails() is var (_, methodId))
      {
        return sharedData.FindMethodDetails(methodId)!.Fqn;
      }

      return null;
    }, trace => ProgramMethodCallTreeDumper.CreateDump(trace, filter.ToString(), e => e.TryGetMethodDetails() switch
    {
      var (_, id) => (sharedData.FindMethodDetails(id)!.Fqn, e.GetMethodEventKind() == MethodKind.Begin),
      _ => null
    }));
  }
}