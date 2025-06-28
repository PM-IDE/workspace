using System.Collections.Concurrent;
using System.Runtime.CompilerServices;
using System.Threading.Channels;
using Ficus;
using FicusDashboardBackend.Utils;
using Google.Protobuf.WellKnownTypes;
using GrpcModels;

namespace FicusDashboardBackend.Features.PipelineUpdates.Services;

public interface IPipelinePartsUpdatesRepository
{
  Task<GrpcSubscriptionAndPipelinesStateResponse> GetCurrentState();
  Task<GrpcCaseContextValues> GetCaseContextValues(GrpcGetPipelineCaseContextValuesRequest request);

  Task ProcessUpdate(GrpcKafkaUpdate update);
}

public class PipelinePartsUpdatesRepository(ILogger<PipelinePartsUpdatesRepository> logger) : IPipelinePartsUpdatesRepository
{
  private class PipelinePartExecutionResult
  {
    public required List<GrpcContextValueWithKeyName> ContextValues { get; init; }
  }

  private class PipelinePartExecutionResults
  {
    public required string PipelinePartName { get; init; }
    public required List<PipelinePartExecutionResult> Results { get; init; }
  }

  private class PipelinePartsExecutionResults
  {
    public required Guid ExecutionId { get; set; }
    public required ulong Stamp { get; set; }
    public required Dictionary<Guid, PipelinePartExecutionResults> PipelinePartsResults { get; init; }
  }

  private class CaseData
  {
    public required PipelinePartsExecutionResults ExecutionResults { get; init; }
    public required List<KeyValuePair<string, string>> Metadata { get; init; }

    public required string PipelineName { get; init; }
    public required string SubscriptionName { get; init; }
  }

  private record CaseName(string DisplayName, List<string> NameParts)
  {
    public override int GetHashCode()
    {
      if (NameParts.Count == 0) return 0;

      var hashCode = NameParts.First().GetHashCode();
      foreach (var part in NameParts[1..])
      {
        hashCode = HashCode.Combine(hashCode, part.GetHashCode());
      }

      return hashCode;
    }

    public virtual bool Equals(CaseName? other) =>
      other is { } &&
      NameParts.Count == other.NameParts.Count &&
      NameParts.Zip(other.NameParts).All((pair) => pair.First.Equals(pair.Second));
  }

  private record CaseKey(Guid SubscriptionId, Guid PipelineId, string ProcessName, CaseName CaseName);


  private readonly SemaphoreSlim myLock = new(1);
  private readonly Dictionary<CaseKey, CaseData> myCases = [];
  private readonly ConcurrentDictionary<Guid, Channel<GrpcKafkaUpdate>> myChannels = [];


  public Task<GrpcSubscriptionAndPipelinesStateResponse> GetCurrentState()
  {
    return myLock.Execute(() =>
    {
      var response = new GrpcSubscriptionAndPipelinesStateResponse();
      foreach (var (caseKey, @case) in myCases)
      {
        response.Cases.Add(new GrpcProcessCaseMetadataWithStamp()
        {
          Stamp = @case.ExecutionResults.Stamp,
          Metadata = new GrpcProcessCaseMetadata
          {
            ProcessName = caseKey.ProcessName,
            CaseName = new GrpcCaseName
            {
              DisplayName = caseKey.CaseName.DisplayName,
              FullNameParts = { caseKey.CaseName.NameParts }
            },
            PipelineId = caseKey.PipelineId.ToGrpcGuid(),
            SubscriptionId = caseKey.SubscriptionId.ToGrpcGuid(),
            PipelineName = @case.PipelineName,
            SubscriptionName = @case.SubscriptionName,
            Metadata =
            {
              @case.Metadata.Select(pair => new GrpcStringKeyValue
              {
                Key = pair.Key,
                Value = pair.Value
              })
            }
          },
        });
      }

      return Task.FromResult(response);
    });
  }

  public Task<GrpcCaseContextValues> GetCaseContextValues(GrpcGetPipelineCaseContextValuesRequest request)
  {
    var key = new CaseKey(
      request.SubscriptionId.ToGuid(),
      request.PipelineId.ToGuid(),
      request.ProcessName,
      new CaseName(
        request.CaseName.DisplayName,
        request.CaseName.FullNameParts.ToList()
      )
    );

    return myLock.Execute(() =>
    {
      if (!myCases.TryGetValue(key, out var caseData))
      {
        throw new KeyNotFoundException();
      }

      return Task.FromResult(new GrpcCaseContextValues
      {
        Stamp = caseData.ExecutionResults.Stamp,
        ContextValues =
        {
          caseData.ExecutionResults.PipelinePartsResults.Select(x => new GrpcPipelinePartContextValues
          {
            Stamp = Timestamp.FromDateTime(DateTime.UtcNow),
            ExecutionResults =
            {
              x.Value.Results.Select(e => new GrpcCasePipelinePartExecutionResult
              {
                ContextValues = { e.ContextValues }
              })
            },
            PipelinePartInfo = new GrpcPipelinePartInfo
            {
              ExecutionId = caseData.ExecutionResults.ExecutionId.ToGrpcGuid(),
              Name = x.Value.PipelinePartName,
              Id = x.Key.ToGrpcGuid()
            }
          })
        }
      });
    });
  }

  public Task ProcessUpdate(GrpcKafkaUpdate update)
  {
    return myLock.Execute(async () =>
    {
      var updateProcessingId = Guid.NewGuid();
      using var _ = logger.BeginScope(new Dictionary<string, object>
      {
        ["UpdateId"] = updateProcessingId
      });

      logger.LogInformation("Processing update: {Update}", update.GetType());

      var caseKey = new CaseKey(
        update.ProcessCaseMetadata.SubscriptionId.ToGuid(),
        update.ProcessCaseMetadata.PipelineId.ToGuid(),
        update.ProcessCaseMetadata.ProcessName,
        new CaseName(
          update.ProcessCaseMetadata.CaseName.DisplayName,
          update.ProcessCaseMetadata.CaseName.FullNameParts.ToList()
        )
      );

      var currentExecutionId = update.PipelinePartInfo.ExecutionId.ToGuid();
      if (!myCases.TryGetValue(caseKey, out var caseData))
      {
        logger.LogInformation("Creating new process data for update");

        caseData = new CaseData
        {
          Metadata = update.ProcessCaseMetadata.Metadata.Select(kv => new KeyValuePair<string, string>(kv.Key, kv.Value)).ToList(),
          ExecutionResults = new PipelinePartsExecutionResults
          {
            ExecutionId = currentExecutionId,
            PipelinePartsResults = [],
            Stamp = 0
          },
          PipelineName = update.ProcessCaseMetadata.PipelineName,
          SubscriptionName = update.ProcessCaseMetadata.SubscriptionName
        };

        myCases[caseKey] = caseData;
      }

      var guid = Guid.Parse(update.PipelinePartInfo.Id.Guid);

      if (caseData.ExecutionResults.ExecutionId != currentExecutionId)
      {
        logger.LogInformation("Resetting all execution results after execution id changed");
        caseData.ExecutionResults.ExecutionId = currentExecutionId;
        caseData.ExecutionResults.PipelinePartsResults.Clear();
      }

      if (!caseData.ExecutionResults.PipelinePartsResults.TryGetValue(guid, out var pipelinePartResults))
      {
        logger.LogInformation("Creating new cases context values");
        pipelinePartResults = new PipelinePartExecutionResults
        {
          PipelinePartName = update.PipelinePartInfo.Name,
          Results = []
        };

        caseData.ExecutionResults.PipelinePartsResults[guid] = pipelinePartResults;
      }

      pipelinePartResults.Results.Add(new PipelinePartExecutionResult
      {
        ContextValues = update.ContextValues.ToList(),
      });

      caseData.ExecutionResults.Stamp++;

      foreach (var (id, chanel) in myChannels)
      {
        logger.LogInformation("Started writing update to channel {Id}", id);
        await chanel.Writer.WriteAsync(update);

        logger.LogInformation("Finished writing update to channel {Id}", id);
      }
    });
  }
}