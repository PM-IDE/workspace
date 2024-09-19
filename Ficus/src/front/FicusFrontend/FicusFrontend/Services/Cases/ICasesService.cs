using System.Runtime.CompilerServices;
using Ficus;
using Google.Protobuf.WellKnownTypes;
using ObservableCollections;

namespace FicusFrontend.Services.Cases;

public interface ICasesService
{
  IObservableCollection<Case> Cases { get; }

  IReadOnlyObservableDictionary<Guid, CaseData.PipelinePartExecutionResult> CreateCaseValuesObservable(Case selectedCase);
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

public class CasesService : ICasesService
{
  private readonly ObservableList<Case> myLiveCases = [];
  private readonly Dictionary<string, CaseData> myCurrentCases = [];
  private readonly GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient _client;


  public IObservableCollection<Case> Cases => myLiveCases;


  public CasesService(GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client)
  {
    _client = client;

    Task.Factory.StartNew(StartProcessingUpdates, TaskCreationOptions.LongRunning);
  }

  
  
  public IReadOnlyObservableDictionary<Guid, CaseData.PipelinePartExecutionResult> CreateCaseValuesObservable(Case selectedCase)
  {
    return myCurrentCases[selectedCase.Name].ContextValues;
  }

  private async Task StartProcessingUpdates()
  {
    await foreach (var update in OpenCasesUpdatesStream(CancellationToken.None))
    {
      switch (update)
      {
        case CasesListUpdate casesListUpdate:
        {
          myLiveCases.Add(casesListUpdate.Case);
          break;
        }
        case CaseContextValuesUpdate caseContextValuesUpdate:
        {
          var caseData = myCurrentCases[caseContextValuesUpdate.CaseName];
          caseData.ContextValues[caseContextValuesUpdate.PipelinePartGuid] = new CaseData.PipelinePartExecutionResult
          {
            ContextValues = caseContextValuesUpdate.NewContextValues,
            PipelinePartName = caseContextValuesUpdate.PipelinePartName
          };

          break;
        }
        default:
          throw new ArgumentOutOfRangeException(nameof(update));
      }
    }
  }
  
  private async IAsyncEnumerable<CaseUpdate> OpenCasesUpdatesStream([EnumeratorCancellation] CancellationToken token)
  {
    var reader = _client.StartUpdatesStream(new Empty(), cancellationToken: token).ResponseStream;

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

  private IEnumerable<CaseUpdate> ProcessInitialState(GrpcCurrentCasesResponse initialCases)
  {
    foreach (var @case in initialCases.Cases)
    {
      var caseModel = new Case
      {
        Name = @case.CaseName,
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

      myCurrentCases[caseModel.Name] = new CaseData
      {
        Case = caseModel,
        ContextValues = new ObservableDictionary<Guid, CaseData.PipelinePartExecutionResult>(initialState)
      };

      yield return new CasesListUpdate
      {
        Case = caseModel
      };
    }
  }

  private IEnumerable<CaseUpdate> ProcessCaseUpdate(GrpcKafkaUpdate delta)
  {
    var caseName = delta.CaseName;
    if (!myCurrentCases.TryGetValue(caseName, out var caseData))
    {
      caseData = new CaseData
      {
        Case = new Case
        {
          Name = caseName,
          CreatedAt = DateTime.Now,
        },
        ContextValues = []
      };

      yield return new CasesListUpdate
      {
        Case = caseData.Case
      };

      myCurrentCases[caseName] = caseData;
    }

    yield return new CaseContextValuesUpdate
    {
      CaseName = caseName,
      PipelinePartName = delta.PipelinePartInfo.Name,
      PipelinePartGuid = Guid.Parse(delta.PipelinePartInfo.Id.Guid),
      NewContextValues = delta.ContextValues.ToList()
    };
  }
}