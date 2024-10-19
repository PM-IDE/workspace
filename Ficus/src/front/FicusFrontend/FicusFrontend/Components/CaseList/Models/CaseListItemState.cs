using FicusFrontend.Services.Cases;

namespace FicusFrontend.Components.CaseList.Models;

public enum CaseListItemState
{
  New,
  UpdatesReceived,
  Seen
}

public class ProcessListItemDto
{
  public required ProcessData ProcessData { get; init; }
  public required List<CaseListItemDto> Cases { get; init; }
}

public class CaseListItemDto
{
  public required Case Case { get; init; }
  public CaseListItemState State { get; set; } = CaseListItemState.New;
}