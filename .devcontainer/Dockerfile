FROM rust:1.62

RUN cargo install project_init
RUN cargo install diesel_cli --no-default-features --features postgres
RUN apt update
RUN apt install -y rust-lldb 
RUN apt install -y git
RUN apt install -y vim