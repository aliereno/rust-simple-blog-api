FROM rustlang/rust:nightly

WORKDIR /usr/src/app
COPY ./.. .

RUN rustc --version
RUN rustup default nightly
RUN cargo install diesel_cli
RUN cargo install cargo-watch