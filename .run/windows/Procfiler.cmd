dotnet build %PM_IDE_ROOT%/All.sln

%PM_IDE_ROOT%/Procfiler/src/dotnet/Procfiler/bin/Release/net9.0/Procfiler.exe procfiler^
 collect-to-xes^
 -csproj^
 %PM_IDE_ROOT%\Procfiler\test_data\source\LOHAllocations\LOHAllocations.csproj^
 -o^
 %PM_IDE_OUTPUT_DIR%\log.xes^
 --providers^
 Gc^
 --log-serialization-format^
 xes^