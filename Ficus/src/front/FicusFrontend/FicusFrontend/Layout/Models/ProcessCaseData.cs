using FicusFrontend.Services.Cases;

namespace FicusFrontend.Layout.Models;

public class ProcessCaseData
{
  public required ProcessData ProcessData { get; init; }
  public required Case Case { get; init; }


  public override int GetHashCode() => HashCode.Combine(ProcessData.ProcessName.GetHashCode(), Case.Name.GetHashCode());

  public override bool Equals(object? obj)
  {
    if (obj is not ProcessCaseData other) return false;

    return other.ProcessData.ProcessName == ProcessData.ProcessName &&
           other.Case.Name == Case.Name;
  }
}