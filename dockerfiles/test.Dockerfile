FROM rust:latest


WORKDIR /usr/src/rusty_borg

RUN apt-get update && apt-get install -y \
  build-essential borgbackup libssh-dev openssh-client

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src/ src/

RUN cargo build

CMD [ "cargo", "test" ]
