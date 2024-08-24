namespace Core.Utils;

public class PercentValue
{
  private int AllCases { get; set; }
  private int SuitableCases { get; set; }

  public double Percent => (double)SuitableCases / AllCases;

  public void AddCase(bool suitable)
  {
    if (suitable)
    {
      ++SuitableCases;
    }

    ++AllCases;
  }
}