FROM rust as builder

WORKDIR /app

COPY ./ /app

RUN cargo build --release

FROM debian

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/slack-cron /usr/bin/slack-cron

CMD ["/usr/bin/slack-cron"]
