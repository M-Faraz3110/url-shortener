FROM --platform=linux/amd64 rust:1.86 as build

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /app
COPY . .
COPY .sqlx .sqlx

RUN cargo build --release

FROM --platform=linux/amd64 debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/url-shortener /usr/local/bin/url-shortener

RUN chmod +x /usr/local/bin/url-shortener

CMD ["/usr/local/bin/url-shortener"]
