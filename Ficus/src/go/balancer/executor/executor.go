package executor

import (
	"balancer/backends"
	"balancer/contextvalues"
	"balancer/plan"
	"balancer/result"
	"balancer/utils"
	"context"
	"fmt"
	"grpcmodels"
	"io"

	"github.com/google/uuid"
	cmap "github.com/orcaman/concurrent-map/v2"
	"go.uber.org/zap"
)

type PipelineExecutor interface {
	GetContextValues(executionId uuid.UUID) (map[string]uuid.UUID, bool)
	DropExecutionResult(id uuid.UUID)

	Execute(
		plan *plan.ExecutionPlan,
		initialContextValues []uuid.UUID,
		outputChannel chan *grpcmodels.GrpcPipelinePartExecutionResult,
		logger *zap.SugaredLogger,
	) result.Result[uuid.UUID]
}

type pipelineExecutor struct {
	backendsInfo         backends.BackendsInfo
	contextValuesStorage contextvalues.Storage
	executions           cmap.ConcurrentMap[uuid.UUID, map[string]uuid.UUID]
}

func NewPipelineExecutor(backendsInfo backends.BackendsInfo, storage contextvalues.Storage) PipelineExecutor {
	return &pipelineExecutor{
		backendsInfo,
		storage,
		cmap.NewStringer[uuid.UUID, map[string]uuid.UUID](),
	}
}

type contextValuesWithStorage struct {
	values  []uuid.UUID
	storage contextvalues.Storage
}

func (this *pipelineExecutor) GetContextValues(executionId uuid.UUID) (map[string]uuid.UUID, bool) {
	return this.executions.Get(executionId)
}

func (this *pipelineExecutor) DropExecutionResult(id uuid.UUID) {
	this.executions.Remove(id)
}

func (this *pipelineExecutor) Execute(
	plan *plan.ExecutionPlan,
	initialContextValues []uuid.UUID,
	outputChannel chan *grpcmodels.GrpcPipelinePartExecutionResult,
	logger *zap.SugaredLogger,
) result.Result[uuid.UUID] {
	defer close(outputChannel)

	if len(plan.GetNodes()) == 0 {
		return result.Ok(&uuid.Nil)
	}

	executionId, err := uuid.NewV7()
	if err != nil {
		return result.Err[uuid.UUID](err)
	}

	logger.Infow("Generated execution id", "execution_id", executionId, "initial_context_values", initialContextValues)

	currentContextValues := &contextValuesWithStorage{initialContextValues, this.contextValuesStorage}

	for index, node := range plan.GetNodes() {
		logger = logger.With("index", index)
		logger.Infow("Started executing new node")

		contextValuesIdsRes := this.setContextValues(node.GetBackend(), currentContextValues, logger)
		if contextValuesIdsRes.IsErr() {
			return result.Err[uuid.UUID](contextValuesIdsRes.Err())
		}

		logger.Infow("Set context values", "current_context_values", currentContextValues)

		lastParts := index == len(plan.GetNodes())-1

		newContextValuesIdsRes := this.executePipelineParts(
			node.GetBackend(),
			node.GetPipelineParts(),
			*contextValuesIdsRes.Ok(),
			outputChannel,
			executionId,
			lastParts,
		)

		if newContextValuesIdsRes.IsErr() {
			return result.Err[uuid.UUID](newContextValuesIdsRes.Err())
		}

		logger.Infow("Executed pipeline parts")

		newContextValuesRes := getContextValues(node.GetBackend(), newContextValuesIdsRes.Ok().GetContextValues())
		if newContextValuesRes.IsErr() {
			return result.Err[uuid.UUID](newContextValuesRes.Err())
		}

		var newStorage contextvalues.Storage

		if lastParts {
			logger.Infow("Using shared context values storage")
			newStorage = this.contextValuesStorage
		} else {
			logger.Infow("Using new context values storage")
			newStorage = contextvalues.NewContextValuesStorage()
		}

		var newContextValuesIds []uuid.UUID

		for _, newContextValue := range *newContextValuesRes.Ok() {
			cvId, err := uuid.NewV7()
			if err != nil {
				return result.Err[uuid.UUID](err)
			}

			newStorage.AddContextValue(cvId, newContextValue.Key, newContextValue.Value)
			logger.Infow("Added new context value to storage", "context_value_id", cvId, "context_value_key", newContextValue.Key)

			newContextValuesIds = append(newContextValuesIds, cvId)
		}

		if lastParts {
			executionResultContextValues := make(map[string]uuid.UUID)
			for _, id := range newContextValuesIds {
				cv, ok := newStorage.GetContextValue(id)
				if !ok {
					return result.Err[uuid.UUID](fmt.Errorf("execution result context value is not found for id %w", id))
				}

				executionResultContextValues[cv.Key.Name] = id
			}

			logger.Infow("Set final execution result context values", "final_context_values", executionResultContextValues)
			this.executions.Set(executionId, executionResultContextValues)
			break
		} else {
			currentContextValues.values = newContextValuesIds
			currentContextValues.storage = newStorage
		}
	}

	logger.Infow("Finished execution of pipeline")

	return result.Ok(&executionId)
}

func (this *pipelineExecutor) setContextValues(
	backend string,
	currentContextValues *contextValuesWithStorage,
	logger *zap.SugaredLogger,
) result.Result[[]*grpcmodels.GrpcGuid] {
	return grpcmodels.ExecuteWithContextValuesClient[[]*grpcmodels.GrpcGuid](
		backend,
		func(client grpcmodels.GrpcContextValuesServiceClient) result.Result[[]*grpcmodels.GrpcGuid] {
			defer func() {
				if currentContextValues.storage != this.contextValuesStorage {
					logger.Infow("Cleaning context values storage")
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
					res := utils.MarshallContextValue(utils.ContextValueWithKey{Key: cv.Key.Name, Value: cv.Value}, stream)
					if res.IsErr() {
						return result.Err[[]*grpcmodels.GrpcGuid](err)
					}

					cvId, err := stream.CloseAndRecv()
					if err != nil {
						return result.Err[[]*grpcmodels.GrpcGuid](err)
					}

					logger.Infow("Added context value", "context_value_id", cvId)
					contextValuesIds = append(contextValuesIds, cvId)
				}
			}

			logger.Infow("Final context values", "context_value_ids", contextValuesIds)
			return result.Ok(&contextValuesIds)
		},
	)
}

func getContextValues(backend string, contextValuesIds []*grpcmodels.GrpcGuid) result.Result[[]*utils.ContextValueWithKey] {
	return grpcmodels.ExecuteWithContextValuesClient[[]*utils.ContextValueWithKey](
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

func (this *pipelineExecutor) executePipelineParts(
	backend string,
	pipelineParts []*grpcmodels.GrpcPipelinePartBase,
	contextValuesIds []*grpcmodels.GrpcGuid,
	outputChannel chan *grpcmodels.GrpcPipelinePartExecutionResult,
	executionId uuid.UUID,
	lastParts bool,
) result.Result[grpcmodels.GrpcGetAllContextValuesResult] {
	return grpcmodels.ExecuteWithBackendClient[grpcmodels.GrpcGetAllContextValuesResult](
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
					if backendExecutionId := finalResult.GetSuccess(); backendExecutionId != nil {
						execId = backendExecutionId

						if lastParts {
							patchedResult := &grpcmodels.GrpcPipelinePartExecutionResult{
								Result: &grpcmodels.GrpcPipelinePartExecutionResult_FinalResult{
									FinalResult: &grpcmodels.GrpcPipelineFinalResult{
										ExecutionResult: &grpcmodels.GrpcPipelineFinalResult_Success{
											Success: &grpcmodels.GrpcGuid{
												Guid: executionId.String(),
											},
										},
									},
								},
							}

							outputChannel <- patchedResult
						}

						break
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
