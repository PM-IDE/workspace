FROM python:3.10

WORKDIR /app

COPY ./Ficus/src/python ./
RUN python -m pip install -r requirements.txt
RUN python -m pip install .

ENTRYPOINT [ "python", "/app/examples/kafka_pipelines/kafka_pipeline.py" ]