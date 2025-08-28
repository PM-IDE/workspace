package integrationtests

import (
	"balancer/result"
	"balancer/utils"
	"balancer/void"
	"context"
	"errors"
	"grpcmodels"
	"io"
	"os"
	"testing"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func TestPipelineExecution(t *testing.T) {
	backend, ok := os.LookupEnv("BALANCER_BACKEND")
	if !ok {
		assert.Fail(t, "balancer backend is not specified")
		return
	}

	res := utils.ExecuteWithContextValuesClient(
		backend,
		func(client grpcmodels.GrpcContextValuesServiceClient) result.Result[grpcmodels.GrpcGuid] {
			outputStream, err := client.SetContextValue(context.Background())
			assert.Nil(t, err)

			contextValue := &grpcmodels.GrpcContextValue{
				ContextValue: &grpcmodels.GrpcContextValue_NamesLog{
					NamesLog: &grpcmodels.GrpcNamesEventLogContextValue{
						Log: &grpcmodels.GrpcNamesEventLog{
							Traces: []*grpcmodels.GrpcNamesTrace{
								{
									Events: []string{"A", "B", "C", "D"},
								},
							},
						},
					},
				},
			}

			res := utils.MarshallContextValue(utils.ContextValueWithKey{Key: "names_event_log", Value: contextValue}, outputStream)
			assert.True(t, res.IsOk())

			cvId, err := outputStream.CloseAndRecv()
			assert.Nil(t, err)

			return result.Ok(cvId)
		},
	)

	assert.True(t, res.IsOk())

	backendRes := utils.ExecuteWithBackendClient(
		backend,
		func(client grpcmodels.GrpcBackendServiceClient) result.Result[void.Void] {
			id, _ := uuid.NewV7()
			request := &grpcmodels.GrpcProxyPipelineExecutionRequest{
				ContextValuesIds: []*grpcmodels.GrpcGuid{res.Ok()},
				Pipeline: &grpcmodels.GrpcPipeline{
					Parts: []*grpcmodels.GrpcPipelinePartBase{
						{
							Part: &grpcmodels.GrpcPipelinePartBase_DefaultPart{
								DefaultPart: &grpcmodels.GrpcPipelinePart{
									Name:          "UseNamesEventLog",
									Configuration: &grpcmodels.GrpcPipelinePartConfiguration{},
								},
							},
						},
						{
							Part: &grpcmodels.GrpcPipelinePartBase_ComplexContextRequestPart{
								ComplexContextRequestPart: &grpcmodels.GrpcComplexContextRequestPipelinePart{
									Keys: []*grpcmodels.GrpcContextKey{{Name: "names_event_log"}},
									BeforePipelinePart: &grpcmodels.GrpcPipelinePart{
										Name:          "GetNamesEventLog",
										Configuration: &grpcmodels.GrpcPipelinePartConfiguration{},
									},
									FrontendPartUuid:         &grpcmodels.GrpcUuid{Uuid: id.String()},
									FrontendPipelinePartName: "PrintEventLog",
								},
							},
						},
					},
				},
			}

			outputStream, err := client.ExecutePipeline(context.Background(), request)
			assert.Nil(t, err)

			results := make([]*grpcmodels.GrpcPipelinePartExecutionResult, 0, 3)

			for {
				res, err := outputStream.Recv()
				if errors.Is(err, io.EOF) {
					break
				}

				if err != nil {
					assert.Fail(t, err.Error())
					panic(err.Error())
				}

				results = append(results, res)
			}

			assert.Len(t, results, 6)

			lastResult := results[len(results)-1]
			assert.NotNil(t, lastResult.GetFinalResult())
			assert.NotNil(t, lastResult.GetFinalResult().GetSuccess())
			assert.NotEmpty(t, lastResult.GetFinalResult().GetSuccess().Guid)

			return result.Ok(void.Instance)
		},
	)

	assert.True(t, backendRes.IsOk())
}
