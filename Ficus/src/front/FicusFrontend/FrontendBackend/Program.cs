using FrontendBackend.Features.PipelineUpdates.BackgroundJobs;
using FrontendBackend.Features.PipelineUpdates.Kafka.PipelineUpdates;
using FrontendBackend.Features.PipelineUpdates.Services;
using FrontendBackend.Features.PipelineUpdates.Settings;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<IPipelinePartsUpdatesConsumer, PipelinePartsUpdatesConsumer>();
builder.Services.AddHostedService<UpdatesConsumerJob>();

var section = builder.Configuration.GetSection(nameof(PipelinePartsUpdateKafkaSettings));
builder.Services.Configure<PipelinePartsUpdateKafkaSettings>(section);

builder.Services.AddGrpc();

var app = builder.Build();

app.MapGrpcService<PipelinePartsContextValuesService>();

app.Run();