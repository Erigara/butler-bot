FROM alpine:3.15

WORKDIR /usr/local/bin/butler-bot
COPY ./templates ./templates
COPY ./target/aarch64-unknown-linux-musl/release/butler-bot ./
CMD ["./butler-bot"]