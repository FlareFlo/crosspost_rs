FROM archlinux
RUN pacman -Syu --noconfirm rustup git base-devel
RUN pacman --noconfirm -Scc
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu
RUN git clone https://github.com/FlareFlo/crosspost_rs.git
WORKDIR ./crosspost_rs

RUN cargo fetch

COPY token.txt ./assets/token.txt
COPY whitelist.txt ./assets/whitelist.txt

RUN cargo build --release