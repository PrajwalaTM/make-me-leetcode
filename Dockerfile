FROM clux/muslrust:stable as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM alpine
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/make-me-leetcode .
ENTRYPOINT [ "/make-me-leetcode" ]
