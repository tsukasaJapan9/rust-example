FROM rust:1.43
WORKDIR /todo
COPY Cargo.toml Cargo.toml

RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
COPY ./templates ./templates

RUN rm -f target/release/deps/todo*

RUN cargo build --release
RUN cargo intall --path
CMD ["todo"]