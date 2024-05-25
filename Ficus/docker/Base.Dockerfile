FROM --platform=linux/amd64 ubuntu:latest as system_setup

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt install software-properties-common -y
RUN add-apt-repository ppa:deadsnakes/ppa

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt install graphviz -y
RUN apt install curl -y
RUN apt install build-essential -y
RUN apt install protobuf-compiler -y
RUN apt-get -y install python3-pip
RUN apt install python3.10 -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV cargo="/root/.cargo/bin/cargo"
ENV rustup="/root/.cargo/bin/rustup"
ENV python="/usr/bin/python3.10"

RUN $rustup update 1.75.0
RUN $python -m pip install pytest

FROM system_setup as source_build

COPY ./Ficus/ ./pmide/ficus/
COPY ./bxes/ ./pmide/bxes/

RUN $python -m pip install --upgrade pip setuptools
RUN $python -m pip install -r /pmide/ficus/src/python/requirements.txt
RUN $cargo build --manifest-path /pmide/ficus/src/rust/ficus_backend/Cargo.toml --release