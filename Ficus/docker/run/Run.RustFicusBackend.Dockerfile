FROM --platform=linux/amd64 rust:1.75.0 as build

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt install protobuf-compiler -y

RUN apt-get -y install ninja-build
RUN apt-get -y install cmake

COPY ./Ficus/src/rust/ ./pmide/ficus/src/rust/
COPY ./Ficus/protos/ ./pmide/ficus/protos/
COPY ./bxes/ ./pmide/bxes/

RUN cargo build --manifest-path /pmide/ficus/src/rust/Cargo.toml --release

FROM --platform=linux/amd64 gcr.io/distroless/cc as run
EXPOSE 8080

WORKDIR /app

COPY --from=build /pmide/ficus/src/rust/target/release/ ./
COPY --from=build /lib/*-linux-gnu /lib/

ENTRYPOINT ["/app/ficus_backend"]