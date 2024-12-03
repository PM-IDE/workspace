using Ficus;

namespace GrpcModels;

public static class GrpcModelsUtil
{
  public static GrpcGuid ToGrpcGuid(this Guid guid) => new() { Guid = guid.ToString() };
}