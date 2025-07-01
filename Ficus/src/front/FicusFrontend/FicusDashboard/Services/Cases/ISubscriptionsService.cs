using Ficus;
using FicusDashboard.Layout.Models;
using Google.Protobuf.WellKnownTypes;
using GrpcModels;
using JetBrains.Collections.Viewable;

namespace FicusDashboard.Services.Cases;

public interface ISubscriptionsService
{
  IViewableMap<Guid, Subscription> Subscriptions { get; }
  ISignal<Pipeline> AnyPipelineSubEntityUpdated { get; }

  void StartUpdatesRequestingRoutine(CancellationToken token);
  Task<IReadOnlyDictionary<Guid, PipelinePartExecutionResults>> GetCaseExecutionResult(ProcessCaseData data);
}

public class SubscriptionsService(
  GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client,
  ILogger<SubscriptionsService> logger
)
  : ISubscriptionsService
{
  private readonly ViewableMap<Guid, Subscription> mySubscriptions = [];


  public IViewableMap<Guid, Subscription> Subscriptions => mySubscriptions;
  public ISignal<Pipeline> AnyPipelineSubEntityUpdated { get; } = new Signal<Pipeline>();


  public void StartUpdatesRequestingRoutine(CancellationToken token)
  {
    Task.Factory.StartNew(async () =>
    {
      while (true)
      {
        try
        {
          if (token.IsCancellationRequested)
          {
            logger.LogDebug("The cancellation is requested, exiting updates processing routine");
            return;
          }

          ProcessState(await client.GetSubscriptionAndPipelinesStateAsync(new Empty()));

          await Task.Delay(1_000, token);
        }
        catch (Exception ex)
        {
          logger.LogError(ex, "Error when processing update, will reopen an updates stream");
        }
      }
    }, token);
  }

  public async Task<IReadOnlyDictionary<Guid, PipelinePartExecutionResults>> GetCaseExecutionResult(ProcessCaseData data)
  {
    var result = await client.GetPipelineCaseContextValueAsync(new GrpcGetPipelineCaseContextValuesRequest
    {
      CaseName = new GrpcCaseName
      {
        DisplayName = data.Case.DisplayName,
        FullNameParts = { data.Case.NameParts }
      },
      SubscriptionId = data.ProcessData.ParentPipeline.ParentSubscription.Id.ToGrpcGuid(),
      PipelineId = data.ProcessData.ParentPipeline.Id.ToGrpcGuid(),
      ProcessName = data.ProcessData.ProcessName
    });

    return result.ContextValues.Select((value, order) =>
    {
      var id = value.PipelinePartInfo.Id.ToGuid();

      var results = value.ExecutionResults.Select(r => new PipelinePartExecutionResult
      {
        ContextValues = r.ContextValues.Select(c => new ContextValueWrapper(c)).ToList()
      }).ToList();

      var partResults = new PipelinePartExecutionResults
      {
        PipelinePartName = value.PipelinePartInfo.Name,
        Order = (uint)order,
        Results = new ViewableList<PipelinePartExecutionResult>(results)
      };

      return (id, partResults);
    }).ToDictionary();
  }

  private void ProcessState(GrpcSubscriptionAndPipelinesStateResponse reponse)
  {
    logger.LogDebug("Started processing of the initial state");

    foreach (var @case in reponse.Cases)
    {
      var processData = GetOrCreateProcessData(@case.Metadata);
      var fullCaseName = CreateFullCaseName(@case.Metadata.CaseName);

      if (!processData.ProcessCases.TryGetValue(fullCaseName, out var existingCase))
      {
        var caseModel = new Case
        {
          NameParts = @case.Metadata.CaseName.FullNameParts.ToList(),
          ParentProcess = processData,
          DisplayName = @case.Metadata.CaseName.DisplayName,
          FullName = CreateFullCaseName(@case.Metadata.CaseName),
          CreatedAt = DateTime.Now,
          ExecutionResultsStamp = @case.Stamp
        };

        var caseName = caseModel.FullName;
        logger.LogDebug("Updating case {CaseName} with initial state", caseName);
        processData.ProcessCases[caseName] = caseModel;
        FirePipelineSubEntityUpdatedEvent(processData.ParentPipeline);
      }
      else if (existingCase.ExecutionResultsStamp != @case.Stamp)
      {
        existingCase.ExecutionResultsStamp = @case.Stamp;
        existingCase.ExecutionResultsChanged.Fire();
      }
    }
  }

  private ProcessData GetOrCreateProcessData(GrpcProcessCaseMetadata metadata)
  {
    var subscription = GetOrCreateSubscription(metadata);
    var pipeline = GetOrCreatePipeline(subscription, metadata);

    var processName = metadata.ProcessName;
    if (pipeline.Processes.TryGetValue(processName, out var processData))
    {
      logger.LogDebug("Process data for process {ProcessName} already exists", processName);
      return processData;
    }

    processData = new ProcessData
    {
      ParentPipeline = pipeline,
      ProcessName = metadata.ProcessName,
      ProcessCases = new ViewableMap<string, Case>()
    };

    pipeline.Processes[processName] = processData;
    logger.LogDebug("Added new process data for process {ProcessName}", processName);

    FirePipelineSubEntityUpdatedEvent(pipeline);

    return processData;
  }

  private void FirePipelineSubEntityUpdatedEvent(Pipeline pipeline) => AnyPipelineSubEntityUpdated.Fire(pipeline);

  private Subscription GetOrCreateSubscription(GrpcProcessCaseMetadata metadata)
  {
    var subscriptionId = metadata.SubscriptionId.ToGuid();
    if (mySubscriptions.TryGetValue(subscriptionId, out var subscription))
    {
      logger.LogDebug("Subscription {SubscriptionId} already exists", subscriptionId);
      return subscription;
    }

    subscription = new Subscription
    {
      Id = subscriptionId,
      Name = metadata.SubscriptionName,
      Pipelines = new ViewableMap<Guid, Pipeline>()
    };

    mySubscriptions[subscriptionId] = subscription;
    logger.LogDebug("Added new subscription {SubscriptionId}", subscriptionId);

    return subscription;
  }

  private Pipeline GetOrCreatePipeline(Subscription subscription, GrpcProcessCaseMetadata metadata)
  {
    var pipelineId = metadata.PipelineId.ToGuid();
    if (subscription.Pipelines.TryGetValue(pipelineId, out var pipeline))
    {
      logger.LogDebug("Pipeline {PipelineId} already exists", pipelineId);
      return pipeline;
    }

    pipeline = new Pipeline
    {
      ParentSubscription = subscription,
      Id = pipelineId,
      Name = metadata.PipelineName,
      Processes = new ViewableMap<string, ProcessData>()
    };

    subscription.Pipelines[pipelineId] = pipeline;
    logger.LogDebug("Added new pipeline {PipelineId}", pipelineId);

    return pipeline;
  }

  private static string CreateFullCaseName(GrpcCaseName caseName) => string.Join(string.Empty, caseName.FullNameParts);
}