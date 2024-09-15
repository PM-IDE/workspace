using Ficus;
using FicusFrontend;
using FicusFrontend.Services;
using Grpc.Net.Client;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddSingleton<ICasesService, CasesService>();

builder.Services.AddSingleton(_ =>
{
  var channel = GrpcChannel.ForAddress("localhost:5122");
  return new GrpcPipelinePartsContextValuesService.GrpcPipelinePartsContextValuesServiceClient(channel);
});

await builder.Build().RunAsync();