FROM rust:1.65-slim as builder
WORKDIR /usr/src/help-bot
COPY . .
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/help-bot /usr/local/bin/help-bot
VOLUME "/persistent"
CMD help-bot
