using Core.Container;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler.ShadowStacks;
using Procfiler.Utils;

namespace Procfiler.Core;

public interface IStackTraceSerializer
{
  void SerializeStackTraces(SessionGlobalData globalData, string directory);
}

[AppComponent]
public class StackTraceSerializer : IStackTraceSerializer
{
  public void SerializeStackTraces(SessionGlobalData globalData, string directory)
  {
    switch (globalData.Stacks)
    {
      case IFromEventsShadowStacks fromEventsShadowStacks:
        SerializeFromEventsStacks(fromEventsShadowStacks, directory);
        return;
      case ICppShadowStacks cppShadowStacks:
        SerializeCppStacks(cppShadowStacks, globalData, directory);
        return;
      default:
        throw new ArgumentOutOfRangeException(globalData.Stacks.GetType().Name);
    }
  }

  private void SerializeFromEventsStacks(IFromEventsShadowStacks shadowStacks, string directory)
  {
    var sb = new StringBuilder();
    using var fs = File.OpenWrite(Path.Combine(directory, "stacks.txt"));
    using var sw = new StreamWriter(fs);

    foreach (var (_, stack) in shadowStacks.StackTraceInfos)
    {
      sw.Write($"{stack.StackTraceId}\n");

      sb = sb.Clear();
      foreach (var frame in stack.Frames)
      {
        sb = sb.AppendTab().Append(frame).AppendNewLine();
      }

      sb = sb.AppendNewLine();

      sw.Write(sb.ToString());
    }
  }

  private void SerializeCppStacks(ICppShadowStacks shadowStacks, SessionGlobalData globalData, string directory)
  {
    foreach (var shadowStack in shadowStacks.EnumerateStacks())
    {
      var path = Path.Combine(directory, $"stacks_{shadowStack.ManagedThreadId}.txt");
      SerializeStack(shadowStack, globalData, path);
    }
  }

  public void SerializeStack(
    ICppShadowStack shadowStack, SessionGlobalData globalData, string savePath)
  {
    using var fs = File.OpenWrite(savePath);
    using var sw = new StreamWriter(fs);

    var indent = 0;

    foreach (var frame in shadowStack)
    {
      if (!frame.IsStart)
      {
        --indent;
      }

      for (var i = 0; i < indent; ++i)
      {
        sw.Write(' ');
      }

      sw.Write(frame.Serialize(globalData));
      sw.Write('\n');

      if (frame.IsStart)
      {
        ++indent;
      }
    }
  }
}