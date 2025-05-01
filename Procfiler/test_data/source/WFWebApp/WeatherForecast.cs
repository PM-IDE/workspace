namespace WFWebApp;

public class WeatherForecast
{
  public DateOnly Date { get; set; }

  private int myTemperatureC;
  public int TemperatureC
  {
    get => myTemperatureC;
    set
    {
      if (value > 3)
      {
        throw new Exception();
      }

      myTemperatureC = value;
    }
  }

  public int TemperatureF => 32 + (int)(TemperatureC / 0.5556);

  public string? Summary { get; set; }
}