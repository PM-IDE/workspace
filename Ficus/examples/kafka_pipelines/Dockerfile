FROM --platform=linux/amd64 python:3.10 as setup

ARG SCRIPT_FILE_NAME

RUN apt update -y && apt upgrade -y
RUN apt-get update -y
RUN apt install build-essential -y

WORKDIR /app

COPY ./Ficus/ ./

WORKDIR /app/src/python
RUN python -m pip install --upgrade pip setuptools
RUN python -m pip install -r requirements.txt
RUN python -m pip install .

WORKDIR /app

ENV SCRIPT_FILE_NAME_ENV=$SCRIPT_FILE_NAME
ENTRYPOINT python /app/examples/kafka_pipelines/$SCRIPT_FILE_NAME_ENV