using System.Collections.Concurrent;
using System.Runtime.CompilerServices;
using System.Threading.Channels;
using Ficus;
using FrontendBackend.Utils;
using Google.Protobuf.WellKnownTypes;

namespace FrontendBackend.Features.PipelineUpdates.Services;

public interface IPipelinePartsUpdatesRepository
{
  IAsyncEnumerable<GrpcPipelinePartUpdate> StartUpdatesStream(CancellationToken token);
  Task ProcessUpdate(GrpcKafkaUpdate update);
}

public class PipelinePartsUpdatesRepository(ILogger<PipelinePartsUpdatesRepository> logger) : IPipelinePartsUpdatesRepository
{
  private class PipelinePartResult
  {
    public required string PipelinePartName { get; init; }
    public required List<GrpcContextValueWithKeyName> ContextValues { get; init; }
  }
  
  private class CaseData
  {
    public required Dictionary<Guid, PipelinePartResult> PipelinePartsResults { get; init; }
    public required List<KeyValuePair<string, string>> Metadata { get; init; }
  }


  private readonly SemaphoreSlim myLock = new(1);
  private readonly Dictionary<string, Dictionary<string, CaseData>> myProcesses = [];
  private readonly ConcurrentDictionary<Guid, Channel<GrpcKafkaUpdate>> myChannels = [];


  public async IAsyncEnumerable<GrpcPipelinePartUpdate> StartUpdatesStream([EnumeratorCancellation] CancellationToken token)
  {
    var updatesStreamId = Guid.NewGuid();

    using var _ = logger.BeginScope(new Dictionary<string, object>
    {
      ["UpdateStreamId"] = updatesStreamId
    });

    logger.LogInformation("Starting a new update stream");
    var (sessionGuid, channel, currentState) = await myLock.Execute(() =>
    {
      var state = GetCurrentState();
      var channel = Channel.CreateBounded<GrpcKafkaUpdate>(new BoundedChannelOptions(10)
      {
        FullMode = BoundedChannelFullMode.Wait,
        SingleReader = false,
        SingleWriter = true,
      });

      var sessionGuid = Guid.NewGuid();
      myChannels[sessionGuid] = channel;
      return Task.FromResult((sessionGuid, channel, state));
    });
    
    logger.LogInformation("Created an initial state");

    try
    {
      yield return new GrpcPipelinePartUpdate
      {
        CurrentCases = currentState
      };

      while (true)
      {
        if (token.IsCancellationRequested) yield break;

        var delta = await channel.Reader.ReadAsync(token);
        
        logger.LogInformation("Received delta: {Delta}", delta.GetType());
        
        yield return new GrpcPipelinePartUpdate
        {
          Delta = delta
        };
      }
    }
    finally
    {
      myChannels.Remove(sessionGuid, out var __);
    }
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
      if (!myProcesses.TryGetValue(update.ProcessCaseMetadata.ProcessName, out var cases))
      {
        logger.LogInformation("Creating new process data for update");
        cases = new Dictionary<string, CaseData>();
        myProcesses[update.ProcessCaseMetadata.ProcessName] = cases;
      }

      if (!cases.TryGetValue(update.ProcessCaseMetadata.CaseName, out var caseData))
      {
        logger.LogInformation("Creating new case data for update");
        caseData = new CaseData
        {
          Metadata = update.ProcessCaseMetadata.Metadata.Select(kv => new KeyValuePair<string, string>(kv.Key, kv.Value)).ToList(),
          PipelinePartsResults = []
        };

        cases[update.ProcessCaseMetadata.CaseName] = caseData;
      }

      var guid = Guid.Parse(update.PipelinePartInfo.Id.Guid);

      if (!caseData.PipelinePartsResults.TryGetValue(guid, out var contextValues))
      {
        logger.LogInformation("Creating new cases context values");
        contextValues = new PipelinePartResult
        {
          ContextValues = [],
          PipelinePartName = update.PipelinePartInfo.Name
        };

        caseData.PipelinePartsResults[guid] = contextValues;
      }

      contextValues.ContextValues.Clear();
      contextValues.ContextValues.AddRange(update.ContextValues);

      foreach (var (id, chanel) in myChannels)
      {
        logger.LogInformation("Started writing update to channel {Id}", id);
        await chanel.Writer.WriteAsync(update);

        logger.LogInformation("Finished writing update to channel {Id}", id);
      }
    });
  }

  private GrpcCurrentCasesResponse GetCurrentState()
  {
    var response = new GrpcCurrentCasesResponse();
    foreach (var (processName, cases) in myProcesses)
    {
      foreach (var (caseName, caseData) in cases)
      {
        response.Cases.Add(new GrpcCase
        {
          ProcessCaseMetadata = new GrpcProcessCaseMetadata
          {
            ProcessName = processName,
            CaseName = caseName,
            Metadata =
            {
              caseData.Metadata.Select(pair => new GrpcStringKeyValue
              {
                Key = pair.Key,
                Value = pair.Value
              })
            }
          },
          ContextValues =
          {
            caseData.PipelinePartsResults.Select(x => new GrpcPipelinePartContextValues
            {
              Stamp = Timestamp.FromDateTime(DateTime.UtcNow),
              ContextValues = { x.Value.ContextValues },
              PipelinePartInfo = new GrpcPipelinePartInfo
              {
                Name = x.Value.PipelinePartName,
                Id = new GrpcGuid
                {
                  Guid = x.Key.ToString()
                }
              }
            })
          }
        });
      } 
    }

    return response;
  }
}