FROM golang

COPY ./Ficus/src/go/ ./

WORKDIR ./balancer
RUN go build

ENTRYPOINT ["./balancer"]
EXPOSE 8080