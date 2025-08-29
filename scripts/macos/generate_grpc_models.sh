cd ../../Ficus/src/python/infra
/usr/local/bin/python3.10 generate_models.py ficus.grpc_pipelines ../ficus/grpc_pipelines/ ../../../protos

export GOPATH=$HOME/go
export PATH=$PATH:$GOPATH/bin
cd ../../go/grpcmodels
protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative --proto_path=../../../protos/  ../../../protos/*.proto

cd ../../front/FicusFrontend/FicusDashboard/Npm
sudo node ./node_modules/.bin/proto-loader-gen-types --longs=Number --enums=number --defaults --oneofs --grpcLib=@grpc/grpc-js --outDir=./src/protos/ ../../../../../protos/*.proto --inputTemplate "%s_DONTUSE" --outputTemplate "%s"

