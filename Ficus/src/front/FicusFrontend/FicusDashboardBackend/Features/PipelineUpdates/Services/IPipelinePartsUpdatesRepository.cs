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

public class PipelinePartsUpdatesRepository(
  ILogger<PipelinePartsUpdatesRepository> logger,
  GrpcKafkaService.GrpcKafkaServiceClient kafkaClient
) : IPipelinePartsUpdatesRepository
{
  private class CaseData
  {
    public required List<KeyValuePair<string, string>> Metadata { get; init; }

    public required string PipelineName { get; init; }
    public required string SubscriptionName { get; init; }
    public required ulong Stamp { get; set; }
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


  public Task<GrpcSubscriptionAndPipelinesStateResponse> GetCurrentState()
  {
    return myLock.Execute(() =>
    {
      var response = new GrpcSubscriptionAndPipelinesStateResponse();
      foreach (var (caseKey, @case) in myCases)
      {
        response.Cases.Add(new GrpcProcessCaseMetadataWithStamp()
        {
          Stamp = @case.Stamp,
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

      return response;
    });
  }

  public async Task<GrpcCaseContextValues> GetCaseContextValues(GrpcGetPipelineCaseContextValuesRequest request)
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

    var stream = kafkaClient.GetCurrentContextValues(new GrpcGetCurrentContextValuesRequest
    {
      PipelineId = request.PipelineId,
      ProcessName = request.ProcessName,
      SubscriptionId = request.SubscriptionId,
    });

    var executionId = Guid.NewGuid();
    var executionResults = new List<GrpcPipelinePartContextValues>();
    while (await stream.ResponseStream.MoveNext(CancellationToken.None))
    {
      var current = stream.ResponseStream.Current;
      if (current.ResultCase == GrpcPipelinePartExecutionResult.ResultOneofCase.PipelinePartResult)
      {
        executionResults.Add(new GrpcPipelinePartContextValues
        {
          ExecutionResults =
          {
            new GrpcCasePipelinePartExecutionResult
            {
              ContextValues = { current.PipelinePartResult.ContextValues }
            }
          },
          Stamp = DateTime.UtcNow.ToTimestamp(),
          PipelinePartInfo = new GrpcPipelinePartInfo
          {
            ExecutionId = executionId.ToGrpcGuid(),
            Id = current.PipelinePartResult.PipelinePartId,
            Name = current.PipelinePartResult.PipelinePartName,
          }
        });
      }
    }

    return await myLock.Execute(() =>
    {
      if (!myCases.TryGetValue(key, out var caseData))
      {
        throw new KeyNotFoundException();
      }

      return new GrpcCaseContextValues
      {
        Stamp = caseData.Stamp,
        ContextValues =
        {
          executionResults
        }
      };
    });
  }

  public Task ProcessUpdate(GrpcKafkaUpdate update)
  {
    return myLock.Execute(() =>
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

      if (!myCases.TryGetValue(caseKey, out var caseData))
      {
        logger.LogInformation("Creating new process data for update");

        caseData = new CaseData
        {
          Stamp = 0,
          Metadata = update.ProcessCaseMetadata.Metadata.Select(kv => new KeyValuePair<string, string>(kv.Key, kv.Value)).ToList(),
          PipelineName = update.ProcessCaseMetadata.PipelineName,
          SubscriptionName = update.ProcessCaseMetadata.SubscriptionName
        };

        myCases[caseKey] = caseData;
      }

      caseData.Stamp++;
      return Task.CompletedTask;
    });
  }
}