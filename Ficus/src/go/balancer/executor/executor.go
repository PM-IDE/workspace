package executor

import (
	"balancer/backends"
	"balancer/contextvalues"
	"balancer/grpcmodels"
	"balancer/plan"
	"balancer/result"
	"balancer/utils"
	"balancer/void"

	"github.com/google/uuid"
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
) {
	for _, node := range plan.GetNodes() {
		utils.ExecuteWithBackendClient[void.Void](
			node.GetBackend(),
			func(client grpcmodels.GrpcBackendServiceClient) result.Result[void.Void] {
				return result.Ok(void.Instance)
			},
		)
	}
}
