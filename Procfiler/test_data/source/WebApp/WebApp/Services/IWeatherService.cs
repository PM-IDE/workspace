namespace WebApp.Services;

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

public class WeatherRepository : IWeatherRepository
{
  public async Task<IReadOnlyList<WeatherForecast>> GetWeather()
  {
    var result = new List<WeatherForecast>();
    for (var i = 0; i < 10; ++i)
    {
      result.Add(new WeatherForecast
      {
        Date = DateOnly.FromDateTime(DateTime.Now),
        TemperatureC = 10,
        Summary = "xd"
      });

      await Task.Delay(500);
    }

    return result;
  }
}