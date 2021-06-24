FROM rust:latest

WORKDIR /app

COPY . .
RUN rustup default nightly
# RUN cargo build --release
RUN cargo build
RUN cargo add partiql-rs


# ENTRYPOINT /app/target/release/partiqlx
ENTRYPOINT /app/target/debug/partiqlx
