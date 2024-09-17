using System.Runtime.CompilerServices;
using Ficus;
using Google.Protobuf.WellKnownTypes;

namespace FicusFrontend.Services.Cases;

public interface ICasesService
{
  IAsyncEnumerable<Models> OpenCasesUpdatesStream(CancellationToken token);
}

public class CasesService(GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client) : ICasesService
{
  private class CaseData
  {
    public required Case Case { get; init; }
    public required Dictionary<Guid, List<GrpcContextValueWithKeyName>> ContextValues { get; init; }
  }

  private readonly Dictionary<string, CaseData> myCurrentCases = [];


  public async IAsyncEnumerable<Models> OpenCasesUpdatesStream([EnumeratorCancellation] CancellationToken token)
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

  private IEnumerable<Models> ProcessInitialState(GrpcCurrentCasesResponse initialCases)
  {
    foreach (var @case in initialCases.Cases)
    {
      var caseModel = new Case
      {
        Name = @case.CaseName,
        CreatedAt = DateTime.Now,
      };

      myCurrentCases[caseModel.Name] = new CaseData
      {
        Case = caseModel,
        ContextValues = @case.ContextValues
          .Select(v =>
          {
            var id = Guid.Parse(v.PipelinePartInfo.Id.Guid);
            return (id, v.ContextValues.ToList());
          })
          .ToDictionary()
      };

      yield return new CasesListUpdate
      {
        Case = caseModel
      };
    }
  }

  private IEnumerable<Models> ProcessCaseUpdate(GrpcKafkaUpdate delta)
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

    var pipelinePartGuid = Guid.Parse(delta.PipelinePartGuid.Guid);

    caseData.ContextValues[pipelinePartGuid] = delta.ContextValues.ToList();

    yield return new CaseContextValuesUpdate
    {
      CaseName = caseName,
      PipelinePartGuid = pipelinePartGuid,
    };
  }
}