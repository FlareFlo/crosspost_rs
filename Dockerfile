FROM docker.io/clux/muslrust:stable as builder
RUN rustup install stable --profile minimal
WORKDIR /build
COPY . ./
RUN ls

COPY ./assets/token.txt ./assets/token.txt
COPY ./assets/whitelist.txt ./assets/whitelist.txt

RUN cargo build --release

FROM alpine
RUN apk add curl
WORKDIR /running
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/crosspost_rs .

ENV UPTIME_URL=""

CMD ./crosspost_rs