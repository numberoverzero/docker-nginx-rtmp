FROM rust:1.72.1-alpine AS build
ARG PROJECT=nginx-rtmp-auth
RUN apk add musl-dev upx
RUN update-ca-certificates

WORKDIR /usr/src
RUN rustup update nightly && rustup default nightly && \
    rustup component add rust-src --toolchain nightly

RUN USER=root cargo new ${PROJECT}
WORKDIR /usr/src/${PROJECT}
COPY Cargo.toml Cargo.lock ./
RUN cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-musl --release

COPY src ./src
RUN cargo install --path .
RUN cp /usr/local/cargo/bin/${PROJECT} /entrypoint
RUN upx --best --lzma /entrypoint

FROM scratch
COPY --from=build /entrypoint /
USER 1000
ENTRYPOINT ["/entrypoint"]
