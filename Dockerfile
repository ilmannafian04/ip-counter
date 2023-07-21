FROM rust:1.71.0-alpine3.18 as api-builder
WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:3.18
COPY --from=api-builder /app/target/release/ip-counter /usr/local/bin
CMD [ "ip-counter" ]
