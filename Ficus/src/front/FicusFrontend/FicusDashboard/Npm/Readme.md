Generaing protos:

sudo node ./node_modules/.bin/proto-loader-gen-types --longs=Number --enums=String --defaults --oneofs --grpcLib=@grpc/grpc-js --outDir=./src/protos/ /Users/aero/work/workspace/Ficus/protos/*.proto --inputTemplate "%s_DONTUSE" --outputTemplate "%s"