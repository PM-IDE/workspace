FROM --platform=linux/amd64 rust:1.75.0 as build

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt install protobuf-compiler -y

RUN apt-get update \
  && apt-get -y install build-essential \
  && apt-get install -y wget \
  && rm -rf /var/lib/apt/lists/* \
  && wget https://github.com/Kitware/CMake/releases/download/v3.24.1/cmake-3.24.1-Linux-x86_64.sh \
      -q -O /tmp/cmake-install.sh \
      && chmod u+x /tmp/cmake-install.sh \
      && mkdir /opt/cmake-3.24.1 \
      && /tmp/cmake-install.sh --skip-license --prefix=/opt/cmake-3.24.1 \
      && rm /tmp/cmake-install.sh \
      && ln -s /opt/cmake-3.24.1/bin/* /usr/local/bin

RUN apt-get update && apt-get -y install ninja-build

COPY ./Ficus ./pmide/ficus/
COPY ./bxes/ ./pmide/bxes/

ENTRYPOINT cargo test --manifest-path /pmide/ficus/src/rust/ficus/Cargo.toml --release