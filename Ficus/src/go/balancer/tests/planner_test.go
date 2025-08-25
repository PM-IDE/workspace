package tests

import (
	"balancer/backends"
	"balancer/grpcmodels"
	"balancer/plan"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPlannerErrNonExistingPart(t *testing.T) {
	planner := createTestPlanner()

	planRes := planner.CreatePlan(&grpcmodels.GrpcPipeline{
		Parts: []*grpcmodels.GrpcPipelinePartBase{
			{
				Part: &grpcmodels.GrpcPipelinePartBase_DefaultPart{
					DefaultPart: &grpcmodels.GrpcPipelinePart{
						Name: "xd",
					},
				},
			},
		},
	})

	assert.True(t, planRes.IsErr())
}

func createTestPlanner() plan.ExecutionPlanner {
	return plan.NewExecutionPlanner(backends.NewBackendsInfoWithPredefinedParts(map[string][]string{
		"Part1": {"backend-1"},
		"Part2": {"backend-1"},
		"Part3": {"backend-1"},
		"Part4": {"backend-2"},
		"Part5": {"backend-2"},
		"Part6": {"backend-2"},
		"Part7": {"backend-3"},
		"Part8": {"backend-3"},
		"Part9": {"backend-3"},
	}))
}

func TestPlannerOk(t *testing.T) {
	planner := createTestPlanner()

	planRes := planner.CreatePlan(&grpcmodels.GrpcPipeline{
		Parts: []*grpcmodels.GrpcPipelinePartBase{
			{
				Part: &grpcmodels.GrpcPipelinePartBase_DefaultPart{
					DefaultPart: &grpcmodels.GrpcPipelinePart{
						Name: "Part1",
					},
				},
			},
			{
				Part: &grpcmodels.GrpcPipelinePartBase_DefaultPart{
					DefaultPart: &grpcmodels.GrpcPipelinePart{
						Name: "Part4",
					},
				},
			},
			{
				Part: &grpcmodels.GrpcPipelinePartBase_DefaultPart{
					DefaultPart: &grpcmodels.GrpcPipelinePart{
						Name: "Part7",
					},
				},
			},
			{
				Part: &grpcmodels.GrpcPipelinePartBase_DefaultPart{
					DefaultPart: &grpcmodels.GrpcPipelinePart{
						Name: "Part2",
					},
				},
			},
			{
				Part: &grpcmodels.GrpcPipelinePartBase_DefaultPart{
					DefaultPart: &grpcmodels.GrpcPipelinePart{
						Name: "Part3",
					},
				},
			},
		},
	})

	assert.True(t, planRes.IsOk())
	assert.Equal(
		t,
		planRes.Ok().String(),
		"(backend-1)[Part1], (backend-2)[Part4], (backend-3)[Part7], (backend-1)[Part2, Part3]",
	)
}
