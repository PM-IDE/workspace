using Ficus;
using Google.Protobuf.WellKnownTypes;
using GrpcModels;
using JetBrains.Collections.Viewable;

namespace FicusFrontend.Services.Cases;

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
        .Select(v =>
        {
          var id = Guid.Parse(v.PipelinePartInfo.Id.Guid);
          return (id, new PipelinePartExecutionResult
          {
            ContextValues = v.ContextValues.ToList(),
            PipelinePartName = v.PipelinePartInfo.Name
          });
        })
        .ToDictionary();

      var processData = GetOrCreateProcessData(@case.ProcessCaseMetadata);

      var caseModel = new Case
      {
        ParentProcess = processData,
        Name = @case.ProcessCaseMetadata.CaseName,
        CreatedAt = DateTime.Now,
        ContextValues = new ViewableMap<Guid, PipelinePartExecutionResult>(initialState)
      };

      processData.ProcessCases[caseModel.Name] = caseModel;
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

  private Case GetOrCreateCaseData(ProcessData processData, string caseName)
  {
    if (processData.ProcessCases.TryGetValue(caseName, out var @case)) return @case;

    @case = new Case
    {
      ParentProcess = processData,
      Name = caseName,
      CreatedAt = DateTime.Now,
      ContextValues = new ViewableMap<Guid, PipelinePartExecutionResult>()
    };

    processData.ProcessCases[caseName] = @case;

    FirePipelineSubEntityUpdatedEvent(@case.ParentProcess.ParentPipeline);

    return @case;
  }

  private void HandleCaseUpdate(GrpcKafkaUpdate delta)
  {
    var processData = GetOrCreateProcessData(delta.ProcessCaseMetadata);
    var caseData = GetOrCreateCaseData(processData, delta.ProcessCaseMetadata.CaseName);

    var partId = Guid.Parse(delta.PipelinePartInfo.Id.Guid);
    caseData.ContextValues[partId] = new PipelinePartExecutionResult
    {
      ContextValues = delta.ContextValues.ToList(),
      PipelinePartName = delta.PipelinePartInfo.Name
    };
  }
}