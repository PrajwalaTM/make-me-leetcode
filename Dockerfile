FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR app

FROM chef as planner
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin make-me-leetcode

FROM debian:bullseye-slim AS runtime
WORKDIR make-me-leetcode
COPY --from=builder /app/target/release/make-me-leetcode /usr/local/bin
RUN \
  apt-get  update && \
   apt-get -y -qq install ca-certificates && \
      apt-get clean
ENTRYPOINT ["/usr/local/bin/make-me-leetcode"]


