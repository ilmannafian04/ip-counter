FROM ekidd/rust-musl-builder:stable as builder
COPY --chown=rust:rust . .
RUN ["cargo", "build", "--release"]

FROM alpine:3
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/ip-counter /ip-counter
CMD ["/ip-counter"]
