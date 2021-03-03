FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/idkhtnb
COPY . .
RUN cargo run install --path .

FROM debian:buster-slim
RUN apt update && apt install -y 
