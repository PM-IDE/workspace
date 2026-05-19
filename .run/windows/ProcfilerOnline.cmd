set ProduceEventsToKafka=true
set ProduceBxesKafkaEvents=true
set ProduceGcEvents=true
set OnlineProcfilerSettings__KafkaSettings__TopicName=my-topic
set OnlineProcfilerSettings__KafkaSettings__BootstrapServers=localhost:9092

dotnet clean %PM_IDE_ROOT%/All.sln
dotnet build %PM_IDE_ROOT%/All.sln -c Release
dotnet build %PM_IDE_ROOT%/Procfiler/src/dotnet/ProcfilerLoggerProvider/ -c Release
dotnet build %PM_IDE_ROOT%/Procfiler/test_data/source/WFWebApp -c Release

%PM_IDE_ROOT%/Procfiler/src/dotnet/ProcfilerOnline/bin/Release/net10.0/ProcfilerOnline.exe procfiler-online collect-online^
 -dll-path D:\work\DPN-Soundness-Verification\DPN.VerificationApp\bin\Debug\net10.0-windows\DPNVerifier.Desktop.dll^
 --target-methods-regex^
 DPN\.Soundness^
 --methods-filter-regex^
 DPN\.Soundness^