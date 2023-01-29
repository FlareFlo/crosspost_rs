FROM archlinux:latest
RUN pacman -Syu --noconfirm git base-devel rust
RUN git clone https://github.com/FlareFlo/crosspost_rs.git
WORKDIR ./crosspost_rs

RUN cargo fetch

COPY token.txt ./assets/token.txt
COPY whitelist.txt ./assets/whitelist.txt

RUN cargo build --release

RUN mv target/release/crosspost_rs crosspost_rs
RUN rm -rf target
RUN pacman --noconfirm -Rns rust
RUN pacman --noconfirm -Scc

RUN ./crosspost_rs