using Ficus;
using FicusFrontend;
using FicusFrontend.Services.Cases;
using Grpc.Net.Client;
using Grpc.Net.Client.Web;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Microsoft.Extensions.Options;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddSingleton<ICasesService, CasesService>();
builder.Services.Configure<ApplicationSettings>(builder.Configuration.GetSection(nameof(ApplicationSettings)));

builder.Services.AddSingleton(services =>
{
  var settings = services.GetRequiredService<IOptions<ApplicationSettings>>().Value;
  var httpHandler = new GrpcWebHandler(GrpcWebMode.GrpcWebText, new HttpClientHandler());

  var channel = GrpcChannel.ForAddress(settings.BackendUrl, new GrpcChannelOptions { HttpHandler = httpHandler });

  return new GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient(channel);
});

await builder.Build().RunAsync();