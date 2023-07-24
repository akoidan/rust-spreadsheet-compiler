FROM rust:slim-buster as builder

WORKDIR /app
COPY . .
RUN cargo build --bin transaction_parser

CMD ["/app/target/debug/transaction_parser", "/app/assets/transactions.csv"]