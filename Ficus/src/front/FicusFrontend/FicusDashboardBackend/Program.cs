using FicusDashboardBackend.Features.PipelineUpdates.BackgroundJobs;
using FicusDashboardBackend.Features.PipelineUpdates.Services;
using FicusKafkaIntegration;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<IPipelinePartsUpdatesConsumer, PipelinePartsUpdatesConsumer>();
builder.Services.AddSingleton<IPipelinePartsUpdatesRepository, PipelinePartsUpdatesRepository>();
builder.Services.AddHostedService<UpdatesConsumerJob>();

var section = builder.Configuration.GetSection(nameof(PipelinePartsUpdateKafkaSettings));
builder.Services.Configure<PipelinePartsUpdateKafkaSettings>(section);

builder.Services.AddGrpc();

const string CorsPolicyName = nameof(CorsPolicyName);
builder.Services.AddCors(options =>
{
  options.AddPolicy(CorsPolicyName, builder =>
  {
    builder.AllowAnyOrigin()
      .AllowAnyMethod()
      .AllowAnyHeader()
      .WithExposedHeaders("Grpc-Status", "Grpc-Message", "Grpc-Encoding", "Grpc-Accept-Encoding");
  });
});

var app = builder.Build();

app.UseGrpcWeb();
app.UseCors(CorsPolicyName);

app.MapGrpcService<PipelinePartsContextValuesService>().EnableGrpcWeb().RequireCors(CorsPolicyName);

app.Run();