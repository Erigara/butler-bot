FROM messense/rust-musl-cross:aarch64-musl as builder

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/butler-bot
COPY . .

RUN cargo build --release

FROM alpine:3.15

WORKDIR /usr/local/bin/butler-bot
COPY ./templates ./templates
COPY --from=builder /usr/src/butler-bot/target/aarch64-unknown-linux-musl/release/butler-bot .
CMD ["./butler-bot"]