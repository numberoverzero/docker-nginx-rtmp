FROM rust:1.72.1 AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

RUN USER=root cargo new micro-rtmp-auth
WORKDIR /usr/src/micro-rtmp-auth
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/micro-rtmp-auth /
USER 1000
ENTRYPOINT ["/micro-rtmp-auth"]