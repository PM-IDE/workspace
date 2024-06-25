FROM --platform=linux/amd64 ficus_python_client:latest

WORKDIR /pmide/ficus/src/python

ENTRYPOINT python -m pytest .