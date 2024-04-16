using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Core;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.Serialization.MethodsCallTree;

public interface IMethodTreeEventSerializer : IEventsSerializer;

[AppComponent]
public class MethodTreeSerializer : IMethodTreeEventSerializer
{
  public void SerializeEvents(IEnumerable<EventRecordWithMetadata> events, string path)
  {
    using var fs = File.OpenWrite(path);
    using var sw = new StreamWriter(fs);

    var indent = 0;
    var sb = new StringBuilder();

    foreach (var eventRecord in events)
    {
      sb.Clear();

      void AddIndent()
      {
        for (var i = 0; i < indent; i++)
        {
          sb.Append("--");
        }
      }

      if (eventRecord.TryGetMethodStartEndEventInfo() is var (_, isStart))
      {
        if (isStart)
        {
          AddIndent();
          ++indent;
        }
        else
        {
          --indent;
          AddIndent();
        }
      }
      else
      {
        AddIndent();
      }

      sb.Append($"[{eventRecord.Stamp}] ");
      sb.Append(eventRecord.EventName);
      using (sb.AppendBraces())
      {
        foreach (var (key, value) in eventRecord.Metadata)
        {
          sb.Append(key).Append('=').Append(value).Append(',');
        }

        if (eventRecord.Metadata.Count > 0)
        {
          sb.Remove(sb.Length - 1, 1);
        }
      }

      sb.Append('\n');

      sw.Write(sb.ToString());
    }
  }
}