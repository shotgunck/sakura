FROM rustlang/rust:nightly

ARG BOT_TOKEN
ARG JD_CLI_ID
ARG JD_CLI_SECRET
ARG PORT=8088

ENV PORT=$PORT
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/sakura
COPY . .

RUN cargo build --release

CMD ./target/release/sakura