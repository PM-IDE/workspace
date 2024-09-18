using Ficus;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;

namespace FrontendBackend.Features.PipelineUpdates.Services;

public class PipelinePartsContextValuesService(
  IPipelinePartsUpdatesRepository repository,
  ILogger<PipelinePartsContextValuesService> logger
) : GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceBase
{
  public override async Task StartUpdatesStream(
    Empty request, IServerStreamWriter<GrpcPipelinePartUpdate> responseStream, ServerCallContext context)
  {
    try
    {
      logger.LogInformation("Received a new updates stream request");
      await foreach (var update in repository.StartUpdatesStream(context.CancellationToken))
      {
        logger.LogInformation("Sending update to client");
        await responseStream.WriteAsync(update);
      }
    }
    catch (OperationCanceledException)
    {
      logger.LogInformation("The updates stream was cancelled");
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "The stream ended with error");
    }
  }
}