using FicusDashboard.Services.Cases;

namespace FicusDashboard.Layout.Models;

public class ProcessCaseData
{
  public required ProcessData ProcessData { get; init; }
  public required Case Case { get; init; }


  public override int GetHashCode() => HashCode.Combine(ProcessData.ProcessName.GetHashCode(), Case.FullName.GetHashCode());

  public override bool Equals(object? obj)
  {
    if (obj is not ProcessCaseData other) return false;

    return other.ProcessData.ProcessName == ProcessData.ProcessName &&
           other.Case.FullName == Case.FullName;
  }
}