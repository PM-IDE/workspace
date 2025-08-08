Generating protos:

macOS
sudo node ./node_modules/.bin/proto-loader-gen-types --longs=Number --enums=number --defaults --oneofs --grpcLib=@grpc/grpc-js --outDir=./src/protos/ ../../../../../protos/*.proto --inputTemplate "%s_DONTUSE" --outputTemplate "%s"

Windows
./node_modules/.bin/proto-loader-gen-types.ps1 --longs=Number --enums=number --defaults --oneofs --grpcLib=@grpc/grpc-js --outDir=./src/protos/ ../../../../../protos/pipelines_and_context.proto --inputTemplate "%s_DONTUSE" --outputTemplate "%s"