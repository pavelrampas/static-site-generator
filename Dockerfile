FROM rust:latest

COPY ./src /usr/src/myapp/src
COPY ./Cargo.lock /usr/src/myapp/Cargo.lock
COPY ./Cargo.toml /usr/src/myapp/Cargo.toml

WORKDIR /usr/src/myapp

RUN apt update && apt install -y rsync
RUN cargo build
