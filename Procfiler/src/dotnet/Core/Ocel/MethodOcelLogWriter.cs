using System.Text;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.Ocel;

public class MethodOcelLogWriter(string outputFileName, IProcfilerLogger logger)
{
  private class State(Guid id, string name, DateTimeOffset startTime)
  {
    public Guid Id { get; } = id;
    public string Name { get; } = name;
    public DateTimeOffset StartTime { get; } = startTime;
    public DateTimeOffset EndTime { get; set; } = startTime;

    public Dictionary<string, List<int>> Events { get; } = [];
  }

  private readonly List<State> myStack = [];
  private readonly List<State> myOutput = [];


  public void Process(EventRecordWithMetadata evt)
  {
    if (evt.IsOcelActivityBegin(out var activityId, out var activityName))
    {
      myStack.Add(new State(activityId, activityName, evt.Time.LoggedAt.ToUniversalTime()));
    }
    else if (evt.IsOcelActivityEnd(out activityId, out activityName))
    {
      if (myStack.FindIndex(e => e.Id == activityId) is var entryIndex and >= 0)
      {
        myStack[entryIndex].EndTime = evt.Time.LoggedAt.ToUniversalTime();
        myOutput.Add(myStack[entryIndex]);
        myStack.RemoveAt(entryIndex);
      }
      else
      {
        logger.LogWarning("Failed to find activity with ID {Id}", activityId);
      }
    }
    else if (evt.IsOcelObjectEvent(out var objectId, out var category))
    {
      foreach (var entry in myStack)
      {
        entry.Events.GetOrCreate(category ?? string.Empty, static () => []).Add(objectId);
      }
    }
  }

  public void Flush()
  {
    var categories = myOutput.SelectMany(s => s.Events.Keys).ToHashSet().ToList();
    if (categories.Count == 0) return;

    using var fs = File.OpenWrite(outputFileName);
    using var sw = new StreamWriter(fs);

    sw.WriteLine("event_activity;start;end;" + string.Join(';', categories));

    foreach (var state in myOutput)
    {
      var sb = new StringBuilder($"{state.Name};{state.StartTime:O};{state.EndTime.ToUniversalTime():O};");
      foreach (var category in categories)
      {
        sb.Append($"[{string.Join(',', state.Events.GetValueOrDefault(category, []))}];");
      }

      sb.Remove(sb.Length - 1, 1);
      sw.WriteLine(sb);
    }
  }
}