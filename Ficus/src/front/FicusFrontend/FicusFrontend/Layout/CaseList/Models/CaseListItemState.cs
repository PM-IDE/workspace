using FicusFrontend.Services;

namespace FicusFrontend.Layout.CaseList.Models;

public enum CaseListItemState
{
  New,
  UpdatesReceived,
  Seen
}

public class CaseListItemDto
{
  public required Case Case { get; init; }
  public CaseListItemState State { get; set; } = CaseListItemState.New;
}