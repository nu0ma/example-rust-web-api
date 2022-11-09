FROM rust:1.61.0 as builder

WORKDIR /app
COPY  . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo build --release && \
  strip /app/target/release/app 

FROM debian:10.4
COPY --from=builder /app/target/release/app /usr/local/bin/
CMD ["app"]
