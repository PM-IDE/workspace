dotnet build %PM_IDE_ROOT%/All.sln

%PM_IDE_ROOT%/Procfiler/src/dotnet/ProcfilerOnline/bin/Release/net9.0/ProcfilerOnline.exe procfiler-online collect-online^
 -dll-path %PM_IDE_ROOT%\Procfiler\test_data\source\WFWebApp\WFWebApp\bin\Release\net9.0\WFWebApp.dll^
 --target-methods-regex^
 WFWebApp^
 --methods-filter-regex^
 WFWebApp^