using Ficus;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;

namespace FrontendBackend.Features.PipelineUpdates.Services;

public class PipelinePartsContextValuesService(
  IPipelinePartsUpdatesRepository repository
) : GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceBase
{
  public override async Task StartUpdatesStream(Empty request, IServerStreamWriter<GrpcPipelinePartUpdate> responseStream, ServerCallContext context)
  {
    await foreach (var update in repository.StartUpdatesStream(context.CancellationToken))
    {
      await responseStream.WriteAsync(update);
    }
  }
}