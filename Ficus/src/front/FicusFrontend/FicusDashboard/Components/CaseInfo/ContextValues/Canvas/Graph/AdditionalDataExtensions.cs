using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph;

internal static class AdditionalDataExtensions
{
  public static bool IsRichUiAdditionalData(this GrpcNodeAdditionalData.DataOneofCase data) =>
    data is
      GrpcNodeAdditionalData.DataOneofCase.SoftwareData or
      GrpcNodeAdditionalData.DataOneofCase.MultithreadedFragment or
      GrpcNodeAdditionalData.DataOneofCase.PatternInfo;

  public static bool IsRichUiAdditionalData(this GrpcGraphEdgeAdditionalData.DataOneofCase data) =>
    data is GrpcGraphEdgeAdditionalData.DataOneofCase.SoftwareData;
}