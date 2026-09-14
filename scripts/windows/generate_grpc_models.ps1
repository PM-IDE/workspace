$ErrorActionPreference = "Stop"

Push-Location "$PSScriptRoot/../../Ficus/src/python/infra"
python generate_models.py ficus.grpc_pipelines ../ficus/grpc_pipelines/ ../../../protos
Pop-Location

$env:GOPATH = "$HOME/go"
$env:PATH = "$env:PATH;$env:GOPATH/bin"

Push-Location "$PSScriptRoot/../../Ficus/src/go/grpcmodels"
protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative --proto_path=../../../protos/ ../../../protos/*.proto
Pop-Location

Push-Location "$PSScriptRoot/../../Ficus/src/front/FicusFrontend/FicusDashboard/Npm"
Get-ChildItem "../../../../../protos/*.proto" | ForEach-Object {
    & .\node_modules\.bin\proto-loader-gen-types.ps1 --longs=Number --enums=number --defaults --oneofs --grpcLib=@grpc/grpc-js --outDir=./src/protos/ "../../../../../protos/$($_.Name)" --inputTemplate "%s_DONTUSE" --outputTemplate "%s"
}
Pop-Location