FROM rustlang/rust:nightly

ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/sakura
COPY . .

RUN cargo run --release

CMD ./target/release/sakura