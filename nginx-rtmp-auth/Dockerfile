FROM rust:1.72.1 AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

RUN USER=root cargo new nginx-rtmp-auth
WORKDIR /usr/src/nginx-rtmp-auth
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/nginx-rtmp-auth /
USER 1000
ENTRYPOINT ["/nginx-rtmp-auth"]