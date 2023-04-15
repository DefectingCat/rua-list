FROM rust:alpine as builder

WORKDIR /app

# Create appuser
ENV USER=xfy
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}" \
    && mkdir $HOME/.cargo \
    && echo "[source.crates-io]" >> $HOME/.cargo/config \
    && echo "replace-with = 'ustc'" >> $HOME/.cargo/config \
    && echo "" >> $HOME/.cargo/config \
    && echo "[source.ustc]" >> $HOME/.cargo/config \
    && echo "registry = \"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\"" >> $HOME/.cargo/config \
    && rustup target add x86_64-unknown-linux-musl \
    &&  update-ca-certificates

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rua-list /
COPY --from=builder /app/config.json /
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER xfy

CMD [ "/rua-list", "-c", "/config.json" ]