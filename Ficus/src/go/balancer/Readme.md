﻿Protos generation command (in models folder):

protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative --proto_path=../../../../protos/  ../../../../protos/*.proto