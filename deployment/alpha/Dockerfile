FROM rust:1.98-bookworm AS builder
WORKDIR /source
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --locked --release -p gameverse-server --bin gameverse-presence-server-m2

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home gameverse \
    && install -d -o gameverse -g gameverse /var/lib/gameverse/identity
COPY --from=builder /source/target/release/gameverse-presence-server-m2 /usr/local/bin/
COPY deployment/alpha/entrypoint.sh /usr/local/bin/gameverse-alpha
RUN chmod 0555 /usr/local/bin/gameverse-alpha /usr/local/bin/gameverse-presence-server-m2
USER gameverse
EXPOSE 30122/udp
HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 CMD ["/bin/sh", "-c", "grep -aq gameverse-presence-server-m2 /proc/1/cmdline"]
ENTRYPOINT ["/usr/local/bin/gameverse-alpha"]
