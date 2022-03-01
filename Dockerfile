FROM rustlang/rust:nightly

ARG BOT_TOKEN
ARG JD_CLI_ID
ARG JD_CLI_SECRET
ARG LAVALINK_HOST
ARG LAVALINK_PASS
ARG SW
ARG PORT=5000

ENV PORT=$PORT
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/sakura
COPY . .

RUN apt-get -y update
RUN apt-get -y install libopus0
RUN cargo build --release

ENTRYPOINT ["./target/release/sakura"]