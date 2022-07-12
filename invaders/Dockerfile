FROM rust:slim-bullseye AS build
WORKDIR /usr/src/invaders
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/invaders /usr/local/bin/invaders
ENTRYPOINT ["invaders"]