# マルチステージビルドを使用し、Rustのプログラムをビルドする
FROM rust:1.91-slim-bookworm AS builder
WORKDIR /app

# 以下の2行を追加
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY . .
RUN cargo build --release

# 不要なソフトウェアを同梱する必要がないので軽量なbookworm-slimイメージを使用
FROM debian:bookworm-slim
WORKDIR /app

# 後続の説明で使用するためユーザーを作成しておく
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

# 8080番ポートを開放しアプリケーションを起動
ENV PORT=8080
EXPOSE $PORT
ENTRYPOINT [ "./target/release/app" ]
