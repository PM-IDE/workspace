﻿using System.Runtime.CompilerServices;
using Ficus;
using Google.Protobuf.WellKnownTypes;
using ObservableCollections;

namespace FicusFrontend.Services.Cases;

public interface IProcessesService
{
  IAsyncEnumerable<ProcessUpdate> OpenCasesUpdatesStream(CancellationToken token);

  IReadOnlyObservableDictionary<Guid, CaseData.PipelinePartExecutionResult> CreateCaseValuesObservable(
    ProcessData processData, Case selectedCase);
}

public class ProcessData
{
  public required string ProcessName { get; init; }
  public required Dictionary<string, CaseData> ProcessCases { get; init; }
}

public class CaseData
{
  public class PipelinePartExecutionResult
  {
    public required string PipelinePartName { get; init; }
    public required List<GrpcContextValueWithKeyName> ContextValues { get; init; }
  }

  public required Case Case { get; init; }
  public required ObservableDictionary<Guid, PipelinePartExecutionResult> ContextValues { get; init; }
}

public class ProcessesService(GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client)
  : IProcessesService
{
  private readonly Dictionary<string, ProcessData> myCurrentProcesses = [];
  

  public IReadOnlyObservableDictionary<Guid, CaseData.PipelinePartExecutionResult> CreateCaseValuesObservable(
    ProcessData processData, Case selectedCase)
  {
    return myCurrentProcesses[processData.ProcessName].ProcessCases[selectedCase.Name].ContextValues;
  }
  
  public async IAsyncEnumerable<ProcessUpdate> OpenCasesUpdatesStream([EnumeratorCancellation] CancellationToken token)
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
        CreatedAt = DateTime.Now,
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

      anyUpdates |= GetOrCreateProcessData(@case.ProcessCaseMetadata.ProcessName, out var processData);
      processData.ProcessCases[caseModel.Name] = new CaseData
      {
        Case = caseModel,
        ContextValues = new ObservableDictionary<Guid, CaseData.PipelinePartExecutionResult>(initialState)
      };
    }

    if (anyUpdates)
    {
      yield return new ProcessesListUpdate
      {
        Processes = myCurrentProcesses.Values.ToList()
      };
    }
  }

  private bool GetOrCreateProcessData(string processName, out ProcessData processData)
  {
    if (myCurrentProcesses.TryGetValue(processName, out processData))
    {
      return false;
    }

    processData = new ProcessData
    {
      ProcessName = processName,
      ProcessCases = []
    };

    myCurrentProcesses[processName] = processData;
    return true;
  }

  private bool GetOrCreateCaseData(ProcessData processData, string caseName, out CaseData caseData)
  {
    if (processData.ProcessCases.TryGetValue(caseName, out caseData))
    {
      return false;
    }

    caseData = new CaseData
    {
      Case = new Case
      {
        Name = caseName,
        CreatedAt = DateTime.Now,
      },
      ContextValues = []
    };

    processData.ProcessCases[caseName] = caseData;

    return true;
  }

  private IEnumerable<ProcessUpdate> ProcessCaseUpdate(GrpcKafkaUpdate delta)
  {
    var anyUpdates = GetOrCreateProcessData(delta.ProcessCaseMetadata.ProcessName, out var processData);
    anyUpdates |= GetOrCreateCaseData(processData, delta.ProcessCaseMetadata.CaseName, out _);

    if (anyUpdates)
    {
      yield return new ProcessesListUpdate
      {
        Processes = myCurrentProcesses.Values.ToList()
      };
    }

    yield return new ProcessContextValuesUpdate
    {
      CaseName = delta.ProcessCaseMetadata.CaseName,
      ProcessName = delta.ProcessCaseMetadata.ProcessName
    };
  }
}