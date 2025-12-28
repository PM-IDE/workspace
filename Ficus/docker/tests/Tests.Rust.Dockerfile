FROM rust:1.92.0 as build

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt install protobuf-compiler -y

RUN apt-get -y install cmake
RUN apt-get -y install ninja-build

COPY ./Ficus ./pmide/ficus/
COPY ./bxes/ ./pmide/bxes/

ENTRYPOINT exec cargo test --manifest-path /pmide/ficus/src/rust/ficus/Cargo.toml --release