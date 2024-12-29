FROM rust:latest

WORKDIR /usr/src/rust_website
COPY . .
RUN cargo build --release

CMD ["cargo", "run", "--release"]