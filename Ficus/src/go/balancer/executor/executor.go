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

type contextValuesWithStorage struct {
	values  []uuid.UUID
	storage *contextvalues.Storage
}

func (this *PipelineExecutor) Execute(
	plan *plan.ExecutionPlan,
	initialContextValues []uuid.UUID,
	outputChannel chan *grpcmodels.GrpcPipelinePartExecutionResult,
) result.Result[void.Void] {
	if len(plan.GetNodes()) == 0 {
		return result.Ok(void.Instance)
	}

	currentContextValues := &contextValuesWithStorage{initialContextValues, this.contextValuesStorage}

	for _, node := range plan.GetNodes() {
		contextValuesIdsRes := this.setContextValues(node.GetBackend(), currentContextValues)
		if contextValuesIdsRes.IsErr() {
			return result.FromErr(contextValuesIdsRes.Err())
		}

		newContextValuesIdsRes := this.executePipeline(node.GetBackend(), node.GetPipelineParts(), *contextValuesIdsRes.Ok(), outputChannel)
		if newContextValuesIdsRes.IsErr() {
			return result.Err[void.Void](newContextValuesIdsRes.Err())
		}

		newContextValuesRes := getContextValues(node.GetBackend(), newContextValuesIdsRes.Ok().GetContextValues())
		if newContextValuesRes.IsErr() {
			return result.Err[void.Void](newContextValuesRes.Err())
		}

		newStorage := contextvalues.NewContextValuesStorage()
		var newContextValuesIds []uuid.UUID
		for _, newContextValue := range *newContextValuesRes.Ok() {
			cvId, err := uuid.NewV7()
			if err != nil {
				return result.Err[void.Void](err)
			}

			newStorage.AddContextValue(cvId, newContextValue.Key, newContextValue.Value)
			newContextValuesIds = append(newContextValuesIds, cvId)
		}

		currentContextValues.values = newContextValuesIds
		currentContextValues.storage = newStorage
	}

	return result.Ok(void.Instance)
}

func (this *PipelineExecutor) setContextValues(
	backend string,
	currentContextValues *contextValuesWithStorage,
) result.Result[[]*grpcmodels.GrpcGuid] {
	return utils.ExecuteWithContextValuesClient[[]*grpcmodels.GrpcGuid](
		backend,
		func(client grpcmodels.GrpcContextValuesServiceClient) result.Result[[]*grpcmodels.GrpcGuid] {
			defer func() {
				if currentContextValues.storage != this.contextValuesStorage {
					currentContextValues.storage.Clear()
				}
			}()

			var contextValuesIds []*grpcmodels.GrpcGuid
			for _, cvId := range currentContextValues.values {
				stream, err := client.SetContextValue(context.Background())
				if err != nil {
					return result.Err[[]*grpcmodels.GrpcGuid](err)
				}

				cv, ok := currentContextValues.storage.GetContextValue(cvId)
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
}

func getContextValues(backend string, contextValuesIds []*grpcmodels.GrpcGuid) result.Result[[]*utils.ContextValueWithKey] {
	return utils.ExecuteWithContextValuesClient[[]*utils.ContextValueWithKey](
		backend,
		func(client grpcmodels.GrpcContextValuesServiceClient) result.Result[[]*utils.ContextValueWithKey] {
			var contextValues []*utils.ContextValueWithKey

			for _, cvId := range contextValuesIds {
				stream, err := client.GetContextValue(context.Background(), cvId)
				if err != nil {
					return result.Err[[]*utils.ContextValueWithKey](err)
				}

				cvRes := utils.UnmarshallContextValue(stream)
				if cvRes.IsErr() {
					return result.Err[[]*utils.ContextValueWithKey](cvRes.Err())
				}

				contextValues = append(contextValues, cvRes.Ok())
			}

			return result.Ok(&contextValues)
		},
	)
}

func (this *PipelineExecutor) executePipeline(
	backend string,
	pipelineParts []*grpcmodels.GrpcPipelinePartBase,
	contextValuesIds []*grpcmodels.GrpcGuid,
	outputChannel chan *grpcmodels.GrpcPipelinePartExecutionResult,
) result.Result[grpcmodels.GrpcGetAllContextValuesResult] {
	return utils.ExecuteWithBackendClient[grpcmodels.GrpcGetAllContextValuesResult](
		backend,
		func(client grpcmodels.GrpcBackendServiceClient) result.Result[grpcmodels.GrpcGetAllContextValuesResult] {
			newPipeline := &grpcmodels.GrpcPipeline{
				Parts: pipelineParts,
			}

			request := &grpcmodels.GrpcProxyPipelineExecutionRequest{
				ContextValuesIds: contextValuesIds,
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
}
