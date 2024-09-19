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

public class PipelinePartsUpdatesRepository : IPipelinePartsUpdatesRepository
{
  private class CaseData
  {
    public required List<GrpcContextValueWithKeyName> ContextValues { get; init; }
    public required string PipelinePartName { get; init; }
  }


  private readonly SemaphoreSlim myLock = new(1);
  private readonly Dictionary<string, Dictionary<string, Dictionary<Guid, CaseData>>> myProcesses = [];
  private readonly ConcurrentDictionary<Guid, Channel<GrpcKafkaUpdate>> myChannels = [];


  public async IAsyncEnumerable<GrpcPipelinePartUpdate> StartUpdatesStream([EnumeratorCancellation] CancellationToken token)
  {
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

    try
    {
      yield return new GrpcPipelinePartUpdate
      {
        CurrentCases = currentState
      };

      while (true)
      {
        if (token.IsCancellationRequested) yield break;

        yield return new GrpcPipelinePartUpdate
        {
          Delta = await channel.Reader.ReadAsync(token)
        };
      }
    }
    finally
    {
      myChannels.Remove(sessionGuid, out _);
    }
  }

  public Task ProcessUpdate(GrpcKafkaUpdate update)
  {
    return myLock.Execute(async () =>
    {
      if (!myProcesses.TryGetValue(update.ProcessName, out var cases))
      {
        cases = new Dictionary<string, Dictionary<Guid, CaseData>>();
        myProcesses[update.ProcessName] = cases;
      }

      if (!cases.TryGetValue(update.CaseName, out var pipelinePartContextValues))
      {
        pipelinePartContextValues = new Dictionary<Guid, CaseData>();
        cases[update.CaseName] = pipelinePartContextValues;
      }

      var guid = Guid.Parse(update.PipelinePartInfo.Id.Guid);

      if (!pipelinePartContextValues.TryGetValue(guid, out var caseData))
      {
        caseData = new CaseData
        {
          ContextValues = [],
          PipelinePartName = update.PipelinePartInfo.Name
        };

        pipelinePartContextValues[guid] = caseData;
      }

      caseData.ContextValues.AddRange(update.ContextValues);

      foreach (var chanel in myChannels.Values)
      {
        await chanel.Writer.WriteAsync(update);
      }
    });
  }

  private GrpcCurrentCasesResponse GetCurrentState()
  {
    var response = new GrpcCurrentCasesResponse();
    foreach (var (processName, cases) in myProcesses)
    {
      foreach (var (caseName, contextValues) in cases)
      {
        response.Cases.Add(new GrpcCase
        {
          ProcessName = processName, 
          CaseName = caseName,
          ContextValues =
          {
            contextValues.Select(x => new GrpcPipelinePartContextValues
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