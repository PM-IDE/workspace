using Ficus;
using FicusDashboard;
using FicusDashboard.Services.Cases;
using Grpc.Net.Client;
using Grpc.Net.Client.Web;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Microsoft.Extensions.Options;
using Radzen;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddSingleton<ISubscriptionsService, SubscriptionsService>();
builder.Services.Configure<ApplicationSettings>(builder.Configuration.GetSection(nameof(ApplicationSettings)));
builder.Services.AddBlazorBootstrap();
builder.Services.AddRadzenComponents();

builder.Services.AddSingleton(services =>
{
  var settings = services.GetRequiredService<IOptions<ApplicationSettings>>().Value;
  var httpHandler = new GrpcWebHandler(GrpcWebMode.GrpcWebText, new HttpClientHandler());

  var channel = GrpcChannel.ForAddress(settings.BackendUrl, new GrpcChannelOptions
  {
    HttpHandler = httpHandler,
    MaxReceiveMessageSize = 512 * 1024 * 1024
  });

  return new GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient(channel);
});

var app = builder.Build();

using var source = new CancellationTokenSource();

try
{
  app.Services.GetRequiredService<ISubscriptionsService>().StartUpdatesStream(source.Token);
  await app.RunAsync();
}
finally
{
  source.Cancel();
}