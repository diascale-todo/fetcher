FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["fetcher"]


# FROM docker.io/library/rust:latest AS builder
# 
# WORKDIR /usr/src/app
# COPY Cargo.toml Cargo.lock ./
# COPY src ./src
# 
# ENV DATABASE_URL=postgres://postgres:V5zrumHjB7@35.228.20.219:5432/postgres
# 
# RUN cargo build --release
# 
# 
# FROM docker.io/library/debian:bookworm-slim
# 
# WORKDIR /usr/src/app
# COPY --from=builder /usr/src/app/target/release/fetcher .
# 
# EXPOSE 3000
# 
# # CMD ["sleep 10000"]
# 
# CMD ["./fetcher"]
