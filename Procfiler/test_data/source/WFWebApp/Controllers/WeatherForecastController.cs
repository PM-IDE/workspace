using Microsoft.AspNetCore.Mvc;
using WFWebApp.Services;

namespace WFWebApp.Controllers;

[ApiController]
[Route("[controller]")]
public class WeatherForecastController(ILogger<WeatherForecastController> logger, IWeatherService weatherService) : ControllerBase
{
  private static readonly string[] Summaries = new[]
  {
    "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy", "Hot", "Sweltering", "Scorching"
  };

  [HttpGet(Name = "GetWeatherForecast")]
  public async Task<IReadOnlyList<WeatherForecast>> Get()
  {
    logger.LogInformation("Received a new weather forecast request");
    var weatherForecast = await weatherService.GetWeather();
    
    foreach (var forecast in weatherForecast)
    {
      forecast.TemperatureC = Random.Shared.Next(1, 20);
      forecast.Summary = $"{forecast.TemperatureC}C";
    }

    return weatherForecast;
  }
}