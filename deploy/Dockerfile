FROM rust:1.62

RUN apt-get update -yqq && apt-get install -yqq cmake g++
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./target/release ./workspace
WORKDIR ./workspace

RUN cargo build

EXPOSE 80001

CMD ["cargo", "run"]