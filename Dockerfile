FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xzf - -C /usr/local/cargo/bin
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

RUN dx build --package ui --release --fullstack --force-sequential

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/dx/ui/release/web/ui ./ui
COPY --from=builder /app/target/dx/ui/release/web/public ./public
RUN mkdir -p crates/infra/src/config/toml data
ENTRYPOINT ["./ui"]
