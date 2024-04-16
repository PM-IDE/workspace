FROM --platform=linux/amd64 ficus_base:latest

WORKDIR /pmide/ficus/src/python

ENTRYPOINT $python -m pytest .