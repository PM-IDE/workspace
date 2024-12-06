using System.Runtime.CompilerServices;
using Ficus;
using Google.Protobuf.WellKnownTypes;
using GrpcModels;
using JetBrains.Collections.Viewable;

namespace FicusFrontend.Services.Cases;

public interface IProcessesService
{
  IAsyncEnumerable<ProcessUpdate> OpenCasesUpdatesStream(CancellationToken token);
}

public class ProcessesService(GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client)
  : IProcessesService
{
  private readonly Dictionary<Guid, Subscription> mySubscriptions = [];


  public ViewableMap<Guid, CaseData.PipelinePartExecutionResult> CreateCaseValuesObservable(
    ProcessData processData, Case selectedCase)
  {
    return processData.ProcessCases[selectedCase.Name].ContextValues;
  }

  public async IAsyncEnumerable<ProcessUpdate> OpenCasesUpdatesStream(
    [EnumeratorCancellation] CancellationToken token)
  {
    var reader = client.StartUpdatesStream(new Empty(), cancellationToken: token).ResponseStream;

    while (await reader.MoveNext(token))
    {
      var updates = reader.Current.UpdateCase switch
      {
        GrpcPipelinePartUpdate.UpdateOneofCase.CurrentCases => ProcessInitialState(reader.Current.CurrentCases),
        GrpcPipelinePartUpdate.UpdateOneofCase.Delta => ProcessCaseUpdate(reader.Current.Delta),
        _ => throw new ArgumentOutOfRangeException()
      };

      foreach (var update in updates)
      {
        yield return update;
      }
    }
  }

  private IEnumerable<ProcessUpdate> ProcessInitialState(GrpcCurrentCasesResponse initialCases)
  {
    var anyUpdates = false;
    foreach (var @case in initialCases.Cases)
    {
      var caseModel = new Case
      {
        Name = @case.ProcessCaseMetadata.CaseName,
        CreatedAt = DateTime.Now
      };

      var initialState = @case.ContextValues
        .Select(v =>
        {
          var id = Guid.Parse(v.PipelinePartInfo.Id.Guid);
          return (id, new CaseData.PipelinePartExecutionResult
          {
            ContextValues = v.ContextValues.ToList(),
            PipelinePartName = v.PipelinePartInfo.Name
          });
        })
        .ToDictionary();

      anyUpdates |= GetOrCreateProcessData(@case.ProcessCaseMetadata, out var processData);
      processData.ProcessCases[caseModel.Name] = new CaseData
      {
        Case = caseModel,
        ContextValues = new ViewableMap<Guid, CaseData.PipelinePartExecutionResult>(initialState)
      };
    }

    if (anyUpdates)
    {
      yield return new ProcessesListUpdate
      {
        Processes = GetAllProcesses()
      };
    }
  }

  private List<ProcessData> GetAllProcesses()
  {
    return mySubscriptions.Values.SelectMany(s => s.Pipelines.Values).SelectMany(p => p.Processes.Values).ToList();
  }

  private bool GetOrCreateProcessData(GrpcProcessCaseMetadata metadata, out ProcessData processData)
  {
    var subscription = GetOrCreateSubscription(metadata);
    var pipeline = GetOrCreatePipeline(subscription, metadata);

    var processName = metadata.ProcessName;
    if (pipeline.Processes.TryGetValue(processName, out processData)) return false;

    processData = new ProcessData
    {
      ParentPipeline = pipeline,
      ProcessName = metadata.ProcessName,
      ProcessCases = []
    };

    pipeline.Processes[processName] = processData;
    return true;
  }

  private Subscription GetOrCreateSubscription(GrpcProcessCaseMetadata metadata)
  {
    var subscriptionId = metadata.SubscriptionId.ToGuid();
    if (mySubscriptions.TryGetValue(subscriptionId, out var subscription)) return subscription;

    subscription = new Subscription
    {
      Id = subscriptionId,
      Name = metadata.SubscriptionName,
      Pipelines = []
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
      Processes = []
    };

    subscription.Pipelines[pipelineId] = pipeline;

    return pipeline;
  }

  private bool GetOrCreateCaseData(ProcessData processData, string caseName, out CaseData caseData)
  {
    if (processData.ProcessCases.TryGetValue(caseName, out caseData)) return false;

    caseData = new CaseData
    {
      Case = new Case
      {
        Name = caseName,
        CreatedAt = DateTime.Now
      },
      ContextValues = []
    };

    processData.ProcessCases[caseName] = caseData;

    return true;
  }

  private IEnumerable<ProcessUpdate> ProcessCaseUpdate(GrpcKafkaUpdate delta)
  {
    var anyUpdates = GetOrCreateProcessData(delta.ProcessCaseMetadata, out var processData);
    anyUpdates |= GetOrCreateCaseData(processData, delta.ProcessCaseMetadata.CaseName, out var caseData);

    if (anyUpdates)
    {
      yield return new ProcessesListUpdate
      {
        Processes = GetAllProcesses()
      }; 
    }

    var partId = Guid.Parse(delta.PipelinePartInfo.Id.Guid);
    caseData.ContextValues[partId] = new CaseData.PipelinePartExecutionResult
    {
      ContextValues = delta.ContextValues.ToList(),
      PipelinePartName = delta.PipelinePartInfo.Name
    };

    yield return new ProcessContextValuesUpdate
    {
      CaseName = delta.ProcessCaseMetadata.CaseName,
      ProcessName = delta.ProcessCaseMetadata.ProcessName
    };
  }
}