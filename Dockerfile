FROM rust:1.41.1

WORKDIR /usr/src/seer
COPY . .

RUN cargo install --path .

ENTRYPOINT ["seer"]
CMD []