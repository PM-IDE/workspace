using System.Runtime.CompilerServices;
using Ficus;
using Google.Protobuf.WellKnownTypes;

namespace FicusFrontend.Services;

public class Case
{
  public required string Name { get; init; }
  public required DateTime CreatedAt { get; init; }


  public override int GetHashCode() => Name.GetHashCode();
  public override bool Equals(object? obj) => obj is Case { Name: var name } && name == Name;
}

public interface ICasesService
{
  IAsyncEnumerable<CaseUpdate> OpenCasesUpdatesStream(CancellationToken token);
}

public abstract class CaseUpdate;

public sealed class CasesListUpdate : CaseUpdate
{
  public required Case Case { get; init; }
}

public sealed class CaseContextValuesUpdate : CaseUpdate
{
  public required string CaseName { get; init; }
  public required Guid PipelinePartGuid { get; init; }
}

public class CasesService(GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client) : ICasesService
{
  private class CaseData
  {
    public required Case Case { get; init; }
    public required Dictionary<Guid, List<GrpcContextValueWithKeyName>> ContextValues { get; init; }
  }

  private readonly Dictionary<string, CaseData> myCache = [];


  public async IAsyncEnumerable<CaseUpdate> OpenCasesUpdatesStream([EnumeratorCancellation] CancellationToken token)
  {
    var reader = client.StartUpdatesStream(new Empty(), cancellationToken: token).ResponseStream;

    while (await reader.MoveNext(token))
    {
      switch (reader.Current.UpdateCase)
      {
        case GrpcPipelinePartUpdate.UpdateOneofCase.CurrentCases:
        {
          foreach (var @case in reader.Current.CurrentCases.Cases)
          {
            var caseModel = new Case
            {
              Name = @case.CaseName,
              CreatedAt = DateTime.Now,
            };

            myCache[caseModel.Name] = new CaseData
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

          break;
        }
        case GrpcPipelinePartUpdate.UpdateOneofCase.Delta:
        {
          var delta = reader.Current.Delta;

          var caseName = delta.CaseName;
          if (!myCache.TryGetValue(caseName, out var caseData))
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

            myCache[caseName] = caseData;
          }

          var pipelinePartGuid = Guid.Parse(delta.PipelinePartGuid.Guid);

          caseData.ContextValues[pipelinePartGuid] = delta.ContextValues.ToList();

          yield return new CaseContextValuesUpdate
          {
            CaseName = caseName,
            PipelinePartGuid = pipelinePartGuid,
          };

          break;
        }
        case GrpcPipelinePartUpdate.UpdateOneofCase.None:
        default:
          throw new ArgumentOutOfRangeException();
      }
    }
  }
}