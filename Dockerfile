FROM rust:slim-buster

LABEL authors="akodain"

WORKDIR ~/app
COPY . .

RUN cargo build
ENTRYPOINT ["cargo", "run"]