using FrontendBackend.Integrations.Kafka;
using FrontendBackend.Integrations.Kafka.PipelineUpdates;
using FrontendBackend.Services;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<IPipelinePartsUpdatesConsumer, PipelinePartsUpdatesConsumer>();

builder.Services.AddGrpc();

var app = builder.Build();

app.MapGrpcService<PipelinePartsContextValuesService>();

app.Run();