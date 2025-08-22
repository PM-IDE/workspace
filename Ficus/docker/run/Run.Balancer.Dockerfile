FROM golang

COPY ./Ficus/src/go/balancer ./

RUN go build

ENTRYPOINT ["./balancer"]
EXPOSE 8080