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
  private readonly SemaphoreSlim myLock = new(1);
  private readonly Dictionary<string, Dictionary<Guid, List<GrpcContextValueWithKeyName>>> myCases = [];
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
      if (!myCases.TryGetValue(update.CaseName, out var pipelinePartContextValues))
      {
        pipelinePartContextValues = new Dictionary<Guid, List<GrpcContextValueWithKeyName>>();
        myCases[update.CaseName] = pipelinePartContextValues;
      }

      var guid = Guid.Parse(update.PipelinePartInfo.Id.Guid);

      if (!pipelinePartContextValues.TryGetValue(guid, out var contextValues))
      {
        contextValues = [];
        pipelinePartContextValues[guid] = contextValues;
      }

      contextValues.AddRange(update.ContextValues);

      foreach (var chanel in myChannels.Values)
      {
        await chanel.Writer.WriteAsync(update);
      }
    });
  }

  private GrpcCurrentCasesResponse GetCurrentState()
  {
    var response = new GrpcCurrentCasesResponse();
    foreach (var (caseName, contextValues) in myCases)
    {
      response.Cases.Add(new GrpcCase
      {
        CaseName = caseName,
        ContextValues =
        {
          contextValues.Select(x => new GrpcPipelinePartContextValues
          {
            Stamp = Timestamp.FromDateTime(DateTime.UtcNow),
            ContextValues = { x.Value },
            PipelinePartInfo = new GrpcPipelinePartInfo
            {
              Name = "xd",
              Id = new GrpcGuid
              {
                Guid = x.Key.ToString()
              }
            }
          })
        }
      });
    }

    return response;
  }
}