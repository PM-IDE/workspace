FROM golang

COPY ./Ficus/src/go/ ./

WORKDIR ./integrationtests

CMD go test -v
EXPOSE 8080