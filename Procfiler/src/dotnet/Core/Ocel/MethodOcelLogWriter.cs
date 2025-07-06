using System.Text;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.Ocel;

public class MethodOcelLogWriter(string outputFilePath, IProcfilerLogger logger)
{
  private class State(Guid id, string name, DateTimeOffset startTime)
  {
    public Guid Id { get; } = id;
    public string Name { get; } = name;

    public ActivityInfo Info { get; } = new(startTime);
  }

  private class ActivityInfo(DateTimeOffset startDate)
  {
    public DateTimeOffset StartDate { get; } = startDate;
    public Dictionary<string, List<long>> Events { get; } = [];

    public DateTimeOffset EndDate { get; set; } = startDate;
  }

  private readonly List<State> myStack = [];
  private readonly List<State> myOutput = [];
  private readonly Dictionary<string, ActivityInfo> myGlobalActivities = [];


  public void Process(EventRecordWithMetadata evt)
  {
    if (evt.IsOcelGlobalEvent(out var objectId, out var activityName, out var category))
    {
      var state = myGlobalActivities.GetOrCreate(activityName, () => new ActivityInfo(evt.Time.LoggedAt.ToUniversalTime()));
      state.Events.GetOrCreate(category ?? string.Empty, static () => []).Add(objectId);
      state.EndDate = evt.Time.LoggedAt.ToUniversalTime();
    }
    else if (evt.IsOcelActivityBegin(out var activityId, out activityName))
    {
      myStack.Add(new State(activityId, activityName, evt.Time.LoggedAt.ToUniversalTime()));
    }
    else if (evt.IsOcelActivityEnd(out activityId, out activityName))
    {
      if (myStack.FindIndex(e => e.Id == activityId) is var entryIndex and >= 0)
      {
        myStack[entryIndex].Info.EndDate = evt.Time.LoggedAt.ToUniversalTime();
        myOutput.Add(myStack[entryIndex]);
        myStack.RemoveAt(entryIndex);
      }
      else
      {
        logger.LogWarning("Failed to find activity with ID {Id}", activityId);
      }
    }
    else if (evt.IsOcelObjectEvent(out objectId, out category))
    {
      foreach (var entry in myStack)
      {
        entry.Info.Events.GetOrCreate(category ?? string.Empty, static () => []).Add(objectId);
      }
    }
  }

  public void Flush()
  {
    var categories = myOutput
      .SelectMany(s => s.Info.Events.Keys)
      .Concat(myGlobalActivities.SelectMany(a => a.Value.Events.Keys))
      .ToHashSet()
      .ToList();

    if (categories.Count == 0) return;

    if (Path.GetDirectoryName(outputFilePath) is not { } directory)
    {
      logger.LogError("Cant get output directory for path {FilePath}", outputFilePath);
      return;
    }

    PathUtils.EnsureEmptyDirectory(directory, logger);

    using var fs = File.OpenWrite(outputFilePath);
    using var sw = new StreamWriter(fs);

    sw.WriteLine("event_activity;start;end;" + string.Join(';', categories));

    var activities = myOutput
      .Select(o => (o.Name, o.Info))
      .Concat(myGlobalActivities.Select(g => (g.Key, g.Value)))
      .OrderBy(a => a.Item2.StartDate);

    foreach (var (name, info) in activities)
    {
      var sb = new StringBuilder($"{name};{info.StartDate:O};{info.EndDate.ToUniversalTime():O};");
      foreach (var category in categories)
      {
        sb.Append($"[{string.Join(',', info.Events.GetValueOrDefault(category, []))}];");
      }

      sb.Remove(sb.Length - 1, 1);
      sw.WriteLine(sb);
    }
  }
}