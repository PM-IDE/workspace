using System.Buffers;
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

    CreateThread();
    foreach (var forecast in weatherForecast)
    {
      var client = new HttpClient();
      await client.GetAsync("https://google.com");
      await client.GetAsync("https://ya.ru");

      CreateThread();

      var array = ArrayPool<byte[]>.Shared.Rent(Random.Shared.Next(2000));

      try
      {
        forecast.TemperatureC = Random.Shared.Next(1, 20);
        forecast.Summary = $"{forecast.TemperatureC}C";
      }
      catch (Exception ex)
      {
        logger.LogError(ex, "Failed to update forecast");
      }

      ArrayPool<byte[]>.Shared.Return(array);
    }

    return weatherForecast;
  }

  private void CreateThread()
  {
    var thread = new Thread(() => { logger.LogInformation("xd"); });
    thread.Start();
    thread.Join();
  }
}