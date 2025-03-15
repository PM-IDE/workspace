using Ficus;
using Google.Protobuf.WellKnownTypes;
using GrpcModels;
using JetBrains.Collections.Viewable;

namespace FicusDashboard.Services.Cases;

public interface ISubscriptionsService
{
  IViewableMap<Guid, Subscription> Subscriptions { get; }
  ISignal<Pipeline> AnyPipelineSubEntityUpdated { get; }

  void StartUpdatesStream(CancellationToken token);
}

public class SubscriptionsService(GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client)
  : ISubscriptionsService
{
  private readonly ViewableMap<Guid, Subscription> mySubscriptions = [];


  public IViewableMap<Guid, Subscription> Subscriptions => mySubscriptions;
  public ISignal<Pipeline> AnyPipelineSubEntityUpdated { get; } = new Signal<Pipeline>();


  public void StartUpdatesStream(CancellationToken token)
  {
    Task.Factory.StartNew(async () =>
    {
      var reader = client.StartUpdatesStream(new Empty(), cancellationToken: token).ResponseStream;
      while (await reader.MoveNext(token))
      {
        switch (reader.Current.UpdateCase)
        {
          case GrpcPipelinePartUpdate.UpdateOneofCase.CurrentCases:
            ProcessInitialState(reader.Current.CurrentCases);
            break;
          case GrpcPipelinePartUpdate.UpdateOneofCase.Delta:
            HandleCaseUpdate(reader.Current.Delta);
            break;
          default:
            throw new ArgumentOutOfRangeException();
        }
      }
    }, token);
  }

  private void ProcessInitialState(GrpcCurrentCasesResponse initialCases)
  {
    foreach (var @case in initialCases.Cases)
    {
      var initialState = @case.ContextValues
        .Select((v, order) =>
        {
          var id = Guid.Parse(v.PipelinePartInfo.Id.Guid);
          var partResults = new PipelinePartExecutionResults
          {
            PipelinePartName = v.PipelinePartInfo.Name,
            Order = (uint)order,
            Results = v.ExecutionResults.Select(r => new PipelinePartExecutionResult
            {
              ContextValues = r.ContextValues.Select(c => new ContextValueWrapper(c)).ToList()
            }).ToList()
          };

          return (id, partResults);
        })
        .ToDictionary();

      var processData = GetOrCreateProcessData(@case.ProcessCaseMetadata);

      var caseModel = new Case
      {
        NameParts = @case.ProcessCaseMetadata.CaseName.FullNameParts.ToList(),
        ParentProcess = processData,
        DisplayName = @case.ProcessCaseMetadata.CaseName.DisplayName,
        FullName = CreateFullCaseName(@case.ProcessCaseMetadata.CaseName),
        CreatedAt = DateTime.Now,
        PipelineExecutionResults = new PipelinePartsExecutionResults
        {
          Results = new ViewableMap<Guid, PipelinePartExecutionResults>(initialState),
          ExecutionId = @case.ContextValues.FirstOrDefault()?.PipelinePartInfo.ExecutionId.ToGuid() ?? Guid.Empty
        }
      };

      processData.ProcessCases[caseModel.FullName] = caseModel;
    }
  }

  private ProcessData GetOrCreateProcessData(GrpcProcessCaseMetadata metadata)
  {
    var subscription = GetOrCreateSubscription(metadata);
    var pipeline = GetOrCreatePipeline(subscription, metadata);

    var processName = metadata.ProcessName;
    if (pipeline.Processes.TryGetValue(processName, out var processData)) return processData;

    processData = new ProcessData
    {
      ParentPipeline = pipeline,
      ProcessName = metadata.ProcessName,
      ProcessCases = new ViewableMap<string, Case>()
    };

    pipeline.Processes[processName] = processData;

    FirePipelineSubEntityUpdatedEvent(pipeline);

    return processData;
  }

  private void FirePipelineSubEntityUpdatedEvent(Pipeline pipeline) => AnyPipelineSubEntityUpdated.Fire(pipeline);

  private Subscription GetOrCreateSubscription(GrpcProcessCaseMetadata metadata)
  {
    var subscriptionId = metadata.SubscriptionId.ToGuid();
    if (mySubscriptions.TryGetValue(subscriptionId, out var subscription)) return subscription;

    subscription = new Subscription
    {
      Id = subscriptionId,
      Name = metadata.SubscriptionName,
      Pipelines = new ViewableMap<Guid, Pipeline>()
    };

    mySubscriptions[subscriptionId] = subscription;
    return subscription;
  }

  private Pipeline GetOrCreatePipeline(Subscription subscription, GrpcProcessCaseMetadata metadata)
  {
    var pipelineId = metadata.PipelineId.ToGuid();
    if (subscription.Pipelines.TryGetValue(pipelineId, out var pipeline)) return pipeline;

    pipeline = new Pipeline
    {
      ParentSubscription = subscription,
      Id = pipelineId,
      Name = metadata.PipelineName,
      Processes = new ViewableMap<string, ProcessData>()
    };

    subscription.Pipelines[pipelineId] = pipeline;

    return pipeline;
  }

  private Case GetOrCreateCaseData(ProcessData processData, GrpcCaseName caseName)
  {
    var fullCaseName = CreateFullCaseName(caseName);
    if (processData.ProcessCases.TryGetValue(fullCaseName, out var @case)) return @case;

    @case = new Case
    {
      ParentProcess = processData,
      DisplayName = caseName.DisplayName,
      NameParts = caseName.FullNameParts.ToList(),
      FullName = fullCaseName,
      CreatedAt = DateTime.Now,
      PipelineExecutionResults = new PipelinePartsExecutionResults
      {
        ExecutionId = Guid.Empty,
        Results = new ViewableMap<Guid, PipelinePartExecutionResults>()
      }
    };

    processData.ProcessCases[fullCaseName] = @case;

    FirePipelineSubEntityUpdatedEvent(@case.ParentProcess.ParentPipeline);

    return @case;
  }

  private static string CreateFullCaseName(GrpcCaseName caseName) => string.Join(string.Empty, caseName.FullNameParts);

  private void HandleCaseUpdate(GrpcKafkaUpdate delta)
  {
    var processData = GetOrCreateProcessData(delta.ProcessCaseMetadata);
    var caseData = GetOrCreateCaseData(processData, delta.ProcessCaseMetadata.CaseName);

    var executionId = delta.PipelinePartInfo.ExecutionId.ToGuid();
    if (caseData.PipelineExecutionResults.ExecutionId != executionId)
    {
      caseData.PipelineExecutionResults.ExecutionId = executionId;
      caseData.PipelineExecutionResults.Results.Clear();
    }

    var partId = Guid.Parse(delta.PipelinePartInfo.Id.Guid);
    var result = new PipelinePartExecutionResult
    {
      ContextValues = delta.ContextValues.Select(c => new ContextValueWrapper(c)).ToList()
    };

    if (!caseData.PipelineExecutionResults.Results.ContainsKey(partId))
    {
      caseData.PipelineExecutionResults.Results[partId] = new PipelinePartExecutionResults
      {
        Order = (uint)caseData.PipelineExecutionResults.Results.Count,
        PipelinePartName = delta.PipelinePartInfo.Name,
        Results = [result]
      };
    }
    else
    {
      caseData.PipelineExecutionResults.Results[partId].Results.Add(result);
    }
  }
}