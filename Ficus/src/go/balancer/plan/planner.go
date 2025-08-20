package plan

import (
	"balancer/backends"
	grpcmodels "balancer/models"
	"balancer/result"
	"fmt"
)

type ExecutionPlanner struct {
	backendsInfo *backends.BackendsInfo
}

func NewExecutionPlanner(info *backends.BackendsInfo) *ExecutionPlanner {
	return &ExecutionPlanner{info}
}

type ExecutionPlan struct {
	nodes []ExecutionPlanNode
}

type ExecutionPlanNode struct {
	backend       string
	pipelineParts []*grpcmodels.GrpcPipelinePartBase
}

func (this *ExecutionPlanner) CreatePlan(request *grpcmodels.GrpcPipelineExecutionRequest) result.Result[ExecutionPlan] {
	pipeline := request.GetPipeline()

	var lastUsedBackend *string = nil
	plan := ExecutionPlan{[]ExecutionPlanNode{}}

	for _, part := range pipeline.Parts {
		if defaultPart := part.GetDefaultPart(); defaultPart != nil {
			res := this.backendsInfo.GetBackends(defaultPart.GetName())
			if res.IsErr() {
				return result.Err[ExecutionPlan](res.Err())
			}

			partBackends := *res.Ok()

			if len(partBackends) == 0 {
				return result.Err[ExecutionPlan](fmt.Errorf("there are no backends for pipeline part %s", defaultPart.GetName()))
			}

			var selectedBackend string
			if len(partBackends) == 1 {
				selectedBackend = partBackends[0]
			} else {
				for _, candidateBackend := range partBackends {
					if lastUsedBackend != nil && candidateBackend == *lastUsedBackend {
						selectedBackend = *lastUsedBackend
						break
					}
				}

				selectedBackend = partBackends[0]
			}

			if lastUsedBackend != nil && selectedBackend == *lastUsedBackend {
				if len(plan.nodes) == 0 {
					return result.Err[ExecutionPlan](fmt.Errorf("plan should have nodes already"))
				}

				lastNodeParts := &plan.nodes[len(plan.nodes)-1].pipelineParts
				*lastNodeParts = append(*lastNodeParts, part)
			} else {
				newNode := ExecutionPlanNode{
					backend:       selectedBackend,
					pipelineParts: []*grpcmodels.GrpcPipelinePartBase{part},
				}

				plan.nodes = append(plan.nodes, newNode)
			}

			lastUsedBackend = &selectedBackend
		}
	}

	return result.Ok(&plan)
}
