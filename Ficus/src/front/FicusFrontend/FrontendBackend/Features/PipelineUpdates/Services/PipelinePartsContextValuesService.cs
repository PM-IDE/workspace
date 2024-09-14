using Ficus;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;

namespace FrontendBackend.Features.PipelineUpdates.Services;

public class PipelinePartsContextValuesService : GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceBase
{
  public override Task<GrpcCurrentCasesResponse> GetCurrentCases(Empty request, ServerCallContext context)
  {
    return base.GetCurrentCases(request, context);
  }

  public override Task<Empty> StartUpdatesStream(IAsyncStreamReader<GrpcKafkaUpdate> requestStream, ServerCallContext context)
  {
    return base.StartUpdatesStream(requestStream, context);
  }
}