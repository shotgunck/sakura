FROM rustlang/rust:nightly

WORKDIR /usr/src/sakura
COPY . .

RUN cargo run --release

CMD ./target/release/sakura