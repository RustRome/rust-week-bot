FROM ekidd/rust-musl-builder:nightly
WORKDIR /home/rust/src/
COPY --chown=rust:rust Cargo.toml ./
RUN mkdir ./src/
RUN echo 'fn main() {}' >> ./src/main.rs
RUN cargo build
COPY src/* ./src/
RUN cargo build --release


FROM alpine:3.5
COPY --from=0 /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-week-bot /rust-week-bot
CMD ["/rust-week-bot"]
