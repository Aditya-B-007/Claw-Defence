FROM rust:1.76 as builder
WORKDIR /usr/src/clawdefence
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/clawdefence/target/release/clawdefence /usr/local/bin/clawdefence
ENV HOST=0.0.0.0
ENV PORT=8080
EXPOSE 8080
CMD ["clawdefence"]