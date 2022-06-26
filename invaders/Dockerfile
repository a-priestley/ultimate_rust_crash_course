FROM rust:slim
WORKDIR /usr/src/invaders
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .
ENTRYPOINT ["invaders"]