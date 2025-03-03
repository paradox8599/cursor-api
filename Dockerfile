# builder
FROM rust:1.85.0-slim-bookworm AS builder

WORKDIR /app

COPY . .

RUN \
  # install deps for build
  apt-get update && \
  apt-get install -y --no-install-recommends \
  build-essential \
  protobuf-compiler \
  pkg-config \
  libssl-dev \
  nodejs \
  npm \
  openssl

# build
RUN RUSTFLAGS="-C link-arg=-s" cargo build --release && \
  cp target/release/cursor-api /app/cursor-api

# runner
FROM debian:bookworm-slim AS runner

ENV TZ=Australia/Sydney

WORKDIR /app

# install deps for running
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  tzdata \
  openssl \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/cursor-api .

ENV PORT=3000

EXPOSE ${PORT}

CMD ["./cursor-api"]

