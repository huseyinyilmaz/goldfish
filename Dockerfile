FROM rust:latest AS cargo-build

RUN apt-get update
RUN apt-get install musl-dev musl-tools -y
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/goldfish
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

ENV GOLDFISH_LOG_LEVEL=info

WORKDIR /usr/local/goldfish

COPY --from=cargo-build /usr/src/goldfish/target/x86_64-unknown-linux-musl/release/goldfish .

CMD ["/usr/local/goldfish/goldfish"]
