package executor

import (
	"balancer/backends"
	"balancer/contextvalues"
	"balancer/grpcmodels"
	"balancer/plan"
	"balancer/result"
	"balancer/utils"
	"balancer/void"
	"context"
	"fmt"
	"io"
	"slices"

	"github.com/google/uuid"
	"google.golang.org/protobuf/proto"
)

type PipelineExecutor struct {
	backendsInfo         *backends.BackendsInfo
	contextValuesStorage *contextvalues.Storage
}

func NewPipelineExecutor(backendsInfo *backends.BackendsInfo, storage *contextvalues.Storage) *PipelineExecutor {
	return &PipelineExecutor{backendsInfo, storage}
}

func (this *PipelineExecutor) Execute(
	plan *plan.ExecutionPlan,
	initialContextValues []uuid.UUID,
	outputChannel chan *grpcmodels.GrpcPipelinePartExecutionResult,
) result.Result[void.Void] {
	if len(plan.GetNodes()) == 0 {
		return result.Ok(void.Instance)
	}

	currentContextValuesIds := initialContextValues
	for _, node := range plan.GetNodes() {
		contextValuesIds := utils.ExecuteWithContextValuesClient[[]*grpcmodels.GrpcGuid](
			node.GetBackend(),
			func(client grpcmodels.GrpcContextValuesServiceClient) result.Result[[]*grpcmodels.GrpcGuid] {
				var contextValuesIds []*grpcmodels.GrpcGuid
				for _, cvId := range currentContextValuesIds {
					stream, err := client.SetContextValue(context.Background())
					if err != nil {
						return result.Err[[]*grpcmodels.GrpcGuid](err)
					}

					cv, ok := this.contextValuesStorage.GetContextValue(cvId)
					if ok {
						cvBytes, err := proto.Marshal(cv.Value)
						if err != nil {
							return result.Err[[]*grpcmodels.GrpcGuid](err)
						}

						for chunk := range slices.Chunk(cvBytes, 1024) {
							err = stream.Send(&grpcmodels.GrpcContextValuePart{
								Key:   cv.Key.Name,
								Bytes: chunk,
							})

							if err != nil {
								return result.Err[[]*grpcmodels.GrpcGuid](err)
							}
						}

						reply, err := stream.CloseAndRecv()
						if err != nil {
							return result.Err[[]*grpcmodels.GrpcGuid](err)
						}

						contextValuesIds = append(contextValuesIds, reply)
					}
				}

				return result.Ok(&contextValuesIds)
			},
		)

		newContextValuesRes := utils.ExecuteWithBackendClient[grpcmodels.GrpcGetAllContextValuesResult](
			node.GetBackend(),
			func(client grpcmodels.GrpcBackendServiceClient) result.Result[grpcmodels.GrpcGetAllContextValuesResult] {
				newPipeline := &grpcmodels.GrpcPipeline{
					Parts: node.GetPipelineParts(),
				}

				request := &grpcmodels.GrpcProxyPipelineExecutionRequest{
					ContextValuesIds: *contextValuesIds.Ok(),
					Pipeline:         newPipeline,
				}

				resultsStream, err := client.ExecutePipeline(context.Background(), request)

				if err != nil {
					return result.Err[grpcmodels.GrpcGetAllContextValuesResult](err)
				}

				var execId *grpcmodels.GrpcGuid
				for {
					execResult, err := resultsStream.Recv()
					if err == io.EOF {
						break
					}

					if err != nil {
						return result.Err[grpcmodels.GrpcGetAllContextValuesResult](err)
					}

					if finalResult := execResult.GetFinalResult(); finalResult != nil {
						if success := finalResult.GetSuccess(); success != nil {
							execId = success
						} else {
							outputChannel <- execResult
							err = fmt.Errorf("the final result is error: %s", finalResult.GetError())
							return result.Err[grpcmodels.GrpcGetAllContextValuesResult](err)
						}
					}

					outputChannel <- execResult
				}

				newContextValues, err := client.GetAllContextValues(context.Background(), execId)
				if err != nil {
					return result.Err[grpcmodels.GrpcGetAllContextValuesResult](err)
				}

				return result.Ok(newContextValues)
			},
		)

		if newContextValuesRes.IsErr() {
			return result.Err[void.Void](newContextValuesRes.Err())
		}

		var newContextValuesIds []uuid.UUID
		for _, newContextValue := range newContextValuesRes.Ok().GetContextValues() {
			if value := newContextValue.GetValue(); value != nil {
				cvId, err := uuid.NewV7()
				if err != nil {
					return result.Err[void.Void](err)
				}

				this.contextValuesStorage.AddContextValue(cvId, newContextValue.Key, value)
				newContextValuesIds = append(newContextValuesIds, cvId)
			}
		}

		currentContextValuesIds = newContextValuesIds
	}

	return result.Ok(void.Instance)
}
