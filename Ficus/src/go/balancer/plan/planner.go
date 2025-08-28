package plan

import (
	"balancer/backends"
	"balancer/result"
	"balancer/void"
	"fmt"
	"grpcmodels"
	"strings"
)

type ExecutionPlanner interface {
	CreatePlan(pipeline *grpcmodels.GrpcPipeline) result.Result[ExecutionPlan]
}

type executionPlanner struct {
	backendsInfo backends.BackendsInfo
}

func NewExecutionPlanner(info backends.BackendsInfo) ExecutionPlanner {
	return &executionPlanner{info}
}

type ExecutionPlan struct {
	nodes []*ExecutionPlanNode
}

func (this *ExecutionPlan) String() string {
	var sb strings.Builder

	for nodeIndex, node := range this.GetNodes() {
		sb.WriteString(fmt.Sprintf("(%s)", node.GetBackend()))
		sb.WriteRune('[')

		for partIndex, part := range node.GetPipelineParts() {
			if name := getPartNameOrNil(part); name != nil {
				sb.WriteString(*name)
			} else {
				sb.WriteString("UNRESOLVED")
			}

			if partIndex < len(node.GetPipelineParts())-1 {
				sb.WriteString(", ")
			}
		}

		sb.WriteRune(']')

		if nodeIndex < len(this.GetNodes())-1 {
			sb.WriteString(", ")
		}
	}

	return sb.String()
}

func getPartNameOrNil(part *grpcmodels.GrpcPipelinePartBase) *string {
	if defaultPart := part.GetDefaultPart(); defaultPart != nil {
		return &defaultPart.Name
	}

	if complexCvPart := part.GetComplexContextRequestPart(); complexCvPart != nil {
		return &complexCvPart.BeforePipelinePart.Name
	}

	return nil
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

func (this *executionPlanner) CreatePlan(pipeline *grpcmodels.GrpcPipeline) result.Result[ExecutionPlan] {
	var lastUsedBackend *string = nil
	plan := ExecutionPlan{[]*ExecutionPlanNode{}}

	for _, part := range pipeline.Parts {
		partName := getPartNameOrNil(part)

		if partName != nil {
			res := this.processNamedPipelinePart(part, *partName, &lastUsedBackend, &plan)
			if res.IsErr() {
				return result.Err[ExecutionPlan](res.Err())
			}

			continue
		}

		if simpleCvPart := part.GetSimpleContextRequestPart(); simpleCvPart != nil {
			this.addGetContextValuePipelinePart(part, &plan)
		}
	}

	return result.Ok(&plan)
}

func (this *executionPlanner) addGetContextValuePipelinePart(
	basePart *grpcmodels.GrpcPipelinePartBase,
	plan *ExecutionPlan,
) result.Result[void.Void] {
	if len(plan.GetNodes()) == 0 {
		return result.Err[void.Void](fmt.Errorf("there should be already nodes in the execution plan"))
	}

	lastNode := plan.nodes[len(plan.nodes)-1]
	lastNode.pipelineParts = append(lastNode.pipelineParts, basePart)

	return result.Ok(void.Instance)
}

func (this *executionPlanner) processNamedPipelinePart(
	basePart *grpcmodels.GrpcPipelinePartBase,
	partName string,
	lastUsedBackend **string,
	plan *ExecutionPlan,
) result.Result[void.Void] {
	res := this.backendsInfo.GetBackends(partName)
	if res.IsErr() {
		return result.Err[void.Void](res.Err())
	}

	selectedBackendRes := findBackendForPartName(partName, *res.Ok(), *lastUsedBackend)
	if selectedBackendRes.IsErr() {
		return result.Err[void.Void](selectedBackendRes.Err())
	}

	selectedBackend := selectedBackendRes.Ok()
	if *lastUsedBackend != nil && *selectedBackend == **lastUsedBackend {
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

	*lastUsedBackend = selectedBackend

	return result.Ok(void.Instance)
}

func findBackendForPartName(partName string, backends []string, lastUsedBackend *string) result.Result[string] {
	if len(backends) == 0 {
		return result.Err[string](fmt.Errorf("there are no backends for pipeline part %s", partName))
	}

	selectedBackend := backends[0]

	if len(backends) > 1 {
		for _, candidateBackend := range backends {
			if lastUsedBackend != nil && candidateBackend == *lastUsedBackend {
				selectedBackend = *lastUsedBackend
				break
			}
		}
	}

	return result.Ok(&selectedBackend)
}
