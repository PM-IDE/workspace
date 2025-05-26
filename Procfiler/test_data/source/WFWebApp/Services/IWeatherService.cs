namespace WFWebApp.Services;

public interface IWeatherService
{
  Task<IReadOnlyList<WeatherForecast>> GetWeather();
}

public class WeatherService(IWeatherRepository weatherRepository) : IWeatherService
{
  public async Task<IReadOnlyList<WeatherForecast>> GetWeather()
  {
    return await weatherRepository.GetWeather();
  }
}

public interface IWeatherRepository
{
  Task<IReadOnlyList<WeatherForecast>> GetWeather();
}

public class WeatherRepository(ILogger<WeatherRepository> logger) : IWeatherRepository
{
  public async Task<IReadOnlyList<WeatherForecast>> GetWeather()
  {
    logger.LogInformation("Starting getting weather forecasts");

    var result = new List<WeatherForecast>();
    for (var i = 0; i < 10; ++i)
    {
      var forecast = new WeatherForecast
      {
        Date = DateOnly.FromDateTime(DateTime.Now),
        TemperatureC = 1,
        Summary = "xd"
      };

      result.Add(forecast);

      logger.LogInformation(
        "Adding new weatherforecast to the list, temperature: {TempC}, {TempF}", forecast.TemperatureC, forecast.TemperatureF);
    }

    return result;
  }
}