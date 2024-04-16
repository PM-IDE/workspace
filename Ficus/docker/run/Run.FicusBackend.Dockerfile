FROM --platform=linux/amd64 ficus_base:latest
EXPOSE 8080

ENTRYPOINT /pmide/ficus/src/rust/ficus_backend/target/release/ficus_backend