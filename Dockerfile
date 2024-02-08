FROM rust:1.75 as builder

COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./src /app/src
COPY ./.sqlx /app/.sqlx

WORKDIR /app

RUN cargo build --release

FROM ubuntu:23.10 as final

COPY --from=builder /app/target/release/rinha-backend-2024 /app/rinha-backend-2024

WORKDIR /app

CMD ["./rinha-backend-2024"]

EXPOSE 3000
