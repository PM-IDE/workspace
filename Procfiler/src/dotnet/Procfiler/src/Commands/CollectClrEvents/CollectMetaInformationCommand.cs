using Core.Container;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Base;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core;
using Procfiler.Utils;

namespace Procfiler.Commands.CollectClrEvents;

public interface ICollectMetaInformationCommand : ICommandWithContext<CollectClrEventsContext>;

[CommandLineCommand]
public class CollectMetaInformationCommand(
  IProcfilerLogger logger,
  ICommandExecutorDependantOnContext commandExecutor
) : CollectCommandBase(logger, commandExecutor), ICollectMetaInformationCommand
{
  public override void Execute(CollectClrEventsContext context)
  {
    ExecuteCommand(context, events =>
    {
      PathUtils.CheckIfDirectoryOrThrow(context.CommonContext.OutputPath);
      var map = SplitEventsHelper.SplitByKey(Logger, events.Events, SplitEventsHelper.EventClassKeyExtractor);

      foreach (var (name, eventsByName) in map)
      {
        var payloadValues = new Dictionary<string, Dictionary<string, int>>();
        foreach (var (_, eventRecord) in eventsByName)
        {
          foreach (var (payloadName, payloadValue) in eventRecord.Metadata)
          {
            var valuesCount = payloadValues.GetOrCreate(payloadName, static () => new Dictionary<string, int>());
            var count = valuesCount.GetOrCreate(payloadValue, static () => 0);
            valuesCount[payloadValue] = count + 1;
          }
        }

        SerializeMetadata(context, name, payloadValues);
      }
    });
  }

  protected override Command CreateCommandInternal() =>
    new("meta-info", "Collects meta-information about CLR events received during listening to process");

  private static void SerializeMetadata(
    CollectClrEventsContext context,
    string name,
    Dictionary<string, Dictionary<string, int>> payloadValues)
  {
    var outputFormat = context.CommonContext.SerializationContext.OutputFormat;
    var extension = outputFormat.GetExtension();
    var pathToMetadataFile = Path.Combine(context.CommonContext.OutputPath, $"{name}.{extension}");
    using var fs = new FileStream(pathToMetadataFile, FileMode.OpenOrCreate, FileAccess.Write);
    using var sw = new StreamWriter(fs);

    switch (outputFormat)
    {
      case FileFormat.Csv:
      {
        sw.WriteLine("PayloadName;PayloadValue;Count");
        foreach (var (payloadName, valuesCount) in payloadValues)
        {
          foreach (var (valueString, count) in valuesCount)
          {
            sw.WriteLine($"{payloadName};{valueString};{count}");
          }
        }

        break;
      }

      default:
        throw new ArgumentOutOfRangeException();
    }
  }
}