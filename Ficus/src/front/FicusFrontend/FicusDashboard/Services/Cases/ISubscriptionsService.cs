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

public class SubscriptionsService(
  GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client,
  ILogger<SubscriptionsService> logger
)
  : ISubscriptionsService
{
  private readonly ViewableMap<Guid, Subscription> mySubscriptions = [];


  public IViewableMap<Guid, Subscription> Subscriptions => mySubscriptions;
  public ISignal<Pipeline> AnyPipelineSubEntityUpdated { get; } = new Signal<Pipeline>();


  public void StartUpdatesStream(CancellationToken token)
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

          var reader = client.StartUpdatesStream(new Empty(), cancellationToken: token).ResponseStream;
          logger.LogDebug("Started an updates stream");

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
        }
        catch (Exception ex)
        {
          logger.LogError(ex, "Error when processing update, will reopen an updates stream");
        }
      }
    }, token);
  }

  private void ProcessInitialState(GrpcCurrentCasesResponse initialCases)
  {
    logger.LogDebug("Started processing of the initial state");

    foreach (var @case in initialCases.Cases)
    {
      var initialState = @case.ContextValues
        .Select((v, order) =>
        {
          var id = Guid.Parse(v.PipelinePartInfo.Id.Guid);
          var results = v.ExecutionResults.Select(r => new PipelinePartExecutionResult
          {
            ContextValues = r.ContextValues.Select(c => new ContextValueWrapper(c)).ToList()
          }).ToList();

          var partResults = new PipelinePartExecutionResults
          {
            PipelinePartName = v.PipelinePartInfo.Name,
            Order = (uint)order,
            Results = new ViewableList<PipelinePartExecutionResult>(results)
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

      var caseName = caseModel.FullName;
      logger.LogDebug("Updating case {CaseName} with initial state", caseName);
      processData.ProcessCases[caseName] = caseModel;
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

  private Case GetOrCreateCaseData(ProcessData processData, GrpcCaseName caseName)
  {
    var fullCaseName = CreateFullCaseName(caseName);
    if (processData.ProcessCases.TryGetValue(fullCaseName, out var @case))
    {
      logger.LogDebug("Case {CaseName} already exists", caseName);
      return @case;
    }

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
    logger.LogDebug("Added case {CaseName}", caseName);

    FirePipelineSubEntityUpdatedEvent(@case.ParentProcess.ParentPipeline);

    return @case;
  }

  private static string CreateFullCaseName(GrpcCaseName caseName) => string.Join(string.Empty, caseName.FullNameParts);

  private void HandleCaseUpdate(GrpcKafkaUpdate delta)
  {
    using var _ = logger.BeginScope(
      "Started processing of the case update, process {ProcessName}, case {CaseName}",
      delta.ProcessCaseMetadata.ProcessName,
      delta.ProcessCaseMetadata.CaseName
    );

    var processData = GetOrCreateProcessData(delta.ProcessCaseMetadata);
    var caseData = GetOrCreateCaseData(processData, delta.ProcessCaseMetadata.CaseName);

    var executionId = delta.PipelinePartInfo.ExecutionId.ToGuid();
    if (caseData.PipelineExecutionResults.ExecutionId != executionId)
    {
      caseData.PipelineExecutionResults.ExecutionId = executionId;
      caseData.PipelineExecutionResults.Results.Clear();
      logger.LogDebug("Execution ids are not equal, cleared all results");
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
        Results = new ViewableList<PipelinePartExecutionResult> { result }
      };

      logger.LogDebug("Added new pipeline part exec. results with id {PartId}", partId);
    }
    else
    {
      caseData.PipelineExecutionResults.Results[partId].Results.Add(result);
      logger.LogDebug("Added results to existing pipeline part exec. result with id {PartId}", partId);
    }
  }
}