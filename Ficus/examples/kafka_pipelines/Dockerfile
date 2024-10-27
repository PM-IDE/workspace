FROM --platform=linux/amd64 python:3.10 as setup

RUN apt update -y && apt upgrade -y
RUN apt-get update -y
RUN apt install build-essential -y

FROM setup as launch

WORKDIR /app

COPY ./Ficus/ ./

WORKDIR /app/src/python
RUN python -m pip install --upgrade pip setuptools
RUN python -m pip install -r requirements.txt
RUN python -m pip install .

WORKDIR /app

ENTRYPOINT [ "python", "/app/examples/kafka_pipelines/kafka_pipeline.py" ]