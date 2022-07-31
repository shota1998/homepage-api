FROM rust:1.62

RUN cargo install project_init
RUN cargo install diesel_cli --no-default-features --features postgres
RUN apt update && apt install -y rust-lldb 
CMD ["cargo", "run"]