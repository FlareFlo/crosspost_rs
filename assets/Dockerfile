FROM docker.io/clux/muslrust:stable as builder
RUN rustup install stable --profile minimal
WORKDIR /build
RUN git clone https://github.com/FlareFlo/crosspost_rs.git
WORKDIR /build/crosspost_rs

COPY token.txt ./assets/token.txt
COPY whitelist.txt ./assets/whitelist.txt

RUN cargo build --release

FROM alpine
WORKDIR /running
COPY --from=builder /build/crosspost_rs/target/x86_64-unknown-linux-musl/release/crosspost_rs .

ENV UPTIME_URL=""

CMD ./crosspost_rs 1