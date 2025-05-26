export ProduceEventsToKafka=true
export ProduceBxesKafkaEvents=true
export ProduceGcEvents=false
export OnlineProcfilerSettings__KafkaSettings__TopicName=my-topic
export OnlineProcfilerSettings__KafkaSettings__BootstrapServers=localhost:9092

dotnet build "$PM_IDE_ROOT"/Procfiler/test_data/source/WFWebApp/WFWebApp.csproj -c Release
dotnet build "$PM_IDE_ROOT"/All.sln -c Release

"$PM_IDE_ROOT"/Procfiler/src/dotnet/ProcfilerOnline/bin/Release/net9.0/ProcfilerOnline procfiler-online collect-online \
 -dll-path "$PM_IDE_ROOT"/Procfiler/test_data/source/WFWebApp/bin/Release/net9.0/WFWebApp.dll \
 --target-methods-regex \
 WFWebApp \
 --methods-filter-regex \
 WFWebApp