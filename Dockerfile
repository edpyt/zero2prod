FROM rust:1.82.0-slim-bookworm

WORKDIR /app

RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

ENTRYPOINT [ "./target/release/zero2prod" ]
