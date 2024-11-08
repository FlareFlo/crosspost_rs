FROM rust:1-slim-bookworm as builder
WORKDIR /build
COPY . ./
RUN ls

COPY ./assets/token.txt ./assets/token.txt
COPY ./assets/whitelist.txt ./assets/whitelist.txt

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /running
COPY --from=builder /build/target/x86_64-unknown-linux/release/crosspost_rs .

ENV UPTIME_URL=""

CMD ./crosspost_rs