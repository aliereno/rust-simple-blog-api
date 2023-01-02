FROM rust:1.66.0

WORKDIR /usr/src/app

RUN apt-get update -qq && \
    apt-get install -y libpq-dev libsqlite3-dev default-libmysqlclient-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo fetch

COPY ./.. .

RUN rustc --version
RUN cargo install diesel_cli
RUN cargo install cargo-watch
