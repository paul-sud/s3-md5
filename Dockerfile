FROM rust:1.52.1-slim-buster
WORKDIR /s3-md5
COPY . .
RUN cargo build --release
