package integrationtests

import (
	"balancer/result"
	"balancer/utils"
	"balancer/void"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"grpcmodels"
	"io"
	"os"
	"testing"

	"github.com/gkampitakis/go-snaps/snaps"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func TestPipelineExecution(t *testing.T) {
	balancerBackend, firstFicusBackend, secondFicusBackend, err := getEnvVars()
	if err != nil {
		assert.Fail(t, err.Error())
		return
	}

	cvIdRes := setInitialContextValue(t, balancerBackend)
	assert.True(t, cvIdRes.IsOk())

	balancerRes := setPipelinePartsToBackendsMap(t, balancerBackend, firstFicusBackend, secondFicusBackend)
	assert.True(t, balancerRes.IsOk())

	backendRes := executePipeline(t, balancerBackend, cvIdRes.Ok())
	assert.True(t, backendRes.IsOk())
}

func getEnvVars() (string, string, string, error) {
	balancerBackend, ok := os.LookupEnv("BALANCER_BACKEND")
	if !ok {
		return "", "", "", fmt.Errorf("BALANCER_BACKEND is not specified")
	}

	firstFicusBackend, ok := os.LookupEnv("FICUS_BACKEND_1")
	if !ok {
		return "", "", "", fmt.Errorf("FICUS_BACKEND_1 is not specified")
	}

	secondFicusBackend, ok := os.LookupEnv("FICUS_BACKEND_2")
	if !ok {
		return "", "", "", fmt.Errorf("FICUS_BACKEND_2 is not specified")
	}

	return balancerBackend, firstFicusBackend, secondFicusBackend, nil
}

func setInitialContextValue(t *testing.T, balancerBackend string) result.Result[grpcmodels.GrpcGuid] {
	return grpcmodels.ExecuteWithContextValuesClient(
		balancerBackend,
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
}

func setPipelinePartsToBackendsMap(
	t *testing.T,
	balancerBackend string,
	firstFicusBackend string,
	secondFicusBackend string,
) result.Result[void.Void] {
	return grpcmodels.ExecuteWithBalancerClient(
		balancerBackend,
		func(client grpcmodels.GrpcBackendBalancerServiceClient) result.Result[void.Void] {
			request := &grpcmodels.GrpcPredefinedPipelinePartsToBackendsMap{
				PartsToBackends: []*grpcmodels.GrpcPipelinePartToBackends{
					{
						PartName: "UseNamesEventLog",
						Backends: []string{firstFicusBackend},
					},
					{
						PartName: "GetNamesEventLog",
						Backends: []string{secondFicusBackend},
					},
				},
			}

			_, err := client.SetPipelinePartsToBackendsMap(context.Background(), request)
			assert.Nil(t, err)

			return result.Ok(void.Instance)
		},
	)
}

func createPipeline(initialContextValuesIds []*grpcmodels.GrpcGuid) *grpcmodels.GrpcProxyPipelineExecutionRequest {
	id, _ := uuid.NewV7()
	return &grpcmodels.GrpcProxyPipelineExecutionRequest{
		ContextValuesIds: initialContextValuesIds,
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
							Keys:                     []*grpcmodels.GrpcContextKey{{Name: "names_event_log"}},
							FrontendPartUuid:         &grpcmodels.GrpcGuid{Guid: id.String()},
							FrontendPipelinePartName: "PrintEventLog",
							BeforePipelinePart: &grpcmodels.GrpcPipelinePart{
								Name:          "GetNamesEventLog",
								Configuration: &grpcmodels.GrpcPipelinePartConfiguration{},
							},
						},
					},
				},
				{
					Part: &grpcmodels.GrpcPipelinePartBase_SimpleContextRequestPart{
						SimpleContextRequestPart: &grpcmodels.GrpcSimpleContextRequestPipelinePart{
							FrontendPipelinePartName: "xd",
							Key:                      &grpcmodels.GrpcContextKey{Name: "names_event_log"},
							FrontendPartUuid:         &grpcmodels.GrpcGuid{Guid: id.String()},
						},
					},
				},
			},
		},
	}
}

func executePipeline(t *testing.T, balancerBackend string, cvId *grpcmodels.GrpcGuid) result.Result[void.Void] {
	return grpcmodels.ExecuteWithBackendClient(
		balancerBackend,
		func(client grpcmodels.GrpcBackendServiceClient) result.Result[void.Void] {
			request := createPipeline([]*grpcmodels.GrpcGuid{cvId})

			outputStream, err := client.ExecutePipeline(context.Background(), request)
			assert.Nil(t, err)

			allResults := make([]*grpcmodels.GrpcPipelinePartExecutionResult, 0)
			pipelinePartsResults := make([][]*grpcmodels.GrpcContextValueWithKeyName, 0)

			for {
				res, err := outputStream.Recv()
				if errors.Is(err, io.EOF) {
					break
				}

				if err != nil {
					assert.Fail(t, err.Error())
					return result.Err[void.Void](err)
				}

				if partResult := res.GetPipelinePartResult(); partResult != nil {
					pipelinePartsResults = append(pipelinePartsResults, partResult.ContextValues)
				}

				allResults = append(allResults, res)
			}

			lastResult := allResults[len(allResults)-1]
			assert.NotNil(t, lastResult.GetFinalResult())
			assert.NotNil(t, lastResult.GetFinalResult().GetSuccess())
			assert.NotEmpty(t, lastResult.GetFinalResult().GetSuccess().Guid)

			jsonResults, err := json.MarshalIndent(pipelinePartsResults, "", "  ")
			assert.Nil(t, err)

			snaps.MatchSnapshot(t, string(jsonResults))

			executionId := lastResult.GetFinalResult().GetSuccess()
			cvIds, err := client.GetAllContextValues(context.Background(), executionId)
			assert.Nil(t, err)
			assert.Len(t, cvIds.ContextValues, 2)

			_, err = client.DropExecutionResult(context.Background(), executionId)
			assert.Nil(t, err)

			cvIds, err = client.GetAllContextValues(context.Background(), executionId)
			assert.NotNil(t, err)

			return result.Ok(void.Instance)
		},
	)
}
