using System.Text;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.Ocel;

public class MethodOcelLogWriter(string outputFileName, IProcfilerLogger logger)
{
  public void Process(List<EventRecordWithMetadata> trace)
  {
    var stack = new List<(Guid Id, string Name, DateTimeOffset StartTime, Dictionary<string, List<int>> Events)>();

    using var fs = File.OpenWrite(outputFileName);
    using var sw = new StreamWriter(fs);

    var categories = new HashSet<string>();
    foreach (var evt in trace)
    {
      if (evt.IsOcelObjectEvent(out var _, out var category))
      {
        categories.Add(category ?? string.Empty);
      }
    }

    var orderedCategories = categories.Order().ToList();
    sw.WriteLine("event_activity;start;end;" + string.Join(';', orderedCategories));

    foreach (var evt in trace)
    {
      if (evt.IsOcelActivityBegin(out var activityId, out var activityName))
      {
        stack.Add((activityId, activityName, evt.Time.LoggedAt.ToUniversalTime(), []));
      }
      else if (evt.IsOcelActivityEnd(out activityId, out activityName))
      {
        if (stack.FindIndex(e => e.Id == activityId) is var entryIndex and >= 0)
        {
          var entry = stack[entryIndex];
          var sb = new StringBuilder($"{entry.Name};{entry.StartTime:O};{evt.Time.LoggedAt.ToUniversalTime():O};");
          foreach (var category in orderedCategories)
          {
            sb.Append($"[{string.Join(',', entry.Events.GetValueOrDefault(category, []))}];");
          }

          sb.Remove(sb.Length - 1, 1);
          sw.WriteLine(sb);

          stack.RemoveAt(entryIndex);
        }
        else
        {
          logger.LogWarning("Failed to find activity with ID {Id}", activityId);
        }
      }
      else if (evt.IsOcelObjectEvent(out var objectId, out var category))
      {
        foreach (var entry in stack)
        {
          entry.Events.GetOrCreate(category ?? string.Empty, static () => []).Add(objectId);
        }
      }
    }
  }
}