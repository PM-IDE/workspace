using FrontendBackend.Services;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddGrpc();

var app = builder.Build();

app.MapGrpcService<PipelinePartsContextValuesService>();

app.Run();