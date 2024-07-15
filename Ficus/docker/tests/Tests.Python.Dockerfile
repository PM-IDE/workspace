FROM --platform=linux/amd64 python:3.10 as system_setup

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt install graphviz -y
RUN apt install curl -y
RUN apt install build-essential -y
RUN apt install protobuf-compiler -y

RUN python -m pip install pytest

FROM system_setup as source_build

COPY ./Ficus/ ./pmide/ficus/

RUN python -m pip install --upgrade pip setuptools
RUN python -m pip install -r /pmide/ficus/src/python/requirements.txt

WORKDIR /pmide/ficus/src/python

ENTRYPOINT python -m pytest .