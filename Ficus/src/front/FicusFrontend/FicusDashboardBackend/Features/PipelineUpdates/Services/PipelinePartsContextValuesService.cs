using Ficus;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;

namespace FicusDashboardBackend.Features.PipelineUpdates.Services;

public class PipelinePartsContextValuesService(
  IPipelinePartsUpdatesRepository repository,
  ILogger<PipelinePartsContextValuesService> logger
) : GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceBase
{
  public override Task<GrpcCaseContextValues> GetPipelineCaseContextValue(
    GrpcGetPipelineCaseContextValuesRequest request, ServerCallContext context)
  {
    return repository.GetCaseContextValues(request);
  }

  public override Task<GrpcSubscriptionAndPipelinesStateResponse> GetSubscriptionAndPipelinesState(
    Empty request, ServerCallContext context)
  {
    return repository.GetCurrentState();
  }
}