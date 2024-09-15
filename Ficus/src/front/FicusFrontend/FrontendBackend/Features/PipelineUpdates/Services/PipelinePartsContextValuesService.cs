using Ficus;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;

namespace FrontendBackend.Features.PipelineUpdates.Services;

public class PipelinePartsContextValuesService(
  IPipelinePartsUpdatesRepository repository
) : GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceBase
{
  public override Task<Empty> StartUpdatesStream(IAsyncStreamReader<GrpcKafkaUpdate> requestStream, ServerCallContext context)
  {
    return base.StartUpdatesStream(requestStream, context);
  }
}