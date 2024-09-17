using Ficus;
using FicusFrontend;
using FicusFrontend.Services;
using Grpc.Net.Client;
using Grpc.Net.Client.Web;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddSingleton<ICasesService, CasesService>();

builder.Services.AddSingleton(_ =>
{
  var httpHandler = new GrpcWebHandler(GrpcWebMode.GrpcWebText, new HttpClientHandler());
  var channel = GrpcChannel.ForAddress("http://localhost:5122", new GrpcChannelOptions { HttpHandler = httpHandler });
  return new GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient(channel);
});

await builder.Build().RunAsync();