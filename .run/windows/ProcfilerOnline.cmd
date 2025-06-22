set ProduceEventsToKafka=true
set ProduceBxesKafkaEvents=true
set ProduceGcEvents=false
set OnlineProcfilerSettings__KafkaSettings__TopicName=my-topic
set OnlineProcfilerSettings__KafkaSettings__BootstrapServers=localhost:9092

dotnet build %PM_IDE_ROOT%/All.sln -c Release
dotnet build %PM_IDE_ROOT%/Procfiler/src/dotnet/ProcfilerLoggerProvider/ -c Release

%PM_IDE_ROOT%/Procfiler/src/dotnet/ProcfilerOnline/bin/Release/net9.0/ProcfilerOnline.exe procfiler-online collect-online^
 -dll-path %PM_IDE_ROOT%\Procfiler\test_data\source\WFWebApp\bin\Release\net9.0\WFWebApp.dll^
 --target-methods-regex^
 WFWebApp^
 --methods-filter-regex^
 WFWebApp^