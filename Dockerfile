FROM rustlang/rust:nightly

WORKDIR /app
COPY . .

RUN cargo run --release

CMD ./target/release/sakura