package integrationtests

import (
  "balancer/result"
  "balancer/utils"
  "balancer/void"
  "context"
  "encoding/json"
  "errors"
  "grpcmodels"
  "io"
  "os"
  "testing"

  "github.com/gkampitakis/go-snaps/snaps"
  "github.com/google/uuid"
  "github.com/stretchr/testify/assert"
)

func TestPipelineExecution(t *testing.T) {
  balancerBackend, ok := os.LookupEnv("BALANCER_BACKEND")
  if !ok {
    assert.Fail(t, "balancer balancerBackend is not specified")
    return
  }

  ficusBackend, ok := os.LookupEnv("FICUS_BACKEND")
  if !ok {
    assert.Fail(t, "balancer balancerBackend is not specified")
    return
  }

  res := utils.ExecuteWithContextValuesClient(
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

  assert.True(t, res.IsOk())

  balancerRes := utils.ExecuteWithBalancerClient(
    balancerBackend,
    func(client grpcmodels.GrpcBackendBalancerServiceClient) result.Result[void.Void] {
      request := &grpcmodels.GrpcPredefinedPipelinePartsToBackendsMap{
        PartsToBackends: []*grpcmodels.GrpcPipelinePartToBackends{
          {
            PartName: "UseNamesEventLog",
            Backends: []string{ficusBackend},
          },
          {
            PartName: "GetNamesEventLog",
            Backends: []string{ficusBackend},
          },
        },
      }

      _, err := client.SetPipelinePartsToBackendsMap(context.Background(), request)
      assert.Nil(t, err)

      return result.Ok(void.Instance)
    },
  )

  assert.True(t, balancerRes.IsOk())

  backendRes := utils.ExecuteWithBackendClient(
    balancerBackend,
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

      return result.Ok(void.Instance)
    },
  )

  assert.True(t, backendRes.IsOk())
}
