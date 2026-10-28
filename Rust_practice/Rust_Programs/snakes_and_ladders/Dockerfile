FROM rust:1.98-bookworm AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libx11-6 \
    libxi6 \
    libxkbcommon0 \
    libgl1 \
    libgl1-mesa-dri \
    libasound2 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/snake_and_ladders_graphical /usr/local/bin/snakes-and-ladders

ENTRYPOINT ["/usr/local/bin/snakes-and-ladders"]
