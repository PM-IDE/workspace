package plan

import (
	"balancer/backends"
	"balancer/grpcmodels"
	"balancer/result"
	"balancer/void"
	"fmt"
)

type ExecutionPlanner struct {
	backendsInfo *backends.BackendsInfo
}

func NewExecutionPlanner(info *backends.BackendsInfo) *ExecutionPlanner {
	return &ExecutionPlanner{info}
}

type ExecutionPlan struct {
	nodes []*ExecutionPlanNode
}

func (this *ExecutionPlan) GetNodes() []*ExecutionPlanNode {
	return this.nodes
}

type ExecutionPlanNode struct {
	backend       string
	pipelineParts []*grpcmodels.GrpcPipelinePartBase
}

func (this *ExecutionPlanNode) GetBackend() string {
	return this.backend
}

func (this *ExecutionPlanNode) GetPipelineParts() []*grpcmodels.GrpcPipelinePartBase {
	return this.pipelineParts
}

func (this *ExecutionPlanner) CreatePlan(pipeline *grpcmodels.GrpcPipeline) result.Result[ExecutionPlan] {
	var lastUsedBackend *string = nil
	plan := ExecutionPlan{[]*ExecutionPlanNode{}}

	for _, part := range pipeline.Parts {
		if defaultPart := part.GetDefaultPart(); defaultPart != nil {
			this.processDefaultPipelinePart(part, defaultPart, lastUsedBackend, &plan)
		}
	}

	return result.Ok(&plan)
}

func (this *ExecutionPlanner) processDefaultPipelinePart(
	basePart *grpcmodels.GrpcPipelinePartBase,
	defaultPart *grpcmodels.GrpcPipelinePart,
	lastUsedBackend *string,
	plan *ExecutionPlan,
) result.Result[void.Void] {
	res := this.backendsInfo.GetBackends(defaultPart.GetName())
	if res.IsErr() {
		return result.Err[void.Void](res.Err())
	}

	selectedBackendRes := findBackendForPartName(defaultPart.GetName(), *res.Ok(), lastUsedBackend)
	if selectedBackendRes.IsErr() {
		return result.Err[void.Void](selectedBackendRes.Err())
	}

	selectedBackend := selectedBackendRes.Ok()
	if lastUsedBackend != nil && *selectedBackend == *lastUsedBackend {
		if len(plan.nodes) == 0 {
			return result.Err[void.Void](fmt.Errorf("plan should have nodes already"))
		}

		lastNodeParts := &plan.nodes[len(plan.nodes)-1].pipelineParts
		*lastNodeParts = append(*lastNodeParts, basePart)
	} else {
		plan.nodes = append(plan.nodes, &ExecutionPlanNode{
			backend:       *selectedBackend,
			pipelineParts: []*grpcmodels.GrpcPipelinePartBase{basePart},
		})
	}

	lastUsedBackend = selectedBackend

	return result.Ok(void.Instance)
}

func findBackendForPartName(partName string, backends []string, lastUsedBackend *string) result.Result[string] {
	if len(backends) == 0 {
		return result.Err[string](fmt.Errorf("there are no backends for pipeline part %s", partName))
	}

	var selectedBackend string
	if len(backends) == 1 {
		selectedBackend = backends[0]
	} else {
		for _, candidateBackend := range backends {
			if lastUsedBackend != nil && candidateBackend == *lastUsedBackend {
				selectedBackend = *lastUsedBackend
				break
			}
		}

		selectedBackend = backends[0]
	}

	return result.Ok(&selectedBackend)
}
