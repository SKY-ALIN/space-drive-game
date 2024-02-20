FROM rust:1.73.0 as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY core core/
COPY server server/
COPY python python/

RUN cargo build --target x86_64-unknown-linux-musl --package space_drive_game_server --release

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/space_drive_game_server /bin/

EXPOSE 3333

CMD ["/bin/space_drive_game_server"]
