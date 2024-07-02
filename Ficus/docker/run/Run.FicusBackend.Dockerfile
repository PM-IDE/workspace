FROM --platform=linux/amd64 rust:1.75.0 as build
EXPOSE 8080

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt install protobuf-compiler -y

COPY ./Ficus/src/rust/ ./pmide/ficus/src/rust/
COPY ./Ficus/protos/ ./pmide/ficus/protos/
COPY ./bxes/ ./pmide/bxes/

RUN cargo build --manifest-path /pmide/ficus/src/rust/ficus_backend/Cargo.toml --release

ENTRYPOINT ./pmide/ficus/src/rust/ficus_backend/target/release/ficus_backend