FROM rust:1.67

WORKDIR /app
RUN cargo install cargo-watch

COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build

EXPOSE 2001
COPY ./.env /app/.env
COPY ./diesel.toml /app/diesel.toml

CMD ["cargo", "watch", "-x", "run", "--clear"]
# CMD ["cargo", "run"]