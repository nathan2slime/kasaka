FROM rust:1.83.0-slim-bullseye AS build

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    curl \
    build-essential

WORKDIR /build

COPY . ./

RUN rustup target add wasm32-unknown-unknown 
RUN cargo install trunk
RUN trunk build --release

FROM debian:bullseye-slim AS final

RUN apt-get update && apt-get install -y \
    npm \
    && npm install -g http-server

COPY --from=build /build/dist /app

CMD ["sh", "-c", "http-server -p $PORT /app"]
