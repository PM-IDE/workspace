using System.Runtime.CompilerServices;
using Ficus;
using Google.Protobuf.WellKnownTypes;

namespace FicusFrontend.Services;

public class Case
{
  public required string Name { get; init; }
  public required DateTime CreatedAt { get; init; }
}

public interface ICasesService
{
  IAsyncEnumerable<Case> OpenCasesStream(CancellationToken token);
}

public class CasesService(GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient client) : ICasesService
{
  public async IAsyncEnumerable<Case> OpenCasesStream([EnumeratorCancellation] CancellationToken token)
  {
    var reader = client.StartUpdatesStream(new Empty(), cancellationToken: token).ResponseStream;

    while (await reader.MoveNext(token))
    {
      yield return new Case
      {
        Name = "xd",
        CreatedAt = DateTime.Now,
      };
    }
  }
}